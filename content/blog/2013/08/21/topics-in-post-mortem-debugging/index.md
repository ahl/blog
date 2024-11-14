---
title: "Topics in post-mortem debugging"
date: "2013-08-21"
categories: 
  - "illumos"
---

A couple of weeks ago, Joyent hosted [A Midsummer Night's Systems meetup](http://smartos.org/2013/08/12/a-midsummer-nights-system/), a fun event with talks ranging from Node.js fatwas to big data for [Mario Kart 64](http://kartlytics.com/). My colleague [Jeremy Jones](http://blog.delphix.com/jjones/) had recently done some amazing work, perfect for the meetup, but with his first child less than a day old, Jeremy allowed me to present in his stead. In this short video (16 minutes) I talk about Jeremy's investigation of a nasty bug that necessitated the creation of two awesome post-mortem tools. The first is what I call jdump, a [Volatility](http://code.google.com/p/volatility/) plugin that takes a VMware snapshot and produces an [illumos](http://en.wikipedia.org/wiki/Illumos) kernel crash dump. The second is ::gcore, an mdb command that can extract a fully functioning core file from a kernel crash dump. Together, they let us at Delphix scoop up all the state we'd need for a diagnosis with minimal interruption even when there's no hard failure. Jeremy's tools are close to magic, and without them the problem was close to undebuggable.

\[youtube\_sc url="OLFqOBxUhfM"\]

Thanks, Jeremy for letting me present on your great work. And thanks to Deirdre Straughan and Joyent for the great event!
