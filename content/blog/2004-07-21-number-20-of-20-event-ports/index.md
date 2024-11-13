---
title: "Number 20 of 20: event ports"
date: "2004-07-21"
categories: 
  - "opensolaris"
---

[go to the Solaris 10 top 11-20 list for more](http://dtrace.org/blogs/ahl/the_solaris_10_top_11)

[Bart Smaalders](http://blogs.sun.com/barts) has written some [great stuff](http://blogs.sun.com/roller/page/barts/20040720#entry_2_event_ports) about **event ports** including an extensive coding example. Event ports provide a single API for tying together disparate souces of events. We had baby steps in the past with [poll(2)](http://docs.sun.com/db/doc/817-0691/6mgfmmdr6?a=view) and [select(3c)](http://docs.sun.com/db/doc/817-0692/6mgfnkuos?a=view), but event ports let you have the file descriptor and timer monitoring as well as dealing with asynchronous I/O and your own custom events.
