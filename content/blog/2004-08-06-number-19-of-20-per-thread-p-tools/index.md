---
title: "Number 19 of 20: per-thread p-tools"
date: "2004-08-06"
categories: 
  - "opensolaris"
---

[go to the Solaris 10 top 11-20 list for more](http://dtrace.org/blogs/ahl/the_solaris_10_top_11)

### p-tools

Since Solaris 7 we've included a bunch of process observability tools -- the so called "p-tools". Some of them inspect aspects of the process of the whole. For example, the [pmap(1)](http://docs.sun.com/db/doc/816-5165/6mbb0m9le?a=view) command shows you information about a process's mappings, their location and ancillary information (the associated file, shmid, etc.). [pldd(1)](http://docs.sun.com/db/doc/816-5165/6mbb0m9m2?a=view) is another example; it shows which shared objects a process has opened.

Other p-tools apply to the threads in a process. The [pstack(1)](http://docs.sun.com/db/doc/816-5165/6mbb0m9m2?a=view) utility shows the call stacks for each thread in a process. New in Solaris 10 [Eric](http://blogs.sun.com/eschrock) and [Andrei](http://blogs.sun.com/andrei) have modified the p-tools that apply to threads so that you can specify the threads you're interested in rather than having to sift through all of them.

### pstack(1)

Developers and administrators often use pstack(1) to see what a process is doing and if it's making progress. You'll often turn to pstack(1) after [prstat(1)](http://docs.sun.com/db/doc/816-5166/6mbb1kqal?a=view) or top(1) shows a process consuming a bunch of CPU time -- what's that guy up to. Complex processes can many many threads; fortunately prstat(1)'s -L flag will split out each thread in a process as its own row so you can quickly see that thread 5, say, is the one that's hammering the processor. Now rather than sifting through all 100 threads to find thread 5, you can just to this:

```
$ pstack 107/5
100225: /usr/sbin/nscd
----------------- lwp# 5 / thread# 5  --------------------
c2a0314c nanosleep (c25edfb0, c25edfb8)
08056a96 gethost_revalidate (0) + 4b
c2a02d10 _thr_setup (c2949000) + 50
c2a02ed0 _lwp_start (c2949000, 0, 0, c25edff8, c2a02ed0, c2949000)

```

Alternatively, you can specify a range of threads (`5-7` or `11-`), and combinations of ranges (`5-7,11-`). Giving us something like this:

```
$ pstack 107/5-7,11-
100225: /usr/sbin/nscd
----------------- lwp# 5 / thread# 5  --------------------
c2a0314c nanosleep (c25edfb0, c25edfb8)
08056a96 gethost_revalidate (0) + 4b
c2a02d10 _thr_setup (c2949000) + 50
c2a02ed0 _lwp_start (c2949000, 0, 0, c25edff8, c2a02ed0, c2949000)
----------------- lwp# 6 / thread# 6  --------------------
c2a0314c nanosleep (c24edfb0, c24edfb8)
080577d6 getnode_revalidate (0) + 4b
c2a02d10 _thr_setup (c2949400) + 50
c2a02ed0 _lwp_start (c2949400, 0, 0, c24edff8, c2a02ed0, c2949400)
----------------- lwp# 7 / thread# 7  --------------------
c2a0314c nanosleep (c23edfb0, c23edfb8)
08055f56 getgr_revalidate (0) + 4b
c2a02d10 _thr_setup (c2949800) + 50
c2a02ed0 _lwp_start (c2949800, 0, 0, c23edff8, c2a02ed0, c2949800)
----------------- lwp# 11 / thread# 11  --------------------
c2a0314c nanosleep (c1fcdf60, c1fcdf68)
0805887d reap_hash (80ca918, 8081140, 807f2f8, 259) + ed
0805292a nsc_reaper (807f92c, 80ca918, 8081140, 807f2f8, c1fcdfec, c2a02d10) + 6d
08055ded getpw_uid_reaper (0) + 1d
c2a02d10 _thr_setup (c20d0800) + 50
c2a02ed0 _lwp_start (c20d0800, 0, 0, c1fcdff8, c2a02ed0, c20d0800)
...

```

The thread specification syntax also works for core files if you're just trying to drill down on, say, the thread that caused the fatal problem:

```
$ pstack core/2
core 'core/2' of 100225:        /usr/sbin/nscd
----------------- lwp# 2 / thread# 2  --------------------
c2a04888 door     (c28fbdc0, 74, 0, 0, c28fde00, 4)
080540bd ???????? (deadbeee, c28fddec, 11, 0, 0, 8053d33)
c2a0491c _door_return () + bc

```

### truss(1)

The [truss(1)](http://docs.sun.com/db/doc/816-5165/6mbb0m9q3?a=view) utility is the mother of all p-tools. It lets you trace a process's system calls, faults, and signals as well as user-land function calls. In addition to consuming pretty much every lower- and upper-case command line option, truss(1) now also supports the thread specification syntax. Now you can follow just the threads that are doing something interesting:

```
truss -p 107/5
openat(-3041965, ".", O_RDONLY|O_NDELAY|O_LARGEFILE) = 3
fcntl(3, F_SETFD, 0x00000001)                   = 0
fstat64(3, 0x08047800)                          = 0
getdents64(3, 0xC2ABE000, 8192)                 = 8184
brk(0x080721C8)                                 = 0
...

```

### pbind(1)

The [pbind(1)](http://docs.sun.com/db/doc/816-5166/6mbb1kq9l?a=view) utility isn't an observability tool, rather this p-tool binds a process to a particular CPU so that it will only run on that CPU (except in some unusual circumstances; see the [man page](http://docs.sun.com/db/doc/816-5166/6mbb1kq9l?a=view) for details). For multi-threaded processes, the process is clearly not the right granularity for this kind of activity -- you want to be able to bind this thread to that CPU, and those threads to some other CPU. In Solaris 10, that's a snap:

```
$ pbind -b 1 107/2
lwp id 107/2: was not bound, now 1
$ pbind -b 0 107/2-5
lwp id 107/2: was 1, now 0
lwp id 107/3: was not bound, now 0
lwp id 107/4: was not bound, now 0
lwp id 107/5: was not bound, now 0

```

These are perfect examples of Solaris responding to requests from users: there was no easy way to solve these problems, and that was causing our users pain, so we fixed it. After the BOF at OSCON, a Solaris user had a laundry lists of problems and requests, and was skeptical about our interest in fixing them, but I convinced him that we do care, but we need to hear about them. So let's hear about your gripes and wish lists for Solaris. Many of the usability features (the p-tools for example) came out of our own use of Solaris in kernel development -- once [OpenSolaris](http://dtrace.org/blogs/ahl/linux_solaris_and_open_source) lets everyone be a Solaris kernel developer, I'm sure we'll be stumbling onto many more quality of life tools like pstack(1), truss(1), and pbind(1).
