---
title: "APFS in Detail: Conclusions"
date: "2016-06-19"
categories: 
  - "software"
tags: 
  - "apfs"
---

_This series of posts covers APFS, Apple's new filesystem announced at WWDC 2016. See the [first post]( http://dtrace.org/blogs/ahl/2016/06/19/apfs-part1) for the table of contents._

## Summing Up

I’m not sure Apple absolutely had to replace HFS+, but likely they had passed an inflection point where continuing to maintain and evolve the 30+ year old software was more expensive than building something new. APFS is a product born of that assessment.

Based on what Apple has shown I’d surmise that its core design goals were:

- satisfying all consumers (laptop, phone, watch, etc.)
- encryption as a first-class citizen
- snapshots for modernized backup.

Those are great goals that will benefit all Apple users, and based on the WWDC demos APFS seems to be on track (though the macOS Sierra beta isn’t quite as far along).

In the process of implementing a new file system the APFS team has added some expected features. HFS was built when 400KB floppies ruled the Earth (recognized now only as the ubiquitous and anachronistic save icon). Any file system started in 2014 should of course consider huge devices, and SSDs--check and check. Copy-on-write (COW) snapshots are the norm; making the Duplicate command in the Finder faster wasn’t much of a detour. The use case is unclear, it’s a classic [garbage can theory](http://faculty.babson.edu/krollag/org_site/encyclop/garbage_can.html) solution, a solution in search of a problem, but it doesn’t hurt and it makes for a fun demo. The beach ball of doom earned its nickname; APFS was naturally built to avoid it.

There are some seemingly absent or ancillary design goals: performance, openness, and data integrity. Squeezing the most IOPS or throughput out of a device probably isn’t critical on watchOS, and it’s relevant only to a small percentage of macOS users. It will be interesting to see how APFS performs once it ships (measuring any earlier would only misinform the public and insult the APFS team). The APFS development docs have a bullet on open source: [“An open source implementation is not available at this time.”](https://developer.apple.com/library/prerelease/content/documentation/FileManagement/Conceptual/APFS_Guide/UsingtheAppleFileSystem/UsingtheAppleFileSystem.html#//apple_ref/doc/uid/TP40016999-CH4-DontLinkElementID_23) I don’t expect APFS to be open source at this time or any other, but prove me wrong, Apple. If APFS becomes world-class I’d love to see it in Linux and FreeBSD--maybe Microsoft would even jettison their ReFS experiment. My experience with OpenZFS has shown that open source accelerates that path to excellence. It’s a shame that APFS lacks checksums for user data and doesn’t provide for data redundancy. Data integrity should be job one for a file system, and I believe that that’s true for a watch or phone as much as it is for a server.

At stability, APFS will be an improvement, for Apple users of all kinds, on every device. There are some clear wins and some missed opportunities. Now that APFS has been shared with the world the development team is probably listening. While Apple is clearly years past the decision to build from scratch rather than adopting existing modern technology, there’s time to raise the priority of data integrity and openness. I’m impressed by Apple’s goal of using APFS by default within 18 months. Regardless of how it goes, it will be an exciting transition.
