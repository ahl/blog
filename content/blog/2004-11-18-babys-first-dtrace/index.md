---
title: "Baby&#039;s first DTrace"
date: "2004-11-18"
categories: 
  - "dtrace"
---

At the Solaris 10 launch on Monday I was talking to a sysadmin about DTrace. He was clearly very excited about it -- finally he could end a fight between the database guys and the appserver guys about whose stuff was to blame -- but he had one reservation: Where do I start? Since DTrace lets you look at almost anything on the system, it can be hard knowing the first thing to look at, here's what I told him:

### start with the tools you know

You've probably used truss(1) or mpstat(1M) or prstat(1) or iostat(1M) or whatever. They give you a static view of what's happening on the system -- static in that you can't get any more, you can't get any other degree of detail, and you can't dive deeper. So start from those points, and go deeper. Each statistic in those observability tools has at least one associated probe in DTrace. If you're looking at mpstat(1M) output, maybe cross-calls (xcal) are high, or spins on mutexes (smtx) are high. You don't have to guess anymore; you can actually drill down and figure out what application or what user or what zone they correspond to by enabling their corresponding DTrace probes (`sysinfo:::xcalls` and `lockstat:::\*-spin` respectively) and trace the data you want.

### figure out what functions are being called

When you're trying to optimize an application, it helps to know where the app is spending its time. A simple DTrace invocation like this:

```
# dtrace -n 'pid$target:::entry{ @[probefunc] = count() }' -p <process-id>

```

can give you a coarse idea of where you're spending time. When you do this, a lot of it will make sense, but some of it will probably be a surprise: "Why am I calling malloc(3C) a bazillion times?" So find those aberrant cases and figure out what's going on: "OK, how much are we allocating each time?" (`dtrace -n 'pid$target::malloc:entry{ @ = quantize(arg0) }' -p <process-id>`).

### look for lock contention

In multi-threaded apps, lock contention can be **huge** performance killer. Run the new [plockstat(1)](http://dtrace.org/blogs/ahl/plockstat) command to see if your app suffers from lock contention. If it does, you'll see long spin and contention times. These are pretty easy problems to solve, but if you can't track down the source of the problem, plockstat -- of course -- lets you dig deeper by using the plockstat provider.

Those are a few places I've started from in the past, but, of course, every application is different. DTrace isn't meant to supplant your knowledge about your app and the system at large, rather it should complement it and let you do more with what you already know.
