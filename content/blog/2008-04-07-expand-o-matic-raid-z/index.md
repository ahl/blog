---
title: "Expand-O-Matic RAID-Z"
date: "2008-04-07"
categories:
  - "zfs"
tags:
  - "opensolaris"
  - "raid-z"
  - "solaris"
  - "zfs"
permalink: /2008/04/07/expand-o-matic-raid-z/
---

I was having a conversation with an OpenBSD user and developer the other day, and he mentioned some ongoing work in the community to consolidate support for RAID controllers. The problem, he was saying, was that each controller had a different administrative model and utility -- but all I could think was that the _real_ problem was the presence of a RAID controller in the first place! As far as I'm concerned, [ZFS](http://opensolaris.org/os/community/zfs/) and [RAID-Z](http://en.wikipedia.org/wiki/Non-standard_RAID_levels#RAID-Z) have obviated the need for hardware RAID controllers.

ZFS users seem to love RAID-Z, but a [frustratingly](http://www.opensolaris.org/jive/thread.jspa?messageID=15219) [frequent](http://www.opensolaris.org/jive/thread.jspa?messageID=51601) [request](http://www.opensolaris.org/jive/thread.jspa?messageID=135517) is to be able to expand the width of a RAID-Z stripe. While the ZFS community may care about solving this problem, it's not the highest priority for Sun's customers and, therefore, for the ZFS team. It's common for a home user to want to increase his total storage capacity by a disk or two at a time, but enterprise customers typically want to grow by multiple terabytes at once so adding on a new RAID-Z stripe isn't an issue. When the request has come up on the ZFS discussion list, we have, perhaps unhelpfully, pointed out that the code is all open source and ready for that contribution. Partly, it's because we don't have time to do it ourselves, but also because it's a tricky problem and we weren't sure how to solve it.

[Jeff Bonwick](http://blogs.sun.com/bonwick) did a great job explaining [how RAID-Z works](http://blogs.sun.com/bonwick/entry/raid_z), so I won't go into it too much here, but the structure of RAID-Z makes it a bit trickier to expand than other RAID implementations. On a typical RAID with N+M disks, N data sectors will be written with M parity sectors. Those N data sectors may contain unrelated data so adding modifying data on just one disk involves reading the data off that disk and updating both those data and the parity data. Expanding a RAID stripe in such a scheme is as simple as adding a new disk and updating the parity (if necessary). With RAID-Z, blocks are never rewritten in place, and there may be multiple logical RAID stripes (and multiple parity sectors) in a given row; we therefore can't expand the stripe nearly as easily.

A couple of weeks ago, I had lunch with [Matt Ahrens](http://blogs.sun.com/ahrens) to come up with a mechanism for expanding RAID-Z stripes -- we were both tired of having to deflect reasonable requests from users -- and, lo and behold, we figured out a viable technique that shouldn't be very tricky to implement. While Sun still has no plans to allocate resources to the problem, this roadmap should lend credence to the suggestion that someone in the community might work on the problem.

The rest of this post will discuss the implementation of expandable RAID-Z; it's not intended for casual users of ZFS, and there are no alchemic secrets buried in the details. It would probably be useful to familiarize yourself with [the basic structure of ZFS](http://opensolaris.org/os/community/zfs/source/), [space maps](http://blogs.sun.com/bonwick/entry/space_maps) (**totally** cool by the way), and the code for [RAID-Z](http://src.opensolaris.org/source/xref/onnv/onnv-gate/usr/src/uts/common/fs/zfs/vdev_raidz.c).

### Dynamic Geometry

ZFS uses vdevs -- virtual devices -- to store data. A vdev may correspond to a disk or a file, or it may be an aggregate such as a mirror or RAID-Z. Currently the RAID-Z vdev determines the stripe width from the number of child vdevs. To allow for RAID-Z expansion, the geometry would need to be a more dynamic property. The storage pool code that uses the vdev would need to determine the geometry for the current block and then pass that as a parameter to the various vdev functions.

There are two ways to record the geometry. The simplest is to use the GRID bits (an 8 bit field) in the DVA (Device Virtual Address) which have already been set aside, but are currently unused. In this case, the vdev would need to have a new callback to set the contents of the GRID bits, and then a parameter to several of its other functions to pass in the GRID bits to indicate the geometry of the vdev when the block was written. An alternative approach suggested by Jeff and [Bill Moore](http://blogs.sun.com/bill) is something they call **time-dependent geometry**. The basic idea is that we store a record each time the geometry of a vdev is modified and then use the creation time for a block to infer the geometry to pass to the vdev. This has the advantage of conserving precious bits in the fixed-width DVA (though at 128 bits its still quite big), but it is a bit more complex since it would require essentially new metadata hanging off each RAID-Z vdev.

### Metaslab Folding

When the user requests a RAID-Z vdev be expanded (via an existing or new zpool(1M) command-line option) we'll apply a new **fold** operation to the space map for each metaslab. This transformation will take into account the space we're about to add with the new devices. Each range \[a, b\] under a fold from width n to width m will become

> \[ m \* (a / n) + (a % n), m \* (b / n) + b % n \]

The alternative would have been to account for m - n free blocks at the end of every stripe, but that would have been overly onerous both in terms of processing and in terms of bookkeeping. For space maps that are resident, we can simply perform the operation on the AVL tree by iterating over each node and applying the necessary transformation. For space maps which aren't in core, we can do something rather clever: by taking advantage of the log structure, we can simply append a new type of space map entry that indicates that this operation should be applied. Today we have allocated, free, and debug; this would add fold as an additional operation. We'd apply that fold operation to each of the 200 or so space maps for the given vdev. Alternatively, using the idea of time-dependent geometry above, we could simply append a marker to the space map and access the geometry from that repository.

Normally, we only rewrite the space map if the on-disk, log-structure is twice as large as necessary. I'd argue that the fold operation should always trigger a rewrite since processing it always requires a O(n) operation, but that's really an ancillary point.

### vdev Update

At the same time as the previous operation, the vdev metadata will need to be updated to reflect the additional device. This is mostly just bookkeeping, and a matter of chasing down the relevant code paths to modify and augment.

### Scrub

With the steps above, we're actually done for some definition since new data will spread be written in stripes that include the newly added device. The problem is that extant data will still be stored in the old geometry and most of the capacity of the new device will be inaccessible. The solution to this is to scrub the data reading off every block and rewriting it to a new location. Currently this isn't possible on ZFS, but Matt and [Mark Maybee](http://blogs.sun.com/markm/) have been working on something they call block pointer rewrite which is needed to solve a variety of other problems and nicely completes this solution as well.

### That's It

After Matt and I had finished thinking this through, I think we were both pleased by the relative simplicity of the solution. That's not to say that implementing it is going to be easy -- there's still plenty of gaps to fill in -- but the basic algorithm is sound. A nice property that falls out is that in addition to changing the number of data disks, it would also be possible to use the same mechanism to add an additional parity disk to go from singl e- to double-parity RAID-Z -- another common request.

So I can now extend a slightly more welcoming invitation to the ZFS community to engage on this problem and contribute in a very concrete way. I've posted some [diffs](http://cr.opensolaris.org/~ahl/expand-o-raid/) which I used sketch out some ideas; that might be a useful place to start. If anyone would like to create a project on OpenSolaris.org to host any ongoing work, I'd be happy to help set that up.
