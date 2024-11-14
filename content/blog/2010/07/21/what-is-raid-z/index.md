---
title: "What is RAID-Z?"
date: "2010-07-21"
categories: 
  - "zfs"
tags: 
  - "fishworks"
  - "hsp"
  - "jeffbonwick"
  - "raid"
  - "raid-z"
  - "zfs"
---

The mission of ZFS was to simplify storage and to construct an enterprise level of quality from volume components by building smarter software — indeed that notion is at the heart of the [7000 series](http://www.oracle.com/us/products/servers-storage/storage/unified-storage/index.html). An important piece of that puzzle was eliminating the expensive RAID card used in traditional storage and replacing it with high performance, software RAID. To that end, [Jeff invented RAID-Z](http://blogs.sun.com/bonwick/entry/raid_z); it's key innovation over other software RAID techniques was to close the "RAID-5 write hole" by using variable width stripes. RAID-Z, however, is definitely not RAID-5 despite that being the most common comparison.

### RAID levels

Last year I wrote about the [need for triple-parity RAID](http://dtrace.org/blogs/ahl/acm_triple_parity_raid), and in that article I summarized the various RAID levels as enumerated by Gibson, Katz, and Patterson, along with Peter Chen, Edward Lee, and myself:

- **RAID-0** Data is striped across devices for maximal write performance. It is an outlier among the other RAID levels as it provides no actual data protection.
- **RAID-1** Disks are organized into mirrored pairs and data is duplicated on both halves of the mirror. This is typically the highest-performing RAID level, but at the expense of lower usable capacity.
- **RAID-2** Data is protected by memory-style ECC (error correcting codes). The number of parity disks required is proportional to the log of the number of data disks.
- **RAID-3** Protection is provided against the failure of any disk in a group of N+1 by carving up blocks and spreading them across the disks — bitwise parity. Parity resides on a single disk.
- **RAID-4** A group of N+1 disks is maintained such that the loss of any one disk would not result in data loss. A single disks is designated as the dedicated parity disk. Not all disks participate in reads (the dedicated parity disk is not read except in the case of a failure). Typically parity is computed simply as the bitwise XOR of the other blocks in the row.
- **RAID-5** N+1 redundancy as with RAID-4, but with distributed parity so that all disks participate equally in reads.
- **RAID-6** This is like RAID-5, but employs two parity blocks, P and Q, for each logical row of N+2 disk blocks.
- **RAID-7** Generalized M+N RAID with M data disks protected by N parity disks (without specifications regarding layout, parity distribution, etc).

### RAID-Z: RAID-5 or RAID-3?

Initially, ZFS supported just one parity disk (raidz1), and later added [two (raidz2)](http://dtrace.org/blogs/ahl/double_parity_raid_z) and then [three (raidz3)](http://dtrace.org/blogs/ahl/triple_parity_raid_z) parity disks. But raidz1 is not RAID-5, and raidz2 is not RAID-6. RAID-Z avoids the RAID-5 write hole by distributing logical blocks among disks whereas RAID-5 aggregates unrelated blocks into fixed-width stripes protected by a parity block. This actually means that RAID-Z is far more similar to RAID-3 where blocks are carved up and distributed among the disks; whereas RAID-5 puts a single block on a single disk, RAID-Z and RAID-3 must access all disks to read a single block thus reducing the effective IOPS.

RAID-Z takes a significant step forward by enabling software RAID, but at the cost of backtracking on the evolutionary hierarchy of RAID. Now with advances like flash pools and [the Hybrid Storage Pool](http://dtrace.org/blogs/ahl/hybrid_storage_pools_in_cacm), the IOPS from a single disk may be of less importance. But a RAID variant that shuns specialized hardware like RAID-Z and yet is economical with disk IOPS like RAID-5 would be a significant advancement for ZFS.
