---
title: "DTrace on Geek Muse"
date: "2006-06-16"
categories:
  - "dtrace"
permalink: /2006/06/16/dtrace-on-geek-muse/
---

[DTrace](http://www.opensolaris.org/os/community/dtrace/) was recently featured on [episode 35](http://geekmuse.net/blog/index.php?entry=entry060607-224250) of [Geek Muse](http://geekmuse.net/). DTrace was brought to their attention because of John Birrell's [recent work](http://blogs.sun.com/roller/page/bmc?entry=dtrace_on_freebsd) to port it to FreeBSD (nice work, John!). The plug was nice, but I did want to respond to a few things:

DTrace was referred to as "a scripting language for debugging". While I can understand why one might get that impression, it's kind of missing the point. DTrace, concisely, is a systemic observability framework that's designed explicitly for use on mission-critical systems. It lets users and system administrators get concise answers to arbitrary questions. The scripting language aspect to DTrace lets you express those questions, but that's really just a component. James Dickens [took a stab](http://uadmin.blogspot.com/2006/05/what-is-dtrace.html) at an all-encompassing definition of DTrace....

One of the podcasters said something to the effect of "I'm just a web developer..." One of the great things about DTrace is that it has uses for developers at almost any layer of the stack. Initially DTrace could only view the kernel, and C and C++ code, but its release in Solaris 10 well over a year ago, DTrace has been extended to [Java](http://blogs.sun.com/roller/page/kamg?entry=built_in_dtrace_probes_in), [Ruby](http://blogs.sun.com/roller/page/bmc?entry=dtrace_and_ruby), [php](http://blogs.sun.com/roller/page/bmc?entry=dtrace_and_php), python, perl, and a handful of other dynamic languages that folks who are "just web developers" tend to use. In addition to being able to understand how your own code works, you'll be able to see how it interacts with every level of the system all the way down to things like disk I/O and the CPU scheduler.

Shortly after that, someone opined "I could use it for looking at XML-RPC communication". For sure! DTrace is **crazy** useful for understanding communication between processes, and in particular for XML-RPC for viewing calls and replies quickly and easily.

At one point they also identified the need to make sure users can't use DTrace to spy on each other. By default, DTrace is only executable by the root user. System administrators can dole out various levels of DTrace privilege to users as desired. Check out [the manual](http://docs.sun.com/app/docs/doc/817-6223) -- and [the security](http://docs.sun.com/app/docs/doc/817-6223/6mlkidln0?a=view) chapter in particular.

* * *

Technorati Tags: [DTrace](http://technorati.com/tag/DTrace) [Geek Muse](http://technorati.com/tag/Geek Muse)
