---
title: "It's tested or it's broken"
date: "2006-12-12"
categories: 
  - "software"
---

It's amazing how lousy software is. That we as a society have come to accept buggy software as an inevitability is either a testament to our collective tolerance, or -- much more likely -- the near ubiquity of crappy software. So we are guilty of accepting low standards for software, but the smaller we of software writers are guilty of setting those low expectations. And I mean _we_: all of us. Every programmer has at some time written buggy software (or has never written any software of any real complexity), and while we're absolutely at fault its not from lack of exertion. From time immemorial PhD candidates have scratched their whiteboard markers dry in attempts to eliminate bugs with new languages, analysis, programming techniques, and styles. The simplest method for finding bugs before they're released into the wild remains the most generally effective: testing.

Of course, programmers perform at least nominal checks before integrating new code, but there's only so much a person can test by hand. So we've invented tests suites -- collections of tests that require no interaction. Testing rigor is regarded by university computer science departments a bit like ditch-digging is by civil engineering departments: a bit pedestrian. So people tend to sort it out for themselves. Here are a couple of tips for software tests that have come out of my experience using and developing tests suites (and the DTrace test suite in particular):

### It has to be easy to run

A favorite mantra of [a colleague of mine](http://blogs.sun.com/bill/) is that software is only as good as its test suite. While slightly less pithy, I'd add that **a test suite is only as good as one's ability to run it**. At Sun we have test suites for all kinds of crazy things. Many of them require elaborate configurations, and complex installations. Even when you manage to get everything set up (or, as often as not, find someone else to get it set up) and run, comprehending the results can require a visit from the high priestess of QA to scrutinize the pigeon entrails of the output logs.

Installing and executing a test suite needs to be so simple that it can be done by any moron who might have the wherewithall to be able to modify the software it tests (hint: that's usually a lower bar than you'd like). The same goes for understanding the results. Building the DTrace test suite creates a package which you then install wherever you want to perform the testing. Running it (by executing a single command) produces output indicating how many tests passed and how many failed. A single failure represents a bug. I've used test suites where there are expected failures (things are no more broken than they were), and unexpected failures (you broke something), but differentiating the two can be nearly impossible for a novice. Keep it simple and easy to understand, or don't bother at all -- no one will run tests they can't figure out.

### Complete and up-to-date

Now that people are executing the test suite because it's such a breeze, it actually needs to _test_ the software. I think it's productive to write tests both from the perspective of the implementation and the documented behavior, but there just needs to be adequate coverage -- and the extent of the coverage is often you can test for with some accuracy. As the software is evolving, the test suite needs to evolve with it. Every enhancement or bug fix should be accompanied with new tests to verify the change, and to ensure that it's not regressed in the future. On projects I've worked on, the tests for certain features have required much more thought and effort than the feature itself, but skipping the test is absolutely unacceptable. In short: **a test suite should completely test the target software at any given moment**.

### With the code

Originally we developed the DTrace test suite as a separate code base. This caused some unanticipated problems. Since they were in different places, we would often integrate a change to DTrace and forget about the test for a couple of days -- violating the constraint noted above. Also, projects that lagged behind the main repository would run the test suite and encounter a bunch of spurious failures because they were effectively testing out of date software. We had similar problems when back-porting new DTrace features and fixes to Solaris 10.

The solution -- in a rare split decision among the DTrace team -- was to integrate the test suite into the same repository as the code. This has absolutely been the right move. **Now we can update the code and the test suite literally at the same time**, and we're forced to think about testing sooner and more rigorously. It's also proved beneficial for the back-porting effort since a given snapshot of the source base contains the correct tests for that code.

### Run automatically

Ideally it shouldn't be necessary, but automatically running tests is a great way to ensure that errors don't creep in because of sloppy engineering or seemingly unrelated changes. This is actually an area where DTrace is a less compelling role model. If we had put this procedure in place, it would have helped us to catch at least one bug quite a bit earlier. Solaris Nevada -- the code name for the next Solaris release -- recently changed compiler versions which resulted in a DTrace bug due to a newly aggressive optimizer on SPARC. The DTrace test suite picked this up immediately, but it wasn't run for at least a week after the compiler switch was made. We're working to have it run nightly, and our new project has been running nightly tests for a few weeks now.

### Go forth and test

I've spent too many hours trying to figure out how to run arcane test suites -- just so I can't be accused of unduly contributing to the crappy state of software. I hope some of these (admittedly less-than-brilliant) lessons learned from testing DTrace have been helpfull. If you want to check out the DTrace test suite, you can see the code [here](http://src.opensolaris.org/source/xref/onnv/onnv-gate/usr/src/cmd/dtrace/test/) and find the documentation for it [here](http://opensolaris.org/os/community/dtrace/dtest/).

* * *

Technorati Tags: [DTrace](http://technorati.com/tag/DTrace) [OpenSolaris](http://technorati.com/tag/OpenSolaris) [Testing](http://technorati.com/tag/Testing) [QA](http://technorati.com/tag/QA)
