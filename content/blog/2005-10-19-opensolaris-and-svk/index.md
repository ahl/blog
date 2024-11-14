---
title: "OpenSolaris and svk"
date: "2005-10-19"
categories:
  - "opensolaris"
permalink: /2005/10/19/opensolaris-and-svk/
---

Today at [EuroOSCON](http://conferences.oreillynet.com/eurooscon/), I attended a introductory [talk on svn](http://conferences.oreillynet.com/cs/eurooscon/view/e_sess/7293) by [Chia-liang Kao](http://svk.elixus.org/?ChiaLiangKao). I was hopeful that [svk](http://svk.elixus.org/) might address [some of the issues](http://dtrace.org/blogs/ahl/opensolaris_and_subversion) that I thought would prevent us from adopting Subversion for [OpenSolaris](http://opensolaris.org). In particular, Subversion requires a centralized repository whereas svk, which is built on top of Subversion, provides the distributed revision control system that we'd need. After the talk, my overall impression was that svk seemed to lack a certain polish, but after struggling to phrase that in less subjective terms, I'm coming around a bit.

I got a little nervous when the first example of svk's use was for keeping the contents of `/etc` under revision control. The **big problem** that svk solved was having random directories (.svc, SCCS, whatever) in, for example, `/etc/skel`. Talk about trivia (almost as relevant as a demo implementing a weblog in Ruby on Rails). I guess it's nice that svk solves a problem for that particularly esoteric scenario, but wasn't there some mention that this might be used to, you know, hold onto source code? Was that _actually_ the design center for svk?

Fortunately we did get to the details of using svk for a source code repository. I think this is just my bias coming from teamware, but some of the mechanisms seem a bit strange. In particular, you do `svk mirror` to make one kind of copy of the main repository (the kind that's a "local repository"), and `svk checkout` to make a different kind of copy (the kind that's the "working area"). In other words, you have a tree structure, but the branches and leaves are different entities entirely and editing can only be done on the leaves. I guess that's not such a huge pain, but I think this reflects the origins: taking a local revision control system and making it distributed. Consequentially, there's a bunch of stuff left over from Subversion (like branches) that seem rather redundant in a distributed revision control system (don't take branch, make another local repository, right?); it's not that these actually hurt anything, it just means that there's a bunch of complexity for what's essentially ancillary functionality.

Another not-a-big-deal-that-rubs-me-the-wrong-way is that svk is a pile of perl modules (of course, there's probably a specific perlism for that; "epocalyptus" or something I'm sure). I suppose we'll only have to wrestle that bear to the ground once, and stuff it in a tar ball (yes, Allan, or a package). To assuage my nervousness, I'd at least like to be confident that we could maintain this ourselves, but I don't think we have the collective perl expertise (except for the aforementioned [Alan](/alanbur)) to be able to fix a bug or add a feature we desperately need.

I want this thing to work, because svk seems to be the best option currently available, but I admit that I was a bit disappointed. If we were going to use this for OpenSolaris, we'd certainly need to distribute it in a self-contained form, and really take it through the paces to make sure it could do all the things we need in the strangest edge cases. As I mentioned, we currently use teamware which is an old Sun product that's been in constant use despite being end-of-lifed several years ago. While I like it's overall design, there's a bunch of work that would need to happen for it to be suitable for OpenSolaris. In particular, it currently requires a shared NFS filesystem, and we require you to be on the host machine when committing a change. Adding networked access capabilities to it would probably be a nightmare. It also relies on SCCS (an _ancient_ version control system) for managing individual files; not a problem per se, but a little crufty. Teamware is great and has withstood the test of time, but svk is probably closer to satisfying our requirements.

**Update:** But there are quite a few other options I hadn't looked into. Svk no longer looks like a front runner. If there are other systems you think are worth considering, let me know.

I'll play with svk some more and try to psych myself up for this brave new world. I'd appreciate any testimonials or feedback, and, of course, please correct all the factual errors I'm sure I committed.

* * *

Technorati tags: [EuroOSCON](http://technorati.com/tag/EuroOSCON) [OpenSolaris](http://technorati.com/tag/OpenSolaris) [svk](http://technorati.com/tag/svk)
