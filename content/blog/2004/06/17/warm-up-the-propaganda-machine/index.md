---
title: "Warm up the propaganda machine"
date: "2004-06-17"
categories:
  - "dtrace"
permalink: /2004/06/17/warm-up-the-propaganda-machine/
---

I'm a Solaris Kernel engineer at Sun working on [DTrace](http://www.sun.com/bigadmin/content/dtrace/), the new dynamic instrumentation framework in Solaris 10, along side my coconspirators [Bryan Cantrill](http://blogs.sun.com/bmc) and Mike Shapiro. In addition to spending a large amount of my time improving DTrace, I have and continue to work on observability and debugging tools in Solaris -- mdb(1), the p-tools, /proc file system -- stuff like that.

My goal with this weblog is to write to no one in particular about what we have cooking for DTrace in the future. I'm sure I'll occassionally degenerate into unbridled [rants](http://blogs.sun.com/roller/page/bmc/20040616) as in the vogue for weblogs, but I'll try to keep it vaguely interesting...

Most of my work on DTrace has been directed towards tracing user-level applications. My first contribution, over two years ago, was the `ustack()` action to let you take an application stack backtrace from DTrace. Next I made the `pid` provider that lets you trace not only any user-level function entry and return, but every single instruction in a function. So you can do stuff like this:

```
#!/usr/sbin/dtrace -s
pid$1::$2:entry
{
self->spec = speculation();
}
pid$1::$2:
/self->spec/
{
speculate(self->spec);
printf("%s+%s", probefunc, probename);
}
pid$1::$2:return
/self->spec && arg1 == -1/
{
commit(self->spec);
self->spec = 0;
}
pid$1::$2:return
/self->spec/
{
discard(self->spec);
}

```

Run this with two arguments, the process ID and the function name. What this script does is trace every instruction a function executes _if_ it ends up returning -1. There can be problems where a function works properly 1,000 times and then fails once. Stepping through it with a debugger is brutal if not impossible, but this really simple D script makes understanding the problem a snap.

Anyway, I think every user of Solaris probably has a need for DTrace even if he or she doesn't know it. I've loved working on DTrace, and look forward to sharing some of the future directions and the nitty-gritty, inside-the-sausage-factory stuff.
