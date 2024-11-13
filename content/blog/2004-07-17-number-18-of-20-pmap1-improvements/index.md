---
title: "Number 18 of 20: pmap(1) improvements"
date: "2004-07-17"
categories: 
  - "opensolaris"
---

[go to the Solaris 10 top 11-20 list for more](http://dtrace.org/blogs/ahl/the_solaris_10_top_11)

### pmap(1)

For the uninitiated, [pmap(1)](http://docs.sun.com/db/doc/817-0689/6mgfkpd0f?a=view) is a tool that lets you observe the mappings in a process. Here's some typical output:

```
311981: /usr/bin/sh
08046000       8K rw--- [ stack ]
08050000      80K r-x-- /sbin/sh
08074000       4K rwx-- /sbin/sh
08075000      16K rwx-- [ heap ]
C2AB0000      64K rwx-- [ anon ]
C2AD0000     752K r-x-- /lib/libc.so.1
C2B9C000      28K rwx-- /lib/libc.so.1
C2BA3000      16K rwx-- /lib/libc.so.1
C2BB1000       4K rwxs- [ anon ]
C2BC0000     132K r-x-- /lib/ld.so.1
C2BF1000       4K rwx-- /lib/ld.so.1
C2BF2000       8K rwx-- /lib/ld.so.1
total      1116K

```

You can use this to understand various adresses you might see from a debugger, or you can use other modes of pmap(1) to see the page sizes being used for various mappings, how much of the mappings have actually been faulted in, the attached ISM, DISM or System V shared memory segments, etc. In Solaris 10, pmap(1) has some cool new features -- after a little more thought, I'm not sure that this really belongs on the top 11-20 list, but this is a very cool tool and gets some pretty slick new features; anyways the web affords me the chance for some revisionist history if I feel like updating the list...

### thread and signal stacks

When a process creates a new thread, that thread needs a stack. By default, that stack comes from an anonymous mapping. Before Solaris 10, those mappings just appeared as `\[ anon \]` -- undifferentiated from other anonymous mappings; now we label them as thread stacks:

```
311992: ./mtpause.x86 2
08046000       8K rwx-- [ stack ]
08050000       4K r-x-- /home/ahl/src/tests/mtpause/mtpause.x86
08060000       4K rwx-- /home/ahl/src/tests/mtpause/mtpause.x86
C294D000       4K rwx-R    [ stack tid=3 ]
C2951000       4K rwxs- [ anon ]
C2A5D000       4K rwx-R    [ stack tid=2 ]
...

```

That can be pretty useful if you're trying to figure out what some address means in a debugger; before you could tell that it was from some anonymous mapping, but what the heck was that mapping all about? Now you can tell at a glance that its the stack for a particular thread.

Another kind of stack is the alternate signal stack. Alternate signal stacks let threads handle signals like SIGSEGV which might arise due to a stack overflow of the main stack (leaving no room on that stack for the signal handler). You can establish an alternate signal stack using the [sigaltstack(2)](http://docs.sun.com/db/doc/817-0691/6mgfmmdt5?a=view) interface. If you allocate the stack by creating an anonymous mapping using [mmap(2)](http://docs.sun.com/db/doc/817-0691/6mgfmmdqg?a=view) pmap(1) can now identify the per-thread alternate signal stacks:

```
...
FEBFA000       8K rwx-R    [ stack tid=8 ]
FEFFA000       8K rwx-R    [ stack tid=4 ]
FF200000      64K rw--- [ altstack tid=8 ]
FF220000      64K rw--- [ altstack tid=4 ]
...

```

### core file content

Core files have always contained a _partial_ snapshot of a process's memory mappings. Now that you can you manually adjust the content of a core file (see my [previous entry](http://dtrace.org/blogs/ahl/number_13_of_20_core)) some ptools will give you warnings like this:  
`pargs: core 'core' has insufficient content`  
So what's in that core file? pmap(1) now let's you see that easily; mappings whose data is missing from the core file are marked with a `\*`:

```
$ coreadm -P heap+stack+data+anon
$ cat
^\Quit - core dumped
$ pmap core
core 'core' of 312077:  cat
08046000       8K rw--- [ stack ]
08050000       8K r-x--* /usr/bin/cat
08062000       4K rwx-- /usr/bin/cat
08063000      40K rwx-- [ heap ]
C2AB0000      64K rwx--
C2AD0000     752K r-x--* /lib/libc.so.1
C2B9C000      28K rwx-- /lib/libc.so.1
C2BA3000      16K rwx-- /lib/libc.so.1
C2BC0000     132K r-x--* /lib/ld.so.1
C2BF1000       4K rwx-- /lib/ld.so.1
C2BF2000       8K rwx-- /lib/ld.so.1
total      1064K

```

If you're looking at a core file from an earlier release or from a customer in the field, you can quickly tell if you're going to be able to get the data you need out of the core file or if the core file can only be interpreted on the original machine or whatever.
