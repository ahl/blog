---
title: "old school Solaris and Solaris quality of life"
date: "2004-07-01"
categories:
  - "opensolaris"
permalink: /2004/07/01/old-school-solaris-and-solaris-quality-of-life/
---

I don't like to dwell on past Solaris releases, but in Solaris 9 I wrote a cool update for `nohup(1)`. The `nohup(1)` utility takes a command and its arguments and makes sure that it keeps runnning even if your shell dies or your telnet session drops. Usually the way people use `nohup(1)` is they login from home, start up a long running process, forget that they should have been running it under `nohup(1)` and either take their chances with their ISDN line or kill the process and restart it: `nohup long-running-command ...`.

In Solaris 9, I implemented a `\-p` flag that lets users apply `nohup(1)` to a live running process. And if you've never run `nohup(1)` you might not care, but if you have, you know how useful this is. Solaris is full of these kind of quality of life tools. Check out all of the p-tools which [Eric Schrock](http://blogs.sun.com/eschrock) has been writing about.

The other night, I was talking to some serious BSD-heads who pointed out that `dmsg(1)` isn't so useful on Solaris. And that's a bug! We in Solaris take very seriously these sorts of simple quality of life issues, and welcome suggestions. If there's something in Solaris that pisses you off or is better elsewhere, let us know.
