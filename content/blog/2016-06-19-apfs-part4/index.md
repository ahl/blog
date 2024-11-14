---
title: "APFS in Detail: Performance"
date: "2016-06-19"
categories:
  - "software"
tags:
  - "apfs"
  - "flash"
  - "ftl"
  - "nand"
  - "performance"
  - "ssd"
  - "trim"
permalink: /2016/06/19/apfs-part4/
---

_This series of posts covers APFS, Apple's new filesystem announced at WWDC 2016. See the [first post]( http://dtrace.org/blogs/ahl/2016/06/19/apfs-part1) for the table of contents._

## Performance

APFS claims to be optimized for flash. Flash memory (NAND) is the stuff in your speedy SSD. Apple changed the computing industry when it put flash into the iPod and iPhone, volumes for which fundamentally changed the economics of flash. This consumer change impacted the enterprise (as it often does), giving rise to [hybrid](http://dtrace.org/blogs/ahl/2008/11/10/hybrid-storage-pools-in-the-7410/) and [all-flash arrays](https://techcrunch.com/2015/11/19/how-pure-storage-took-a-different-approach-to-storage/). [Ten years ago flash cost as much as DRAM](http://www.storagesearch.com/ssd-ram-flash%20pricing.html); now it’s challenging the economics of hard disks.

SSDs mimic the block interface of conventional hard drives, but the underlying technology is completely different. In particular while magnetic media can read or write sectors arbitrarily, flash erases large chunks (blocks) and reads and writes smaller chunks (pages). The management is done by what’s called the flash translation layer (FTL), software that makes blocks and pages appear more like a hard drive. An FTL is very similar to a file system, creating a virtual mapping (a translation) between block addresses and locations within the media. Apple controls the full stack including the SSD, FTL, and file system; they could have built something differentiated, [optimizing this components to work together](http://queue.acm.org/detail.cfm?id=2463636). What APFS does, however, is simply write in patterns known to be more easily handled by NAND. It’s a file system with flash-aware characteristics rather than one written explicitly for the native flash interfaces, more or less what you'd expect in 2016.

Also on the topic of flash, APFS includes TRIM support. TRIM is a command in the ATA protocol that allows a file system to indicate to an SSD (specifically, its FTL) that some space has been freed. SSDs require significant free space and perform better when there’s more of it; they include more physical space than they advertise. For example, my 1TB SSD includes 1TB (240 = 10244) bytes of flash but only reports 931GB of available space, sneakily matching the storage industry’s self-serving definition of 1TB (10004 = 1 trillion bytes). With more free space, FTLs can trade off space efficiency for performance and longevity. TRIM has become expected of file systems; it’s unsurprising that APFS supports it. The problem with TRIM though is that it’s only useful when there’s free space: it’s something of a benchmark special. Once your disk is mostly full (as mine are in my laptop and phone basically at all times) TRIM doesn’t do anything for you. I doubt that TRIM will bring any discernible benefit for APFS users beyond the placebo effect of feature parity.

APFS also focuses on latency; Apple’s number one goal is to avoid the beachball of doom. APFS addresses this with I/O QoS (quality of service) to prioritize accesses that are immediately visible to the user over background activity that doesn’t have the same time-constraints. This is inarguably a benefit to users and a sophisticated file system capability.

 

_Next in this series: [Data Integrity](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part5/)_
