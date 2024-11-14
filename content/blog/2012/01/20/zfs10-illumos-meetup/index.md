---
title: "ZFS+10: illumos meetup"
date: "2012-01-20"
categories:
  - "delphix"
  - "illumos"
permalink: /2012/01/20/zfs10-illumos-meetup/
---

ZFS recently celebrated its informal [10th anniversary](http://blog.delphix.com/ahl/2011/zfs-10th-anniversary/); to mark the occasion, [Delphix](http://www.delphix.com) hosted a [ZFS-themed meetup](http://www.meetup.com/illumos-User-Group/events/41665962/) for the illumos community (sponsored generously by [Joyent](http://www.joyent.com)). Many thanks to [Deirdre Straughan](https://twitter.com/#!/DeirdreS), the new illumos community manager, for helping to organize and for filming the event. Three of my colleagues at Delphix presented work they've been doing in the ZFS ecosystem.

<iframe width="224" height="126" src="http://www.youtube.com/embed/iJ0S91ygErE" frameborder="0" allowfullscreen class="alignright"></iframe>

[**Matt Ahrens**](http://blog.delphix.com/matt), who (with Jeff Bonwick) invented ZFS back in 2001, started the program with a discussion of a new stable interface for ZFS. Initially **libzfs** had been designed as a set of helper functions in support of the zfs(1M) and zpool(1M) commands; since then, it has outgrown those humble ambitions and a new, simple, stable interface is needed for programmatic consumers of ZFS. In Matt's talk and [blog post](http://blog.delphix.com/matt/2012/01/17/the-future-of-libzfs/), he lays out a series of guiding principles for the new libzfs\_core library; he's already started to implement these ideas for new ZFS features in development at Delphix.

<iframe width="224" height="126" src="http://www.youtube.com/embed/Yp8_hNfUGTg" frameborder="0" allowfullscreen class="alignright"></iframe>

[**John Kennedy**](http://blog.delphix.com/jkennedy) has been working on a relatively neglected part of illumos: automated testing. At the meetup John spoke about the work he's been doing to revitalize the **ZFS test suite**, and to build a unit testing framework for illumos at large. I found the questions and enthusiasm from the people in the room particularly encouraging -- everyone knows that we need to be doing more testing, but until John stepped up, no one was leading the charge. The ZFS test suite is [available on github](https://github.com/delphix/zfstest). Take a look at [John's blog post](http://blog.delphix.com/jkennedy/2012/01/18/resurrecting-the-zfs-test-suite/) to see how to execute the ZFS test suite, and how you can contribute to illumos by helping him diagnose and fix the [60+ outstanding failures](https://github.com/delphix/zfstest/wiki/Known-zfstest-failures).

<iframe width="224" height="126" src="http://www.youtube.com/embed/REzvy59jQnw" frameborder="0" allowfullscreen class="alignright"></iframe>

[**Chris Siden**](http://blog.delphix.com/csiden) has been at Delphix just since he graduated from Brown University this past spring, but he's already made a tremendous impact on ZFS. Chris presented both the work he's done to finish the work started by Basil Crow (also of Brown, and soon full-time at Delphix) on **ZFS feature flags** ([originally presented to the ZFS community by Matt back in May](http://mail.opensolaris.org/pipermail/zfs-discuss/2011-May/048514.html)). Previously, ZFS features followed a single, linear versioning; with Chris and Basil's work it's not a land-grab for the next version, rather each feature can be enabled discretely. Chris also implemented the world's first flagged ZFS feature, Async Destroy (also known to ZFS feature flags as com.delphix:async\_destroy) which allows datasets to be destroyed asynchronously in the background -- a huge boon when destroying gigantic ZFS datasets. Chris also presented some work he's been doing on backward compatibility testing for ZFS; check out his [blog post on both subjects](http://blog.delphix.com/csiden/2012/01/11/illumos-meetup-january-2012/).

The illumos meetup was a great success. Thank you to everyone who attended in person or on the web. To get involved with the ZFS community, join the [illumos ZFS mailing list](https://www.listbox.com/subscribe/?listname=zfs@lists.illumos.org), and for information on the next illumos meetup, [join the group](http://www.meetup.com/illumos-User-Group/members/).
