---
title: "Delphix git integration"
date: "2012-02-16"
categories: 
  - "delphix"
tags: 
  - "git"
  - "zubairkhan"
---

[![](images/delphix_git-300x225.jpg "delphix_git")](http://ahl.dtrace.org/wp-content/uploads/2012/02/delphix_git.pdf)

Tonight, my Delphix colleague Zubair Khan and I presented the integration we've done with [git](http://git-scm.com/) at the [SF Bay Area Large-Scale Production Engineering](http://www.meetup.com/SF-Bay-Area-Large-Scale-Production-Engineering/) meetup. When I started at Delphix, we were using Subversion -- my ire for which the margins of this blog are too narrow to contain. We switched to git, and in the process I became an unabashed git fanboy.

Git is a powerful tool generally, but in particular has some powerful hook points that we use to enforce our code integration criteria and to do some handy things after we integrate. For this, we wrote some custom bash scripts, and python integrations with Bugzilla and Review Board. You can check out the [slides](http://ahl.dtrace.org/wp-content/uploads/2012/02/delphix_git.pdf), and we've open sourced it all on [github](https://github.com/delphix/git-hooks) with the hope that it might help people with their own integrations.
