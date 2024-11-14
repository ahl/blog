---
title: "Tuning the OpenZFS write throttle"
date: "2014-08-31"
categories: 
  - "zfs"
tags: 
  - "dtrace"
  - "mattahrens"
  - "openzfs"
  - "performance"
  - "tuning"
  - "zfs"
---

[![](images/Magnetic-Fun-and-Facts-300x207.jpg "Magnetic-Fun-and-Facts")](http://ahl.dtrace.org/wp-content/uploads/2014/08/Magnetic-Fun-and-Facts.jpg)In previous posts I discussed [the problems with the legacy ZFS write throttle](http://dtrace.org/blogs/ahl/2013/12/27/zfs-fundamentals-the-write-throttle/) that cause degraded performance and wildly variable latencies. I then presented the [new OpenZFS write throttle and I/O scheduler](http://dtrace.org/blogs/ahl/2014/02/10/the-openzfs-write-throttle/) that Matt Ahrens and I designed. In addition to solving several problems in ZFS, the new approach was designed to be easy to reason about, measure, and adjust. In this post I’ll cover performance analysis and tuning — using DTrace of course. These details are intended for those using OpenZFS and trying to optimize performance — if you have only a casual interest in ZFS consider yourself warned!

## Buffering dirty data

OpenZFS limits the amount of dirty data on the system according to the tunable zfs\_dirty\_data\_max. It’s default value is 10% of memory up to 4GB. The tradeoffs are pretty simple:

<table><tbody><tr><td><strong>Lower</strong></td><td><strong>Higher</strong></td></tr><tr><td>Less memory reserved for use by OpenZFS</td><td>More memory reserved for use by OpenZFS</td></tr><tr><td>Able to absorb less workload variation before throttling</td><td>Able to absorb more workload variation before throttling</td></tr><tr><td>Less data in each transaction group</td><td>More data in each transaction group</td></tr><tr><td>Less time spent syncing out each transaction group</td><td>More time spent syncing out each transaction group</td></tr><tr><td>More metadata written due to less amortization</td><td>Less metadata written due to more amortization</td></tr></tbody></table>

 

Most workloads contain variability. Think of the dirty data as a buffer for that variability. Let’s say the LUNs assigned to your OpenZFS storage pool are able to sustain 100MB/s in aggregate. If a workload consistently writes at 100MB/s then only a very small buffer would be required. If instead the workload oscillates between 200MB/s and 0MB/s for 10 seconds each, then a small buffer would limit performance. A buffer of 800MB would be large enough to absorb the full 20 second cycle over which the average is 100MB/s. A buffer of only 200MB would cause OpenZFS to start to throttle writes — inserting artificial delays — after less than 2 seconds during which the LUNs could flush 200MB of dirty data while the client tried to generate 400MB.

Track the amount of outstanding dirty data within your storage pool to know which way to adjust zfs\_dirty\_data\_max:

```
txg-syncing
{
        this->dp = (dsl_pool_t *)arg0;
}

txg-syncing
/this->dp->dp_spa->spa_name == $$1/
{
        printf("%4dMB of %4dMB used", this->dp->dp_dirty_total / 1024 / 1024,
            `zfs_dirty_data_max / 1024 / 1024);
}

# dtrace -s dirty.d pool
dtrace: script 'dirty.d' matched 2 probes
CPU ID FUNCTION:NAME
11 8730 txg_sync_thread:txg-syncing 966MB of 4096MB used
0 8730 txg_sync_thread:txg-syncing 774MB of 4096MB used
10 8730 txg_sync_thread:txg-syncing 954MB of 4096MB used
0 8730 txg_sync_thread:txg-syncing 888MB of 4096MB used
0 8730 txg_sync_thread:txg-syncing 858MB of 4096MB used
```

The write throttle kicks in once the amount of dirty data exceeds zfs\_delay\_min\_dirty\_percent of the limit (60% by default). If the the amount of dirty data fluctuates above and below that threshold, it might be possible to avoid throttling by increasing the size of the buffer. If the metric stays low, you may reduce zfs\_dirty\_data\_max. Weigh this tuning against other uses of memory on the system (a larger value means that there’s less memory for applications or the OpenZFS ARC for example).

A larger buffer also means that flushing a transaction group will take longer. This is relevant for certain OpenZFS administrative operations (sync tasks) that occur when a transaction group is committed to stable storage such as creating or cloning a new dataset. If the interactive latency of these commands is important, consider how long it would take to flush zfs\_dirty\_data\_max bytes to disk. You can measure the time to sync transaction groups ([recall, there are up to three active at any given time](http://dtrace.org/blogs/ahl/2012/12/13/zfs-fundamentals-transaction-groups/)) like this:

```
txg-syncing
/((dsl_pool_t *)arg0)->dp_spa->spa_name == $$1/
{
        start = timestamp;
}

