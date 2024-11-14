---
title: "The Solaris 10 top 11-20"
date: "2004-07-12"
categories: 
  - "opensolaris"
---

Solaris 10 has _way_ more features than any release of Solaris that I can remember, and Sun's been marketing the hell out of them. Here's my top 10 list roughly in order of how cool I think each is:

1. **DTrace**\- of course...
2. **ZFS**\- the amazing new file system
3. **AMD64 Support**\- Opteron is so obviously great
4. **Zones**\- N1 Grid Containers for those of you keeping score at home
5. **Predictive Self Healing**\- never worry about flaky hardware again
6. **Performance**\- Solaris 10 is just faster, faster networking, faster everything
7. **Linux Compatibility** - run linux _binaries_unmodified, 'nuf said
8. **Service Management Facility**\- managing a box just got much easier
9. **Process Rights Management**\- super-user is no longer a binary proposition
10. **NFSv4** - nfs++ (++++++)

Blah blah blah. That's for sure amazing stuff, but there are dozens of places where you can read about it (I was going to include some links to news stories, but I'm sure [google](http://www.google.com) can find you the same stuff it found for me).

But is that it for Solaris 10? Not by a long shot. There are literally dozens of features and improvements which would have cracked the top 10 for the last few Solaris releases. Without further ado, I present my Solaris 10 top 11-20 list:

11. [**`libumem`**](http://dtrace.org/blogs/ahl/2004/07/13/number-11-of-20-libumem/) - _the_tool for debugging dynamic allocation problems; oh, and it scales as well or better than any other memory allocator
12. [**`pfiles(1)` with file names**](http://dtrace.org/blogs/ahl/2004/07/13/number-12-of-20-file-names-in-pfiles1/) - you can get at the file name info through `/proc`too; very cool
13. [**Improved `coreadm(1M)`**](http://dtrace.org/blogs/ahl/2004/07/15/number-13-of-20-core-file-improvements/)\- core files are now actually useful on other machines, administrators and users can specify the content of core files
14. **System V IPC**\- no more clumsy system tunables and reboots, it's all dynamic, and -- guess what? -- faster too
15. [**kmdb**](http://dtrace.org/blogs/ahl/2004/08/23/solaris-10-top-11-20-number-15-kmdb/) - if you don't care, ok, but if you do care, you really really care: `mdb(1)`'s cousin replaces `kadb(1M)`
16. [**Watchpoints**](http://dtrace.org/blogs/ahl/2004/07/18/number-16-of-20-improved-watchpoints/) - now they work _and_they scale
17. [**`pstack(1)` for java**](http://dtrace.org/blogs/ahl/2004/07/20/number-17-of-20-java-stack-traces/)\- see java stack frames in a JVM or core file and through DTrace
18. [**`pmap(1)` features**](http://dtrace.org/blogs/ahl/2004/07/17/number-18-of-20-pmap1-improvements/)\- see thread stacks, and core file content
19. [**per-thread p-tools**](http://dtrace.org/blogs/ahl/2004/08/06/number-19-of-20-per-thread-p-tools/) - apply `pstack(1)` and `truss(1)`to just the threads you care about
20. [**Event Ports**](http://dtrace.org/blogs/ahl/2004/07/21/number-20-of-20-event-ports/) - a generic API for dealing with heterogeneous event sources

There were some other features in the running (Serial ATA support, `vacation(1)` filtering, other p-tools improvements, etc.), but I had to draw the line somewhere. Remember this is _my_ list; Solaris 10 has fancy new java and gnome stuff, but, while it's cool I guess, it doesn't do it for me in the way these things do. I'd be doing these features an injustice if I tried to summarize them all in one weblog entry, so I'll bite off one at a time and explain them in detail over the next few days; stay tuned.
