---
title: "Assuaging OpenSolaris fears"
date: "2004-08-11"
categories: 
  - "opensolaris"
---

While trawling through b.s.c., a comment caught my eye in this [post](http://blogs.sun.com/roller/comments/gaw/Weblog/hey_jane_stop_this_crazy) from [Glenn's weblog](http://blogs.sun.com/gaw):

> _As a shareholder, I do NOT want you to "open source" solaris in its entirety (ESPECIALLY DTrace!). I want you to keep the good stuff completely sun-only, accessible only under NDA._

Certainly, this echoes some of the same concern I had when I started hearing rumblings about OpenSolaris -- we in Solaris have spent years of our lives making these innovations (ESPECIALLY DTrace!), and we don't want to see them robbed. I'm also a shareholder and I don't want to see my investments of time, effort, and -- forgive me -- money go to waste.

Now that I know more about OpenSolaris and [open source in general](http://dtrace.org/blogs/ahl/linux_solaris_and_open_source), I'm confident that Sun isn't selling out Solaris or giving away the company's crown jewel, rather we're going to make Solaris better, and more widely used. That sounds a little Rah Rah Solaris, but let's look more closely at OpenSolaris and what it might mean for Solaris and for Sun (and for the author of the comment, a shareholder).

Open source is an interesting dichotomy: on one side there are the developers and the community with the spirit of the free trade of software and ideas, and on the other side there are the Linux vendors selling service contracts to fat cat customers. The former is clearly the benefit of OpenSolaris -- a larger community of developers and users will improve Solaris and grow its audicent. The latter is the potential risk -- we're concerned that other companies might directly steals Sun's customers by using Sun's technology. The specifics of the OpenSolaris license haven't been finalized so it's possible that the license and patents will prevent Linux vendors from selling technologies developed in Solaris outright. Regardless, Solaris isn't just a bunch of code, it's the support and service and documentation and us, the Solaris developers.

When a Sun customer pays for Solaris, they're paying for someone over here to answer the phone when they call and for me and others in Solaris kernel development to do the things they need. Even when they source code is available, customers will still want to tap into the origins of that code and talk to the people who made it. If there are problems they'll want to be able to rely on the experts to fix them.

What about documentation? The [Solaris Dynamic Tracing Guide](http://www.sun.com/bigadmin/content/dtrace/d10_latest.pdf) is still going to be free but only as in beer -- we're not open sourcing our documentation (as least as far as I know). So let's say `dtrace.c` was dropped into Linux, would they then rewrite the entire answer book (400 pages and counting) from scratch? Maybe this wouldn't matter much to ordinary users, but if you're giving some Linux vendor a big sweaty wad of cash to support DTrace on Linux you expect some documentation! The Solaris docs would be close enough for some users, but not customers shelling out the big big dollars for a service contract.

Even if Linux were able to replicate DTrace and document it and a linux vendor were able to support it, I'm confident the existing and growing Solaris community could keep innovating and push Solaris ahead. On a more person note, I'm also excited about OpenSolaris because it means if I were ever to leave Sun, I could still work on [DTrace](http://www.sun.com/bigadmin/content/dtrace/), mdb, [nohup](http://dtrace.org/blogs/ahl/inside_nohup_p), and the other parts of Solaris that I consider my own.

OpenSolaris can only help Sun. If it succeeds, there will be a larger community of Solaris developers making it work with more platforms and devices, fixing more problems, and improving the quality of life on Solaris which will spawn an even larger community of Solaris users, both individuals and paying customers; if OpenSolaris fails, then that it won't help to create those communities, and I think that's the only consequence.
