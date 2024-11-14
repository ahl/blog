---
title: "OpenZFS: the next phase of ZFS development"
date: "2013-09-16"
categories: 
  - "zfs"
tags: 
  - "freebsd"
  - "illumos"
  - "linux"
  - "macosx"
  - "mattahrens"
  - "openzfs"
  - "zfs"
---

[![](images/openzfs-trans.png "openzfs-trans")](http://ahl.dtrace.org/wp-content/uploads/2013/09/openzfs-trans.png)I've been watching ZFS from moments after its inception at the hands of [Matt Ahrens](http://blog.delphix.com/matt/) and Jeff Bonwick, so I'm excited to see it enter its newest phase of development in [OpenZFS](http://open-zfs.org/wiki/Announcement). While ZFS has long been regarded as the hottest filesystem on 128 bits, and has shipped in many different products, what's been most impressive to me about ZFS development has been the constant iteration and reinvention.

Before shipping in Solaris 10 update 2, major components of ZFS had already advanced to "2.0" and "3.0". I've been involved with several ZFS-related products: Solaris 10, the ZFS Storage Appliance (nee Sun Storage 7000), and the Delphix Engine. Each new product and each new use has stressed ZFS in new ways, but also brought renewed focus to development. I've come to realize that ZFS will never be completed. I thought I'd use this post to cover the many ways that ZFS had failed in the products I've worked on over the years -- and it has failed spectacularly at time -- but this distracted from the most important aspect of ZFS. For each new failure in each new product with each new use and each new workload ZFS has adapted and improved.

OpenZFS doesn't need a caretaker community for a finished project; if that were the case, porting OpenZFS to Linux, FreeBSD, and Mac OS X would have been the end. Instead, it was the beginning. The need for the OpenZFS community grew out of the porting efforts who wanted the world's most advanced filesystem on their platforms and in their products. I wouldn't trust my customers' data to a filesystem that hadn't been through those trials and triumphs over more than a decade. I can't wait to see the next phase of evolution that OpenZFS brings.

 

If you're at LinuxCon today, stop by the [talk by Matt Ahrens and Brian Behlendor](http://sched.co/15NfsCV) for more on OpenZFS; follow [@OpenZFS](https://twitter.com/openzfs) for all OpenZFS news.