txg-synced
/start && ((dsl_pool_t *)arg0)->dp_spa->spa_name == $$1/
{
        this->d = timestamp - start;
        printf("sync took %d.%02d seconds", this->d / 1000000000,
            this->d / 10000000 % 100);
}

# dtrace -s duration.d pool
dtrace: script 'duration.d' matched 2 probes
CPU ID FUNCTION:NAME
5 8729 txg_sync_thread:txg-synced sync took 5.86 seconds
2 8729 txg_sync_thread:txg-synced sync took 6.85 seconds
11 8729 txg_sync_thread:txg-synced sync took 6.25 seconds
1 8729 txg_sync_thread:txg-synced sync took 6.32 seconds
11 8729 txg_sync_thread:txg-synced sync took 7.20 seconds
1 8729 txg_sync_thread:txg-synced sync took 5.14 seconds
```

Note that the value of zfs\_dirty\_data\_max is relevant when sizing a separate intent log device (SLOG). zfs\_dirty\_data\_max puts a hard limit on the amount of data in memory that has yet been written to the main pool; at most, that much data is active on the SLOG at any given time. This is why small, fast devices such as the [DDRDrive](http://www.ddrdrive.com/) make for [great log devices](http://dtrace.org/blogs/ahl/2010/07/19/ddrdrive/). As an aside, consider the [ostensible upgrade](https://blogs.oracle.com/7000tips/entry/new_logzilla_announced_today) that Oracle brought to the ZFS Storage Appliance a few years ago replacing the 18GB “Logzilla” with a 73GB upgrade.

## I/O scheduler

Where ZFS had a single IO queue for all IO types, OpenZFS has five IO queues for each of the different IO types: sync reads (for normal, demand reads), async reads (issued from the prefetcher), sync writes (to the intent log), async writes (bulk writes of dirty data), and scrub (scrub and resilver operations). Note that bulk dirty data described above are scheduled in the async write queue. See [vdev\_queue.c](http://src.illumos.org/source/xref/illumos-gate/usr/src/uts/common/fs/zfs/vdev_queue.c#138) for the related tunables:

```
uint32_t zfs_vdev_sync_read_min_active = 10;
uint32_t zfs_vdev_sync_read_max_active = 10;
uint32_t zfs_vdev_sync_write_min_active = 10;
uint32_t zfs_vdev_sync_write_max_active = 10;
uint32_t zfs_vdev_async_read_min_active = 1;
uint32_t zfs_vdev_async_read_max_active = 3;
uint32_t zfs_vdev_async_write_min_active = 1;
uint32_t zfs_vdev_async_write_max_active = 10;
uint32_t zfs_vdev_scrub_min_active = 1;
uint32_t zfs_vdev_scrub_max_active = 2;
```

Each of these queues has tunable values for the min and max number of outstanding operations of the given type that can be issued to a leaf vdev (LUN). The tunable zfs\_vdev\_max\_active limits the number of IOs issued to a single vdev. If its value is less than the sum of the zfs\_vdev\_\*\_max\_active tunables, then the minimums come into play. The minimum number of each queue will be scheduled and the remainder of zfs\_vdev\_max\_active is issued from the queues in priority order.

At a high level, the appropriate values for these tunables will be specific to your LUNs. Higher maximums lead to higher throughput with potentially higher latency. On some devices such as storage arrays with distinct hardware for reads and writes, some of the queues can be thought of as independent; on other devices such as traditional HDDs, reads and writes will likely impact each other.

A simple way to tune these values is to monitor I/O throughput and latency under load. Increase values by 20-100% until you find a point where throughput no longer increases, but latency is acceptable.

```
#pragma D option quiet

BEGIN
{
        start = timestamp;
}

io:::start
{
        ts[args[0]->b_edev, args[0]->b_lblkno] = timestamp;
}

io:::done
/ts[args[0]->b_edev, args[0]->b_lblkno]/
{
        this->delta = (timestamp - ts[args[0]->b_edev, args[0]->b_lblkno]) / 1000;
        this->name = (args[0]->b_flags & (B_READ | B_WRITE)) == B_READ ?
            "read " : "write ";

        @q[this->name] = quantize(this->delta);
        @a[this->name] = avg(this->delta);
        @v[this->name] = stddev(this->delta);
        @i[this->name] = count();
        @b[this->name] = sum(args[0]->b_bcount);

        ts[args[0]->b_edev, args[0]->b_lblkno] = 0;
}

