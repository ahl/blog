---
title: "ZFS trivia: metaslabs and growing vdevs"
date: "2012-11-08"
categories: 
  - "zfs"
tags: 
  - "georgewilson"
  - "mattahrens"
  - "metaslab"
  - "spacemap"
  - "zfs"
---

Lately, I've been rooting around in the bowels of [ZFS](http://en.wikipedia.org/wiki/ZFS) as we've explored some long-standing performance pathologies. To that end I've been fortunate to learn at the feet of [Matt Ahrens](http://blog.delphix.com/matt/) who was half of the ZFS founding team and George Wilson who has forgotten more about ZFS than most people will ever know. I wanted to start sharing some of the interesting details I've unearthed.

For allocation purposes, ZFS carves vdevs (disks) into a number of "metaslabs" -- simply smaller, more manageable chunks of the whole. How many metaslabs? Around 200:

```
void
vdev_metaslab_set_size(vdev_t *vd)
{
        /*
         * Aim for roughly 200 metaslabs per vdev.
         */
        vd->vdev_ms_shift = highbit(vd->vdev_asize / 200);
        vd->vdev_ms_shift = MAX(vd->vdev_ms_shift, SPA_MAXBLOCKSHIFT);
}
```

[http://src.illumos.org/source/xref/illumos-gate/usr/src/uts/common/fs/zfs/vdev.c#1553](https://blogs.oracle.com/bonwick/entry/space_maps)

Why 200? Well, that just kinda worked and was never revisited. Is it optimal? Almost certainly not. Should there be more or less? Should metaslab size be independent of vdev size? How much better could we do? All completely unknown.

The space in the vdev is allotted proportionally, and contiguously to those metaslabs. But what happens when a vdev is expanded? This can happen when a disk is replaced by a larger disk or if an administrator grows a SAN-based LUN. It turns out that ZFS simply creates more metaslabs -- an answer whose simplicity was only obvious in retrospect.

For example, let's say we start with a 2T disk; then we'll have 200 metaslabs of 10G each. If we then grow the LUN to 4TB then we'll have 400 metaslabs. If we started instead from a 200GB LUN that we eventually grew to 4TB we'd end up with 4,000 metaslabs (each 1G). Further, if we started with a 40TB LUN (why not) and grew it by 100G ZFS would not have enough space to allocate a full metaslab and we'd therefore not be able to use that additional space.

At Delphix our metaslabs can become highly fragmented because most of our datasets use a 8K record size (read up on [space maps](https://blogs.oracle.com/bonwick/entry/space_maps) to understand how metaslabs are managed -- truly fascinating), and our customers often expand LUNs as a mechanism for adding more space. It's not clear how much room there is for improvement, but these are curious phenomena that we intend to investigate along with the structure of space maps, the idiosyncrasies of the allocation path, and other aspects of ZFS as we continue to understand and improve performance. Stay tuned.
