---
title: "Inside Solaris Express"
date: "2004-08-19"
categories:
  - "opensolaris"
permalink: /2004/08/19/inside-solaris-express/
---

Since a few people in various forums have been asking about it, I thought I'd explain a little about how [Solaris Express](http://wwws.sun.com/software/solaris/solaris-express/sol_index.html) works. I know the story best from the kernel side, but keep in mind there are other parts of Solaris -- Java, the X server, etc. -- that have slightly different processes.

In kernel development we cut a build of Solaris 10 every two weeks; these are numbered s10\_XX (for example, Solaris Express 7/04 is s10\_60). Those take a week or two to coagulate into the WOS (Wad Of Stuff) which combines the kernel with the latest cut of the X server, gnome, etc. We spend another week or three making sure there's nothing too toxic in that build and release it in the form of a Solaris Express build. The lag time between when the build cuts and when it hits the streets in a Solaris Express build is usually about 4-6 weeks. We're about to release Solaris Express 8/04 (s10\_63) and we just cut s10\_66 on Monday. Note that Solaris Express isn't some release which we spend extensive time polishing; unless there's some real tragic problem, you're using the same bits that I'm using on my desktop. Since we cut a build every 2 weeks, we choose the best, most stable of the two or three builds since the last Solaris Express release, but usually it's the latest stuff. It can be pretty daunting to know that once you integrate a change into Solaris there's very little time to make sure its right -- we take a lot of pride in making sure Solaris is stable not just for every release of Solaris Express, but every numbered build and, in fact, every nightly build.

As far as what to expect in future releases, I have some hints for DTrace [here](http://blogs.sun.com/roller/page/ahl/dtracesched), but other than, that I think you just have to bite your nails and wait for the release notes. I will tell you that SX 9/04 is going to be _exciting_ -- check out Stephen's weblog for [why](http://blogs.sun.com/roller/page/sch/20040726#enabling_and_disabling).

As I mentioned, SX 8/04 will be out _very_ soon. Check out my [DTrace Solaris Express](http://blogs.sun.com/roller/page/ahl/dtracesched) decoder ring to see what new DTrace features are in this release (hint: -c and -p are _way_ cool). [Dan Price](http://blogs.sun.com/dp) has written up a [great description](http://blogs.sun.com/roller/page/dp/20040822#what_s_new_in_solaris) of all the stuff that's new in Solaris Express 8/04.
