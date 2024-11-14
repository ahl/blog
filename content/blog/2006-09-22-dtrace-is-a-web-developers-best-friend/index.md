---
title: "DTrace is a web developer's best friend"
date: "2006-09-22"
categories:
  - "dtrace"
permalink: /2006/09/22/dtrace-is-a-web-developers-best-friend/
---

I have this friend who might be most accurately described as a web developer. When DTrace was able to [observe php](http://blogs.sun.com/bmc/entry/dtrace_and_php_demonstrated) he was interested. Me: "I should give you a demo some time." Him: "Absolutely..."

When [DTrace ticked Ruby off its list](http://blogs.sun.com/bmc/date/20050821), he was more enthusiastic. Him: "Cool! I loves me the Ruby!" Me: "Let me know when you want that demo".

The other day I got an IM from my friend. Him: "DTrace for JavaScript, eh?" Me: "How 'bout that, huh?" Him: "So when can I get that demo?"

Last week [Brendan Gregg](http://blogs.sun.com/brendan) released [Helper Monkey](http://blogs.sun.com/brendan/entry/dtrace_meets_javascript) -- a DTrace-enabled version of Mozilla's Spider Monkey JavaScript engine. Why was this the tipping point for my friend the web developer? Probably not because he's more fond of JavaScript than php or Ruby; much more likely, it's because JavaScript is an observability atrocity. Even if you don't use any probes in DTrace other than the ones associated with JavaScript, Helper Monkey is a galactic improvement on the state of JavaScript development. Consider the next 40,000 - 200,000 DTrace probes gravy.

* * *

Technorati Tags: [DTrace](http://technorati.com/tag/DTrace) [JavaScript](http://technorati.com/tag/JavaScript)
