---
title: "The case of the un-unmountable tmpfs"
date: "2011-12-12"
categories: 
  - "dtrace"
tags: 
  - "anonymous"
  - "boot"
  - "dtrace"
  - "pageout"
  - "tmpfs"
  - "zfs"
---

Every once in a rare while our development machines encounter an fatal error during boot because we couldn't unmount tmpfs. This weekend I cracked the case, so I thought I'd share my uses of [boot-time DTrace](http://wikis.sun.com/display/DTrace/Anonymous+Tracing), and the musty corners of the operating systems that I encountered along the way. First I should explain a little bit about what happens during boot and why we were unmounting a tmpfs filesystem.

## Upgrade and boot

When you upgrade your Delphix system from one version to the next, we perserve your configuration. Part of that system configuration lives in SMF, the Service Management Facility, which is stored in the filesystem at `/etc/svc`. We keep `/etc/svc` in its own ZFS dataset so that we can snapshot and clone it for an upgrade to save the old data (in case we need to roll back) and keep the settings. This gets a tad tricky because the kernel mounts tmpfs on `/etc/svc/volatile` to provide scratch space for early processes like init(1); before we can mount on `/etc/svc`, we have to unmount `/etc/svc/volatile`. Here's what that part of our boot script looks like:

```
#
# The kernel mounts tmpfs on /etc/svc/volatile so we we need to
# unmount that before mounting the svc dataset on /etc/svc.
#
umount /etc/svc/volatile
mount -F zfs $base/running/svc /etc/svc
mount -F tmpfs /etc/svc/volatile
```

## The problem

Infrequently -- but not never -- we'd see that unmount of`/etc/svc/volatile` fail with EBUSY. The boot script would stop and report the error; subsequent attempts to unmount /etc/svc/volatile would succeed. So there wasn't much to go on. The tmpfs code did reveal this:

```
static int
tmp_unmount(struct vfs *vfsp, int flag, struct cred *cr)
{
        ...
        tnp = tm->tm_rootnode;
        if (TNTOV(tnp)->v_count > 1) {
                mutex_exit(&tm->tm_contents);
                return (EBUSY);
        }
        for (tnp = tnp->tn_forw; tnp; tnp = tnp->tn_forw) {
                if ((vp = TNTOV(tnp))->v_count > 0) {
                        ...
                        return (EBUSY);
                }
                VN_HOLD(vp);
        }
        ...
```

So someone had an additional hold on either the root of the filesystem or a file in it. I looked at the contents of `/etc/svc/volatile` and found one file: init.state. Digging through the code for init(1) I was surprised to find that init(1) keeps a state file around with a list of processes of interest. It doesn't keep the file descriptor open (which would prevent us from unmounting the filesystem), but it does rewrite the file from time to time. I was worried that init(1) might be racing with our script. I didn't want to understand the brutal compexity of the code, so I amended our boot script to do the following:

```
pstop 1 # stop init
umount /etc/svc/volatile
mount -F zfs $base/running/svc /etc/svc
mount -F tmpfs /etc/svc/volatile
prun 1 # resume init
```

If the unmount failed, I'd be able to use pfiles(1) to see if init(1) did, in fact, have something open in /etc/svc/volatile. I was convinced that in trying to observe the problem, I'd chase it away -- a Heisenbug -- but after a short while of running a reboot loop, we hit the problem, and init(1) didn't have anything open in `/etc/svc/volatile`. What next…

## Boot-time DTrace

The problem was that by the time I'd get to the system, the conditions that caused the error had resolved themselves. What I wanted to do was panic the system when tmp\_unmount() returned EBUSY so that I could poke around with a debugger. On many systems that would entail compiling in debug logic, but fortunately a DTrace-enabled system has a better option. My former colleague [Bryan Cantrill](http://dtrace.org/blogs/bmc) invented anonymous DTrace for looking at boot-time issues -- earlier in boot than when one could execute the dtrace(1M) command-line utility. To use boot-time DTrace, specify the D program as usual, but add the -A option to add the D program to the DTrace kernel module's boot-time configuration. After rebooting, DTrace will enable the program whose output can later be retreived with `dtrace -a`. In my case, I wanted to drop into the kernel debugger when tmp\_unmount() returned EBUSY, so I ran DTrace like this:

```
dtrace -A -w -n 'fbt:tmpfs:tmp_unmount:return/arg1 == EBUSY/{ panic(); }'
```

Again after many reboots, we hit the problem and dropped into the debugger. Thanks to infrastructure put into the kernel by my colleague [Eric Schrock](http://blog.delphix.com/eschrock), I was able to quickly see the identity of the file whose reference count prevented us from unmounting tmpfs:

```
[5]> ffffff0197321b00::print vnode_t v_path
v_path = 0xffffff0197125d60 "/etc/svc/volatile/init-next.state"
```

It's worth noting that v\_path isn't guaranteed to be correct, but may reflect some state state. In this case, I examined the directory structure of tmpfs and found that the filename was actually /etc/svc/volatile/init.state -- [v\_path isn't updated on a rename](http://mail.opensolaris.org/pipermail/dtrace-discuss/2006-April/001338.html). But I couldn't for the life of me figure out who had that file open. None of the (few) other processes were touching the file. I looked through the fsflush code which flushes cached data back to disk, but that didn't make a lot of sense, and didn't seem to be causing problems. The pageout thread isn't supposed to run unless the system is low on memory. I used kmdb's `::kgrep` command to find places where the vnode\_t or its associated page were referenced. There were many, and I quickly got lost in the bowels of the VM system. Rather than groveling through the kernel's structures, I decided to turn back to DTrace. The next question I wanted to answer was this: after tmp\_unmount() returns EBUSY, who is it that releases the reference on that tmpfs vnode\_t? To answer it, I wrote this D script:

```
fbt:tmpfs:tmp_unmount:entry
{
        self->vfs = args[0];
}

fbt:tmpfs:tmp_unmount:return
/arg1 == EBUSY/
{
        gotit = self->vfs;
}

fbt:tmpfs:tmp_unmount:return
{
        self->vfs = NULL;
}

fbt:genunix:vn_rele:entry
/gotit != NULL && args[0]->v_vfsp = gotit/
{
        panic();
}
```

I installed that as my anonymous DTrace enabling, rebooted, and waited.

## Who dunnit

<iframe width="240" height="180" src="http://www.youtube.com/embed/JuQYXhnzS1g?rel=0" frameborder="0" allowfullscreen class="alignright"></iframe>

Like the [Mystery Inc. gang](http://en.wikipedia.org/wiki/Scooby-Doo) unmasking the criminal, helplessly caught by the elaborate trap, I used the kernel debugger to identify the subsystem to find that it was none other than harmless old Mr. Pageout. Gasp! Why was pageout running at all? The system had plenty of memory so it wouldn't normally be running except there's an exception made very very early in boot (it turns out). In the first second after boot, pageout will execute exactly four times in order to fill in certain performance-related parameters that let it predict how long it will take to page out memory in the future. When it executes, pageout will identify unused pages and take a temporary hold on them -- this is exactly the pathology at the root of our problem!

## Solution

I'm still working on exactly how to solve this. The simplest approach would be to sleep for a second before trying the unmount. Slightly more complicated would be to try unmounting in a loop until a second had passed (checking $SECONDS in bash). More complicated still would be to do a rethink of [pageout](http://src.illumos.org/source/xref/illumos-gate/usr/src/uts/common/os/vm_pageout.c) -- I still don't fully understand how it works, but it really seems like it's making assumptions that have been invalidated in the past decade and contains this [gem of a comment](http://src.illumos.org/source/xref/illumos-gate/usr/src/uts/common/os/vm_pageout.c#649):

```
For now, this thing is in very rough form.
```

Note that "now" in this case referred to 1987 or possibly earlier -- as Roger Faulkner would say, "it came from New Jersey."

## Conclusion

Pageout would have gotten away with it if it hadn't been for these meddling tools! DTrace during boot is awesome -- when you need it, it's a life saver. There are some places so early in boot that DTrace can't help; for that [VProbes](http://blog.delphix.com/mba/2011/one-good-probe-deserves-another/) can give you some DTrace-like functionality. And mature systems can have some musty corners so your tools had better be up to the task.
