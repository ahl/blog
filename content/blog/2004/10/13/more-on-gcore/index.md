---
title: "more on gcore"
date: "2004-10-13"
categories:
  - "opensolaris"
permalink: /2004/10/13/more-on-gcore/
---

Trawling through b.s.c I noticed [Fintan Ryan](http://blogs.sun.com/fintanr) [talking](http://blogs.sun.com/roller/page/fintanr/20041011#gcore) about [gcore(1)](http://docs.sun.com/db/doc/816-5165/6mbb0m9h7?a=view), and I realized that I hadn't sufficently promoted this cool utility. As part of my work adding [variable core file content](http://dtrace.org/blogs/ahl/number_13_of_20_core), I rewote gcore from scratch (it used to be a real pile) to add a few new features and to make it use libproc (i.e. make it slightly less of a pile).

You use gcore to take a core dump of a live running process without actually causing the process to crash. It's not completely uninvasive because gcore stops the process you're taking the core of to ensure a consistent snapshot, but unless the process is _huge_ or it's _really_ cranky about timing the perturbation isn't noticeable. There are a lot of places where taking a snapshot with gcore is plenty useful. Let's say a process is behaving strangely, but you can't attach a debugger because you don't want to take down the service, or you want to have a core file to send to someone who can debug it when you yourself can't -- gcore is perfect. I use to it to take cores of mozilla when it's chugging away on the processor, but not making any visible progress.

I mentioned that big processes can take a while to gcore -- not surprising because we have to dump that whole image out to disk. One of the cool uses of variable core file content is the ability to take faster core dumps by only dumping the sections you care about. Let's say there's some big ISM segment or a big shared memory segment: exclude it and gcore will go faster:

```
hedge /home/ahl -> gcore -c default-ism 256755
gcore: core.256755 dumped

```

Pretty handy, but the coolest I've been making of gcore lately is by mixing it with DTrace and the new(ish) `system()` action. This script snapshots my process once every ten seconds and names the files according to the time they were produced:

```
# cat gcore.d
#pragma D option destructive
#pragma D option quiet
tick-10s
{
doit = 1;
}
syscall:::
/doit && pid == $1/
{
stop();
system("gcore -o core.%%t %d", pid);
system("prun %d", pid);
doit = 0;
}
# dtrace -s gcore.d  256755
gcore: core.1097724567.256755 dumped
gcore: core.1097724577.256755 dumped
gcore: core.1097724600.256755 dumped
^C

```

**WARNING!** When you specify destructive in DTrace, it means destructive. The `system()` and `stop()` actions can be absolutely brutal (I've rendered at least one machine unusable my indelicate use of that Ramirez-Ortiz-ian one-two combo. That said, if you screw something up, you can break into the debugger and set dtrace\_destructive\_disallow to 1.

OK, so be careful, but that script can give you some pretty neat results. Maybe you have some application that seems to be taking a turn for the worse around 2 a.m. -- put together a DTrace script that detects the problem and use gcore to take a snapshot so you can figure out what was going on when to get to the office in the morning. Take a couple of snapshots to see how things are changing. You do like debugging from core dumps, right?
