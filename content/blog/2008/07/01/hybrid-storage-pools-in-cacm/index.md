---
title: "Hybrid Storage Pools in CACM"
date: "2008-07-01"
categories:
  - "fishworks"
tags:
  - "cacm"
  - "flash"
  - "hsp"
  - "zfs"
permalink: /2008/07/01/hybrid-storage-pools-in-cacm/
---

[![](images/cacm_2008_07.jpg)](http://mags.acm.org/communications/200807)

As I mentioned in my [previous post](http://dtrace.org/blogs/ahl/flash_hybrid_pools_and_future), I wrote an article about the hybrid storage pool (HSP); that article appears in the recently released [July issue of Communications of the ACM](http://mags.acm.org/communications/200807/). You can find it [here](http://mags.acm.org/communications/200807/?searchterm=adam+leventhal&pg=49). In the article, I talk about a novel way of augmenting the traditional storage stack with flash memory as a new level in the hierarchy between DRAM and disk, as well as the ways in which we've adapted ZFS and optimized it for use with flash.

So what's the impact of the HSP? Very simply, the article demonstrates that, considering the axes of cost, throughput, capacity, IOPS and power-efficiency, HSPs can match and exceed what's possible with either drives or flash alone. Further, an HSP can be built or modified to address specific goals independently. For example, it's common to use 15K RPM drives to get high IOPS; unfortunately, they're expensive, power-hungry, and offer only a modest improvement. It's possible to build an HSP that can match the necessary IOPS count at a much lower cost both in terms of the initial investment and the power and cooling costs. As another example, people are starting to consider all-flash solutions to get very high IOPS with low power consumption. Using flash as primary storage means that some capacity will be lost to redundancy. An HSP can provide the same IOPS, but use conventional disks to provide redundancy yielding a significantly lower cost.

My hope — perhaps risibly naive — is that HSPs will mean the eventual death of the 15K RPM drive. If it also puts to bed the notion of flash as general purpose mass storage, well, I'd be happy to see that as well.
