---
title: "DTrace for Linux"
date: "2011-10-05"
categories: 
  - "dtrace"
tags: 
  - "dtrace"
  - "illumos"
  - "linux"
  - "oel"
  - "oow"
  - "opensolaris"
  - "oracle"
  - "solaris"
---

Yesterday (October 4, 2011) Oracle made the surprising announcement that they would be porting some key Solaris features, DTrace and Zones, to Oracle Enterprise Linux. As one of the original authors, the news about DTrace was particularly interesting to me, so I started digging.

I should note that this isn't the [first time I've written about DTrace for Linux](http://dtrace.org/blogs/ahl/2005/12/13/dtrace-for-linux/). Back in 2005, I worked on Linux-branded Zones, Solaris containers that contained a Linux user environment. I wrote a coyly-titled blog post about examining Linux applications using DTrace. The subject was honest -- we used precisely the same techniques to bring the benefits of DTrace to Linux applications -- but the title wasn't completely accurate. That wasn't exactly "DTrace for Linux", it was more precisely "The Linux user-land for Solaris where users can reap the benefits of DTrace"; I chose the snappier title.

I also wrote about [DTrace knockoffs](http://blogs.oracle.com/ahl/entry/dtrace_knockoffs) in 2007 to examine the Linux counter-effort. While the project is still in development, it hasn't achieved the functionality or traction of DTrace. Suggesting that Linux was inferior brought out the usual NIH reactions which led me to write a subsequent blog post about a [theoretical port of DTrace to Linux](http://dtrace.org/blogs/ahl/2007/08/06/what-if-machine-dtrace-port/). While a year later [Paul Fox started exactly such a port](http://dtrace.org/blogs/bmc/2008/06/30/dtrace-on-linux/), my assumption at the time was that the primary copyright holder of DTrace wouldn't be the one porting DTrace to Linux. Now that Oracle is claiming a port, the calculus may change a bit.

## What is Oracle doing?

Even among Oracle employees, there's uncertainty about what was announced. Ed Screven gave us just a couple of bullet points in his keynote; Sergio Leunissen, the product manager for OEL, didn't have further details in his OpenWorld talk beyond it being a [beta of limited functionality](http://twitter.com/#!/ahl/status/121281990455590912); and the entire Solaris team seemed completely taken by surprise.

## What is in the port?

Leunissen stated that only the kernel components of DTrace are part of the port. It's unclear whether that means just [fbt](http://wikis.sun.com/display/DTrace/fbt+Provider) or includes [sdt](http://wikis.sun.com/display/DTrace/sdt+Provider) and the [related providers](http://wikis.sun.com/display/DTrace/sched+Provider). It sounds certain, though, that it won't pass the [DTrace test suite](http://hub.opensolaris.org/bin/view/Community+Group+dtrace/dtest) which is the deciding criterion between a DTrace port and some sort of work in progress.

## What is the license?

While I abhor GPL v. CDDL discussions, this is a pretty interesting case. According to the release manager for OEL, some small kernel components and header files will be dual-licensed while the bulk of DTrace -- the kernel modules, libraries, and commands -- will use the CDDL as they had under (the now [defunct](http://www.theregister.co.uk/2010/08/13/opensolaris_is_dead/)) OpenSolaris (and to the [consernation of Linux die-hards I'm sure](http://twitter.com/#!/ahl/status/121257501193809920)). Oracle already faces an interesting conundum with their CDDL-licensed files: they can't take the fixes that others have made to, for example, ZFS without needing to release their own fixes. The DTrace port to Linux is interesting in that Oracle apparently thinks that the CDDL license will make DTrace too toxic for other Linux vendors to touch.

## Conclusion

Regardless of how Oracle brings DTrace to Linux, it will be good for DTrace and good for its users -- and perhaps best of all for [the author](http://dtrace.org/blogs/brendan/) of the [DTrace book](http://blogs.oracle.com/brendan/entry/dtrace_book_coming_soon). I'm cautiously optimistic about what this means for the DTrace development community if Oracle does, in fact, release DTrace under the CDDL. While this won't mean much for the broader Linux community, we in the illumos community will happily accept anything of value Oracle adds. The Solaris lover in me was worried when it appeared that OEL was raiding the Solaris pantry, but if this is Oracle's model for porting, then I -- and the entire illumos community I'm sure -- hope that more and more of Solaris is open sourced under the aegis of OEL differentiation.

**10/10/2011 follow-up post, [Oracle's port: this is not DTrace](http://dtrace.org/blogs/ahl/2011/10/10/oel-this-is-not-dtrace/).**
