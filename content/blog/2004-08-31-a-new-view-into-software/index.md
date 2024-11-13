---
title: "a new view into software"
date: "2004-08-31"
categories: 
  - "dtrace"
---

As [Bryan](http://blogs.sun.com/bmc) has observed the past, software has a quality unique to engineering disciplines in that you can build it, but you can't see it. [DTrace](http://www.sun.com/bigadmin/content/dtrace/) changes that by opening windows into parts of the system that were previously unobservable and it does so in a way that minimally changes what you're attempting to observe -- this software "uncertainty principle" has limited the utility of previous observability tools. One of the darkest areas of debugging in user-land has been around **lock contention**.

In multi-threaded programs synchronization primitives -- mutexes, R/W locks, semaphores, etc. -- are required to coordinate each thread's efforts and make sure shared data is accessed safely. If many threads are kept waiting while another thread owns a sychronization primitive, the program is said to suffer from lock contention. In the kernel, we've had [lockstat(1m)](http://docs.sun.com/db/doc/816-5166/6mbb1kq66?a=view) for many years, but in user-land, the techniqes for observing lock behavior and sorting out the cause or even the _presence_ have been very [_ad hoc_](http://www.princeton.edu/~psg/unix/Solaris/troubleshoot/lockcontend.html)

### the plockstat provider

I just finished work on the plockstat provider for DTrace as well as a new plockstat(1m) command for observing user-land synchronization objects. If you're unfamiliar with DTrace, you might want to take a quick look at the [Solaris Dynamic Tracing Guide](http://docs.sun.com/db/doc/817-6223) (look through it for some examples); that will help ground some of this explanation.

The plockstat provider has these probes:

<table><tbody><tr><td>mutex-acquire</td><td>fires when a mutex is acquired</td></tr><tr><td>mutex-release</td><td>fires when a mutex is released</td></tr><tr><td>mutex-block</td><td>fires when a thread blocks waiting for a mutex</td></tr><tr><td>mutex-spin</td><td>fires when a thread spins waiting for a mutex</td></tr><tr><td>rw-acquire</td><td>fires when an R/W lock is acquired</td></tr><tr><td>rw-release</td><td>fires when an R/W lock is released</td></tr><tr><td>rw-block</td><td>fires when a thread blocks waiting for an R/W lock</td></tr></tbody></table>

It's possible with other tools to observe these points, but -- as anyone who's tried it can attest -- other tools can alter the effects you're trying to observe. Traditional debuggers can effectively serialize your parallel program removing any trace of the lock contention you'd see during a normal run. DTrace and the plockstat provider avoid eliminate this problem.

With the plockstat provider you can answer questions that were previously very difficult to solve, such as "where is my program blocked on mutexes":

```
bash-2.05b# dtrace -n plockstat1173:::mutex-block'{ @[ustack()] = count() }'
dtrace: description 'plockstat1173:::mutex-block' matched 2 probes
^C
libc.so.1`mutex_lock_queue+0xa9
libc.so.1`slow_lock+0x3d
libc.so.1`mutex_lock_impl+0xec
libc.so.1`mutex_lock+0x38
libnspr4.so`PR_Lock+0x1a
libnspr4.so`PR_EnterMonitor+0x35
libxpcom.so`__1cGnsPipePGetWriteSegment6MrpcrI_I_+0x3e
libxpcom.so`__1cSnsPipeOutputStreamNWriteSegments6MpFpnPnsIOutputStream_pvpcIIpI_I3I5_I_+0x4f
c4654d3c
libxpcom.so`__1cUnsThreadPoolRunnableDRun6M_I_+0xb0
libxpcom.so`__1cInsThreadEMain6Fpv_v_+0x32
c4ec1d6a
libc.so.1`_thr_setup+0x50
libc.so.1`_lwp_start
1

```

(any guesses as to what program this might be?)

Not just a new view for DTrace, but a new view for user-land.

### the plockstat(1m) command

DTrace is an incredibly powerful tool, but some tasks are so common that we want to make it as easy as possible to use DTrace's facilities without knowing anything about DTrace. The plockstat(1m) command wraps up a bunch of knowledge about lock contention in a neat and easy to use package:

```
# plockstat -s 10 -A -p `pgrep locker`
^C
Mutex block
-------------------------------------------------------------------------------
Count     nsec Lock                         Caller
13 22040260 locker`lock1                 locker`go_lock+0x47
nsec ---- Time Distribution --- count Stack
65536 |@@@@@@@@@@@@@@          |     8 libc.so.1`mutex_lock+0x38
131072 |                        |     0 locker`go_lock+0x47
262144 |@@@@@                   |     3 libc.so.1`_thr_setup+0x50
524288 |                        |     0 libc.so.1`_lwp_start
1048576 |                        |     0
2097152 |                        |     0
4194304 |                        |     0
8388608 |                        |     0
16777216 |@                       |     1
33554432 |                        |     0
67108864 |                        |     0
134217728 |                        |     0
268435456 |@                       |     1
...

```

This has been a bit of a teaser. I only integrated plockstat into Solaris 10 yesterday and it will be a few weeks before you can access plockstat as part of the [Solaris Express](http://wwws.sun.com/software/solaris/solaris-express/sol_index.html) program, but keep an eye on the [DTrace Solaris Express Schedule](http://blogs.sun.com/ahl/dtracesched).
