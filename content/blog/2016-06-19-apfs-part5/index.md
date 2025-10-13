---
title: "APFS in Detail: Data Integrity"
date: "2016-06-19"
categories:
  - "software"
tags:
  - "apfs"
  - "checksums"
  - "raid"
permalink: /2016/06/19/apfs-part5/
---

_This series of posts covers APFS, Apple's new filesystem announced at WWDC 2016. See the [first post]( http://dtrace.org/blogs/ahl/2016/06/19/apfs-part1) for the table of contents._

## Data Integrity

Arguably the most important job of a file system is preserving data integrity. Here’s my data, don’t lose it, don’t change it. If file systems could be trusted absolutely then the “only” reason for backup would be the idiot operators (i.e. you and me). There are a few mechanisms that file systems employ to keep data safe.

### Redundancy

APFS makes no claims with regard to data redundancy. As Apple’s Eric Tamura noted at WWDC, most Apple devices have a single storage device (i.e. one logical SSD) making RAID, for example, moot. Instead redundancy comes from lower layers such as Apple RAID (apparently a thing), hardware RAID controllers, SANs, or even the “single” storage devices themselves..

As an aside note that SSDs in most Apple products where APFS will run include [multiple more-or-less independent NAND chips](https://www.ifixit.com/Teardown/MacBook+Pro+13-Inch+Retina+Display+Early+2015+Teardown/38300#s86949). High-end SSDs do implement data redundancy within the device, but it comes at the price of reduced capacity and performance. As noted above, the “flash-optimization” of APFS doesn’t actually extend much below the surface of the standard block device interface, but the raw materials for innovation are there.

Also, APFS removes the most common way of a user achieving local data redundancy: copying files. A copied file in APFS actually creates a lightweight clone with no duplicated data. Corruption of the underlying device would mean that both “copies” were damaged whereas with full copies localized data corruption would affect just one.

### Crash Consistency

Computer systems can fail at any time—crashes, bugs, power outages, etc.—so file systems need to anticipate and recover from these scenarios. The old old old school method is to plod along and then have a special utility to check and repair the file system during boot (`fsck`, short for file system check). More modern systems labor to achieve an always consistent format, or only narrow windows of inconsistency, obviating the need for the full, expensive `fsck`. ZFS, for example, builds up new state on disk and then atomically transitions from the previous state to the new one with a single atomic operation.

Overwriting data creates the most obvious opening for inconsistency. If the file system needs to overwrite several regions there is a window where some regions represent the new state and some represent the former state. Copy-on-write (COW) is a method to avoid this by always allocating new regions and then releasing old ones for reuse rather than modifying data in-place. APFS claims to implement a “novel copy-on-write metadata scheme”; APFS lead developer Dominic Giampaolo emphasized the novelty of this approach without delving into the details. In conversation later, he made it clear that APFS does not employ the ZFS mechanism of copying all metadata above changed user data which allows for a single, atomic update of the file system structure.

It’s surprising to see that APFS includes `[fsck_apfs](https://forums.developer.apple.com/thread/49207)`—even after asking Dominic I’m not sure why it would be necessary. For comparison I don't believe there’s been an instance where `fsck` for ZFS would have found a problem that the file system itself didn’t already know how to detect. But Dominic was just as confused about why ZFS would forego `fsck`, so perhaps it’s just a matter of opinion.

### Checksums

Notably absent from the APFS intro talk was any mention of checksums. A checksum is a digest or summary of data used to detect (and correct) data errors. The story here is surprisingly nuanced. APFS checksums its own metadata but not user data. The justification for checksumming metadata is strong: there’s relatively not much of it (so the checksums don’t consume much storage) and losing metadata can cast a potentially huge shadow of data loss. If, for example, metadata for a top level directory is corrupted then potentially all data on the disk could be rendered inaccessible. ZFS duplicates metadata (and triple duplicates top-level metadata) for exactly this reason.

Explicitly not checksumming user data is a little more interesting. The APFS engineers I talked to cited strong ECC protection within Apple storage devices. Both flash SSDs and magnetic media HDDs use redundant data to detect and correct errors. The engineers contend that Apple devices basically don’t return bogus data. NAND uses extra data, e.g. 128 bytes per 4KB page, so that errors can be corrected and detected. (For reference, ZFS uses a fixed size 32 byte checksum for blocks ranging from 512 bytes to megabytes. That’s small by comparison, but bear in mind that the SSD’s ECC is required for the expected analog variances within the media.) The devices have a bit error rate that’s tiny enough to expect no errors over the device’s lifetime. In addition, there are other sources of device errors where a file system’s redundant check could be invaluable. SSDs have a multitude of components, and in volume consumer products they rarely contain end-to-end ECC protection leaving the possibility of data being corrupted in transit. Further, their complex firmware can (does) contain bugs that can result in data loss.

The Apple folks were quite interested in my experience with regard to bit rot (aging data silently losing integrity) and other device errors. I’ve seen many instances where devices raised no error but ZFS (correctly) detected corrupted data. Apple has some of the most stringent device qualification tests for its vendors; I trust that they really do procure the best components. Apple engineers I spoke with claimed that bit rot was not a problem for users of their devices, but if your software can’t detect errors then you have no idea how your devices really perform in the field. ZFS has found data corruption on multi-million dollar storage arrays; I would be surprised if it didn’t find errors coming from TLC (i.e. the cheapest) NAND chips in some of Apple’s devices. Recall the (fairly) recent [brouhaha regarding storage problems in the high capacity iPhone 6](http://www.iphonehacks.com/2014/11/128gb-iphone-6-and-plus-boot-loops.html). At least some of Apple’s devices have been imperfect.

As someone who has data he cares about on a Mac, who has seen data lost from HFS, and who knows that even expensive, enterprise-grade equipment can lose data, I would gladly sacrifice 16 bytes per 4KB--less than 1% of my device’s size.

### Scrub

As data ages you might occasionally want to check for bit rot. Likely `fsck_apfs` can accomplish this; as noted though there’s no data redundancy and no checksums for user data, so scrub would only help to find problems and likely wouldn’t help to correct them. And if it makes it any easier for Apple to reverse course, let’s say it’s for the el cheap-o drive I bought from Fry’s not for the gold-plated device I got from Apple.

 

_Next in this series: [Conclusions](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part6/)_
