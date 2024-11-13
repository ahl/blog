---
title: "from linux.kernel"
date: "2004-06-29"
categories: 
  - "dtrace"
---

I noticed the following usenet post the other day:

> Hi, I have a fairly busy mailserver which also has a simple iptables  
> ruleset (blocking some IP's) running 2.6.7 with the deadline i/o  
> scheduler. vmstat was reporting that system time was around 80%. I did  
> the following  
>   
> readprofile -r ; sleep 240 ; readprofile -n -m /boot/System.map-\`uname -r\` | sort -rn -k 1,1 | head -22  
>   
> <snip>  
>   
> I am trying to determine where the system time is going and don't have  
> much zen to begin with. Any assistance would be appreciated ?  

[http://groups.google.com/groups?hl=en&lr=&ie=UTF-8&safe=off&selm=2ayz2-1Um-15%40gated-at.bofh.it](http://groups.google.com/groups?hl=en&lr=&ie=UTF-8&safe=off&selm=2ayz2-1Um-15%40gated-at.bofh.it)

Seems like a tricky problem, and there were some responses on the [thread](http://groups.google.com/groups?hl=en&lr=&ie=UTF-8&safe=off&th=12e9197c76285598&rnum=2) proposing some theories on the source of the problem and requesting more data:

> This doesn't look like very intense context switching in either case. 2.6.7  
> appears to be doing less context switching. I don't see a significant  
> difference in system time, either.  
>   
> Could you please send me complete profiles?  

and

> How many context switches do you get in vmstat?  
> Most likely you just have far too many of them. readprofile will attribute  
> most of the cost to finish\_task\_switch, because that one reenables the  
> interrupts (and the profiling only works with interrupts on)  
>   
> Too many context switches are usually caused by user space.  

This is exactly the type of problem that DTrace was designed for -- my system is slow; why? Rather than just having the output from `readprofile`, you could find out exactly what applications are being forced off CPU while they still have work to do, or what system calls are accounting for the most time on the box, or whatever. And not only could you get the answers to these questions, you could do so quickly and then move onto the next question or revise your initial hypothesis. Interestingly, someone brought this up:

> Hmm, Is there a way to determine which syscall would be the culprit. I  
> guess this is where something like DTrace would be invaluable  

[http://groups.google.com/groups?hl=en&lr=&ie=UTF-8&safe=off&selm=2azXY-2MT-27%40gated-at.bofh.it](http://groups.google.com/groups?hl=en&lr=&ie=UTF-8&safe=off&selm=2azXY-2MT-27%40gated-at.bofh.it)

Which got this reply:

> Sounds like a inferior clone of dprobes to me. But I doubt it  
> would help tracking this down.

[http://groups.google.com/groups?hl=en&lr=&ie=UTF-8&safe=off&selm=2aAKq-3jW-19%40gated-at.bofh.it](http://groups.google.com/groups?hl=en&lr=&ie=UTF-8&safe=off&selm=2aAKq-3jW-19%40gated-at.bofh.it)

For the uninitiated, [DProbes](http://www-124.ibm.com/developerworks/oss/linux/projects/dprobes) is a linux kernel patch that provides some dynamic tracing facilities. It fails to meet a number of the requirements of DTrace in that it's not safe to use on production systems, it can silently lose data, and it lacks many of the data aggregation and filtering facilities of DTrace, but perhaps most importantly, it's not in any linux distro by default (our [USENIX paper](http://www.sun.com/bigadmin/content/dtrace/dtrace_usenix.pdf) on DTrace has a more complete discussion of DProbes). The frustrating thing about this post is that DTrace _would solve this problem_ and yet this particular member of the linux community is too myopic to recognize innovation when it doesn't have a penguin on it.

On the other hand, it's great that DTrace was mentioned, and that someone has noticed that this DTrace thing might actually be the tool they've been needing.
