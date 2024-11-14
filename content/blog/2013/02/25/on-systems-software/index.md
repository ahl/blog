---
title: "On Systems Software"
date: "2013-02-24"
categories:
  - "software"
tags:
  - "systems"
permalink: /2013/02/25/on-systems-software/
---

![](images/cctv.jpeg)]
A prospective new college hire recently related an odd comment from his professor: systems programming is dead. I was nonplussed; what could the professor have meant? Systems is clearly very much alive. Interesting and important projects march under the banner of systems. But as I tried to construct a less emotional rebuttal, I realized I lacked a crisp definition of what systems programming is.

Wikipedia [defines systems software](http://en.wikipedia.org/wiki/System_software) in the narrowest terms: the stuff that interacts with hardware. But that covers a tiny fraction of modern systems. So what is systems software? It depends on **when** you're asking the question. At one time, the web server was the application; now it's the systems software on which many web-facing applications are built. At one time a database was the application; now it's systems software that supports a variety of custom and off-the-shelf applications. Before my time, shells were probably considered a bleeding edge application; now they're systems software on which some of the lowest-level plumbing of modern operating systems are built.

Any layer on which people build applications of increasing complexity is systems software. Most software that endures the transition to systems software does so whether its authors intended it or not. People in the software industry often talk about standing on the shoulders of giants; the systems software accumulated and refined over decades are those giants.

Stable interfaces define systems software. The programs that consume those interfaces expect the underlying systems software to be perfect every time. Initially innovation might happen in the interfaces themselves -- the concurrent model of [Node.js](http://nodejs.org) is a great example. As software matures, the interfaces become commodified; innovation happens behind those stable interfaces. Systems is only "dead" at its edges. Interfaces might be flexible and well-designed, or sclerotic and poorly designed. Regardless, new or improved systems software can increase performance, enhance observability, or simply fit a different economic niche.

There are a few different types of systems software. First there's **supporting systems software**, systems software written as necessary foundation for some new application. This is systems software written with a purpose and designed to solve an unsolved -- or poorly solved -- problem. Chronologically, examples include UNIX, compilers, and libraries like jQuery. You write it because you need it, but it's solving a problem that's likely not unique to your particular application.

Then there's **accidental systems software**. Stick everything from Apache to Excel to the Bourne shell in that category. These didn't necessarily set out to be the foundation on which increasingly complex software would be written, but they definitely are. I'm sure there were times when indoctrination into systems-hood was painful, where the authors wanted to change interfaces, but good systems software respects its consumers and carries them forward. Somewhat famously [make](http://en.wikipedia.org/wiki/Make_(software)) preserved its arcane syntax because two consumers already existed. JavaScript started as a glorified web design tool; now it sits several layers beneath complex client-side applications. Even more recently, developers of Node.js (itself  JavaScript-based) changed a commonly used interface that broke many applications. Historical mistakes can be annoying to live with, but -- as the Node.js team determined -- [compatibility trumps cleanliness](https://github.com/joyent/node/issues/3577).

The largest bucket is **replacement systems software**. Linux, Java, ZFS, and DTrace fall into this category. At the time of their development, each was a notionally compatible replacement for something that already existed. Linux, of course, reimplemented the UNIX design to provide a free, compatible alternative. Java set about building a better runtime (the stable interface being a binary provided to customers to execute) designed to abstract away the operating system entirely. ZFS represented a completely new way of thinking about filesystems, but it did so within the tight constraints of POSIX interfaces and storage hardware. DTrace added new and unique observability to most of the stable interfaces that applications build on.

Finally, there's **intentional systems software**. This is systems software by design, but unlike supporting systems software, there's no consumer. Intentional systems software takes an "if you build it, they will come" approach. This is highly prone to failure -- without an existence proof that your software solves a problem and exposes the right interfaces, it's very difficult to know if you're building the right thing.

Why define these categories? Knowing which you're working with can inform your decisions. If you've written accidental systems software that has had systems-ness thrust upon it, realize that future versions need to respect the consumers -- or willfully cast them aside. When writing replacement systems software, recognize the constraints on the system, and know exactly where you're constrained and where you can innovate (or consider if you don't want to use the existing solution). If you've written supporting systems software, know that others will inevitably need solutions to the same problems. Either invest in maintaining it and keeping it best of breed; resign to the fact that it will need to be replaced as others invest in a different solution; or open source it and hope (or advocate) that it becomes that ubiquitous solution.

TL;DR?

What's systems software? It is the increasingly complex, increasingly capable, increasingly diverse foundation on which applications are built. It's that long and growing tail of the corpus of software at large. The interfaces might be static, but it's a rich venue for innovation. As more and more critical applications build on an interface, the more value there is in improving the systems software beneath it. Systems software is defined by the constraints; it's a mission and a mindset with unique challenges, and unique potential.
