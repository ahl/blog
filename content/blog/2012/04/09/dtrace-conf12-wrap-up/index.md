---
title: "dtrace.conf(12) wrap-up"
date: "2012-04-09"
categories: 
  - "dtrace"
tags: 
  - "bryancantrill"
  - "davepacheco"
  - "dtrace"
  - "dtrace-conf"
  - "ericschrock"
  - "georgewilson"
  - "krisvanhees"
  - "mattahrens"
  - "oel"
---

[![](images/dtrace.conf12_tee.png "dtrace.conf(12) tee")](http://ahl.dtrace.org/wp-content/uploads/2012/04/dtrace.conf12_tee.png)For the [second time](http://dtrace.org/blogs/ahl/2008/05/05/dtrace-conf-post-post-mortem/) in as many quadrennial [dtrace.conf](http://wiki.smartos.org/display/DOC/dtrace.conf)s, I was impressed at how well the unconference format worked out. Sharing coffee with the DTrace community, it was great to see some of the oldest friends of DTrace -- [Jarod Jenson](http://dl.acm.org/citation.cfm?id=1117399), [Stephen O'Grady](https://twitter.com/#!/sogrady), [Jonathan Adams](https://blogs.oracle.com/jwadams/) to name a few -- and to put faces to names -- [Scott Fritchie](https://twitter.com/#!/slfritchie), [Dustin Sallings](https://twitter.com/#!/dlsspy), Blake Irvin, etc -- of the many new additions to the DTrace community. You can see all the [slides and videos](http://wiki.smartos.org/display/DOC/dtrace.conf+Schedule); these are my thoughts and notes on the day.

<iframe width="224" height="126" src="http://www.youtube.com/embed/l_7v7Fn7uMQ" frameborder="0" class="alignright"></iframe>

[Bryan](http://dtrace.org/blogs/bmc/) provided a typically eloquent review of the state of the community. DTrace development is alive and well -- after a lull while Oracle's acquisition of Sun settled in -- with new support for a variety of languages and runtimes, and new products that rely heavily on DTrace as a secret sauce. Bryan laid out some important development goals, areas where many have started straying from the edges of the completed DTrace features into the partially complete or starkly missing. We all then set to work hammering out a loose schedule for the day; I'll admit that at first I was worried that we'd have too many listeners and not enough presenters, but the schedule quickly filled -- and with more topics than we'd end up having time to cover.

### User-land CTF and Dynamic Translators

<iframe width="224" height="126" src="http://www.youtube.com/embed/0QF04ivO_WE" frameborder="0" class="alignright"></iframe>

DTrace, from its inception, has been a systemic analysis tool, but the earliest development focused on kernel observability -- not a surprise since Bryan, Mike, and I developed it while working in the Solaris kernel development. After its use spread (quickly) beyond the kernel team, use shifted more and more to features focused on understanding C and C++ applications in user-land, and then to applications written in a variety of higher-level languages -- Java, Ruby, Perl, Javascript, Erlang, etc. [User-land Statically Defined Tracing](http://dtrace.org/blogs/dap/2011/12/13/usdt-providers-redux/) (USDT) is the DTrace facility that enables rich tracing of higher-level languages. It was a relatively late addition to DTrace (integrated in 2004, well after the initial integration in 2003), and since then we've learned a lot about what we got right, what we got wrong, and where it's rough -- [in some cases very rough](https://twitter.com/#!/bcantrill/status/187246955464884226) -- around the edges.

In his opening remarks, Bryan identified USDT improvements as a key area for the community's focus. In DTrace development we tried to focus on making the impossible possible rather than making the possible easier. In its current form, some things are still impossible with DTrace, namely consumption of type structures from user-land programs; stable, non-privileged use of DTrace; and support for different runtime versions. [Dave Pacheco](http://dtrace.org/blogs/dap/) and I took the first  slot on the schedule and spoke (at length -- sorry) about solutions to these problems.

While others had the benefit of a bit more time to prepare, I did have the advantage of spending many years idly contemplating the problem space and possible solutions. On the subject of user-land type information (in the form of CTF), I identified the key parts of the code that would would need some work. For the USDT enhancements, we discussed dynamic translators -- D code that would be linked and executed at runtime, contrasted with today's static translators that are compiled into a D program -- how they would address the problem, and how these ideas could be extended to the kernel (for once, user-land is actually a bit ahead).

I'll go into the details of our off the cuff proposals, and delve into the code to firm up those ideas in a future blog post. Beyond the extensive implementation work we laid out, the next step is to gather the most complicated, extant USDT providers and proposals for other providers, and figure out what they should look like in the new, dynamic translator world.

### The D Language

<iframe width="224" height="126" src="http://www.youtube.com/embed/1NM7lAvCxFc" frameborder="0" class="alignright"></iframe>

Next up, my long-time colleague, DTrace contributor, [Eric Schrock](http://dtrace.org/blogs/eschrock/) led the discussion on D language additions. The format of a D program is heavily tied to DTrace's implementation: all clauses must trace a fixed amount of data, and infinite loops are forbidden. For this reason, D lacks the backward branches needed for traditional looping, subroutines for common code, and if/else clauses for control flow. Each of these has a work-alike -- unrolled loops, macros, and predicates or the ternary operator -- but their absence renders D confusing to some -- especially those unaware of the motivation. Further, the D language need not necessarily hold the underlying implementation so central.

Eric discussed some proposals for how each might be addressed, and I noted that it would be possible to create a prototype environment where we could try out these "D++" features by compiling into D work-alikes. The next step is to identify the most complicated D scripts, and see what they might look like for various incarnations of those language features.

### Work with DTrace

The next few sessions focused not on changes to DTrace, but interesting work done using DTrace:

John Thompson of Sony talked about their port of DTrace to the Playstation Vita (!). Sony developers are given access to DTrace, but found it to be unfamiliar and unapproachable. John spoke his attempts to remedy this by replacing D with a C++-like interface which he implemented by replacing the D compiler with Clang.

My Fishworks colleague, [Brendan Gregg](https://twitter.com/brendangregg), showed some of beautiful visualizations they've been developing at Joyent, and talked about the analyses those visualizations enabled. As always, it was fascinating stuff. If you don't read [Brendan's blog](http://dtrace.org/blogs/brendan), you really should. Long-time DTrace advocate, [Theo Schlossnagle](http://twitter.com/postwait), talked about the visualizations they're doing in [Circonus](http://circonus.com/) -- also fascinating stuff for anyone thinking about how to present system activity in comprehensible ways. [Richard Elling](http://twitter.com/richardelling) showed the DTrace-based visualizations Nexenta used at VMworld to rave reviews.

[Mark Cavage](https://twitter.com/mcavage) [presented](http://mcavage.github.com/presentations/dtrace_conf_2012-04-03/) Joyent's work bringing DTrace to node.js; [Scott Fritchie](http://twitter.com/slfritchie) [talked about](http://www.snookles.com/scott/publications/dtrace.conf-2012.erlang-vm.pdf) DTrace for Erlang. Both were useful sources of ideas for how we could improve USDT.

Ryan Stone presented the state of DTrace on FreeBSD. That DTrace is not enabled in the build by default remains a key obstacle for adoption. I hope that Ryan et al. are able to persuade the FreeBSD leadership that their licensing fears are misguided.

### DTrace for OEL

<iframe width="224" height="126" src="http://www.youtube.com/embed/NElog3MvUC8" frameborder="0" class="alignright"></iframe>

I was delighted that Kris van Hees was able to attend to present the Oracle port to Linux. DTrace for OEL was [announced at Oracle Open World 2011](http://dtrace.org/blogs/ahl/2011/10/05/dtrace-for-linux-2/), but the initial beta [didn't live up to its billing](http://dtrace.org/blogs/ahl/2011/10/10/oel-this-is-not-dtrace/) at OOW. As is often the case, this was more a failure of messaging than of engineering. Kris and his team are making steady progress. While it's not yet in the public beta, they have the kernel function boundary tracing provider (fbt) implemented. Most heartening of all, Oracle intends to keep DTrace for OEL moving forward as the community evolves and improves DTrace -- rather than forking it. How that plays out, and what that means for DTrace on Oracle Solaris will be interesting to see, but it's great to hear that Kris sees the value of DTrace ubiquity and DTrace compatibility.

As was remarked several times, having DTrace available on the fastest growing deployment platform will be the single most significant accelerator for DTrace adoption. The work Kris and his team at Oracle are doing is probably the most important in the DTrace ecosystem, and I think that I speak for the entire DTrace community in offering to assist in any way possible.

### A ZFS DTrace Provider

Matt Ahrens and George Wilson -- respectively the co-inventor of ZFS, and the preeminent SPA developer -- presented a [proposal for a DTrace provider for ZFS](https://docs.google.com/a/delphix.com/document/d/1wOxlXX6nLm56fccIUPS6iD1pgkX57OwdD78YhQWC8oQ/edit). ZFS is a highly sophisticated filesystem, but one that is also difficult to understand. Building in rich instrumentation is going to be a tremendous step forward for anyone using ZFS (for example, our mutual employer, Delphix).

### Whither DTrace?

Jarod Jenson -- the first DTrace user outside of Sun -- took the stage in the final session to talk about DTrace adoption. Jarod has made DTrace a significant part of his business for many years. What continues to amazing him, despite numerous presentations, demonstrations, and lessons, is the relatively low level of DTrace adoption. DTrace is a tool that comes alive in the hands of a skilled, scientific, incisive practitioner -- and in all of those, Jarod is superlative -- but it can have a high bar of entry. There were many concrete suggestions for how to improve DTrace adoption. Most of them didn't hold water for me -- different avenue for education, further documentation, community outreach, higher level tools, visualizations, etc. -- but two were quite compelling: DTrace for Linux, and DTrace on [stackoverflow.com](http://stackoverflow.com) (and the like). I don't know how much room there is to participate in the former, but by all means if there are DTrace one-liners that solve problems (on Mac OS X for example), post them, and get people covertly using DTrace.

<iframe width="224" height="126" src="http://www.youtube.com/embed/rq4eR9NJMmU" frameborder="0" class="alignright"></iframe>

The core DTrace community is growing. It was great to see old friends like Steve Peters who worked on porting DTrace to Mac OS X in the same room as Kris van Hees as he spoke about his port to Linux. It was inspiring to see so many new members of the community, eager to use, build and improve DTrace. And personally it inspired me to get back into the code to finish up some projects I had in flight, and to chart out the course for some of the projects we discussed.

Thanks to everyone who attended dtrace.conf in person or online. And thanks especially to Deirdre Straughan who made it happen.
