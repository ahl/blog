---
title: "Mac OS X and the missing probes"
date: "2008-01-18"
categories:
  - "dtrace"
permalink: /2008/01/18/mac-os-x-and-the-missing-probes/
---

As has been [thoroughly](http://www.mactech.com/articles/mactech/Vol.23/23.11/ExploringLeopardwithDTrace/index.html) [recorded](http://dtrace.org/blogs/ahl/dtrace_on_mac_os_x), Apple has [included DTrace in Mac OS X](http://www.apple.com/macosx/technology/unix.html). I've been using it as often as I have the opportunity, and it's a joy to be able to use the fruits of our labor on another operating system. But I hit a rather surprising case recently which led me to discover a **serious** problem with Apple's implementation.

A common trick with [DTrace](http://opensolaris.org/os/community/dtrace/) is to use a `tick` probe to report data periodically. For example, the following script reports the ten most frequently accessed files every 10 seconds:

```
io:::start
{
@[args[2]->fi_pathname] = count();
}
tick-10s
{
trunc(@, 10);
printa(@);
trunc(@, 0);
}

```

This was running fine, but it seemed as though sometimes (particularly with certain apps in the background) it would occasionally skip one of the ten second iterations. Odd. So I wrote the following script to see what was going on:

```
profile-1000
{
@ = count();
}
tick-1s
{
printa(@);
clear(@);
}

```

What this will do is fire a probe at 1000hz on all (logical) CPUs. Running this on a [dual-core machine](http://en.wikipedia.org/wiki/MacBook_Pro) we'd expect to see it print out `2000` each time. Instead I saw this:

```
0  22369                         :tick-1s
1803
0  22369                         :tick-1s
1736
0  22369                         :tick-1s
1641
0  22369                         :tick-1s
3323
0  22369                         :tick-1s
1704
0  22369                         :tick-1s
1732
0  22369                         :tick-1s
1697
0  22369                         :tick-1s
5154

```

Kind of bizarre. The missing `tick-1s` probes explain the values _over_ 2000, but weirder were the values so far _under_ 2000. To explore a bit more I performed another DTrace experiment to see what applications were running when the profile probe fired:

```
# dtrace -n profile-997'{ @[execname] = count(); }'
dtrace: description 'profile-997' matched 1 probe
^C
Finder                                                            1
configd                                                           1
DirectoryServic                                                   2
GrowlHelperApp                                                    2
llipd                                                             2
launchd                                                           3
mDNSResponder                                                     3
fseventsd                                                         4
mds                                                               4
lsd                                                               5
ntpd                                                              6
kdcmond                                                           7
SystemUIServer                                                    8
dtrace                                                            8
loginwindow                                                       9
pvsnatd                                                          21
Dock                                                             41
Activity Monito                                                  45
pmTool                                                           52
Google Notifier                                                  60
Terminal                                                        153
WindowServer                                                    238
Safari                                                         1361
kernel_task                                                    4247

```

While there's nothing suspicious about the output in itself, it was strange because I was listening to music at the time. With [iTunes](http://www.apple.com/itunes/). **Where was iTunes?**  
I ran the first experiment again and caused iTunes to do more work which yielded these results:

```
0  22369                         :tick-1s
3856
0  22369                         :tick-1s
1281
0  22369                         :tick-1s
4770
0  22369                         :tick-1s
2271

```

So what was iTunes doing? To answer that I again turned to DTrace and used the following enabling to see what functions were being called most frequently by iTunes (whose process ID was 332):

```
# dtrace -n 'pid332:::entry{ @[probefunc] = count(); }'
dtrace: description 'pid332:::entry' matched 264630 probes

```

I let it run for a while, made iTunes do some work, and the result when I stopped the script? Nothing. The expensive DTrace invocation clearly caused iTunes to do a lot more work, but DTrace was giving me no output.  
Which started me thinking... did they? Surely not. They wouldn't disable DTrace for certain applications.

But that's exactly what Apple's done with their DTrace implementation. The notion of true systemic tracing was a bit too egalitarian for their classist sensibilities so they added this glob of lard into `dtrace_probe()` -- the heart of DTrace:

```
#if defined(__APPLE__)
/*
* If the thread on which this probe has fired belongs to a process marked P_LNOATTACH
* then this enabling is not permitted to observe it. Move along, nothing to see here.
*/
if (ISSET(current_proc()->p_lflag, P_LNOATTACH)) {
continue;
}
#endif /* __APPLE__ */

```

Wow. So Apple is **explicitly** preventing DTrace from examining or recording data for processes which don't permit tracing. This is antithetical to the notion of systemic tracing, antithetical to the goals of DTrace, and antithetical to the spirit of open source. I'm sure this was inserted under pressure from ISVs, but that makes the pill no easier to swallow. To say that Apple has crippled DTrace on Mac OS X would be a bit alarmist, but they've certainly undermined its efficacy and, in doing do, unintentionally damaged some of its most basic functionality. To users of Mac OS X and of DTrace: Apple has done a service by porting DTrace, but let's convince them to go one step further and port it properly.
