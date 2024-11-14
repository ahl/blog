---
title: "Another reason Apple misses ZFS"
date: "2011-05-20"
categories: 
  - "zfs"
tags: 
  - "apple"
  - "hsp"
  - "l2arc"
  - "zfs"
---

Apple recently announced a new iMac model -- in itself, only as notable as the seasons -- but with an interesting option: users can choose to have [**both** an HDD and an SSD](http://www.tuaw.com/2011/05/04/new-imacs-and-hdd-ssd-configurations-how-do-they-work/). Their use of these two is absolutely pedestrian, as noted on the Apple store:

> If you configure your iMac with both the solid-state drive and a Serial ATA hard drive, it will come preformatted with Mac OS X and all your applications on the solid-state drive. Then you can use the hard drive for videos, photos, and other files.

Fantastic. This is hierarchical storage management (HSM) as it was conceived by Alan Turing himself as he toiled against the German war machine (if I remember my history correctly). The onus of choosing the right medium for given data is completely on the user. I guess the user who forks over $500 for this fancy storage probably is savvy enough to copy files to and fro, but aren't computers pretty good at automating stuff like that?

Back at Sun, we built the ZFS [Hybrid Storage Pool](http://dtrace.org/blogs/ahl/2008/07/01/hybrid-storage-pools-in-cacm/) (HSP), a system that combines disk, DRAM, and, yes, flash. A significant part of this is the [L2ARC implemented by Brendan Gregg](http://dtrace.org/blogs/ahl/2008/07/01/hybrid-storage-pools-in-cacm/) that uses SSDs as a cache for often-used data. Hey Apple, does this sound useful?

Curiously, the new iMacs [contain the Intel Z68 chipset](http://www.techspot.com/news/43676-apples-new-imac-gets-early-access-to-intels-z68-chipset.html) which provides support for SSD caching. Similar to what ZFS does in software, the Intel chipset stores a subset of the data from the HDD on the SSD. By the time the hardware sees the data, it's stripped of all semantic meaning -- it's just offsets and sizes. In ZFS, however, the L2ARC knows more about the data so can do a better job about retaining data that's relevant. But iMac users suffer a more fundamental problem: the SSD caching feature of the Z68 doesn't appear to be used.

It's a shame that Apple abandoned the port of ZFS they had completed ostensibly due to "[licensing issues](http://arstechnica.com/apple/news/2009/10/apple-abandons-zfs-on-mac-os-x-project-over-licensing-issues.ars)" ([DTrace in Mac OS X](http://dtrace.org/blogs/ahl/2006/08/07/dtrace_on_mac_os_x/) uses the same license -- perhaps a subject for another blog post). Fortunately, [Ten's Complement](http://tenscomplement.com/about-z410) has picked up the reins. Apple systems with HDDs and SSDs could be the ideal use case ZFS in the consumer environment.
