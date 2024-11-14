---
title: "Apple updates DTrace... again"
date: "2008-10-04"
categories: 
  - "dtrace"
---

Back in January, I [ranted](http://dtrace.org/blogs/ahl/mac_os_x_and_the) about Apple's ham-handed breakage in their DTrace port. After some injured feelings and teary embraces, Apple [cleaned things up a bit](http://dtrace.org/blogs/ahl/apple_updates_dtrace), but some nagging issues remained as I wrote:

> For the Apple folks: I'd argue that revealing the name of otherwise untraceable processes is no more transparent than what Activity Monitor provides — could I have that please?

It would be very un-Apple to — you know — communicate future development plans, but in 10.5.5, DTrace has seen another improvement. Previously when using DTrace to observe the system at large, iTunes and other paranoid apps would be hidden; now they're showing up on the radar:

```
# dtrace -n 'profile-1999{ @[execname] = count(); }'
dtrace: description 'profile-1999' matched 1 probe
^C
loginwindow                                                       2
fseventsd                                                         3
kdcmond                                                           5
socketfilterfw                                                    5
distnoted                                                         7
mds                                                               8
dtrace                                                           12
punchin-helper                                                   12
Dock                                                             20
Mail                                                             25
Terminal                                                         26
SystemUIServer                                                   28
Finder                                                           42
Activity Monito                                                  49
pmTool                                                           67
WindowServer                                                    184
iTunes                                                         1482
kernel_task                                                    4030

```

And of course, you can use generally available probes to observe only those touchy apps with a predicate:

```
# dtrace -n 'syscall:::entry/execname == "iTunes"/{ @[probefunc] = count(); }'
dtrace: description 'syscall:::entry' matched 427 probes
^C
...
pwrite                                                           13
read                                                             13
stat64                                                           13
open_nocancel                                                    14
getuid                                                           22
getdirentries                                                    26
pread                                                            29
stat                                                             32
gettimeofday                                                     34
open                                                             36
close                                                            37
geteuid                                                          65
getattrlist                                                     199
munmap                                                          328
mmap                                                            338

```

Predictably, the details of iTunes are still obscured:

```
# dtrace -n pid42896:::entry
...
dtrace: error on enabled probe ID 225607 (ID 69364: pid42896:libSystem.B.dylib:pthread_mutex_unlock:entry):
invalid user access in action #1
dtrace: error on enabled probe ID 225546 (ID 69425: pid42896:libSystem.B.dylib:spin_lock:entry):
invalid user access in action #1
dtrace: 1005103 drops on CPU 1

```

... which is fine by me; I've got code of my own I should be investigating. While I'm loath to point it out, an astute reader and savvy DTrace user will note that Apple may have left the door open an inch wider than they had anticipated. Anyone care to post some D code that makes use of that inch? I'll post an update as a comment in a week or two if no one sees it.

**Update:** There were some good ideas in the comments. Here's the start of a script that can let you follow the flow of control of a thread in an "untraceable" process:

```
#!/usr/sbin/dtrace -s
#pragma D option quiet
pid$target:libSystem.B.dylib::entry,
pid$target:libSystem.B.dylib::return
{
trace("this program is already traceable\n");
exit(0);
}
ERROR
/self->level level > 40/
{
self->level = 0;
}
ERROR
{
this->p = ((dtrace_state_t *)arg0)->dts_ecbs[arg1 - 1]->dte_probe;
this->mod = this->p->dtpr_mod;
this->func = this->p->dtpr_func;
this->entry = ("entry" == stringof(this->p->dtpr_name));
}
ERROR
/this->entry/
{
printf("%*s-> %s:%s\n", self->level * 2, "",
stringof(this->mod), stringof(this->func));
self->level++;
}
ERROR
/!this->entry/
{
self->level--;
printf("%*slevel * 2, "",
stringof(this->mod), stringof(this->func));
}

```
