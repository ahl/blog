---
title: "DTracing Java"
date: "2005-04-18"
categories: 
  - "dtrace"
---

DTrace has cast light on parts of the system that were previously only dimly illuminated by previous tools, but there have been some parts of the system frustratingly left in the dark. The prevalent example is Java. Java has been relatively unobservable with DTrace; the [jstack() action](http://docs.sun.com/app/docs/doc/817-6223/6mlkidlhg?a=view#chp-actsub-39) has offered a narrow beam of light into interactions between Java code and the rest of the system, but we really need is Java probes in the DTrace framework.

DTrace users really want to be able to trace Java methods in the same way they can trace C function calls in the application (or in the kernel). We haven't quite reached that Xanadu yet, but [Kelly O'Hair](http://blogs.sun.com/roller/page/kto/20050413#java_vm_agents_and_solaris1) (with the inspiration and prodding of Jarod Jenson) has created [JVMPI and JVMTI agents](https://solaris10-dtrace-vm-agents.dev.java.net/) export the instrumentation provided by those frameworks into DTrace.

For example, examining the size of Java allocations is now a snap. The `object-alloc` probe fires every time an object gets allocated, and one of the arguments is the size.

```
# dtrace -n 'djvm$target:::object-alloc{ @ = quantize(arg1) }' -p `pgrep -n java`
dtrace: description 'djvm$target:::object-alloc' matched 1 probe
^C
value  ------------- Distribution ------------- count
4 |                                         0
8 |                                         43
16 |@@@@@@@@@@@@@@@@@                        18771
32 |@@@@@@@@@@@@@@@@                         17482
64 |@@@@@                                    5292
128 |@                                        1486
256 |                                         106
512 |                                         165
1024 |                                         319
2048 |                                         149
4096 |                                         48
8192 |                                         0
16384 |                                         1
32768 |                                         1
65536 |                                         0

```

One of the most troublesome areas when dealing with production Java code seems to be around garbage collection. There are two probes -- that fire at the start and end of a GC run -- that can be used, for example, to look for latency spikes in garbage collection:

```
bash-3.00# dtrace -s /dev/stdin -p `pgrep -n java`
djvm$target:::gc-start
{
self->ts = vtimestamp;
}
djvm$target:::gc-finish
/self->ts/
{
@ = quantize(vtimestamp - self->ts);
self->ts = 0;
}
dtrace: script '/dev/stdin' matched 2 probes
^C
value  ------------- Distribution ------------- count
16777216 |                                         0
33554432 |@@                                       1
67108864 |@@@@@@                                   3
134217728 |@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         16
268435456 |                                         0

```

Let's say there's some itermittent problem where GC takes a long time. This sort of script can help you identify the standard behavior and the outliers, and other DTrace facility -- notably speculative tracing -- will let you drill down on the source of the problem ("I care about these events only when the GC run takes more than 10ms?).

One of the most exciting moments early on with DTrace was observing the flow of control from a user-land function through a system call and into the kernel -- as far as I know we were seeing something that hadn't been done before ([Bryan](http://blogs.sun.com/bmc)'s last example in this [blog post](http://blogs.sun.com/roller/page/bmc/20040805#demo_ing_dtrace) demonstrates this). Here's a script that instruments a particular Java method (java.io.InputStreamReader.read()) and follows the thread of control from that method call through Java, libc, the system call table, and the kernel -- and back out:

```
#pragma D option quiet
djvm$target:::method-entry
/copyinstr(arg0) == "java/io/InputStreamReader" && copyinstr(arg1) == "read"/
{
self->interested = 1;
self->indent = 0;
}
djvm$target:::method-entry
/self->interested/
{
self->indent += 2;
printf("%*s -> %s:%s\n", self->indent, "",
copyinstr(arg0), copyinstr(arg1));
}
djvm$target:::method-return
/self->interested/
{
printf("%*s indent, "",
copyinstr(arg0), copyinstr(arg1));
self->indent -= 2;
}
syscall:::entry
/self->interested/
{
self->indent += 2;
printf("%*s => %s\n", self->indent, "", probefunc);
}
syscall:::return
/self->interested/
{
printf("%*s indent, "", probefunc);
self->indent -= 2;
}
pid$target:libc.so.1::entry
/self->interested/
{
self->indent += 2;
printf("%*s -> %s:%s\n", self->indent, "", probemod, probefunc);
}
pid$target:libc.so.1::return
/self->interested/
{
printf("%*s indent, "", probemod, probefunc);
self->indent -= 2;
}
fbt:::entry
/self->interested/
{
self->indent += 2;
printf("%*s -> %s:%s\n", self->indent, "", probemod, probefunc);
}
fbt:::return
/self->interested/
{
printf("%*s indent, "", probemod, probefunc);
self->indent -= 2;
}
djvm$target:::method-return
/copyinstr(arg0) == "java/io/InputStreamReader" && copyinstr(arg1) == "read" &&
self->interested/
{
self->interested = 0;
exit(0);
}

```

Not especially beautiful -- I had to hand roll my own flowindent -- but the results are pretty spectacular. I've uploaded the [whole shebang](http://dtrace.org/resources/ahl/jtrace.txt), but here's an abbreviated version:

```
-> java/io/InputStreamReader:read
-> sun/nio/cs/StreamDecoder:read
-> sun/nio/cs/StreamDecoder:read0
-> libc.so.1:malloc
-> libc.so.1:_smalloc
<- libc.so.1:_smalloc
 sun/nio/cs/StreamDecoder:read
-> sun/nio/cs/StreamDecoder:ensureOpen
 sun/nio/cs/StreamDecoder$CharsetSD:implRead
-> java/nio/CharBuffer:wrap
-> java/nio/HeapCharBuffer:
-> java/nio/CharBuffer:
-> java/nio/Buffer:
-> libc.so.1:malloc
 java/nio/Buffer:position
<- java/nio/Buffer:position
<- java/nio/Buffer:
<- java/nio/CharBuffer:
<- java/nio/HeapCharBuffer:
 java/nio/charset/CharsetDecoder:decode
-> sun/nio/cs/US_ASCII$Decoder:decodeLoop
-> java/nio/ByteBuffer:hasArray
 java/nio/CharBuffer:hasArray
 sun/nio/cs/US_ASCII$Decoder:decodeArrayLoop
...
<- sun/nio/cs/US_ASCII$Decoder:decodeArrayLoop
 java/nio/charset/CoderResult:isUnderflow
<- java/nio/charset/CoderResult:isUnderflow
 java/nio/charset/CoderResult:isUnderflow
 java/nio/Buffer:position
 sun/nio/cs/StreamDecoder$CharsetSD:readBytes
-> java/nio/HeapByteBuffer:compact
...
 java/io/FileInputStream:read
-> libc.so.1:read
-> libc.so.1:_read
-> genunix:pre_syscall
-> genunix:syscall_mstate
<- genunix:syscall_mstate
 read
-> genunix:read32
 genunix:read
-> genunix:getf
-> genunix:set_active_fd
<- genunix:set_active_fd
 genunix:nbl_need_check
 genunix:fop_rwlock
 nfs:nfs4_rwlock
-> nfs:nfs_rw_enter_sig
<- nfs:nfs_rw_enter_sig
 genunix:fop_read
 nfs:nfs4_read
... ... ...
 genunix:fop_rwunlock
 nfs:nfs4_rwunlock
 nfs:nfs_rw_exit
 genunix:releasef
-> genunix:clear_active_fd
 genunix:cv_broadcast
<- genunix:cv_broadcast
<- genunix:releasef
<- genunix:read
 genunix:post_syscall
<- genunix:post_syscall
<- libc.so.1:_read
<- libc.so.1:read
 java/nio/Buffer:position
 java/nio/Buffer:flip
 java/nio/Buffer:remaining
<- java/nio/Buffer:remaining
 java/nio/charset/CharsetDecoder:decode
-> sun/nio/cs/US_ASCII$Decoder:decodeLoop
-> java/nio/ByteBuffer:hasArray
 java/nio/CharBuffer:hasArray
 sun/nio/cs/US_ASCII$Decoder:decodeArrayLoop
...
<- sun/nio/cs/US_ASCII$Decoder:decodeArrayLoop
 java/nio/charset/CoderResult:isOverflow
<- java/nio/charset/CoderResult:isOverflow
 java/nio/charset/CoderResult:isUnderflow
 java/nio/charset/CoderResult:isOverflow
 java/nio/Buffer:position
 java/nio/Buffer:position
<- java/nio/Buffer:position
<- sun/nio/cs/StreamDecoder$CharsetSD:implRead
<- sun/nio/cs/StreamDecoder:read
<- sun/nio/cs/StreamDecoder:read0
<- sun/nio/cs/StreamDecoder:read
<- java/io/InputStreamReader:read

```

To me that's pretty exciting. From a unified framework, we're able to follow a single thread of control across multiple languages, privilege modes, and execution environments using multiple instrumtation methodologies to get one clear and bright view of that's going on. Obviously, having all method entries and returns coming from two probes isn't ideal and -- in the fullness of time -- we hope to have much better and more tightly integrated support Java in DTrace, but this is a first step and you can use it today. The performance impact can be pretty hefty depending on how you invoke the agent, so while this may or may not be applicable in production, this should be a huge benefit for developers (including, apparently, our own in-house Java developers -- care to explain all the calls to `mutex\_trylock(3c)`?).

I encourage you to check out Kelly's [DTrace/Java Agents](https://solaris10-dtrace-vm-agents.dev.java.net/) and feel free to comment about your experiences.

* * *

Technorati tag: [DTrace](http://technorati.com/tag/DTrace)
