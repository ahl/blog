---
title: "Big News for ZFS on Linux"
date: "2016-03-07"
categories:
  - "zfs"
permalink: /2016/03/07/big-news-for-zfs-on-linux/
---

![](images/Ubuntu-old.png "Ubuntu")Canonical announced a few weeks ago that ZFS will be included in the next release of Ubuntu Linux, on by default and fully supported. And it’s no exaggeration when Dustin Kirkland describes ZFS as "[one of the most exciting new features Linux has seen in a very long time.](https://insights.ubuntu.com/2016/02/16/zfs-is-the-fs-for-containers-in-ubuntu-16-04/)” In the words of our 47th Vice President, [this is a big F—ing deal](https://www.youtube.com/watch?v=HHKq9tt50O8). Ubuntu is an [extremely popular Linux distribution](https://en.wikipedia.org/wiki/Ubuntu_(operating_system)#Installed_base), particularly so for servers, and while the Linux ecosystem doesn’t want for variety when it comes to filesystem choices, there is not a clear champion when it comes to stability, functionality, and performance. By throwing their full weight behind ZFS, Canonical brings the Linux community an enterprise-class, modern filesystem, stable, but still under active development, designed to perform well for a variety of workloads.

### What’s ZFS?

ZFS was originally developed at Sun Microsystem for the Solaris operating system. Some of the most demanding production environments have depended on ZFS for well over a decade. At its core are the principles of data integrity, ease of use, and performance.\[1\] Most notably, ZFS has first-class support for arbitrary numbers of snapshots and writable clones, serialization for replication, compression, and data repair. I’ve contributed code to ZFS at Sun, then Oracle, and to OpenZFS after Oracle abandoned the project in 2010. I’ve also built two products built around ZFS, the ZFS Storage Appliance, a NAS box, and Delphix, a copy data virtual appliance.

### Why ZFS?

![](images/Open-ZFS-Primary-Logo-Colour-150x150.png "OpenZFS")

While the distinguishing features of ZFS are broadly useful, they have become [specifically relevant in a containerizing world](https://docs.docker.com/engine/userguide/storagedriver/zfs-driver/). Users need to save, clone, and replicate containers at will; ZFS provides key facilities for doing so. Containers and ZFS are a fantastic match, something I’ve seen my friends at Joyent demonstrate decisively for the [past decade](http://cuddletech.com/?p=63). Ubuntu has selected the most capable technology for our modern computing ecosystem.

### No good deed...

So high fives and bro hugs all around, right? Not quite. Enter the licensing boogie man. The Linux kernel is licensed under the GPL v2; OpenZFS is licensed under the CDDL. Both are open source, true, but some contend that they are incompatible. Most folks in the tech world—myself among them—spend somewhere in the vicinity of no time at all considering the topic. Far from ignoring it, Canonical had their lawyers review the licenses and [deemed their use of Linux and OpenZFS to be in compliance](https://insights.ubuntu.com/2016/02/18/zfs-licensing-and-linux/). I’m not a lawyer; I don’t have an informed opinion. But there are those who vehemently disagree with Canonical. Notably the [Software Freedom Conservancy](https://sfconservancy.org) whose mission is to "promote, improve, develop, and defend Free, Libre, and Open Source Software (FLOSS) projects” has posted a [lengthy wag of the finger](https://sfconservancy.org/blog/2016/feb/25/zfs-and-linux/) at Canonical. Note that none of this has been specifically tested in the courts so it’s currently just a theoretical disagreement between lawyers (and in many cases, people who engage in lawyerly cosplay).

The wisdom of the crowd has proposed a couple of solutions:

#### “What if we ask Oracle super nicely?"

Oracle holds a copyright on most of OpenZFS since it was forked from the original ZFS code base. It would be within their rights to decide to relicense ZFS under the GPL. Problem solved! No way and not quite. Starting with the easier problem there are many other copyright holders in OpenZFS. It’s not an impossibly large list, but why would they bother? What benefits would they reap when even goodwill isn’t noticeably on offer? And it is the height of delusion to think that Oracle would grow ears to hear, a heart to care, and a brain to decide. Oracle explicitly backed away from OpenSolaris, shutting down the project in 2010. They do not want to encourage open source use of its component technologies. While open source is arguably the most significant shift in technology over the last decade (Stephen O’Grady’s [The Software Paradox](http://www.amazon.com/The-Software-Paradox-Commercial-Market/dp/1491900938) is a must-read), large companies and startups continue to be terrified, confused, and irrational when it comes to open source. Oracle ain’t coming to help.\[2\]

#### “Let’s re-write it! How hard could it be?"

It’s hard to dignify this with an explanation, but OpenZFS is the product of 100s of person-years. It contains some of the most sophisticated mechanisms I've seen designed, by some of the world’s most capable engineers. Re-writing it would probably be no easier than writing it the first time. By way of commentary, this is what makes NIH so distressing. Too often technologies are copied poorly instead of being used and improved, or understood and replaced with something truly superior.

### Now what?

Now that you understand a bit of the context here’s my suggestion: [consider the licenses, but focus on the technology](http://blog.dustinkirkland.com/2016/02/zfs-licensing-and-linux.html). Canonical has (one would presume) chosen to include OpenZFS because it offers the best solution to Ubuntu users. Containers and ZFS are highly complementary with further room to grow together. As with anything, evaluate the technology, evaluate the risks, and move on. Ignore pedants who would deride your pragmatic use of technology as heretical or [immoral](http://thenewstack.io/canonical-encounters-messy-legal-questions-bringing-zfs-ubuntu/).

I personally could not be more excited by the announcement. The Ubuntu community is going to have built-in support for a filesystem that’s better and more capable than anything they’ve had in the past. The OpenZFS community is going to have a ton more users, more interest, and more drivers for innovation. Both are going to be stronger together.

 

 

* * *

\[1\] "ZFS crashed on me once!" Me too, more than once. "ZFS was slow for me!" That happens. "\[some other Linux filesystem\] is better!" Could be, but I doubt it. I'm not denying the events, but this kind of [Inhofian logic](https://www.youtube.com/watch?v=3E0a_60PMR8) doesn't nudge ZFS from its perch.

\[2\] In 2011 the source code for Oracle's new Solaris 11 operating system [appeared on the web replete with CDDL](http://arstechnica.com/business/2011/12/disgruntled-employee-oracle-doesnt-seem-to-care-about-solaris-11-code-leak/) (open source) license notification. For all appearances this looked like open source code, a new version of OpenSolaris. The community asked for clarification: were these stolen goods or something given away intentionally? Was Solaris 11 free and open? Even then Oracle declined to comment.
