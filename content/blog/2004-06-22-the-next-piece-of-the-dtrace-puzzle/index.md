---
title: "The next piece of the DTrace puzzle"
date: "2004-06-22"
categories: 
  - "dtrace"
---

When we first wrote DTrace, we needed to make sure it satisfied our fundamental goals: stable, safe, extensive, available in production, zero probe effect when disabled. By extensive, we meant that every corner of the system had to be covered, from kernel function calls, and kstats through system calls to any instruction in any process.

What we quickly discovered (both from our own use of DTrace and lots of great feedback from the DTrace community) was that while there were certainly enough probes, it often was difficult to find the _right_ probes or required some specific knowledge of one or more Solaris sub-systems. In the kernel we've been working on addressing that with the _stable_ providers: `proc`, `sched`, `io` and soon more. These providers present the _stable_ kernel abstractions ways that are well documented, comprehensible, and maintainable from release to release (meaning that your scripts won't break on Solaris 11).

I'm working on the next logical extension to this idea: stable user-level providers. The idea here is that executables and shared libraries will be able to publish their stable abstractions through stable probes. For example, the first stable user-level provider I plan to add is the `plockstat` provider. This will provide probes for the user-level synchronization primitives -- each time a thread locks a mutex by calling `mutex\_lock(3c)` or `pthread\_mutex\_lock` the `plockstat:::mutex-acquire` probe will fire. Just as the `lockstat(1m)` command has let Solaris users investigate kernel-level lock contention, the `plockstat` provider will bring the same investigative lens to user-land. Very exciting.

With user-level stable providers, there's also an opportunity for application developers to build stable hooks that their customers, or support engineers, or sales engineers, or developers, or whoever can use. Consider databases. Databases, by custom or necessity, seem to have a bunch of knobs to turn, knobs that need experts to turn them properly. Solaris similarly has some knobs that need to be tweaked to get your database to run just so. Now imagine if that database included stable probes for even coarse indicators of what's going on internally. It could then be possible to build DTrace scripts that enable probes in the database _and_ the kernel to get a truly _systemic_ view of database performance. Rather than requiring a database administrator versed in the oral tradition of database tuning, some of that knowledge could be condensed into these DTrace scripts whose output could be advice on how to turn which knobs.

A quick aside: this example highlighs one of the coolest things about DTrace -- it's systemic scope. There are lots of specialized tools for examining a particular part of the system, but correlating and integrating the data from these tools can be difficult or impossible. With DTrace there's one consistent data stream and instrumentation from every corner of the system can be easily tied together in coherent ways.

Back to user-level stable probes. A developer will add a new probe by invoking a macro:

```
void
my_func(my_struct_t *arg)
{
DTRACE_PROBE1(my_provider, my_probe, arg->a);
/* ... */
}

```

You then build the object file, and, before linking the object, you'll use `dtrace(1m)` to post-process all the object files; the [Solaris Dynamic Tracing Guide](http://www.sun.com/bigadmin/content/dtrace/d10_latest.pdf) will describe this in excruciating specificity once I work out the details. This will create this probe `my\_provider<pid>:<object name>my\_func:my\_probe` where <pid> is the process ID of the process that mapped this load object (executable or shared object) and <object name> is the name of that load object.

With user-level stable providers, applications and shared objects will be able to describe their own probes which should lead to simpler administration and support. Later it might be possible to leave in all those debugging printfs as DTrace probes. I'd love to hear about any other interesting ideas for user-level stable providers. Now back to the code...
