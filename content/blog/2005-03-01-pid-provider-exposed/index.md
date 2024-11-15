---
title: "pid provider exposed"
date: "2005-03-01"
categories:
  - "dtrace"
permalink: /2005/03/01/pid-provider-exposed/
---

In the kernel group, we occasionally give presentations on interesting work we've been doing. Last week, I gave a talk on the implementation the DTrace pid provider, and -- in the spirit of community that the [OpenSolaris](http://opensolaris.org/) project has engendered -- I'm posting it on my blog:

<iframe class="speakerdeck-iframe" frameborder="0" src="https://speakerdeck.com/player/b6cab65f1ffe4af5a67c266bf78c59b3" title="Under the Hood: Inside the DTrace pid Provider" allowfullscreen="true" style="border: 0px; background: padding-box padding-box rgba(0, 0, 0, 0.1); margin: 0px; padding: 0px; border-radius: 6px; box-shadow: rgba(0, 0, 0, 0.2) 0px 5px 40px; width: 100%; height: auto; aspect-ratio: 560 / 420;" data-ratio="1.3333333333333333"></iframe>

In this presentation I go into the many details of the general technique for arbitrary instruction instrumentation, the implementation on SPARC, x86 and amd64 as well as the many pitfalls, caveats, tricks, and solutions. While I wouldn't say I go into excruciating details, it's probably falls just short. If you're interested at all in instrumentation or want to know more about the pid provider or are just a super nerd who enjoys reading about cool, low-level hackery, it's probably worth a read.

I've tried to augment it with some helpful commentary, but if something's confusing, vague or seems wrong, please let me know.

* * *

Technorati tag: [DTrace](http://technorati.com/tag/DTrace)
