---
title: "APFS in Detail: Overview"
date: "2016-06-19"
categories:
  - "software"
tags:
  - "apfs"
---

<img src="images/wwdc16-og.jpg" alt="Description" class="float-right">Apple announced a new file system that will make its way into all of its OS variants (macOS, tvOS, iOS, watchOS) in the coming years. Media coverage to this point has been mostly breathless elongations of [Apple’s developer documentation](https://developer.apple.com/library/prerelease/content/documentation/FileManagement/Conceptual/APFS_Guide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40016999-CH1-DontLinkElementID_27). With a dearth of detail I decided to attend the [presentation](http://devstreaming.apple.com/videos/wwdc/2016/701q0pnn0ietcautcrv/701/701_introducing_apple_file_system.pdf) and Q&A with the APFS team at WWDC. Dominic Giampaolo and Eric Tamura, two members of the APFS team, [gave an overview to a packed room](https://developer.apple.com/videos/play/wwdc2016/701/); along with other members of the team, they patiently answered questions later in the day. With those data points and some first hand usage I wanted to provide an overview and analysis both as a user of Apple-ecosystem products and as a long-time operating system and file system developer.

I've divided my review into several sections that span a few posts. I'd encourage you to jump around to topics of interest or skip right to [the conclusion](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part6/#apfs-conclusion) (or to the [tweet summary](https://twitter.com/ahl/status/743923994466758657)). Highest praise goes to [encryption](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part2/#apfs-encryption); ire to [data integrity](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part5/#apfs-data).

- [Basics](#apfs-basics)
- [Paying Down Debt](#apfs-debt)
- [Encryption](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part2/#apfs-encryption)
- [Snapshots and Backup](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part2/#apfs-snapshots)
- [Management and Space Sharing](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part2/#apfs-management)
- [Space Efficiency and Clones](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part3/#apfs-clones)
- [Performance](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part4/#apfs-performance)
- [Data Integrity](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part5/#apfs-data)
- [Conclusion](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part6/#apfs-conclusion)

## Basics

APFS, the Apple File System, was itself started in 2014 with Dominic as its lead engineer. It's a [stand-alone, from-scratch implementation](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part1/#comment-55954) (an earlier version of this post noted a dependency on [Core Storage](https://en.wikipedia.org/wiki/Core_Storage), but Dominic set me straight). I asked him about looking for inspiration in other modern file systems such as [BSD’s HAMMER](https://www.dragonflybsd.org/hammer/), [Linux’s btrfs](https://btrfs.wiki.kernel.org/index.php/Main_Page), or [OpenZFS](http://open-zfs.org/wiki/Main_Page) (Solaris, illumos, [FreeBSD](https://www.freebsd.org/doc/handbook/zfs.html), [Mac OS X](https://openzfsonosx.org), [Ubuntu Linux](https://insights.ubuntu.com/2016/02/16/zfs-is-the-fs-for-containers-in-ubuntu-16-04/), etc.), all of which have features similar to what APFS intends to deliver. (And note that Apple built a fairly complete port of ZFS, though Dominic was not apparently part of the group advocating for it.) Dominic explained that while, as a self-described file system guy (he built the file system in BeOS, unfairly relegated to obscurity when Apple opted to purchase NeXTSTEP instead), he was aware of them, but didn’t delve too deeply for fear, he said, of tainting himself.

Dominic praised the APFS testing team as being exemplary. This is absolutely critical. A common adage is that it takes a decade to mature a file system. And my experience with ZFS more or less confirms this. Apple will be delivering APFS broadly with 3-4 years of development so will need to accelerate quickly to maturity.

## Paying Down Debt

HFS was introduced in 1985 when the Mac 512K (of memory! Holy smokes!) was Apple’s flagship. HFS+, a significant iteration, shipped in 1998 on the G3 PowerMacs with 4GB hard drives. Since then storage capacities have increased by factors of 1,000,000 and 1,000 respectively. HFS+ has been pulled in a bunch of competing directions with different forks for different devices (e.g. the iOS team created their own HFS variant, working so covertly that not even the Mac OS team knew) and different features (e.g. journaling, case insensitive). It’s old; it’s a mess; and, critically, it’s missing a bunch of features that are really considered the basic cost of doing business for most operating systems. [Wikipedia lists](https://en.wikipedia.org/wiki/HFS_Plus#Limitations) nanosecond timestamps, checksums, snapshots, and sparse file support among those missing features. Add to that the obvious gap of large device support and you’ve got a big chunk of the APFS feature list.

APFS first and foremost pays down the unsustainable technical debt that Apple has been carrying in HFS+. (In 2001 ZFS grew from a similar need where UFS had been evolved since 1977.) It unifies the multifarious forks. It introduces the expected features. In general it first brings the derelict building up to code.

Compression is an obvious gap in the APFS feature list that is common in many file systems. It’s conceptually quite easy, I told the development team (we had it in ZFS from the outset), so why not include it? To appeal to Dominic’s BeOS nostalgia I even recalled my job interview with Be in 2000 when they talked about how compression actually improved overall performance since data I/O is far more expensive than computation (obvious now, but novel then). The Apple folks agreed, and—in typical Apple fashion—neither confirmed nor denied while strongly implying that it’s definitely a feature we can expect in APFS. I’ll be surprised if compression isn’t included in its public launch.

 

_Next in this series: [Encryption, Snapshots, and Backup](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part2/)_
