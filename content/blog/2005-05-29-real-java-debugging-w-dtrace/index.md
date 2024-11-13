---
title: "Real Java debugging w/ DTrace"
date: "2005-05-29"
categories: 
  - "dtrace"
---

When I was in [college](http://www.brown.edu) one of the rights of passage in the [computer science](http://www.cs.brown.edu) department was the [software engineering class](http://www.cs.brown.edu/courses/cs032) which involved a large group project. Fresh from completing that class, my brother turned up the other day in San Francisco (where I live); naturally I wanted to try out the game he and his friends had written. [Hogs](http://sourceforge.net/projects/hogs/) is a 3-D tank game written in Java -- when it failed to run on my Solaris 10 laptop I decided to use the new [DTrace agents for the JVM](https://solaris10-dtrace-vm-agents.dev.java.net/) that I blogged about recently.

After downloading the game and the requisite libraries ([jogl](https://jogl.dev.java.net/), OGL, etc.) I tried running it and got this:

```
java.net.UnknownHostException: epizooty: epizooty
at java.net.InetAddress.getLocalHost(InetAddress.java:1308)
at hogs.net.client.RemoteEngine.(RemoteEngine.java:79)
at hogs.net.client.NetEngine.forHost(NetEngine.java:93)
at hogs.common.Controller.(Controller.java:226)
at hogs.common.Controller.main(Controller.java:118)

```

Without understanding much about Java or anything about how my brother's game worked, I guessed that this code was trying to figure out the hostname of my laptop. The strange thing was that it seemed to find the name -- epizooty -- bu then get confused and throw some exception. The stack backtrace didn't give me much to go on so I decided to put this new Java DTrace agent to the test.

Using the `dvm` provider was, initially, a bit of a pain (through no fault of its own). The `dvm` provider is very easy to use for long running Java programs: you just fire up the JVM and enable the probes at some later time. Because of the failure during start up, the game wasn't sticking around long enough for me to enable the probes. And while dtrace(1M) has a -c option that lets you specify a command to examine with DTrace the `dvm` probes don't show up until a little later when the JVM has initialized. It's worth mentioning that on the next version of Solaris (available via [Solaris Express](http://www.sun.com/software/solaris/solaris-express/get.jsp) we've added a feature that lets you specify probes that don't yet exist that will be enabled when they show up; that feature will be in an early Solaris 10 update. Since this was a stock Solaris 10 system though, I had to get creative.

Using some knowledge of how DTrace user-level statically defined tracing (USDT) providers load, I wrote `stop.d` that waits until the `dvm` provider loads and stops the process. After the process is stopped, another invocation of DTrace can then use the `dvm` provider.

```
#!/usr/sbin/dtrace -s
#pragma D option destructive
syscall::close:entry
/pid == $target &&
basename(curthread->t_procp->p_user.u_finfo.fi_list[arg0].uf_file->f_vnode->v_path) == "dtrace@0:helper"/
{
self->interested = 1;
}
syscall::close:entry
/self->interested/
{
cnt++;
}
syscall::close:entry
/self->interested && cnt == 2/
{
stop();
printf("stopped %d\n", pid);
exit(0);
}
syscall::close:return
/self->interested/
{
self->interested = 0;
}

```

DTrace USDT provider and helpers open a special helper psuedo device to register with the DTrace framework. When they're done, they use the close(2) system call to close the file descriptor to the device. What this script does is look for calls to close(2) where the file descriptor corresponds to that pseudo device. It's worth mentioning here that in the next version of Solaris there's a `fds\[\]` array that gives you the file name and other information for an open file descriptor so this will be a little cleaner in the future. The script looks for the _second_ such close(2) because the JVM itself has a DTrace helper which enables the magic of the `jstack()` action. To be clear: I'm not particularly proud of this script, but it got the job done.

Once I had the game stopped at the right spot, I run amid the noise this snippet looked interesting:

```
0  34481       _method_entry:method-entry -> java/net/InetAddress$1.lookupAllHostAddr()
0  34481       _method_entry:method-entry -> java/net/UnknownHostException.()

```

So this `localAllHostAddr()` method was throwing the exception that was causing me so much heartache. I wanted to understand the actual interaction between this method and lower level address resolution. It turned out that the native library calls were in a shared object that the JVM was lazily loading so I needed to stop the process after the native library had been loaded but before the method had completed. I wrote the following as a sort of conditional breakpoint:

```
#!/usr/sbin/dtrace -s
#pragma D option destructive
dvm$target:::method-entry
/copyinstr(arg1) == "getLocalHost"/
{
self->state = 1;
}
dvm$target:::method-entry
/copyinstr(arg1) == "lookupAllHostAddr" && self->state == 1/
{
self->state = 2;
stop();
exit(0);
}
dvm$target:::method-return
/copyinstr(arg1) == "lookupAllHostAddr" && self->state == 2/
{
self->state = 1;
}
dvm$target:::method-return
/copyinstr(arg1) == "getLocalHost" && self->state == 1/
{
self->state = 0;
}

```

Sifting through some more data, I figured out the name of the native function that was being used to implement `lookupAllHostAddr()` and wrote this script to follow the program flow from there:

```
#!/usr/sbin/dtrace -s
#pragma D option flowindent
pid$target::Java_java_net_Inet4AddressImpl_lookupAllHostAddr:entry
{
self->interested = 1;
}
pid$target:::entry
/self->interested/
{
}
pid$target:::return
/self->interested/
{
printf("+%x %x (%d)", arg0, arg1, errno);
}
pid$target::gethostbyname_r:entry
/self->interested/
{
printf("hostname = %s", copyinstr(arg0));
}
pid$target::Java_java_net_Inet4AddressImpl_lookupAllHostAddr:return
/self->interested/
{
self->interested = 0;
}

```

In the output I found a smoking gun: `gethostbyname\_r(3NSL)` was returning NULL. A little more investigation confirmed that the argument to `gethostbyname\_r(3NSL)` was "epizooty"; a little test program showed the same problem. Now well away from Java and in more familar waters, I quickly realized that adding an entry into `/etc/hosts` was all I needed to do to clear up the problem.

This was a great experience: not only was I able to use this `dvm` stuff to great effect (for which my excitement had been largely theoretical), but I got to prove to my brother how amazingly cool this DTrace thing really is. As I haven't done any serious Java debugging for quite a while I'd like to pose this question to anyone who's managed to stay with me so far: How would anyone debug this without DTrace? Are there other tools that let you observe Java _and_ the native calls _and_ the library routines? And, though I didn't need it here, are there tools that let you correlate Java calls to low level kernel facilities? I welcome your feedback.

* * *

Technorati tag: [DTrace](http://technorati.com/tag/DTrace)
