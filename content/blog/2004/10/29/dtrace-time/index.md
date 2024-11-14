---
title: "DTrace time"
date: "2004-10-29"
categories:
  - "dtrace"
permalink: /2004/10/29/dtrace-time/
---

The other day I spent some time with a customer tuning their app using DTrace. I hadn't done much work on apps outside of Sun, so I was excited to see what I could do with DTrace other people's code. I'm certainly not the first person to use DTrace on a real-world app (I'm probably the 10,000th), so I had some expectations to live up to.

The app basically processes transactions: message comes in, the app does some work and sends out another message. There's a lot of computation for each message, but also a lot of bookkeeping and busy work receiving, processing and generating the messages.

#### When sprintf(3C) attacks...

One of the first things we looked at was what functions the app calls the most. Nothing fancy; just a DTrace one-liner:

```
# dtrace -n pid123:::entry'{ @[probefunc] = count(); }'

```

Most of it made sense, but one thing we noticed was that sprintf(3C) was getting called a ton, so the question was "what are those sprintfs doing?" Another one-liner had the answer:

```
# dtrace -n pid123::sprintf:entry'{ @[copyinstr(arg0)] = count(); }'

```

There were about four different format strings being used, two of them farily complex and then these two: "%ld" and "%f". In about 5 seconds, the app was calling sprintf("%ld", ) several thousand times. It turns out that each transactions contains an identifier -- represented by a decimal integer -- so this wasn't unexpected per se, but we speculated that using lltostr(3C) or hand rolling a int->string function might yield better results.

I had just assumed that lltostr(3C) would perform better -- it's a specialized function that doesn't involve all the (extensive and messy) machinery of sprintf(3C). I wrote a little microbenchmark that just converted the number 1000000 to a string with both functions a million times and ran it on an x86 machine; the results were surprising:

```
$ ./test
sprintf(3C) took 272512920ns
lltostr(3C) took 523507925ns

```

What? I checked my test, made sure I was compiling everything properly, had called the function once before I started timing (to avoid any first-call overhead from the dynamic linker), but the results were dead repeatable: lltostr(3C) was about half as fast. I looked at the implementation and while I can't post it here (yet -- I can't wait for OpenSolaris), suffice it to say that it did the obvious thing. The strange thing was that sprintf(3C) had basically the same alogorithm. Just for kicks, I decided to build it amd64 native and run it on the same opteron box; here were the results:

```
sprintf(3C) took 140706282ns
lltostr(3C) took 38804963ns

```

Ah much better (I love opteron). It turns out the problem was that we were doing 64-bit math in 32-bit mode -- hence _ell-ell_\-to-str -- and that is slooooooow. Luckily the app we were looking at was compiled 64-bit native so it wouldn't have had this problem, but there are still plenty of 32-bit apps out there that shouldn't have to pay the 64-bit math tax in this case. I made a new version of lltostr(3C) that checks the top 32-bits of the 64-bit input value and does 32-bit math if those bits are clear. Here's how that performed (on 32-bit x86):

```
sprintf(3C) took 251953795ns
lltostr(3C) took 459720586ns
new lltostr took  32907444ns

```

Much better. For random numbers between 232 and 2^63\-1 the change hurt performance by about 1-2%, but it's probably worth the hit for the 1300% improvement with smaller numbers.

In any case, that was just a _long_ way of saying that for those of you using lltostr(3C) or sprintf(%ld), there are wins to be had.

#### Timing transactions

Our first use of DTrace was really to discover things that the developers of that app didn't know about. This sprintf(3c) stuff was one issues, and there were a couple of others, but, on the whole, the app worked as intended. And that's actually no small feat -- many many programs are doing lots of work that wasn't really intended by the developers or spend their CPU time in places that are completely surprising to the folks who wrote it. The next thing the customers wanted to find was the source of certain latency bubbles. So we used DTrace to time each transaction as it went through multiple processes on the system. While each step was fairly simple, the sum total was by far the largest D script I had written, and in the end we were able to record the time a transaction spent over each leg of the trip through the system.

This is something that could have been done without DTrace, but it would have involved modifying the app, and streaming the data out to some huge file to be post-processed later. Not only is that a huge pain in the ass, it can also have a huge performance impact and is inflexible in terms of the data gathered. With DTrace, the performance impact can be mitigated by instrumenting less, and you can gather arbitrary data so when you get the answer to your first question, you don't have to rebuild and rerun the app to dive deeper to your next question.

I had been telling people for a while about the virtues on real-world applications, I was happy to see first hand that they were all true, and -- perhaps more importantly -- I convinced the customer.
