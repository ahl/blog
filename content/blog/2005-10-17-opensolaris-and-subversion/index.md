---
title: "OpenSolaris and Subversion"
date: "2005-10-17"
categories: 
  - "opensolaris"
---

I just attended [Brian W. Fitzpatrick](http://www.red-bean.com/fitz/)'s [talk on Subversion](http://conferences.oreillynet.com/cs/eurooscon/view/e_sess/7444) at [EuroOSCON](http://conferences.oreillynet.com/eurooscon/). Brian did a great job and Subversion looks like a really complete replacement for cvs -- the stated goal of the project. What I was particularly interested in was the feasibility of using Subversion as the revision control system for [OpenSolaris](http://opensolaris.org); according to the [road map](http://opensolaris.org/os/about/roadmap/) we still have a few months to figure it out, but, as my grandmother always said while working away at her mechanical Turing machine, time flies when you're debating the merits of various revision control systems.

While Subversion seems like some polished software, I don't think it's a solution -- or at least a complete solution -- to the problem we have with OpenSolaris. In particular, it's not a distributed revision control system meaning that there's **one** master repository that manages everything including branches and sub-branches. This means that if you have a development team at the distal point on the globe from the main repository (we pretty much do), all that team's work has to traverse the globe. Now the Subversion folks have ensured that the over the wire protocol is lean, but that doesn't really address the core of the problem -- the concern isn't potentially slow communication, it's that it happens at all. Let's say a bunch of folks -- inside or outside of Sun -- start working on a project; under Subversion there's a single point of failure -- if the one server in Menlo Park goes down (or the connection to it does down), the project can't accept any more integrations. I'm also not clear if branches can have their own policies for integrations. There are a couple other issues we'd need to solve (e.g. comments are made per-integration rather than per-file), but this is by far the biggest.

Brian recommeded a [talk on svk](http://conferences.oreillynet.com/cs/eurooscon/view/e_sess/7293) later this week; [svk](http://svk.elixus.org/) is a distributed revision control and source management system that's built on Subversion. I hope svk solves the problems OpenSolaris would have with Subversion, but it would be great if Subversion could eventually migrate to a distributed model. I'd also like to attend this [BoF on version control systems](http://conferences.oreillynet.com/cs/eurooscon/view/e_sess/7906), but I'll be busy at the [OpenSolaris User Group meeting](http://conferences.oreillynet.com/cs/eurooscon/view/e_sess/7964) -- where I'm sure you'll be as well.

* * *

Technorati tags: [EuroOSCON](http://technorati.com/tag/EuroOSCON) [OpenSolaris](http://technorati.com/tag/OpenSolaris)
