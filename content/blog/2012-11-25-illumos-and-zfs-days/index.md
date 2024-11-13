---
title: "illumos and ZFS days"
date: "2012-11-25"
categories: 
  - "illumos"
  - "zfs"
tags: 
  - "hsp"
  - "illumos"
  - "opensolaris"
  - "oraclesolaris"
  - "talk"
  - "zfs"
---

Back in October I was pleased to attend -- and my employer, Delphix, was pleased to sponsor -- [illumos day and ZFS day](http://zfsday.com), run concurrently with Oracle Open World. Inspired by the success of dtrace.conf(12) in the Spring, the goal was to assemble developers, practitioners, and users of ZFS and illumos-derived distributions to educate, share information, and discuss the future.

## illumos day

The week started with the developer-centric illumos day. While illumos picked up the torch when Oracle re-closed OpenSolaris, each project began with a very different focus. Sun and the OpenSolaris community obsessed with inclusion, and developer adoption -- often counterproductively. The illumos community is led by those building products based on the unique technologies in illumos -- DTrace, ZFS, Zones, COMSTAR, etc. While all are welcome, it's those who contribute the most whose voices are most relevant.

I was asked to give a talk about technologies unique to illumos that are unlikely to appear in Oracle Solaris. It was only when I started to prepare the talk that the difference in focuses of illumos and Oracle Solaris fell into sharp focus. In the illumos community, we've advanced technologies such as ZFS in ways that would benefit Oracle Solaris greatly, but Oracle has made it clear that open source is anathema for its version of Solaris. For example, at Delphix we've recently been fixing bugs, asking ourselves, "how has Oracle never seen this?".

Yet the differences between illumos and Oracle Solaris are far deeper. In illumos we're building products that rely on innovation and differentiation in the operating system, and it's those higher-level products that our various customers use. At Oracle, the priorities are more traditional: support for proprietary SPARC platforms, packaging and updating for administrators, and ease-of-use. In my talk, rather than focusing on the sundry contributions to illumos, I picked a few of my favorites. The [slides](http://www.slideshare.net/ahl0003/illumos-innovations-that-will-never-be-in-oracle-solaris) are more or less incomprehensible on their own; many thanks to Deirdre Straughan for posting the video (and for putting together the event!) -- check out 40:30 for a photo of Jean-Luc Picard attending the DTrace talk at OOW.

\[youtube\_sc url="https://www.youtube.com/watch?v=7YN6\_eRIWWc&t=0m19s"\]

## ZFS day

While illumos day was for developers, ZFS day was for users of ZFS to learn from each others' experiences, and hear from ZFS developers. I had the ignominous task of presenting an update on the [Hybrid Storage Pool (HSP)](http://dtrace.org/blogs/ahl/2008/07/01/hybrid-storage-pools-in-cacm/). We developed the HSP at [Fishworks](https://blogs.oracle.com/fishworks/entry/all_together_now) as the first enterprise storage system to add flash memory into the storage hierarchy to accelerate reads and writes. Since then, economics and physics have thrown up some obstacles: [DRAM has gotten cheaper](http://www.jcmit.com/mem2012.htm), and [flash memory is getting harder and harder to turn into a viable enterprise solution](http://www.theregister.co.uk/2012/10/12/nand_shrink_trap/). In addition, the L2ARC that adds flash as a ZFS read cache, has languished; it has serious problems that no one has been motivated or proficient enough to address.

I'll warn you that after the explanation of the HSP, it's mostly doom and gloom (also I was sick as a dog when I prepared and gave the talk), but check out the [slides](http://www.slideshare.net/ahl0003/hybrid-storage-pools-now-with-the-benefit-of-hindsight) and video for more on the promise and shortcomings of the HSP.

\[youtube\_sc url="http://www.youtube.com/watch?v=P77HEEgdnqE&feature=youtu.be"\]

## Community

For both illumos day and ZFS day, it was a mostly full house. Reuniting with the folks I already knew was fun as always, but even better was to meet so many who I had no idea were building on illumos or using ZFS. The events highlighted that we need to facilitate more collaboration -- especially around ZFS -- between the illumos distros, FreeBSD, and Linux -- hell, even Oracle Solaris would be welcome.
