---
title: "I/O provider in Solaris Express 7/04"
date: "2004-07-14"
categories: 
  - "dtrace"
---

[Solaris Express](http://wwws.sun.com/software/solaris/solaris-express/) 7/04 is out an includes the I/O provider in DTrace. The I/O provider has just a few probes, but with them you can determine the source of I/O on your system (which processes, or users, or disks, etc.) as well as which files are being accessed, and many other facts that were previously difficult or impossible to find out. As always, you can find the documentation in the [Solaris Dynamic Tracing Guide](http://www.sun.com/bigadmin/content/dtrace/d10_latest.pdf) available on the DTrace [home page](http://www.sun.com/bigadmin/content/dtrace/). [Here](http://blogs.sun.com/roller/page/eschrock/20040630#top_i_o_consumers_in)'s some more good stuff about the I/O provider.