END
{
        printa(@q);

        normalize(@i, (timestamp - start) / 1000000000);
        normalize(@b, (timestamp - start) / 1000000000 * 1024);

        printf("%-30s %11s %11s %11s %11s\n", "", "avg latency", "stddev",
            "iops", "throughput");
        printa("%-30s %@9uus %@9uus %@9u/s %@8uk/s\n", @a, @v, @i, @b);
}

# dtrace -s rw.d -c 'sleep 60'

  read
           value  ------------- Distribution ------------- count
              32 |                                         0
              64 |                                         23
             128 |@                                        655
             256 |@@@@                                     1638
             512 |@@                                       743
            1024 |@                                        380
            2048 |@@@                                      1341
            4096 |@@@@@@@@@@@@                             5295
            8192 |@@@@@@@@@@@                              5033
           16384 |@@@                                      1297
           32768 |@@                                       684
           65536 |@                                        400
          131072 |                                         225
          262144 |                                         206
          524288 |                                         127
         1048576 |                                         19
         2097152 |                                         0

  write
           value  ------------- Distribution ------------- count
              32 |                                         0
              64 |                                         47
             128 |                                         469
             256 |                                         591
             512 |                                         327
            1024 |                                         924
            2048 |@                                        6734
            4096 |@@@@@@@                                  43416
            8192 |@@@@@@@@@@@@@@@@@                        102013
           16384 |@@@@@@@@@@                               60992
           32768 |@@@                                      20312
           65536 |@                                        6789
          131072 |                                         860
          262144 |                                         208
          524288 |                                         153
         1048576 |                                         36
         2097152 |                                         0

                               avg latency      stddev        iops  throughput
write                              19442us     32468us      4064/s   261889k/s
read                               23733us     88206us       301/s    13113k/s
```

## Async writes

Dirty data governed by zfs\_dirty\_data\_max is written to disk via async writes. The I/O scheduler treats async writes a little differently than other operations. The number of concurrent async writes scheduled depends on the amount of dirty data on the system. Recall that there is a fixed (but tunable) limit of dirty data in memory. With a small amount of dirty data, the scheduler will only schedule a single operation (zfs\_vdev\_async\_write\_min); the idea is to preserve low latency of synchronous operations when there isn’t much write load on the system. As the amount of dirty data increases, the scheduler will push the LUNs harder to flush it out by issuing more concurrent operations.

The old behavior was to schedule a fixed number of operations regardless of the load. This meant that the latency of synchronous operations could fluctuate significantly. While writing out dirty data ZFS would slam the LUNs with writes, contending with synchronous operations and increasing their latency. After the syncing transaction group had completed, there would be a period of relatively low async write activity during which synchronous operations would complete more quickly. This phenomenon was known as “picket fencing” due to the square wave pattern of latency over time. The new OpenZFS I/O scheduler is optimized for consistency.

In addition to tuning the minimum and maximum number of concurrent operations sent to the device, there are two other tunables related to asynchronous writes: zfs\_vdev\_async\_write\_active\_min\_dirty\_percent and zfs\_vdev\_async\_write\_active\_max\_dirty\_percent. Along with the min and max operation counts (zfs\_vdev\_async\_write\_min\_active and zfs\_vdev\_aysync\_write\_max\_active), these four tunables define a piece-wise linear function that determines the number of operations scheduled as depicted in this lovely ASCII art graph [excerpted from the comments](http://src.illumos.org/source/xref/illumos-gate/usr/src/uts/common/fs/zfs/vdev_queue.c#84):

```
 * The number of concurrent operations issued for the async write I/O class
 * follows a piece-wise linear function defined by a few adjustable points.
 *
 *        |                   o---------| <-- zfs_vdev_async_write_max_active
 *   ^    |                  /^         |
 *   |    |                 / |         |
 * active |                /  |         |
 *  I/O   |               /   |         |
 * count  |              /    |         |
 *        |             /     |         |
 *        |------------o      |         | <-- zfs_vdev_async_write_min_active
 *       0|____________^______|_________|
 *        0%           |      |       100% of zfs_dirty_data_max
 *                     |      |
 *                     |      `-- zfs_vdev_async_write_active_max_dirty_percent
 *                     `--------- zfs_vdev_async_write_active_min_dirty_percent
