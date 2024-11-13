---
title: "Delphix launch at DEMO"
date: "2010-09-14"
categories: 
  - "delphix"
tags: 
  - "delphix"
  - "dtrace"
  - "fishworks"
  - "tivo"
  - "virtualization"
  - "vmware"
---

At 4:08pm today, we will launch Delphix Server at DEMO. At the presentation, Richard Rothschild from TiVo will describe how they have been using Delphix. [TiVo](http://www.tivo.com/), of course, has been canonized as technology that changes the way we live or work. My past work on [DTrace](http://en.wikipedia.org/wiki/DTrace) was described by users as "[TiVo for the kernel](http://quantblog.wordpress.com/2008/10/01/dtrace-fascinating-hilarious-google-video-talk/)" — a revolutionary technology from which there is no going back. The comment that “[Delphix is like TiVo for databases](http://delphix.com/company.php?tab=demo2010news)" is all the more profound coming from the Senior Director of IT at TiVo.

I was introduced to the notion of virtualization in 2000 when some of my classmates signed on at VMware. Later in my work at [Fishworks](http://blogs.sun.com/fishworks) building storage products, storage virtualization was an important topic. Virtualization of both servers and storage decouples the work that's being done, the computation or I/O and data persistence, from specific, inflexible physical resources. Those physical resources can then be used more efficiently (saving money) and moved around more quickly and easily (saving time).

Databases, while not themselves physical resources, are bound to physical resources in an analogous way. They are tied to equipment, duplicated many times over, and complex to provision and deploy — much like both servers and storage. Delphix brings virtualization to the realm of databases, abstracting them away from their physical resources, and creating the same benefits as virtualizing compute or disk. In contrast though, since databases are not tangible like a rack-mount server or storage array, databases can be seamlessly pulled into the virtualization framework, or, just as easily, recast and deployed as physical databases.

Check out the broadcast live today (or [here](http://www.demo.com/alumni/demo2010fall/219461.html) after the fact), or take a look at today's [eWeek article](http://www.eweek.com/c/a/Virtualization/Newcomer-Delphix-Launches-First-Virtualized-Database-Platform-150288/). We've also got some videos posted with [more info on the product](http://delphix.com/resources.php?tab=product-demo) and [customer testimonials](http://delphix.com/customers.php?tab=customer-testimonials). As I write this, I'm midway through my second day at the company; I look forward to writing more about the technology as I dive in.
