---
title: "DTrace for developers"
date: "2004-07-01"
categories: 
  - "dtrace"
---

After presenting our [paper](http://www.sun.com/bigadmin/content/dtrace/dtrace_usenix.pdf) at [USENIX](http://www.usenix.org/events/usenix04/index.html) on Monday, I was talking with a CMU student who said, "DTrace sounds cool, but I'm not a sysadmin or anything..." When we talk about DTrace we often discuss it with a bias towards sysadmins or sytem integrators, but that's just because those folks have been working with nothing for years. DTrace is a [developer's dream](http://slashdot.org/~kma/journal/75427) come true; it lets you see any aspect of a program and in any way you can imagine.

You don't need to know much about DTrace to start using it in development. The simplest use is as an better debugging-with-printfs -- let's say I want to know the input parameters to a function that's acting up:

```
# dtrace -n pid`pgrep testapp`::testfunc:entry'{ printf("%x %x %x", arg0, arg1, arg2); }'
dtrace: description 'pid103127::testfunc:entry' matched 1 probe
CPU     ID                    FUNCTION:NAME
0  32553                   testfunc:entry 3e9 9ff7e23 4e8
0  32553                   testfunc:entry 3e9 9ff7e24 4e8
0  32553                   testfunc:entry 3e9 9ff7e25 4e8
0  32553                   testfunc:entry 3e9 9ff7e26 4e8
...

```

Not that thrilling, but to see that perspective, you'd need to add a printf() and recompile your program, or step through it with a debugger by hand. True there are other tracers like truss(1) that could do the job, but the instrumentation methodology would distort program behavior more, and that would be the end of the investigation. With DTrace, we can dive down when we see something interesting. Let's say there's a function that works 10,000 times, but then fails unexpectedly once. Conventional debugging tools or tracers would be almost completely useless for finding this -- with DTrace, of course, it's a snap using _speculations_:

```
---8<--- fail.d ---8spec = speculation();
}
pid$1:::entry,
pid$1:::return
/self->spec/
{
speculate(self->spec);
trace(epid);
}
pid$1::testfunc:return
/self->spec && arg1 == EINVAL/
{
/* Commit the speculation if we see an error... */
commit(self->spec);
self->spec = 0;
}
pid$1::testfunc:return
/self->spec/
{
/* ... and don't if it succeeds */
discard(self->spec);
self->spec = 0;
}
---8<--- fail.d ---8<---

```

And here's it running on my toy program:

```
bash-2.05# ./fail.d `pgrep testapp` | head -40
dtrace: script './fail.d' matched 5829 probes
CPU FUNCTION
0  -> testfunc                                   2928
0    -> rand                                     1654
0     rand_r                                   1656
0      -> lmutex_lock                             414
0       lmutex_unlock                           413
0      <- lmutex_unlock                          3338
0    <- rand_r                                   4577
0  <- testfunc                                   5827
...

```

Not a complicated D script, but a reallly powerful use for developers that simplifies what would formerly have been an incredibly arduous task. I've mentioned this [before](http://dtrace.org/blogs/ahl/warm_up_the_propaganda_machine), but it certainly bears repeating. Another cool use for developers is evaluating algorithms in running programs. DTrace is great for making sure your hash functions have the distribution you expect:

```
bash-2.05# dtrace -n pid`pgrep testapp`::hash_func:return'{ @ = lquantize(arg1, 0,  50); }'
dtrace: description 'pid103250::hash_func:return' matched 1 probe
^C
value  ------------- Distribution ------------- count
< 0 |                                         0
0 |@@@                                      216450
1 |@@@                                      216593
2 |@@                                       144220
3 |@@                                       144628
4 |@@                                       144438
5 |@@                                       144580
6 |@@                                       143955
7 |@@                                       143975
8 |@@                                       144467
9 |@@                                       144065
10 |@@                                       144100
11 |@@                                       144565
12 |@@                                       143443
13 |@@                                       144123
14 |@@                                       144211
15 |@@                                       144908
16 |@@                                       143775
17 |@@                                       144463
18 |@@                                       144231
19 |@@                                       144732
20 |                                         0

```

A really simple DTrace invocation, but looking at this table, you can immiately see a potential problem with the hash function: we're getting too many entries hashing to buckets 0 and 1.

The power of DTrace for developers it the ability to create arbitrarily complex views into a program's behavior and to gather statistics without the need to modify your code. These are just two simple examples, but hopefully you can see the many many ways you could use DTrace to make debugging easier.
