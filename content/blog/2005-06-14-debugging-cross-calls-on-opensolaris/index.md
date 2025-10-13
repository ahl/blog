---
title: "Debugging cross calls on OpenSolaris"
date: "2005-06-14"
categories:
  - "opensolaris"
permalink: /2005/06/14/debugging-cross-calls-on-opensolaris/
---

I think the thing I love most about debugging software is that each tough bug can seem like an insurmountable challenge -- until you figure it out. But until you do, each tough bugs is the hardest problem you've ever had to solve. There are weeks when every morning presents me with a seemingly impossible challenge, and each afternoon I get to spike my keyboard and do a little victory dance before running for the [train](http://www.caltrain.com).

For my first [OpenSolaris](www.opensolaris.org) blog post, I thought I talk about one of my favorite such bugs. This particularly [nasty bug](http://bugs.opensolaris.org/bugdatabase/view_bug.do?bug_id=5095242) had to do with a tricky interaction between [VMware](http://www.vmware.com) and [DTrace](http://www.sun.com/bigadmin/content/dtrace/) (pre-production versions of each to be clear). My buddy Keith -- a fellow [Brown](http://www.brown.edu) [CS](http://www.cs.brown.edu) Grad -- gave me a call and told me about some strange behavior he was seeting running DTrace inside of a VMware VM. Keith is a [big fan](http://slashdot.org/~kma/journal/75427) of DTrace, but an intermittant, but reproducable problem was putting a damper on his DTrace enthusiasm. Every once in a while, running DTrace would cause the system to freeze. Because Solaris was running in the virtual environment, he could see that both virtual CPUs where spinning away, but they weren't making any forward progress. After a couple of back and forths over email, I made the trip down to Palo Alto so we could work on the problem together.

Using some custom debugging tools, we were able to figure out where the two virtual CPUs were spinning. One CPU was in `xc_common()` and the other was in `xc_serv()` -- code to handle cross calls. So what was going on?

### cross calls

Before I can really delve into the problem, I want to give just a brief overview of cross calls. In general terms, a cross call (xcall) is used in a multi-processor (MP) system when one CPU needs to get another CPU to do some work. It works by sending a special type of interrupt which the remote CPU handles. You may have heard the term interprocessor interrupt (IPI) -- same thing. One example of when xcalls are used is when unmapping a memory region. To unmap a region, a process will typically call the `munmap(2)` system call. Remember that in an MP system, any processor may have run threads in this process so those mappings may be present in that any CPU's TLB. The unmap operation executes on one CPU, but the other CPUs need to remove the relevant mappings from their own TLBs. To accomplish this communication, the kernel uses a xcall.

DTrace uses xcalls synchronize data used by all CPUs by ensuring that all CPUs have reached a certain point in the code. DTrace executes actions with interrupts disabled (an explanation of why this must be so is well beyond the bounds of this discussion) so we can tell that a CPU isn't in probe context if its able to handle our xcall. When DTrace is stopping tracing activity, for example, it will update some data that affects all CPUs and then use a xcall to make sure that every CPU has seen its effects before proceeding:

[`dtrace_state_stop()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/common/dtrace/dtrace.c#dtrace_state_stop)

```
10739           /*
10740            * We'll set the activity to DTRACE_ACTIVITY_DRAINING, and issue a sync
10741            * to be sure that every CPU has seen it.  See below for the details
10742            * on why this is done.
10743            */
10744           state->dts_activity = DTRACE_ACTIVITY_DRAINING;
10745           dtrace_sync();

```

`dtrace_sync()` sends a xcall to all other CPUs and has them spin in a holding pattern until all CPUs have reached that point at which time the CPU which sent the xcall releases them all (and they go back to whatever they had been doing when they received the interrupt). That's the high level overview; let's go into a little more detail on how xcalls work (well, actually a lot more detail).

### xcall implementation

If you follow the sequence of functions called by [`dtrace_sync()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/common/dtrace/dtrace.c#dtrace_sync) (and I encourage you to do so), you'll find that this eventually calls `xc_common()` to do the heavy lifting. It's important to note that this call to [`xc_common()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/i86pc/os/x_call.c#xc_common) will have the `sync` argument set to `1`. What's that mean? In a text book example of good software engineering, [someone](http://blogs.sun.com/JoeBonasera) did a good job documenting what this value means:

[`xc_common()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/i86pc/os/x_call.c#432)

```
411    /*
 412     * Common code to call a specified function on a set of processors.
 413     * sync specifies what kind of waiting is done.
 414     *      -1 - no waiting, don't release remotes
 415     *      0 - no waiting, release remotes immediately
 416     *      1 - run service locally w/o waiting for remotes.
 417     *      2 - wait for remotes before running locally
 418     */
 419    static void
 420    xc_common(
 421            xc_func_t func,
 422            xc_arg_t arg1,
 423            xc_arg_t arg2,
 424            xc_arg_t arg3,
 425            int pri,
 426            cpuset_t set,
 427            int sync)

```

Before you start beating your brain out trying to figure out what you're missing here, in this particular case, this destinction bewteen `sync` having the value of 1 and 2 is nil: the service (function pointer specified by the `func` argument) that we're running is [`dtrace_sync_func()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/i86pc/os/dtrace_subr.c#dtrace_sync_func) which does literally nothing.

Let's start picking apart `xc_common()`:

[`xc_common()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/i86pc/os/x_call.c#467)

```
446            /*
 447             * Request service on all remote processors.
 448             */
 449            for (cix = 0; cix < NCPU; cix++) {
 450                    if ((cpup = cpu[cix]) == NULL ||
 451                        (cpup->cpu_flags & CPU_READY) == 0) {
 452                            /*
 453                             * In case CPU wasn't ready, but becomes ready later,
 454                             * take the CPU out of the set now.
 455                             */
 456                            CPUSET_DEL(set, cix);
 457                    } else if (cix != lcx && CPU_IN_SET(set, cix)) {
 458                            CPU_STATS_ADDQ(CPU, sys, xcalls, 1);
 459                            cpup->cpu_m.xc_ack[pri] = 0;
 460                            cpup->cpu_m.xc_wait[pri] = sync;
 461                            if (sync > 0)
 462                                    cpup->cpu_m.xc_state[pri] = XC_SYNC_OP;
 463                            else
 464                                    cpup->cpu_m.xc_state[pri] = XC_CALL_OP;
 465                            cpup->cpu_m.xc_pend[pri] = 1;
 466                            send_dirint(cix, xc_xlat_xcptoipl[pri]);
 467                    }
 468            }

```

We take a first pass through all the processors; if the processor is ready to go and is in the set of processors we care about (they all are in the case of `dtrace_sync()`) we set the remote CPU's **ack** flag to 0, it's **wait** flag to `sync` (remember, 1 in this case), and it's **state** flag to `XC_SYNC_OP` and then actually send the interrupt with the call to `send_dirint()`.

Next we wait for the remote CPUs to acknowledge that they've executed the requested service which they do by setting the **ack** flag to 1:

[`xc_common()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/i86pc/os/x_call.c#500)

```
479            /*
 480             * Wait here until all remote calls complete.
 481             */
 482            for (cix = 0; cix < NCPU; cix++) {
 483                    if (lcx != cix && CPU_IN_SET(set, cix)) {
 484                            cpup = cpu[cix];
 485                            while (cpup->cpu_m.xc_ack[pri] == 0) {
 486                                    ht_pause();
 487                                    return_instr();
 488                            }
 489                            cpup->cpu_m.xc_ack[pri] = 0;
 490                    }
 491            }

```

That `while` loop spins waiting for **ack** to become 1. If you look at the definition of [`return_instr()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/i86pc/ml/mpcore.s#764) it's name is actually more descriptive that you might imagine: it's just a return instruction -- the most trivial function possible. I'm not absolutely certain, but I think it was put there so the compiler wouldn't "optimize" the loop away. The call to the inline function [`ht_pause()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/intel/amd64/ml/amd64.il#296) is so that the thread spins in such a way that's considerate on an [hyper-threaded](http://www.intel.com/technology/hyperthread/) CPU. The call to `ht_pause()` is probably sufficient to prevent the compiler from being overly clever, but the legacy call to `return_instr()` remains.

Now let's look at the other side of this conversation: what happens on a remote CPU as a result of this interrupt? This code is in `xc_serv()`

[`xc_serv()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/i86pc/os/x_call.c#159)

```
138    	/*
139    	 * Acknowledge that we have completed the x-call operation.
140    	 */
141    	cpup->cpu_m.xc_ack[pri] = 1;
142

```

I'm sure it comes as no surprise that after executing the given function, it just sets the **ack** flag.

Since in this case we're dealing with a synchronous xcall, the remote CPU then needs to just chill out until the CPU that initiated the xcall discovers that all remote CPUs have executed the function and are ready to be released:

[`xc_serv()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/i86pc/os/x_call.c#167)

```
146    	/*
147    	 * for (op == XC_SYNC_OP)
148    	 * Wait for the initiator of the x-call to indicate
149    	 * that all CPUs involved can proceed.
150    	 */
151    	while (cpup->cpu_m.xc_wait[pri]) {
152    		ht_pause();
153    		return_instr();
154    	}
155
156    	while (cpup->cpu_m.xc_state[pri] != XC_DONE) {
157    		ht_pause();
158    		return_instr();
159    	}

```

And here's the code on the initiating side that releases all the remote CPUs by setting the **wait** and **state** flags to the values that the remote CPUs are waiting to see:

[`xc_common()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/i86pc/os/x_call.c#523)

```
 502            /*
 503             * Release any waiting CPUs
 504             */
 505            for (cix = 0; cix < NCPU; cix++) {
 506                    if (lcx != cix && CPU_IN_SET(set, cix)) {
 507                            cpup = cpu[cix];
 508                            if (cpup != NULL && (cpup->cpu_flags & CPU_READY)) {
 509                                    cpup->cpu_m.xc_wait[pri] = 0;
 510                                    cpup->cpu_m.xc_state[pri] = XC_DONE;
 511                            }
 512                    }
 513            }

```

### there's a problem

Wait! Without reading ahead in the code, does anyone see the problem?

Back at VMware, Keith hacked up a version of the virtual machine monitor which allowed us to trace certain points in the code and figure out the precise sequence in which they occurred. We traced the entry and return to `xc_common()` and `xc_serv()`. Almost every time we'd see something like this:

- enter `xc_common()` on CPU 0
- enter `xc_serv()` on CPU 1
- exit `xc_serv()` on CPU 1
- exit `xc_common()` on CPU 0

or:

- enter `xc_common()` on CPU 0
- enter `xc_serv()` on CPU 1
- exit `xc_common()` on CPU 0
- exit `xc_serv()` on CPU 1

But the problem happened when we saw a sequence like this:

- enter `xc_common()` on CPU 0
- enter `xc_serv()` on CPU 1
- exit `xc_common()` on CPU 0
- enter `xc_common()` on CPU 0

And nothing futher. What was happening was that after releasing remote CPUs, CPU 0 was exiting from the call to `xc_common()` and calling it again before the remote invocation of `xc_serv()` on the other CPU had a change to exit.

Recall that one of the the first things that `xc_common()` does is set the **state** flag. If the first call to `xc_common()` sets the **state** flag to release the remote CPU from `xc_sync()`, but when things go wrong, `xc_common()` was overwriting that flag before the remote CPU got a change to see it.

### the problem

We were seeing this repeatably under VMware, but no one had seen this at all on real hardware (yet). The machine Keith and I were using was a 2-way box running Linux. On VMware, each virtual CPU is represented by a thread on the native OS so rather than having absolute control of the CPU, the execution was more or less at the whim of the Linux scheduler.

When this code is running unadulterated on physical CPUs, we won't see this problem. It's just a matter of timing -- the remote CPU has many many fewer instructions to execute before the **state** flag gets overwritten by a second xcall so there's no problem. On VMware, the Linux scheduler might decide that's your second virtual CPU's right to the physical CPU is trumped by moving the hands on your xclock (why not?) so there are no garuantees about how long these operations can take.

### the fix

There are actually quite a few ways to fix this problem -- I'm sure you can think of at least one or two off the top of your head. We just need to make sure that subsequent xcalls can't interfere with each other. When we found this, Solaris 10 was wrapping up -- we were still making changes, but only those deemed of the absolute highest importance. Making changes to the xcall code (which is rather delicate and risky to change) for a bug that only manifests itself on virtual hardware (and which VMware could work around using some clever trickery\[1\]) didn't seem worthy of being designated a show- stopper.

Keith predicted a few possible situations where this same bug could manifest itself on physical CPUs: on hyper-threaded CPUs, or in the presence of service management interrupts. And that prediction turned out to be spot on: a few weeks after root causing the bug under VMware, we hit the same problem on a system with four hyper-threaded chips (8 logical CPUs).

Since at that time we were even closer to shipping Solaris 10, I chose the fix I thought was the safest and least likely to have nasty side effects. After releasing remote CPUs, the code in `xc_common()` would now wait for remote CPUs to check in -- wait for them to acknowledge receipt of the directive to proceed.

[`xc_common()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/i86pc/os/x_call.c#536)

```
 515            /*
 516             * Wait for all CPUs to acknowledge completion before we continue.
 517             * Without this check it's possible (on a VM or hyper-threaded CPUs
 518             * or in the presence of Service Management Interrupts which can all
 519             * cause delays) for the remote processor to still be waiting by
 520             * the time xc_common() is next invoked with the sync flag set
 521             * resulting in a deadlock.
 522             */
 523            for (cix = 0; cix < NCPU; cix++) {
 524                    if (lcx != cix && CPU_IN_SET(set, cix)) {
 525                            cpup = cpu[cix];
 526                            if (cpup != NULL && (cpup->cpu_flags & CPU_READY)) {
 527                                    while (cpup->cpu_m.xc_ack[pri] == 0) {
 528                                            ht_pause();
 529                                            return_instr();
 530                                    }
 531                                    cpup->cpu_m.xc_ack[pri] = 0;
 532                            }
 533                    }
 534            }

```

In that comment, I tried to summarize in 6 lines what has just taken me several pages to describe. And maybe I should have said "livelock" -- oh well. Here's the complementary code in `xc_serv()`:

[`xc_serv()`](http://cvs.opensolaris.org/source/xref/usr/src/uts/i86pc/os/x_call.c#191)

```
 170            /*
 171             * Acknowledge that we have received the directive to continue.
 172             */
 173            ASSERT(cpup->cpu_m.xc_ack[pri] == 0);
 174            cpup->cpu_m.xc_ack[pri] = 1;

```

### conclusions

That was one of my favorite bugs to work on, and it's actually fairly typical of a lot of the bugs I investigate: something's going wrong; figure out why. I think the folks who work on Solaris tend to love that kind of stuff as a rule. We spend tons of time building facilities like DTrace, mdb(1), kmdb, CTF, fancy core files, and libdis so that the hart part of investigating mysterious problems isn't gathering data or testing hypotheses, it's _thinking_ of the questions to answer and inventing new hypotheses. It's my hope that OpenSolaris will attract those types of inquisitive minds that thrive on the (seemingly) unsolvable problem.

* * *

\[1\] This sort of problem is hardly unique to DTrace or to Solaris. Apparently (and not surprisingly) there are problems like this in nearly every operating system where the code implicitly or explicitly relies on the relative timing of certain operations. In these cases, VMware has hacks to do things like execute the shifty code in lock step.

Technorati Tag: [OpenSolaris](http://www.technorati.com/tag/OpenSolaris) Technorati Tag: [Solaris](http://www.technorati.com/tag/Solaris) Technorati Tag: [DTrace](http://www.technorati.com/tag/DTrace) Technorati Tag: [mdb](http://www.technorati.com/tag/mdb)
