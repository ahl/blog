---
title: "Comment out of context"
date: "2005-06-16"
categories:
  - "dtrace"
permalink: /2005/06/16/comment-out-of-context/
---

The [OpenSolaris](http://opensolaris.org) launch has been pretty fun -- I've already had some discussions with customers over the source code. Of course, the first thing people seemed to do with the source code is look for references to "shit" and "fuck". This was titillating to be sure. Unsatisfied with the cheap laugh, ZDNet wanted to [draw some conclusions](http://www.zdnet.com.au/news/software/0,2000061733,39197326,00.htm) from the profanity:

> _
> 
> The much-vaunted dynamic tracing (dtrace) feature of Sun's system may not be as safe to use as most people think.
> 
> "This bit me in the ass a couple of times, so lets toss this in as a cursory sanity check," wrote one careful developer in the dtrace section.
> 
> _

I wrote that code in October of 2002. For those of you keeping score at home, that's almost a year before DTrace integrated into Solaris 10 and more than two years before Solaris 10 hit the streets. Here's the [larger context](http://cvs.opensolaris.org/source/xref/usr/src/uts/sparc/dtrace/fasttrap_isa.c#923) of that comment:

```
923 	/*
924 	 * This bit me in the ass a couple of times, so lets toss this
925 	 * in as a cursory sanity check.
926 	 */
927 	ASSERT(pc != rp->r_g7 + 4);
928 	ASSERT(pc != rp->r_g7 + 8);

```

This gets pretty deep into the [bowels of the pid provider](http://dtrace.org/blogs/ahl/pid_provider_exposed), but the code preceeding these ASSERTs does the work of modifying the registers appropriately to emulate the effects of the traced instruction. For most instructions, we relocate the instruction bits to some per-thread scratch space and execute it there. We keep this scratch space in the user-land per-thread structure which, on SPARC, is always pointed to by the %g7 register (`rp->r\_g7` in the code above). The tricky thing is that while we change the program counter (%pc) to point to the scratch space, we leave the next program counter (%npc) where it was.

A bug I ran into _very_ early in development was winding up executing the wrong code because I had incorrectly emulated an instruction. One way this manifested itself was that the program counter was set to %g7 + 4 or %g7 + 8. I added these ASSERTs after tracking down the problem -- not because it was a condition that I thought should be handled, but because I wanted everything to stop immediately if it did.

In the nearly three years this code has existed, those ASSERTs have never been tripped. Of course, I didn't expect them to be -- they were a cursory sanity check so I could be sure this aberrant condition wasn't occuring. Of course, if I had omitted the curse this might not have inspired such a puerile thrill.

* * *

Technorati Tags: [DTrace](http://technorati.com/tag/DTrace) [OpenSolaris](http://technorati.com/tag/OpenSolaris)
