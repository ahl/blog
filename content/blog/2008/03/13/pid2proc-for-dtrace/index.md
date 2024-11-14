---
title: "pid2proc for DTrace"
date: "2008-03-12"
categories:
  - "dtrace"
permalink: /2008/03/13/pid2proc-for-dtrace/
---

The other day, there was an [interesting post](http://opensolaris.org/jive/thread.jspa?threadID=54199&tstart=0) on the DTrace mailing list asking how to **derive a process name from a pid**. This really ought to be a built-in feature of D, but it isn't (at least not yet). I hacked up a solution to the user's problem by cribbing the algorithm from mdb's `::pid2proc` function whose source code you can find [here](http://cvs.opensolaris.org/source/xref/onnv/onnv-gate/usr/src/cmd/mdb/common/modules/mdb_ks/mdb_ks.c#mdb_pid2proc). The basic idea is that you need to look up the pid in `pidhash` to get a chain of `struct pid` that you need to walk until you find the pid in question. This in turn gives you an index into `procdir` which is an array of pointers to proc structures. To find out more about these structures, poke around the source code or `mdb -k` which is what I did.

The code isn't exactly gorgeous, but it gets the job done. It's a good example of **probe-local** variables (also somewhat misleadingly called **clause-local** variables), and demonstrates how you can use them to communicate values between clauses associated with a given probe during a given firing. You can try it out by running `dtrace -c <your-command> -s <this-script>`.

```
BEGIN
{
this->pidp = `pidhash[$target & (`pid_hashsz - 1)];
this->pidname = "-error-";
}
/* Repeat this clause to accommodate longer hash chains. */
BEGIN
/this->pidp->pid_id != $target && this->pidp->pid_link != 0/
{
this->pidp = this->pidp->pid_link;
}
BEGIN
/this->pidp->pid_id != $target && this->pidp->pid_link == 0/
{
this->pidname = "-no such process-";
}
BEGIN
/this->pidp->pid_id != $target && this->pidp->pid_link != 0/
{
this->pidname = "-hash chain too long-";
}
BEGIN
/this->pidp->pid_id == $target/
{
/* Workaround for bug 6465277 */
this->slot = (*(uint32_t *)this->pidp) >> 8;
/* AHA! We finally have the proc_t. */
this->procp = `procdir[this->slot].pe_proc;
/* For this example, we'll grab the process name to print. */
this->pidname = this->procp->p_user.u_comm;
}
BEGIN
{
printf("%d %s", $target, this->pidname);
}

```

Note that the second clause is the bit that walks the hash chain. You can repeat this clause as many times as you think will be needed to traverse the hash chain -- I really don't have any guidance here, but I imagine that a few times should suffice. Alternatively, you could construct a tick probe that steps along the hash chain to avoid a fixed limit. DTrace attempts to keep **easy** things **easy** and make **difficult** things **possible**. As evidenced by this example, **possible** doesn't necessarily correlate with **beautiful**.
