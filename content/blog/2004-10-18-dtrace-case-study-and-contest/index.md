---
title: "DTrace case study and contest"
date: "2004-10-18"
categories: 
  - "dtrace"
---

Jay Danielson has posted a great [case study](http://developers.sun.com/solaris/articles/dtrace_for_dev.html) of using DTrace to solve a problem porting a linux driver to Solaris. It's not an incredibly complicated example, but I find this kind of tracking a problem from symptoms to root cause to be very helpful at understanding how to use DTrace.

Speaking of using DTrace on sun.com, Sun is sponsoring a [contest](http://wwws.sun.com/software/solaris/10/contest/challenge.html) to come up with the most innovative or creative use of DTrace (or Zones). If you're sitting on some D program that helped you out when no other tool could, or if you found some big performance win, send those in. Or if you want a new HDTV, Opteron laptop or iPod, get cracking (I'm tempted to invent a pseudonym with those kind of goodies on the line). My bet for a sure fire way to finish in the money would be to use DTrace in concert with some GUI or software visualization to get some wholly new look at software, or to find some big performance win in an app that everyone thought had been tuned to perfection with existing tools.
