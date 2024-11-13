---
title: "Hybrid Storage Pools: The L2ARC"
date: "2008-07-23"
categories: 
  - "fishworks"
---

I've [written recently](http://dtrace.org/blogs/ahl/hybrid_storage_pools_in_cacm) about the hybrid storage pool (HSP), using ZFS to augment the conventional storage stack with flash memory. The resulting system improve performance, cost, density, capacity, power dissipation — pretty much evey axis of importance.

An important component of the HSP is something called the second level adaptive replacement cache (L2ARC). This allows ZFS to use flash as a caching tier that falls between RAM and disk in the storage hierarchy, and permits huge working sets to be serviced with latencies under 100us. My colleague, [Brendan Gregg](http://blogs.sun.com/brendan), implemented the L2ARC, and has written a [great summary of how the L2ARC works and some concrete results](http://blogs.sun.com/brendan/entry/test). **Using the L2ARC, Brendan was able to achieve a 730% performance improvement** over 7200RPM drives. Compare that with 15K RPM drives which will improve performance by at most 100-200%, while costing more, using more power, and delivering less total capacity than Brendan's configuration. Score one for the hybrid storage pool!