```

In a relatively steady state we’d like to see the amount of outstanding dirty data stay in a narrow band between the min and max percentages, by default 30% and 60% respectively.

Tune zfs\_vdev\_async\_write\_max\_active as described above to maximize throughput without hurting latency. The only reason to increase zfs\_vdev\_async\_write\_min\_active is if additional writes have little to no impact on latency. While this could be used to make sure data reaches disk sooner, an alternative approach is to decrease zfs\_vdev\_async\_write\_active\_min\_dirty\_percent thereby starting to flush data despite less dirty data accumulating.

To tune the min and max percentages, watch both latency and the number of scheduled async write operations. If the operation count fluctuates wildly and impacts latency, you may want to flatten the slope by decreasing the min and/or increasing the max (note below that you will likely want to increase zfs\_delay\_min\_dirty\_percent if you increase zfs\_vdev\_async\_write\_active\_max\_dirty\_percent -- see below).

```
#pragma D option aggpack
#pragma D option quiet

fbt::vdev_queue_max_async_writes:entry
{
        self->spa = args[0];
}
fbt::vdev_queue_max_async_writes:return
/self->spa && self->spa->spa_name == $$1/
{
        @ = lquantize(args[1], 0, 30, 1);
}

tick-1s
{
        printa(@);
        clear(@);
}

fbt::vdev_queue_max_async_writes:return
/self->spa/
{
        self->spa = 0;
}

# dtrace -s q.d dcenter

min .--------------------------------. max | count
< 0 : ▃▆ : >= 30 | 23279

min .--------------------------------. max | count
< 0 : █ : >= 30 | 18453

min .--------------------------------. max | count
< 0 : █ : >= 30 | 27741

min .--------------------------------. max | count
< 0 : █ : >= 30 | 3455

min .--------------------------------. max | count
< 0 : : >= 30 | 0
```

## Write delay

In situations where LUNs cannot keep up with the incoming write rate, OpenZFS artificially delays writes to ensure consistent latency (see the previous post in this series). Until a certain amount of dirty data accumulates there is no delay. When enough dirty data accumulates OpenZFS gradually increases the delay. By delaying writes OpenZFS effectively pushes back on the client to limit the rate of writes by forcing artificially higher latency. There are two tunables that pertain to delay: how much dirty data there needs to be before the delay kicks in, and the factor by which that delay increases as the amount of outstanding dirty data increases.

The tunable zfs\_delay\_min\_dirty\_percent determines when OpenZFS starts delaying writes. The default is 60%; note that we don’t start delaying client writes until the IO scheduler is pushing out data as fast as it can (zfs\_vdev\_async\_write\_active\_max\_dirty\_percent also defaults to 60%).

The other relevant tunable is zfs\_delay\_scale is really the only magic number here. It roughly corresponds to the inverse of the maximum number of operations per second (denominated in nanoseconds), and is used as a scaling factor.

Delaying writes is an aggressive step to ensure consistent latency. It is required if the client really is pushing more data than the system can handle, but unnecessarily delaying writes degrades overall throughput. There are two goals to tuning delay: reduce or remove unnecessary delay, and ensure consistent delays when needed.

First check to see how often writes are delayed. This simple DTrace one-liner does the trick:

```
# dtrace -n fbt::dsl_pool_need_dirty_delay:return'{ @[args[1] == 0 ? "no delay" : "delay"] = count(); }'
```

If a relatively small percentage of writes are delayed, increasing the amount of dirty data allowed (zfs\_dirty\_data\_max) or even pushing out the point at which delays start (zfs\_delay\_min\_dirty\_percent). When increasing zfs\_dirty\_data\_max consider the other users of DRAM on the system, and also note that a small amount of small delays does not impact performance significantly.

If many writes are being delayed, the client really is trying to push data faster than the LUNs can handle. In that case, check for consistent latency, again, with a DTrace one-liner:

```
# dtrace -n delay-mintime'{ @ = quantize(arg2); }'
```

With high variance or if many write operations are being delayed for the maximum zfs\_delay\_max\_ns (100ms by default) then try increasing zfs\_delay\_scale by a factor of 2 or more, or try delaying earlier by reducing zfs\_delay\_min\_dirty\_percent (remember to also reduce zfs\_vdev\_async\_write\_active\_max\_dirty\_percent).

## Summing up

Our experience at [Delphix](www.delphix.com) tuning the new write throttle has been so much better than in the [old ZFS world](https://www.oracle.com/storage/nas/index.html): each tunable has a clear and comprehensible purpose, their relationships are well-defined, and the issues in tension pulling values up or down are both easy to understand and -- most importantly -- easy to measure. I hope that this tuning guide helps others trying to get the most out of their OpenZFS systems whether on [Linux](http://zfsonlinux.org/), [FreeBSD](https://wiki.freebsd.org/ZFS), [Mac OS X](https://openzfsonosx.org/), [illumos](www.illumos.org) -- not to mention the support engineers for the many products that incorporate OpenZFS into a larger solution.
