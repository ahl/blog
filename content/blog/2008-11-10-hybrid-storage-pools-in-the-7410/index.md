---
title: "Hybrid Storage Pools in the 7410"
date: "2008-11-09"
categories:
  - "fishworks"
permalink: /2008/11/10/hybrid-storage-pools-in-the-7410/
---

The Sun Storage 7000 Series launches today, and with it Sun has the world's first complete product that seamlessly adds flash into the storage hierarchy in what we call the **Hybrid Storage Pool**. The HSP represents a departure from convention, and a new way of thinking designing a storage system. I've [written before](http://dtrace.org/blogs/ahl/hybrid_storage_pools_in_cacm) about the principles of the HSP, but now that it has been formally announced I can focus on the specifics of the Sun Storage 7000 Series and how it implements the HSP.

### Sun Storage 7410: The Cadillac of HSPs

The best example of the HSP in the 7000 Series is the 7410. This product combines a head unit (or two for high availability) with as many as 12 [J4400 JBODs](http://www.sun.com/storage/disk_systems/expansion/4400/). By itself, this is a pretty vanilla box: big, economical, 7200 RPM drives don't win any races, and the maximum of 128GB of DRAM is certainly a lot, but some workloads will be too big to fit in that cache. **With flash**, however, this box turns into quite the speed demon.

### Logzilla

The write performance of 7200 RPM drive isn't terrific. The appalling thing is that the next best solution — 15K RPM drives — aren't really that much better: a factor of two or three _at best_. To blow the doors off, the Sun Storage 7410 allows up to four write-optimized flash drives per JBOD each of which is capable of handling 10,000 writes per second. We call this flash device **Logzilla**.

Logzilla is a flash-based SSD that contains a pretty big DRAM cache backed by a supercapacitor so that the cache can effectively be treated as nonvolatile. We use Logzilla as a ZFS intent log device so that synchronous writes are directed to Logzilla and clients incur only that 100μs latency. This may sound a lot like how NVRAM is used to accelerate storage devices, and it is, but there are some important advantages of Logzilla. The first is capacity: most NVRAM maxes out at 4GB. That might seem like enough, but I've talked to enough customers to realize that it really isn't and that performance cliff is an awful long way down. Logzilla is an 18GB device which is big enough to hold the necessary data while ZFS syncs it out to disk even running full tilt. The second problem with NVRAM scalability: once you've stretched your NVRAM to its limit there's not much you can do. If your system supports it (and most don't) you can add another PCI card, but those slots tend to be valuable resources for NICs and HBAs, and even then there's necessarily a pretty small number to which you could conceivably scale. Logzilla is an SSD sitting in a SAS JBOD so it's easy to plug more devices into ZFS and use them as a growing pool of intent log devices.

### Readzilla

The standard practice in storage systems is to use the available DRAM as a read cache for data that is likely to be frequently accessed, and the 7000 Series does the same. In fact, it can do quite a better job of it because, unlike most storage systems which stop at 64GB of cache, the 7410 has up to 256GB of DRAM to use as a read cache. As I mentioned before, that's still not going to be enough to cache the entire working set for a lot of use cases. This is where we at Fishworks came up with the innovative solution of using flash as a massive read cache. The 7410 can accomodate up to six 100GB, read-optimized, flash SSDs; accordingly, we call this device **Readzilla**.

With Readzilla, a maximum 7410 configuration can have 256GB of DRAM providing sub-μs latency to cached data and 600GB worth of Readzilla servicing read requests in around 50-100μs. Forgive me for stating the obvious: that's 856GB of cache &mdash. That may not suffice to cache all workloads, but it's certainly getting there. As with Logzilla, a wonderful property of Readzilla is its scalability. You can change the number of Readzilla devices to match your workload. Further, you can choose the right combination of DRAM and Readzilla to provide the requisite service times with the appopriate cost and power use. Readzilla is cheaper and less power-hungry than DRAM so applications that don't need the blazing speed of DRAM can prefer the more economical flash cache. It's a flexible solution that can be adapted to specific needs.

### Putting It All Together

We started with DRAM and 7200 RPM disks, and by adding Logzilla and Readzilla the Sun Storage 7410 also has great write and read IOPS. Further, you can design the _specific_ system you need with just the right balance of write IOPS, read IOPS, throughput, capacity, power-use, and cost. Once you have a system, the Hybrid Storage Pool lets you solve problems with targeted solutions. Need capacity? Add disk. Out of read IOPS? Toss in another Readzilla or two. Write bogging down? Another Logzilla will net you another 10,000 write IOPS. In the old model, of course, all problems were simple because the solution was always the same: buy more fast drives. The HSP in the 7410 lets you address the specific problem you're having without paying for a solution to three other problems that you _don't_ have.

Of course, this means that administrators need to better understand the performance limiters, and fortunately the Sun Storage 7000 Series has a great answer to that in **Analytics**. Pop over to [Bryan's blog](http://blogs.sun.com/) where he talks all about that feature of the [Fishworks](http://blogs.sun.com/fishworks) software stack and how to use it to find performance problems on the 7000 Series. If you want to read more details about Hybrid Storage Pools and how exactly all this works, take a look [my article](http://dtrace.org/blogs/ahl/hybrid_storage_pools_in_cacm) on the subject in [CACM](http://cacm.acm.org/), as well as [this post about the L2ARC](http://dtrace.org/blogs/ahl/hybrid_storage_pools_in_cacm) (the magic behind using Readzilla) and [a nice marketing pitch on HSPs](http://dtrace.org/blogs/ahl/hsp_goes_glossy).
