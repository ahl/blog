---
title: "ZIL analysis from Chris George"
date: "2010-11-14"
categories:
  - "zfs"
tags:
  - "chrisgeorge"
  - "ddrdrive"
  - "hsp"
  - "openstoragesummit"
  - "zfs"
  - "zil"
permalink: /2010/11/15/zil-analysis-from-chris-george/
---

Chris George from DDRdrive put together a [great presentation](http://www.ddrdrive.com/zil_accelerator.pdf) at the OpenStorage summit looking at the ZFS intent log (ZIL), and how their product is particularly well-suited as a ZIL device. Chris did a particularly interesting analysis of the I/O pattern ZFS generates to ZIL devices (using DTrace of course). With writes to a single ZFS dataset, writes are almost 100% sequential, but with activity to multiple datasets, writes become significantly more non-sequential. The ZIL was initially designed to accelerate performance with a dedicated hard drive, but the [Hybrid Storage Pool](http://dtrace.org/blogs/ahl/2008/07/01/hybrid-storage-pools-in-cacm/) found a significantly better ZIL device in write-optimized, flash SSDs.

In the [7000 series](http://blogs.sun.com/fishworks), the performance of these SSDs — called Logzillas — aren't particularly sensitive to random write patters. Less sophisticated, cheaper SSDs are more significantly impacted by randomness in that both performance and longevity can suffer.

Chris concludes that NV-DRAM is better suited than flash for the ZIL ([Oracle's Logzilla built by STEC](http://www.stec-inc.com/product/zeusiops.php) actually contains a large amount of NV-DRAM). I completely agree; further, if HDDs and commodity SSDs continue to be target ZIL devices, ZFS could and should do more to ensure that writes are sequential.
