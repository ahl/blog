---
title: "back from the DTrace road show"
date: "2004-10-05"
categories:
  - "dtrace"
permalink: /2004/10/05/back-from-the-dtrace-road-show/
---

A number of factors have conspired to keep me away from blogging, not the least of which being that I've been on a coast-to-coast DTrace road show. Now that I'm back, I've got some news to report from the road.

### Step right up!

At times it felt a bit like a [medicine road show](http://www.snpp.com/episodes/2F07): "Step right up to see the amazing DTraaaaace! The demystifying optimizing tantalizing reenergizing tracing frameworrrrrrrk!" I stopped in the Midwest, and D.C. ([Eric](http://blogs.sun.com/eschrock) helped during that leg as my fellow huckster) then I went back to San Francisco and then back accross the country to New York for the big [Wall Street to do](http://www.sun.com/aboutsun/media/features/wallstreet.html).

I admit that it got a little repetitive -- at one point I ran into A/V problems and was able to run through the presentation from memory -- but the people who I talked to, and their questions kept it interesting.

### Can DTrace do X?

I was impressed by the number of people who had not only heard of DTrace, but who had already started playing around with it. It used to be that the questions were all of the form "Can DTrace do X?" On this trip, more and more I was asked about specific things people were trying to accomplish with DTrace and told about problems they'd found using DTrace. I thought I'd repeat some of the best questions and insights from the trip:

I'm sure this won't come as a huge shock to anyone who's tried to heft a printed copy of the [DTrace docs](http://docs.sun.com/db/doc/817-6223), but some people were a little daunted by the size and complexity of DTrace. To address that, I'd call on an analogy to perl -- no one picks up the O'Reilly books on perl, reads them cover to cover and declares mastery of perl. The way most people learn perl is by calling on the knowledge they have from similar languages, finding examples that make sense and then modifying them. When they need to do something just beyond their grasp, they go back to the documentation and find the nugget they need. We've tried to follow a similar model with DTrace -- find an example that makes sense and work from there; call on your knowledge of perl, java, c, or whatever, and see how it can apply to D. We've tried to design DTrace so things pretty much just work as you (a programmer, sysadmin, whatever) would expect them to, and so that a little time invested with the documentation goes a long way. Seeing too much data? Predicates are easy. Sill not getting just the data you want? Spend fifteen minutes to wrap your head around speculations.

And speaking of perl, a lot of people asked about DTrace's visibility into perl. Right now the only non-natively executed languate DTrace lets you observe is java, but now that we realize how much need there is for visibility into perl, we're going to be working aggressively on making DTrace work well with perl. We've got some neat ideas, but if there are things you'd like to see with DTrace and perl, we'd love to hear about it as we think about what sorts of problems we need to solve.

A lot of people in industry use tools like Intel's VTune and IBM's Purify and Quantify, so I got a lot of questions about how DTrace compares to those tools. Which led to the inevitable question of "Where's the GUI?" First, DTrace by definition can do more than those tools even discounting its systemic scope simply by the ability users have to customize their tracing with D. VTune, Purify, Quantify and other tools present a fairly static view, and I'm sure that users of those tools have always had just one more question, one next step that those tools weren't solve. Because DTrace doesn't present a canned, static view, it's not so clear on what kind of GUI you'd want. Clearly, it's not just a pull down menu with 40,000 probes to choose from, so we're actively working on ways to engage the eyes and the visual cortex, but without strapping DTrace into a static framework, bounded by the same constraints of those traditional tools.

### Back

Whew. It was a good trip though a bit exhausting. I think I convinced a bunch of people about the utility of DTrace in general, but also to their specific problems. But I also learned about some of DTrace's shortcomings which we are now working to address. It's good to be back to coding -- I'm putting the finishing touches on DTrace's AMD64 support which has been a lot of fun. In the next few weeks I'll be writing about the work going on in the kernel group as we put the final coat of polish on Solaris 10 as it gets ready for its release.
