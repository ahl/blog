---
title: "Linux, Solaris, and Open Source"
date: "2004-07-30"
categories: 
  - "opensolaris"
---

This past week at [OSCON](http://conferences.oreillynet.com/os2004/) I've spent my time trying to understand open source processes, talking about Solaris, and trying to figure out what OpenSolaris is going to look like.

### Learning from Linux

I attended a talk by Greg Kroah-Hartman about Linux kernel development. As we work towards open sourcing Solaris, we're trying to figure out how to do it right -- source control, process, licenses, community etc. As I didn't know much about how Linux development works, I was hoping to learn from a largely successful open source operating system.

Linux development is built around fiefdoms maintained by folks like Greg. Ordinary folks can contribute to the repositories they maintain (either directly or by proxy based on some sort of Linux-street-cred it seems). Those repositories are then fed up to a combined unstable repository, and from there Linus himself ordains the patches and welcomes them into the circle of linux 2.6.x. This all seemed to make some sense and work alright. That is, until someone asked about firewire support. The answer, "I wouldn't run firewire -- it should _build_ \[laughter\], but I wouldn't run it. The discussion then led to Linux testing which it seems is highly ad-hoc and unreliable. IBM and Novell are working on nightly testing runs, but very little exists today in terms of quality control tests or general tests that developers themselves can run before they integrate their changes.

In Solaris, testing can be arduous. Some changes are obvious and can be tested on just on architecture, but others require extensive tests on a variety of SPARC and x86 platforms. And linux supports so many more platforms! I have no idea how a developer working on his x86 box can ever be sure that some seemingly innocuous change hasn't broken 64-bit PPC (or whatever). Clearly this is something we have to solve for OpenSolaris -- reliability and testing are at the core of our DNA in the Solaris kernel group, and we need to not only export that _idea_ to the community, but only some subset of facilities so that contributors can adhere to the same levels of quality.

### OpenSolaris

Later in the day a bunch of us from Solaris met with some open source leaders (I don't know quite how one earns that title, but that's what our [liaison](http://blogs.sun.com/roller/page/DaneseCooper) told us they were). We first told them where we were: Yes, we really are going to open source Solaris; no, we don't know the license yet; no, we don't know if it's going to be GPL compatible; no, we aren't planning on moving the [cool stuff in Solaris](http://dtrace.org/blogs/ahl/the_solaris_10_top_11) over to Linux ourselves; and, no, we do _not_ know what the license is going to be, but we promise to tell you when we do.

We got a lot of helpful suggestions from folks involved with [apache](http://www.apache.org/) and other projects. "Bite sized bugs" sound like a great way to get new people involved with Solaris and contributing code without a huge investment of effort. Documentation, partitioning and documenting that partitioning will all be much more important that we had previously anticipated. We get the message: OpenSolaris will be easy to download, build, and install, and we'll make sure it's as easy as possible to get started with development.

The one thing that disappointed me was the lack of knowledge about [Solaris 10](http://wwws.sun.com/software/solaris/10/) -- some comment suggested that the members of the panel think Solaris doesn't really have anything interesting. Fortunately we had the BOF that night...

### The Solaris Community

[Andy](http://blogs.sun.com/tucker), [Bart](http://blogs.sun.com/barts), [Eric](http://blogs.sun.com/eschrock) and I held a BOF session last night to talk about OpenSolaris and Solaris 10. After satiating the crowd's curiosity about open sourcing Solaris (_no_, we don't know what the license is going to be), we gave some Solaris 10 demonstrations.

Since we were tight on time, I buzzed through the [DTrace](http://www.sun.com/bigadmin/content/dtrace/) demo in about 15 minutes touching on the syscall provider, aggregations, the ustack() action, user-land tracing with the pid provider and kernel tracing with the fbt provider. Whew. Then came the questions -- some of the audience members had used DTrace, others had heard of it; almost everyone had a question. When I demo DTrace, there's always this great moment of epiphany that people go through. I can see it on their faces. After the initial demo they look like people who just got off a roller coaster -- windblown and trying to understand what just happened, but there's always this moment, this ah-ha moment, when something -- the answer to a question, an additional example, an anecdote -- sparks them into understanding. It's great to see someone suddenly sit up in her chair and start nodding vigorously at every new site I point out in the DTrace guided tour.

My personal favorite piece of input about OpenSolaris was someone's claim that six months after Solaris goes open source there will be a port to PowerBook hardware. If that's true then everyone in the Solaris kernel group is going to have PowerBooks in 6 months plus a day.

Before the BOF, I was worried that we might not find our community for open source Solaris. Not only was there a good crowd at the BOF, but they were [interested](http://mike.kruckenberg.com/archives/2004/07/looking_forward.html) and [impressed](http://www.livejournal.com/users/brad/2039315.html). That's our community, and those are the people who are going to be contributing to OpenSolaris. And I can't wait for it to happen.
