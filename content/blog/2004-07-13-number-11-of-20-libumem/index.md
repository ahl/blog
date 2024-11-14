---
title: "Number 11 of 20: libumem"
date: "2004-07-13"
categories:
  - "opensolaris"
permalink: /2004/07/13/number-11-of-20-libumem/
---

[go to the Solaris 10 top 11-20 list for more](http://dtrace.org/blogs/ahl/the_solaris_10_top_11)

### libumem

In Solaris 2.4 we replaced the old buddy allocator1 the slab allocator2 invented by Jeff Bonwick. The slab allocator is covered in pretty much every operating systems text book -- and that's because most operating systems are now using it. In Solaris 103, Jonathan Adams brought the slab allocator to user-land in the form of **libumem**4.

Getting started with `libumem` is easy; just do the [linker](http://blogs.sun.com/rie) trick of setting `LD\_PRELOAD` to "libumem.so" and any program you execute will use libumem's `malloc(3C)` and `free(3C)` (or `new` and `delete` if you're into that sort of [thing](http://nothings.org/computer/cpp.html)). Alteratively, if you like what you see, you can start linking your programs against libumem by passing `\-lumem` to your compiler or linker. But I'm getting ahead of myself; why is `libumem` so great?

### Scalability

The slab allocator is designed for systems with many threads and many CPUs. Memory allocation with naive allocators can be a _serious_ bottleneck (in fact we recently used [DTrace](http://www.sun.com/bigadmin/content/dtrace/) to find such a bottleneck; using libumem got us a 50% improvement). There are other highly scalable allocators out there, but libumem is about the same or better in terms of performance, has compelling debugging features, and it's free and fully supported by Sun.

### Debugging

The scalability and performance are impressive, but not unique to libumem; where libumem really sets itself apart is in debugging. If you've ever spent more than 20 seconds debugging heap corruption or chasing down a memory leak, you _need_ libumem. Once you've used libumem it's hard to imagine debugging this sort of problem with out it.

You can use `libumem` to find double-frees, use-after-free, and many other problems, but my favorite is memory leaks. Memory leaks can really be a pain especially in large systems; libumem makes leaks easy to detect, and easy to diagnose. Here's a simple example:

```
$ LD_PRELOAD=libumem.so
$ export LD_PRELOAD
$ UMEM_DEBUG=default
$ export UMEM_DEBUG
$ /usr/bin/mdb ./my_leaky_program
> ::sysbp _exit
> ::run
mdb: stop on entry to _exit
mdb: target stopped at:
libc.so.1`exit+0x14:    ta        8
mdb: You've got symbols!
mdb: You've got symbols!
Loading modules: [ ld.so.1 libumem.so.1 libc.so.1 ]
> ::findleaks
CACHE     LEAKED   BUFCTL CALLER
0002c508       1 00040000 main+4
----------------------------------------------------------------------
Total       1 buffer, 24 bytes
> 00040000::bufctl_audit
ADDR  BUFADDR    TIMESTAMP THR  LASTLOG CONTENTS    CACHE     SLAB     NEXT
DEPTH
00040000 00039fc0 3e34b337e08ef   1 00000000 00000000 0002c508 0003bfb0 00000000
5
libumem.so.1`umem_cache_alloc+0x13c
libumem.so.1`umem_alloc+0x60
libumem.so.1`malloc+0x28
main+4
_start+0x108

```

Obviously, this is a toy leak, but you get the idea, and it's really that simple to find memory leaks. Other utilities exist for debugging memory leaks, but they dramatically impact performance (to the point where it's difficult to actually run the thing you're trying to debug), and can omit or incorrectly identify leaks. Do you have a memory leak today? Go download Solaris Express, slap your app on it and run it under libumem. I'm sure it will be well worth the time spent.

You can use other `mdb` dcmds like ::umem\_verify to look for corruption. The kernel versions of these dcmds are described in the [Solaris Modular Debugger Guide](http://docs.sun.com/db/doc/806-6545) today; we'll be updating the documentation for Solaris 10 to describe all the libumem debugging commands.

### Programmatic Interface

In addition to offering the well-known `malloc()` and `free()`, also has a programmatic interface for creating your own object caches backed by the heap or memory mapped files or whatever. This offers additional flexibility and precision and allows you to futher optimize your application around libumem. Check out the man pages for [`umem\_alloc()`](http://docs.sun.com/db/doc/817-0692/6mgfnkutq?a=view) and [`umem\_cache\_alloc()`](http://docs.sun.com/db/doc/817-0692/6mgfnkutr?a=view) for _all_ the details.

### Summary

Libumem is a hugely important feature in Solaris 10 that just slipped off top 10 list, but I doubt there's a Solaris user (or soon-to-be Solaris user) that won't fall in love with it. I've only just touched on what you can do with libumem, but Jonathan Adams (libumem's author) will soon be joining the ranks of blogs.sun.com to tell you more. Libumem is fast, it makes debugging a snap, it's easy to use, and you can get down and dirty with it's expanded API -- what else couldn anyone ask for in an allocator?

1\. Jeff's [USENIX paper](http://www.usenix.org/publications/library/proceedings/bos94/bonwick.html) is definitely worth a read  
2\. For more about Solaris history, and the internals of the slab allocator check out [Solaris Internals](http://www.solarisinternals.com)  
3\. Actually, Jonathan slipped libumem into Solaris 9 Update 3 so you might have had libumem all this time and not known...  
4\. Jeff and Jonathan wrote a [USENIX paper](http://www.usenix.org/event/usenix01/bonwick.html) about some additions to the allocator and its extension to user-land in the form of libumem
