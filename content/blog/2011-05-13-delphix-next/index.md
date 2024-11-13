---
title: "Delphix Next"
date: "2011-05-13"
categories: 
  - "dtrace"
tags: 
  - "delphix"
  - "software-2"
---

It's rare to get software right the first time. I'm not referring to bugs in implementation requiring narrow fixes, but rather places in a design that simply missed the mark. Even if getting it absolutely right the first time were possible, it would be prohibitively expensive (and time-consuming) so we make the best decisions we can, hammer it out, and then wait. Users of a product quickly become experts on its strengths and weaknesses for them.  Customers aren't beta testers -- again, I'm not talking about bugs -- but rather they expose use cases you never anticipated, and present environments too convoluted to ever conceive at a whiteboard.

When I worked in the [Fishworks](http://blogs.sun.com/fishworks) group at Sun, we learned more about our market in the first three months after shipping 1.0 than we had in the 30 months we spent developing it. We found the product both struggling in unanticipated conditions, and being used to solve problems we could have never predicted.  Some of these we might have guessed earlier given more time, but some will never come to light until you ship. That you need to ship 1.0 before you can write 2.0 is a deeper notion than it appears.

I joined Delphix a couple of weeks before our formal launch at the [DEMO conference](http://www.zdnet.com/blog/gardner/delphix-server-launches-at-demo-to-slash-relational-database-redundant-copies-storage-waste-and-cost/3845). Since then, we've engaged in more proofs-of-contept (PoCs) and more customers have rolled us into use, and we've continued to learn of new use cases for Delphix Server, and found the places where we needed to rethink our assumptions. And we knew this would be the case -- you can't get it right the first time. Over the past several months, we in Delphix engineering have been writing the second version of the most critical components in our stack, incorporating the lessons learned with our customers. The team has enjoyed the opportunity to revisit design decisions with new information; it's fun to feel like we're not just getting it done, but getting it right.

When building 1.0, you make a mental list of the stuff you'd fix if only you had the time. In the next release, you figure out all the whizzy stuff you now _**can**_ build on the stable foundation. We're excited for the forthcoming 2.6 release -- more so even for new ideas we found along the way that will be the basis for our future work. We've got a great team working on a great product. Check in on the [Delphix blogs](http://blog.delphix.com/) in the coming months for details on the 2.6 release and the other stuff we've got in the works.
