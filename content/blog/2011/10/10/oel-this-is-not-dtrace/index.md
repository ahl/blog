---
title: "Oracle&#039;s port: this is not DTrace"
date: "2011-10-10"
categories: 
  - "dtrace"
tags: 
  - "dtrace"
  - "oel"
  - "oracle"
---

After writing about [Oracle's port of DTrace to OEL](http://dtrace.org/blogs/ahl/2011/10/05/dtrace-for-linux-2/), I wanted to take it for a spin. Following the [directions that Wim Coekaerts spelled out](http://blogs.oracle.com/wim/entry/trying_out_dtrace), I installed and configured a VM to run OEL with Oracle's nascent DTrace port. Setting up the system was relatively painless.

Here's my first DTrace invocation on OEL:

```
[root@screven ~]# uname -a
Linux screven 2.6.32-201.0.4.el6uek.x86_64 #1 SMP Tue Oct 4 16:47:00 EDT 2011 x86_64 x86_64 x86_64 GNU/Linux
[root@screven ~]# dtrace -n 'BEGIN{ trace("howdy from linux"); }'
dtrace: description 'BEGIN' matched 1 probe
CPU     ID                    FUNCTION:NAME
0      1                           :BEGIN   howdy from linux
^C
```

Then I wanted to see what was on the system:

```
[root@screven ~]# dtrace -l | wc -l
574
```

Are you kidding me? For comparison, my Mac has 154,918 probe available and our illumos-derived [Delphix](http://www.delphix.com) OS has 77,320 (Mac OS X has many probes pre-created for each process). It looks like this beta only has the syscall provider, but digging around I can see that Wim didn't mention that the profile provider is also there:

```
[root@screven ~]# modprobe profile
[root@screven ~]# dtrace -l | wc -l
587
```

Sweet.

```
[root@screven ~]# dtrace -n profile:::profile-997
dtrace: failed to enable 'profile:::profile-997': Failed to enable probe
```

Not that sweet.

At least I can run my favorite DTrace script:

```
[root@screven ~]# dtrace -n syscall:::entry'{ @[execname] = count(); }'
dtrace: description 'syscall:::entry' matched 285 probes
^C
pickup                                                            9
abrtd                                                            11
qmgr                                                             17
rsyslogd                                                         25
rs:main Q:Reg                                                    35
master                                                           52
tty                                                              60
dircolors                                                        80
hostname                                                         92
tput                                                             92
id                                                              198
unix_chkpwd                                                     550
auditd                                                          599
dtrace                                                          760
bash                                                           1515
sshd                                                           8327
```

I wanted to trace activity when I connected to the system using ssh... but **ssh logins fail with all probes enabled**. To repeat: ssh logins fail with DTrace probes enabled. I'd try to debug it, but I'm too dejected.

### Evaluation

While I'd like to give this obviously nascent port the benefit of the doubt, its current state is frankly embarrassing. It's very clear now why Oracle wasn't demonstrating this at OpenWorld last week: it doesn't stand up to the mildest level of scrutiny. It's fine that Oracle has embarked on a port of DTrace to the so-called unbreakable kernel, but this is months away from being usable. Announcing a product of this low quality and value calls into question Oracle's credibility as a technology provider. Further, this was entirely avoidable; there were [other DTrace ports to Linux](http://www.crisp.demon.co.uk/blog/2011-06.html) that Oracle could have used as a starting point to produce something much closer to functional.

### This is not DTrace

So, OEL users, know that this is not DTrace. This is no better than one of the [DTrace knockoffs](http://blogs.oracle.com/ahl/entry/dtrace_knockoffs) and in many ways much worse. What Oracle released is worse than worthless by violating perhaps the most fundamental tenet of DTrace: don't damage the system. And, to the OEL folks, I'm sure you'll get there, but how about you take down your beta until it's ready? As it is, people might get the wrong impression about [what DTrace is](http://cld.blog-city.com/wsj_2006_innovation_award_gold_winner__dtrace_and_the_troubl.htm).
