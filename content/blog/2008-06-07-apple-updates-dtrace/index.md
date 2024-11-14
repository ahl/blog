---
title: "Apple updates DTrace"
date: "2008-06-07"
categories:
  - "dtrace"
permalink: /2008/06/07/apple-updates-dtrace/
---

Back in January, I [posted about a problem](http://dtrace.org/blogs/ahl/mac_os_x_and_the) with Apple's port of [DTrace](http://opensolaris.org/os/community/dtrace/) to Mac OS X. The heart of the issue is that their port would silently drop data such that certain experiments would be quietly invalid. Unfortunately, [most reactions seized on a headline](http://developers.slashdot.org/article.pl?sid=08/01/22/2156244) paraphrasing a line of the post — albeit with the critical negation omitted (the subject and language were, perhaps, too baroque to expect [the press](http://www.theregister.co.uk/2008/01/22/sun_apple_dtrace/) to read every excruciating word). The good news is that Apple has (quietly) fixed the problem in Mac OS X 10.5.3.

One issue was that timer based probes wouldn't fire if certain applications were actively executing (e.g. iTunes). This was evident both by counting periodic probe firings, and by the absence of certain applications when profiling. Apple chose to solve this problem by allowing the probes to fire while denying any inspection of untraceable processes (and generating a verbose error in that case). This script which should count 1000 firings per virtual CPU gave sporadic results on earlier revisions of Mac OS X 10.5:

```
profile-1000
{
        @ = count();
}
tick-1s
{
        printa(@);
        clear(@);
}

```

On 10.5.3, the output is exactly what one would expect on a 2-core CPU (1,000 executions per core):

```
1  22697                         :tick-1s
2000
1  22697                         :tick-1s
2000

```

On previous revisions, profiling to see what applications were spending the most time on CPU would silently omit certain applications. Now, while we can't actually peer into those apps, we can infer the presence of stealthy apps when we encounter an error:

```
profile-199
{
        @[execname] = count();
}
ERROR
{
        @["=stealth app="] = count();
}

```

Running this DTrace script will generate a lot of errors as we try to evaluate the `execname` variable for secret applications, but at the end we'll end up with a table like this:

```
Adium                                                             1
GrowlHelperApp                                                    1
iCal                                                              1
kdcmond                                                           1
loginwindow                                                       1
Mail                                                              2
Activity Monito                                                   3
ntpd                                                              3
pmTool                                                            6
mlb-nexdef-auto                                                  12
Terminal                                                         14
=stealth app=                                                    29
WindowServer                                                     34
kernel_task                                                     307
Safari                                                          571

```

A big thank you to Apple for making progress on this issue; the situation is now much improved and considerably more palatable. That said, there are a couple of problems. The first is squarely the fault of team DTrace: we should probably have a mode where errors aren't printed particularly if the script is already handling them explicitly using an `ERROR` probe as in the script above. For the Apple folks: I'd argue that revealing the name of otherwise untraceable processes is no more transparent than what Activity Monitor provides — could I have that please? Also, I'm not sure if this has always been true, but the ustack() action doesn't seem to work from the profile action so simple profiling scripts like this one produce a bunch of errors and no output:

```
profile-199
/execname == "Safari"/
{
        @[ustack()] = count();
}

```

But to reiterate: thank you thank you thank you, Steve, James, Tom, and the rest of the DTrace folks at Apple. It's great to see these issues being addressed. The whole DTrace community appreciates it.
