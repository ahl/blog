---
title: "DTrace: a history"
date: "2006-11-02"
categories:
  - "dtrace"
permalink: /2006/11/02/dtrace-a-history/
---

An unsurprisingly common [request](http://www.opensolaris.org/jive/thread.jspa?messageID=63888&#63888) on the [DTrace discussion forum](http://www.opensolaris.org/jive/forum.jspa?forumID=7) has been for updated documentation. People have been -- on the whole -- very pleased with the [Solaris Dynamic Tracing Guide](http://docs.sun.com/app/docs/doc/817-6223) that [w](http://blogs.sun.com/mws)[e](http://blogs.sun.com/bmc) worked hard to produce, but I readily admit that we haven't been nearly as diligent in updating it. OK: we haven't updated it at all.

But we have been updating DTrace itself, adding new variables and functions, tacking on new features, adding new providers, and fixing bugs. But unless you've been scraping our putback logs, or reading between the lines on the discussion forum, these features haven't necessarily been obvious. To that end, I've scraped the putback logs, and tried to tease out some of the bigger features, and put them all on the [DTrace Change Log](http://www.opensolaris.org/os/community/dtrace/ChangeLog/). We'll try to keep this up to date so you can see what features are in the build of Solaris Nevada you're running or the Solaris 10 release.

This is probably going to be handy in its own right and ameliorate the documentation gap, but we do still need to update the documentation. I'm torn between keeping it in SGML (or converting it to XML), and converting it to a wiki. The former has the disadvantage of being overly onerous to update (witness the complete lack of updates), while the latter prevents us from releasing it in printed form (unless someone knows of a tool that can turn a wiki into a book). If anyone from the community is interested in working on this project, it would be a tremendous help to every DTrace user and developer.

* * *

Technorati Tags: [DTrace](http://technorati.com/tag/DTrace) [OpenSolaris](http://technorati.com/tag/OpenSolaris)
