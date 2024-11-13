---
title: "Number 17 of 20: java stack traces"
date: "2004-07-20"
categories: 
  - "opensolaris"
---

[go to the Solaris 10 top 11-20 list for more](http://dtrace.org/blogs/ahl/the_solaris_10_top_11)

Here's a little secret about software development: different groups usually aren't that good at working with one another. That's probably not such a shocker for most of you, but the effects can be seen everywhere, and that's why tight integration can be such a distinguishing feature for a collection of software.

About a year and a half ago, we had the DTrace prototype working on much of the system: from kernel functions, through system calls, to every user-land function and instruction. But we were focused completely on C and C++ based applications and this java thing seemed to be catching on. In a radical move, we worked with some of the java guys to take the first baby step in making DTrace and Solaris's other observability tools begin to work with java.

### ustack() action for java

One of the most powerful features of DTrace is its ability to correlate low level events in the kernel -- disk I/O, scheduler events, networking, etc. -- with user-land activity. What application is generating all this I/O to this disk? DTrace makes answering that a snap. But what about when you want to dive deeper? What is that application actually doing to generate all that kernel activity? The ustack() action records the user-land stack backtrace so even in that prototype over a year ago, you could hone in on the problem.

Java, however, was still a mystery. Stacks in C and C++ are fairly easy to record, but in java, some methods are interpretted and just-in-time (JIT) compilation means that other methods can move around in the java virtual machine's (JVM) address space. DTrace needed help from the JVM. Working with the java guys, we built a facility where the JVM actually contains a little bit of D (DTrace's C-like language) machinery that knows how to interpret java stacks. We enhanced the ustack() action to take an optional second argument for the number of bytes to record (we've also recently added the jstack() action; see the [DTrace Solaris Express Schedule](http://blogs.sun.com/roller/page/ahl/dtracesched) for when it will be available) so when we use the ustack() action in the kernel on a thread in the JVM, that embedded machinery takes over and fills in those bytes with the symbolic interpretation for those methods. Either [Bryan](http://blogs.sun.com/bmc) or I will give a more complete (and comprehensible) description in the future, but an example should speak volumes:

```
# dtrace -n profile-100'/execname == "java"/{ @[ustack(50, 512)] = count() }'
...
java/security/AccessController.doPrivileged
java/net/URLClassLoader.findClass
java/lang/ClassLoader.loadClass
sun/misc/Launcher$AppClassLoader.loadClass
java/lang/ClassLoader.loadClass
java/lang/ClassLoader.loadClassInternal
StubRoutines (1)
...

```

It seems simple, but there's a lot of machinery behind this simple view, _and_ this is actually an incredibly powerful and unique view of the system. Maybe you've had a java application that generated a lot of I/O or had some unexpected latency -- using DTrace and its java-enabled ustack() action, you can finally track the problem down.

### pstack(1) for java

While we had the java guys in the room, we couldn't pass up the opportunity to collaborate on getting stacks working in another observability tool: [pstack(1)](http://docs.sun.com/db/doc/817-0689/6mgfkpd1d?a=view). The pstack(1) utility can print out the stack traces of all the threads in a live process or a [core file](http://dtrace.org/blogs/ahl/number_13_of_20_core). We implemented it slightly differently than DTrace's ustack() action, but pstack(1) now works on java processes and java core files.

Collaboration is a great thing, and I hope you find the fruits of collaborative effort useful. These are just the first steps -- we have much more planned for integrating Solaris and DTrace with java.
