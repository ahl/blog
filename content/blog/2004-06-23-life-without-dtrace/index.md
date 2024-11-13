---
title: "Life without DTrace"
date: "2004-06-23"
categories: 
  - "dtrace"
---

As a member of the Solaris Kernel Group, I've obviously developed an affinity for using Solaris. There are tools like truss(1) and pstack(1) that come out of my fingers before I know what I'm typing, and now DTrace has taken such a central role in how I develop software, administer boxes, and chase down problems that I can't imagine doing without it.

Except sometimes I _do_ have to do without it. As much as I love Solaris, I still own a PowerBook G4 from a few years ago which has survived thousands of miles, and dozens of [drops](http://www.dropsquad.com). It's a great laptop, but I have no idea how anyone does any serious work on it. I think DTrace must have made me lazy, but even finding out simple information is incredibly arduous: tonight the VPN application was being cranky so I wanted to look at what was going on in the kernel; how do I do that? When [Safari](http://www.apple.com/safari)'s soaking up all my CPU time, what's it doing? There are some tools I can use, but they're clunky and much more cumbersome than DTrace.

When I was shopping for a car a few years ago, I looked at [one](http://www.internetautoguide.com/reviews/2001/2001_Volkswagen_Jetta.html) with a very reasonable 190-horsepower engine and [pricier one](http://www.internetautoguide.com/reviews/2001/2001_Audi_S4.html) with a 250-horsepower bi-turbo. From the moment I test drove the latter on the freeway, dropped it into 4th, and felt it leap up to 75mph in an instant, I knew I'd never be satisfied with the former. Be warned: you should only test drive DTrace if you think you'll have the chance to buy it -- I doubt you'll be able to settle for puttering your way up to 65mph again. Luckily, Solaris won't put such a big ding in your wallet ;-)
