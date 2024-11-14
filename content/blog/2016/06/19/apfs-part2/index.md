---
title: "APFS in Detail: Encryption, Snapshots, and Backup"
date: "2016-06-19"
categories: 
  - "software"
tags: 
  - "apfs"
---

_This series of posts covers APFS, Apple's new filesystem announced at WWDC 2016. See the [first post]( http://dtrace.org/blogs/ahl/2016/06/19/apfs-part1) for the table of contents._

## Encryption

Encryption is clearly a core feature of APFS. This comes from diverse requirements from the various devices, for example multiple keys within file systems on the iPhone or per-user keys on laptops. I heard the term “innovative” quite a bit at WWDC, but here the term is aptly applied to APFS. It supports several different encryption choices for a file system:

- Unencrypted
- Single-key for metadata and user data
- Multi-key with different choices for metadata, files, and even sections of a file (“extents”)

Multi-key encryption is particularly relevant for portables where all data might be encrypted, but unlocking your phone provides access to an additional key and therefore additional data. Unfortunately this doesn’t seem to be working in the first beta of macOS Sierra (specifying `fileEncryption` when creating a new volume with `diskutil` results in a file system that reports "Is Encrypted" as "No").

Related to encryption, I noticed an undocumented feature while playing around with `diskutil` (which prompts you for interactive confirmation of the destructive power of APFS unless this is added to the command-line: `\-IHaveBeenWarnedThatAPFSIsPreReleaseAndThatIMayLoseData`; I’m not making this up). APFS (apparently) supports constant time cryptographic file system erase, called “effaceable” in the `diskutil` output. This presumably builds a secret key that cannot be extracted from APFS and encrypts the file system with it. A secure erase then need only delete the key rather than needing to scramble and re-scramble the full disk to ensure total eradication. Various iOS docs refer to this capability requiring some specialized hardware; it will be interesting to see what the option means on macOS. Either way, let’s not mention this to the FBI or NSA, agreed?

## Snapshots and Backup

APFS brings a much-desired file system feature: snapshots. A snapshot lets you freeze the state of a file system at a particular moment and continue to use and modify that file system while preserving the old data. It does so in a space-efficient fashion where, effectively, changes are tracked and only new data takes up additional space. This has the potential to be extremely valuable for backup by efficiently tracking the data that has changed since the last backup.

ZFS includes snapshots and serialization mechanisms that make it efficient to backup file systems or transfer file systems to a remote location. Will APFS work like that? Probably not, answered Dominic Giampaolo, APFS lead developer. ZFS sends all changed data while Time Machine can have exclusion lists and the like. That seems surmountable, but we’ll see what Apple does. APFS right now is incompatible with Time Machine due to the lack of directory hard links, a fairly disgusting implementation that likely contributes to Time Machine’s questionable reliability. Hopefully APFS will create some efficient serialization for Time Machine backup.

While Eric Tamura, APFS dev manager, demonstrated snapshots at WWDC, the required utilities aren’t included in the macOS Sierra beta. I used DTrace (technology I’m increasingly amazed that Apple ported from OpenSolaris) to find a tantalizingly-named new system call `fs\_snapshot`; I’ll leave it to others to reverse engineer its proper use.

## Management

APFS brings another new feature known as space sharing. A single APFS “container” that spans a device can have multiple “volumes” (file systems) within it. Apple contrasts this with the static allocation of disk space to support multiple HFS+ instances, which seems both specious and an uncommon use case. Both ZFS and btrfs have a similar concept of a shared pool of storage with nested file systems for administration and management.

Speaking with Dominic and other members of the APFS team, we discussed how volumes are the unit by which users can control things like snapshots and encryption. You’d want multiple volumes to correspond with different policies around those settings. For example while you might want to snapshot and backup your system each day, the massive `/private/var/vm/sleepimage` (for saving memory when hibernating) should live on its own and not be backed up.

Space sharing is more like an operational detail than a game changing feature. You can think of it like special folders with snapshot and encryption controls… which is probably why Apple’s marketing department has yet to make me a job offer. Unfortunately this feature isn’t working in the macOS Sierra beta, so I wasn’t able to have more than one volume per container. Adding new volumes can fail with an opaque error (-69625 mean anything to you?), but using a larger disk image resolve the problem.

 

_Next in this series: [Space Efficiency and Clones](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part3/)_
