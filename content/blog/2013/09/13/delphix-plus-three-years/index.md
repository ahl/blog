---
title: "Delphix plus three years"
date: "2013-09-13"
categories:
  - "delphix"
permalink: /2013/09/13/delphix-plus-three-years/
---

[![](images/child-measuring-height.jpg "child-measuring-height")](http://ahl.dtrace.org/wp-content/uploads/2013/09/child-measuring-height.jpg)Today marks my third anniversary of joining Delphix. Joining a startup, I knew there would be lots to learn -- indeed there’s been a lesson nearly once-a-day. Here are my top three lessons from my first three years at a startup. Even if the points themselves should have been obvious to me, the degree of their impact certainly wasn't.

## 3\. Tar-Babies are everywhere

Generalists thrive at a startup -- there are many more tasks and problems than there are people. The things you touch inevitably stick to you, for better or for worse. Early on I was the DTrace guy, and the OS guy, and the debugging guy. Later on I became the performance go-to, and upgrade guy, and (proudly) the git neck beard. But I was also lab manager, and the cabler (running cat 6 when we moved offices, and frenetically stripping wires with my teeth as we discovered a second wiring closet), and the real estate guy. When I got approval to open a San Francisco office I asked about the next steps -- "figure it out". And so it goes for pretty much everything that happens at a startup. At big companies roles are subdivided and specialists own their small domains. During my time at Sun I didn't think about many of the things those people did: they seemingly just happened. Being at a startup makes you intimately aware of all the excruciating minutiae that make a company go.

The more you do the more you own. The answer is not to avoid doing work that needs to be done, but to actively and aggressively recruit people to take over tasks. The stickiness was surprising and the need to offload can be uncomfortable. But you're asking people to take on tasks -- important, trivial, or unglamorous -- so you can take on some additional load.

Ownership is important, but you need to share the load or else these tar babies will drag you down.

## 2\. Hiring über alles

It's not surprising that it's all about the people. The right people and the right culture are the hardest problems to solve. It was surprising how much work it is to recruit and hire the best folks. We have a great team at Delphix and I love working with them. The big lesson for me was that hiring is far and away the highest leverage thing you can do for your startup. Hiring great people, great engineers, is hard and time consuming. I can't count the number of coffees, and beers I've had for Delphix -- “first dates” with prospective hires.

College recruiting had been an important focus for me during my years at Sun. I had only been at Delphix a few weeks when I convinced our CEO that we should recruit from colleges, and left to interview at my alma mater, Brown University. Delphix had been more conservative on hiring; some people regarded college recruiting as a bit of a flier, but it has paid off in a huge way. Our first college hire, two years in, is now a clear engineering leader. We've expanded the program from just me going to just one school to a dozen engineers going to ten this fall. About a quarter of our development team joined Delphix straight out of college. The initial effort is high, and you then have to wait 6-9 months for them to show up. But done right it can be the most efficient way to hire great engineers.

Work your ass off to hire great people; they'll repay you for the time you've spent. If you don't feel like hiring is a major time suck you're probably doing it wrong.

## 1\. Everything is your fault

The greatest blessing for a startup is also the greatest curse: having customers. Once people are paying for and using your product they'll find the problems with it and you'll work all hours to fix them. The surprise was how many problems became Delphix problems. Our product has its tendrils throughout the datacenter. We touch databases, operating systems, systems, hypervisors, networks, SANs and storage. Any butterfly flapping its wings in the datacenter can result in a hurricane on Delphix. As both the new component and the new vendor, we now expect and encourage our customers to talk to us early when diagnosing a problem. Our excellent support team (see hiring above) have diagnosed problems as diverse as poor network performance from a damaged cat 6 cable to over-provisioned ESX servers and misconfigured init.ora files. Obviously they've also addressed problems in our own software. But we always take an expansive view of the Delphix solution and work with the customer to chase the problem wherever it leads us.

This realization has also informed the way we build our software. We not only build our software to be resilient to abnormalities and to detect and report problems, but we also use tools to find problems early. A couple of years ago customers might connect Delphix to poor-performing storage -- but that would just look like Delphix performing poorly. Now we run a series of storage benchmarks during every installation and report a grade. We build paranoia into our software and into our sales and support processes.

As a startup it’s even more crucial to insulate ourselves against environmental problems, build facilities to detect problems everywhere, and own the total solution with customers.

### Starting year four

I hope others can benefit from those lessons; it took me a while to fully realize them. I'm sure there will be many more as I start year four at Delphix. Leave a comment and tell me about your own startup successes, failures, and lessons learned.
