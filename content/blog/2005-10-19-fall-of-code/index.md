---
title: "Fall of Code?"
date: "2005-10-19"
categories: 
  - "opensolaris"
---

Actually, Fall, Winter and Spring of code. Sun just announced the [Solaris 10 Univesity Challenge Content](http://www.sun.com/software/solaris/contest/univ_challenge.jsp). That's right, it's a challenge [_and_ a contest](http://snltranscripts.jt.org/75/75ishimmer.phtml) and with three times the staying power of [single season of code](http://code.google.com/summerofcode.html). Apparently in a modern day [treaty of Brest-Litovsk](http://en.wikipedia.org/wiki/Treaty_of_brest-litovsk), Google ceded the rest of the year to Sun. Perhaps this was the real [beef](http://www.eweek.com/article2/0,1895,1872448,00.asp) of the recent [Sun/Google agreement](http://news.zdnet.com/2100-3513-5887923.html).

This is actually pretty cool: be a college student, do something cool on [OpenSolaris](http://opensolaris.org), take a shot at winning the grand prize of $5k and [a sweet box](http://www.sun.com/desktop/workstation/ultra20/) (I imagine there might be prizes for other interesting entries). There are some ideas projects on the contest/challenge/seasonal coding page ranging from good ([MythTV](http://www.mythtv.org/)), to mundane (support for inkjet printers from Epson), to confusing (internet gaming -- I thought online gamling was its own reward), to inane ("A joystick driver - for gaming", oh for gaming? I've been using it for [system administration](http://www.cs.unm.edu/~dlchao/flake/doom/)). Here's my list off the top of my head -- if you want more ideas, feel free to [drop me a line](mailto:ahl_at_you_know_where_dot_com).

#### Work on an existing open source project

- pearpc runs ppc applications on x86. I started working on porting it over and was able to boot Darwin.
- valgrind is very cool (I've only just seen it). It would be great to port it or to use pieces like KCacheGrind and plug in [DTrace](http://www.opensolaris.org/os/community/dtrace/) as a backend.
- Port over your favorite application, or get it running in some emulation environment of your own devising.
- Make something go faster. MySQL, Gnome, mozilla, some random system call, whatever; there's a lot of inefficiency out there.

#### Write something new (using cools stuff in Solaris)

- I'd love to see more dynamic languages with native DTrace support. We've already got support for [Java](http://dtrace.org/blogs/ahl/dtracing_java), [php](http://blogs.sun.com/roller/page/bmc?entry=dtrace_and_php), [Ruby](http://blogs.sun.com/roller/page/bmc?entry=dtrace_and_ruby), and [Perl](http://blogs.sun.com/roller/page/alanbur?entry=dtrace_and_perl) in some form; make it better or add support for some other language you know and love (TCL, python, scheme, LISP, ML, etc.).
- Build another kind of analysis tool on top of DTrace. We're working on a Java binding which is going to make this easier.
- Write a device driver for your favorite crazy device (which I assume is your new iPod nano or something; you're such a [hipster Apple fanboy](http://blogs.sun.com/roller/page/jonathan?entry=an_invitation)).
- Build a tool to simulate a distributed environment on Zones and use DTrace to monitor the communication. WARNING: your distributed systems professor will be your new best friend.

That's what I'd do, but if you have a better idea, go do that instead.

* * *

Technorati tags: [OpenSolaris](http://technorati.com/tag/OpenSolaris) [Summer o' Code](http://technorati.com/tag/summerofcode)
