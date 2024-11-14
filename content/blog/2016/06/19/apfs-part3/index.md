---
title: "APFS in Detail: Space Efficiency and Clones"
date: "2016-06-19"
categories:
  - "software"
tags:
  - "apfs"
  - "snapshots"
permalink: /2016/06/19/apfs-part3/
---

_This series of posts covers APFS, Apple's new filesystem announced at WWDC 2016. See the [first post]( http://dtrace.org/blogs/ahl/2016/06/19/apfs-part1) for the table of contents._

## Space Efficiency

A modern trend in file systems has been to store data more efficiently to effectively increase the size of your device. Common approaches include compression (which, as noted above, is very very likely coming) and deduplication. Dedup finds common blocks and avoids storing them multiply. This is potentially highly beneficial for file servers where many users or many virtual machines might have copies of the same file; it’s probably not useful for the single-user or few-user environments that Apple cares about. (Yes, they have [server-ish offerings](http://www.apple.com/osx/server/features/#xsan) but their heart clearly isn’t into it.) It’s also furiously hard to do well as I learned painfully while supporting ZFS.

Apple’s sort-of-unique contribution to space efficiency is constant time cloning of files and directories. As a quick aside, “files” in macOS are often really directories; it’s a convenient lie they tell to allow logically related collections of files to be treated as an indivisible unit. Right click an application and select “Show Package Contents” to see what I mean. Accordingly, I’m going to use the term “file” rather than “file or directory” in sympathy for the patient readers who have made it this far.

With APFS, if you copy a file within the same file system (or possibly the same container; more on this later), no data is actually duplicated. Instead a constant amount of metadata is updated and the on-disk data is shared. Changes to either copy cause new space to be allocated (so-called “copy on write” or COW).

I haven’t seen this offered in other file systems, and it clearly makes for a good demo, but it got me wondering about the use case (**UPDATE**: btrfs supports this and calls the feature ["reflinks"](https://lwn.net/Articles/331808/)\--link by reference). Copying files between devices (e.g. to a USB stick for sharing) still takes time proportional to the amount of data copied of course. Why would I want to copy a file locally? The common case I could think of is the layman’s version control: “thesis”, “thesis-backup”, “thesis-old”, “thesis-saving because I’m making edits while drunk”.

There are basically three categories of files:

- Files that are fully overwritten each time; images, MS Office docs, videos, etc.
- Files that are appended to, mostly log files
- Files with a record-based structure, such as database files.

For the average user, most files fall into that first category. So with APFS I can make a copy of my document and get the benefits of space sharing, but those benefits will be eradicated as soon as I save the new revision. Perhaps users of larger files have a greater need for this and have a better idea of how it might be used.

Personally, my only use case is taking a file, say time-shifted Game of Thrones episodes falling into the “fair use” section of copyright law, and sticking it in Dropbox. Currently I need to choose to make a copy or permanently move the file to my Dropbox folder. Clones would let me do this more easily. But then so would hard links (a nearly ubiquitous file system feature that lets a file appear in multiple directories).

Clones open the door for potential confusion. While copying a file may take up no space, so too deleting a file may free no space. Imagine trying to free space on your system, and needing to hunt down the last clone of a large file to actually get your space back.

![](images/screenshot853.jpg "screenshot853")
APFS engineers don’t seem to have many use cases in mind; at WWDC they asked for suggestions from the assembled developers (the best I’ve heard is for copied VMs; not exactly a mass-market problem). If the focus is generic revision control, I’m surprised that Apple didn’t shoot for a more elegant solution. One could imagine functionality with APFS that allows a user to enable per-file Time Machine, change tracking for any file. This would create a new type of file where each version is recorded transparently and automatically. You could navigate to previous versions, prune the history, or delete the whole pile of versions at once (with no stray clones to hunt down). In fact, Apple [introduced something related 5 years ago](http://arstechnica.com/apple/2011/07/mac-os-x-10-7/14/#versioning-internals), but I’ve literally never seen or heard of it until researching this post (show of hands if you’ve clicked “Browse All Versions…”). APFS could clean up its implementation, simplify its use, and bring generic support for all applications. None of this solves my Game of Thrones storage problem, but I’m not even sure it’s much of a problem…

Side note: Finder copy creates space-efficient clones, but cp from the command line does not.

 

_Next in this series: [Performance](http://dtrace.org/blogs/ahl/2016/06/19/apfs-part4/)_
