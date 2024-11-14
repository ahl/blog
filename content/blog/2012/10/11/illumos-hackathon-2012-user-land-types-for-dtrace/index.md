---
title: "illumos hackathon 2012: user-land types for DTrace"
date: "2012-10-11"
categories:
  - "dtrace"
tags:
  - "dtrace"
  - "hackathon"
  - "illumos"
  - "pid"
  - "user-land"
permalink: /2012/10/11/illumos-hackathon-2012-user-land-types-for-dtrace/
---

At the illumos hackathon last week, [Robert Mustacchi](http://dtrace.org/blogs/rm/) and I prototyped better support for manipulating user-land structures. As anyone who's used it knows, DTrace is currently very kernel-centric -- this both reflects the reality of how operating systems and DTrace are constructed, and the origins of DTrace itself in the Solaris Kernel Group. Discussions at [dtrace.conf(12)](http://dtrace.org/blogs/ahl/2012/04/09/dtrace-conf12-wrap-up/) this spring prompted me to chart a path to better user-land support. This prototype of copyin-automagic was a first step.

What we implemented was a new 'user' keyword to denote that a type is a user-land structure. For example, let's say we had the address of a 4-byte integer; today we'd access its value using copyin():

```
this->i = *(int *)copyin(this->addr, sizeof (int));
```

With our prototype, this gets simpler and more intuitive:

```
this->i = *(user int *)addr;
```

The impact is even more apparent when it comes to pointer chasing through structures. Today if we need to get to the third element of a linked list, the D code would look like this:

```
this->p = (node_t *)copyin(this->addr, sizeof (node_t));
this->p = (node_t *)copyin((uintptr_t)this->p->next, sizeof (node_t));
this->p = (node_t *)copyin((uintptr_t)this->p->next, sizeof (node_t));
trace(this->p->value);
```

Again, it's much simpler with the user keyword:

```
this->p = (user node_t *)this->addr;
trace(this->p->next->next->value);
```

D programs are compiled into a series of instructions -- DIF -- that are executed by the code DTrace framework when probes fire. We use the new keyword to generate instructions that load from the address space of the currently executing process rather than that of the kernel.

Adding a new keyword feels a little clunky, and I'm not sure if it's the right way forward, but it does demonstrate a simpler model for accessing user-land structures, and was a critical first step. We already have three main sources of user-land values; the next steps are to make use of those.

### Types for system calls

Arguments to system calls (mostly) have well-known types. Indeed those types are encoded in truss in excruciating and exotic detail. We should educate DTrace about those types. What I'd propose is that we create a single repository of all system call metadata. This could be, for example, and XML file that listed every system call, its name, code, subcode, types, etc. Of course we could use that as the source of type information for the syscall provider, but we could also use that to generate everything from the decoding tables in truss to the libc and kernel stubs for system calls.

As an aside, there are a couple of system calls whose parameter types -- ioctl(2) is the obvious example. It would be interesting to assess the utility of an ioctl provider whose probes would be the various codes that are passed as the second parameter. Truss already has this information; why not DTrace?

### Types for the pid provider

Another obvious source of type information is the process being traced. When a user specifies the -p or -c option to dtrace(1M) we're examining a particular process, and that process can have embedded type information. We could use those types and implicitly identify them as belonging to user-space rather than the kernel. Pid provider probes correspond to the entry and return from user-land functions; we could identify the appropriate types for those parameters. We could simplify it further by doing all type handling in libdtrace (in user-land) rather than pushing the types into the kernel.

### Types for USDT

User-land statically defined tracing -- tracepoints explicitly inserted into code -- can already have types associated with them. A first step would be to implicitly identify those types as belonging to user-land. I believe that this could even be done without adversely affecting existing scripts.

### Thorny issues

While there are some clear paths forward, there are some tricky issues that remain. In particular that processes can have different data models -- 32-bit v. 64-bit -- presents a real challenge. Both the width of a load and offsets into structures change depending on the process that's running. There might be some shortcuts for system calls, and we might be able to constrain the problem for the pid provider by requiring -p or -c, or we might have to compile our D program twice and then choose which version to run based on the data model of the process. In the spirit of the hackathon, Robert and I punted for our 'user' keyword prototype, but these problems need to be well understood and sufficiently solved.

### Next steps

I'll be working on some of these problems on the back burner; I'm especially interested in the Grand Unified Syscall Project -- an idea I've been touting for more years than I care to relate -- to bring types to the syscall provider. If you have ideas for user-land tracing with DTrace, or want to work on anything I've mention, leave a comment or drop me a note.
