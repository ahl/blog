---
title: "DTrace OEL update"
date: "2012-02-23"
categories: 
  - "dtrace"
tags: 
  - "dtrace"
  - "oel"
  - "oracle"
---

A few months ago I took [DTrace on OEL](http://dtrace.org/blogs/ahl/2011/10/05/dtrace-for-linux-2/) for a spin after Oracle announced it. The [results were ugly](http://dtrace.org/blogs/ahl/2011/10/10/oel-this-is-not-dtrace/); as one of the authors of DTrace, I admit to being shocked by shoddiness of the effort. Yesterday, [Oracle dropped an updated beta](https://blogs.oracle.com/wim/entry/dtrace_update_to_0_2) so I wanted to see how far they've come in the 4+ months since that initial false start.

### Whither the probes?

Back in October there were 574 functional probes (and 13 more that didn't work). Here's the quantitative state of DTrace for OEL today:

```
[root@screven drivers]# dtrace -l | wc -l
618
```

Okay. Steady improvement. By way of unfair comparison, here's what it looks like on my [Mac OS X](http://dtrace.org/blogs/brendan/2011/10/10/top-10-dtrace-scripts-for-mac-os-x/) laptop:

```
qadi /Users/ahl # dtrace -l | wc -l
  578044
```

### What's new?

Back in October, I tried enabling all system call probes (i.e. all functional probes); the result was that ssh started failing mysteriously. It was a gross violation of the core principles -- it would be unacceptable for DTrace to cause harm to the production systems on which it operates. Good to see that Oracle fixed it.

Previously, profile provider probes weren't working. The profile probes have been removed -- you can't do arbitrary resolution timer-based profiling -- but the simple, tick probes are there:

```
[root@screven drivers]# dtrace -l -n profile:::
ID   PROVIDER            MODULE                          FUNCTION NAME
612    profile                                                     tick-1
613    profile                                                     tick-10
614    profile                                                     tick-100
615    profile                                                     tick-500
616    profile                                                     tick-1000
617    profile                                                     tick-5000
```

... and seem to work:

```
[root@screven ~]# dtrace -n 'tick-1{ printf("%Y", walltimestamp); }'
dtrace: description 'tick-1' matched 1 probe
CPU     ID                    FUNCTION:NAME
  0    612                          :tick-1 2012 Feb 23 04:31:27
  0    612                          :tick-1 2012 Feb 23 04:31:28
  0    612                          :tick-1 2012 Feb 23 04:31:29
```

They've also added some inscrutable SDT ([statically defined tracing](https://wikis.oracle.com/display/DTrace/sdt+Provider)) probes:

```
[root@screven ~]# dtrace -l -n sdt:::
   ID   PROVIDER            MODULE                          FUNCTION NAME
  597        sdt           vmlinux                    __handle_sysrq -handle_sysrq
  601        sdt           vmlinux                  oom_kill_process oom_kill_process
  602        sdt           vmlinux                   check_hung_task check_hung_task
  603        sdt           vmlinux                   sys_init_module init_module
  604        sdt           vmlinux                 sys_delete_module delete_module
  611        sdt           vmlinux                      signal_fault signal_fault
```

More usefully, the beta includes a partially implemented proc provider; the proc provider traces high level process activity ([check the docs](https://wikis.oracle.com/display/DTrace/proc+Provider)).

```
[root@screven ~]# dtrace -l -n proc:::
   ID   PROVIDER            MODULE                          FUNCTION NAME
  598       proc           vmlinux                  do_execve_common exec-success
  599       proc           vmlinux                  do_execve_common exec-failure
  600       proc           vmlinux                  do_execve_common exec
  605       proc           vmlinux             get_signal_to_deliver signal-handle
  606       proc           vmlinux                     __send_signal signal-send
  607       proc           vmlinux                           do_exit exit
  608       proc           vmlinux                           do_exit lwp-exit
  609       proc           vmlinux                           do_fork create
  610       proc           vmlinux                           do_fork lwp-create
```

For reference, here's what it looks like on DelphixOS, an illumos derivative (which of course includes DTrace):

```
root@argos:~# dtrace -l -n proc:::
   ID   PROVIDER            MODULE                          FUNCTION NAME
10589       proc              unix                   lwp_rtt_initial lwp-start
10629       proc              unix                   lwp_rtt_initial start
10631       proc              unix                              trap fault
10761       proc           genunix                      sigtimedwait signal-clear
10762       proc           genunix                              psig signal-handle
10763       proc           genunix                         sigtoproc signal-discard
10764       proc           genunix                         sigtoproc signal-send
10831       proc           genunix                        lwp_create lwp-create
10868       proc           genunix                             cfork create
10870       proc           genunix                         proc_exit exit
10871       proc           genunix                          lwp_exit lwp-exit
10872       proc           genunix                         proc_exit lwp-exit
10873       proc           genunix                       exec_common exec-success
10874       proc           genunix                       exec_common exec-failure
10875       proc           genunix                       exec_common exec
```

Each DTrace probe has arguments that convey information about the activity that caused the probe to fire. For example, with the kernel function boundary tracing (fbt) provider (not yet implemented in OEL), the arguments for the function entry probe correspond to the arguments passed to the function. With static providers such as the proc provider, the parameters include useful information... but I can never seem to remember the types and order. Fortunately, DTrace lets you add in the -v option to get more information about a probe. Unfortunately, this hasn't been hooked up in Oracle's port (just an bug, I'm sure):

```
[root@screven ~]# dtrace -lv -n proc:::signal-send
   ID   PROVIDER            MODULE                          FUNCTION NAME
  606       proc           vmlinux                     __send_signal signal-send

	Probe Description Attributes
		Identifier Names: Private
		Data Semantics:   Private
		Dependency Class: Unknown

	Argument Attributes
		Identifier Names: Evolving
		Data Semantics:   Evolving
		Dependency Class: ISA

	Argument Types
		args[0]: (unknown)
		args[1]: (unknown)
		args[2]: (unknown)
		args[3]: (unknown)
		args[4]: (unknown)
		args[5]: (unknown)
		args[6]: (unknown)
		args[7]: (unknown)
		args[8]: (unknown)
		args[9]: (unknown)
		args[10]: (unknown)
		args[11]: (unknown)
		args[12]: (unknown)
		args[13]: (unknown)
		args[14]: (unknown)
		args[15]: (unknown)
		args[16]: (unknown)
		args[17]: (unknown)
		args[18]: (unknown)
		args[19]: (unknown)
		args[20]: (unknown)
		args[21]: (unknown)
		args[22]: (unknown)
		args[23]: (unknown)
		args[24]: (unknown)
		args[25]: (unknown)
		args[26]: (unknown)
		args[27]: (unknown)
		args[28]: (unknown)
		args[29]: (unknown)
		args[30]: (unknown)
		args[31]: (unknown)
```

Here's what it looks like on DelphixOS:

```
root@argos:~# dtrace -lv -n proc:::signal-send
   ID   PROVIDER            MODULE                          FUNCTION NAME
10764       proc           genunix                         sigtoproc signal-send

        Probe Description Attributes
                Identifier Names: Private
                Data Semantics:   Private
                Dependency Class: Unknown

        Argument Attributes
                Identifier Names: Evolving
                Data Semantics:   Evolving
                Dependency Class: ISA

        Argument Types
                args[0]: lwpsinfo_t *
                args[1]: psinfo_t *
                args[2]: int
```

Even without the type system being hooked up, you can definitely do some useful work with this beta. For example, I can use the proc provider to look at what commands are executing on my system:

```
[root@screven ~]# dtrace -n proc:::exec'{ trace(stringof(arg0)); }'
dtrace: description 'proc:::exec' matched 1 probe
CPU     ID                    FUNCTION:NAME
  0    600            do_execve_common:exec   /usr/bin/staprun
  0    600            do_execve_common:exec   /usr/sbin/perf
  0    600            do_execve_common:exec   /bin/uname
  0    600            do_execve_common:exec   /usr/libexec/perf.2.6.39-101.0.1.el6uek.x86_64
```

On his blog, Wim Coekaerts showed some examples of use of the proc provider that included this common idiom:

```
proc:::create
{
        this->pid = *((int *)arg0 + 171);
        ...
```

It's hard to know where that 171 constant came from or how a user would figure that out. I assume that this is because OEL doesn't yet have proper types and it's a hardcoded offset into some structure. Here's what that would look like on completed DTrace implementations:

```
proc:::create
{
        this->pid = args[0]->pr_pid;
        ...
```

### Progress

There's a long way to go, but it looks like the folks at Oracle are making progress. It will be interesting to see the source code that goes along with this updated beta -- as of this writing, [the git repository has not been updated](http://oss.oracle.com/git/?p=linux-2.6-dtrace-modules-beta.git;a=log). Personally, I'm eager to see what user-land tracing looks like in the form of the [pid provider](http://dtrace.org/blogs/ahl/2005/03/01/pid-provider-exposed/) and [USDT](http://dtrace.org/blogs/dap/2011/12/13/usdt-providers-redux/). In the tradition of other ports such as [Apple's](http://dtrace.org/blogs/ahl/2006/08/07/dtrace_on_mac_os_x/) and [FreeBSD's](http://www.bsdcan.org/2008/schedule/events/66.en.html), I'd invite the Oracle team to present their work at the upcoming DTrace conference, [dtrace.conf](http://wiki.smartos.org/display/DOC/dtrace.conf).
