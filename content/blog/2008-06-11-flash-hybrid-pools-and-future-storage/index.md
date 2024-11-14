---
title: "Flash, Hybrid Pools, and Future Storage"
date: "2008-06-10"
categories:
  - "fishworks"
permalink: /2008/06/11/flash-hybrid-pools-and-future-storage/
---

Jonathan had a [terrific post](http://blogs.sun.com/jonathan/entry/not_a_flash_in_the) yesterday that does an excellent job of presenting Sun's strategy for flash for the next few years. With my colleagues at Fishworks, an advanced product development team, I've spent more than a year working with flash and figuring out ways to integrate flash into ZFS, the storage hierarchy, and our future storage products — a fact to which John Fowler, EVP of storage, [alluded](http://searchstorage.techtarget.com/news/article/0,289142,sid5_gci1316134,00.html) recently. Flash opens surprising new vistas; it's exciting to see Sun leading in this field, and it's frankly exciting to be part of it.

Jonathan's post sketches out some of the basic ideas on how we're going to be integrating flash into ZFS to create what we call **hybrid storage pools** that combine flash with conventional (cheap) disks to create an aggregate that's cost-effective, power-efficient, and high-performing by capitalizing on the strengths of the component technologies (not unlike a hybrid car). We presented some early results at IDF which has already been getting a [bit of buzz](http://seekingalpha.com/article/72839-sandisk-q1-2008-earnings-call-transcript?source=homepage_transcripts_sidebar&page=4). Next month I have an article in [Communications of the ACM](http://cacm.acm.org/) that provides many more details on what exactly a hybrid pool is and how exactly it works. I've pulled out some excerpts from that article and included them below as a teaser and will be sure to post an update when the full article is available in print and online.

> While its prospects are tantalizing, the challenge is to find uses for flash that strike the right balance of cost and performance. Flash should be viewed not as a replacement for existing storage, but rather as a means to enhance it. Conventional storage systems mix dynamic memory (DRAM) and hard drives; flash is interesting because it falls in a sweet spot between those two components for both cost and performance in that flash is significantly cheaper and denser than DRAM and also significantly faster than disk. Flash accordingly can augment the system to form a new tier in the storage hierarchy – perhaps the most significant new tier since the introduction of the disk drive with RAMAC in 1956.
> 
> ...
> 
> A brute force solution to improve latency is to simply spin the platters faster to reduce rotational latency, using 15k RPM drives rather than 10k RPM or 7,200 RPM drives. This will improve both read and write latency, but only by a factor of two or so. ...
> 
> ...
> 
> ZFS provides for the use of a separate intent-log device, a slog in ZFS jargon, to which synchronous writes can be quickly written and acknowledged to the client before the data is written to the storage pool. The slog is used only for small transactions while large transactions use the main storage pool – it's tough to beat the raw throughput of large numbers of disks. The flash-based log device would be ideally suited for a ZFS slog. ... Using such a device with ZFS in a test system, latencies measure in the range of 80-100µs which approaches the performance of NVRAM while having many other benefits. ...
> 
> ...
> 
> By combining the use of flash as an intent-log to reduce write latency with flash as a cache to reduce read latency, we can create a system that performs far better and consumes less power than other system of similar cost. It's now possible to construct systems with a precise mix of write-optimized flash, flash for caching, DRAM, and cheap disks designed specifically to achieve the right balance of cost and performance for any given workload with data automatically handled by the appropriate level of the hierarchy. ... Most generally, this new flash tier can be thought of as a radical form of hierarchical storage management (HSM) without the need for explicit management.

**Updated July, 1:** I've posted the link to the article in my [subsequent blog post](http://dtrace.org/blogs/ahl/hybrid_storage_pools_in_cacm).
