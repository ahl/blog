---
title: "DTrace is open"
date: "2005-01-25"
categories: 
  - "dtrace"
---

It's a pretty exciting day for the DTrace team as our code is the first part of Solaris to be released under the CDDL. I thought I'd take the opportunity to end my blogging hiatus and to draw attention to some of my favorite corners of the code. [Bryan](http://blogs.sun.com/bmc) has written an exhaustive overview of the code structure as well as some of his favorite parts of the code.

#### fasttrap

The biggest component of DTrace that I was wholly responsible for was the user-level tracing component. The pid provider (implemented as the 'fasttrap' kernel module for largely historical reasons) lets DTrace consumers trace function entry and return (as the fbt probider does for the kernel) as well as any individual instruction. It does this all losslessly, without stopping other threads and -- in fact -- without inducing any lock contention or serialization. (Check out the comments at the top of fasttrap.c, and the two fasttrap\_isa.c's for extensive discussion.)

Here's the general technique employed by the pid provider: each traced instruciton is first replaced with a trapping instruction. On sparc we use a ta (trap always) and on x86 (by which I mean i386 _and_ amd64) we use an int3 (0xcc) (see the fasttrap\_tracepoint\_install() function in usr/src/uts/sparc/dtrace/fasttrap\_isa.c and usr/src/uts/intel/dtrace/fasttrap\_isa.c). Now any time a user-level thread executes this instruction it will bounce into the fasttrap module (on x86 this requires a little trickery because the int3 instruction is also used by debuggers to set breakpoints) and into the fasttrap\_pid\_probe() function (in both instances of fasttrap\_isa.c). In fasttrap\_pid\_probe(), we lookup the original instruction in fasttrap\_tpoints -- a global hash table of tracepoints -- and call dtrace\_probe() to invoke the DTrace framework. Here's what it looks like on i386 (fasttrap\_isa.c):

```
821                         uintptr_t s0, s1, s2, s3, s4, s5;
822                         uint32_t *stack = (uint32_t *)rp->r_sp;
823
824                         /*
825                          * In 32-bit mode, all arguments are passed on the
826                          * stack. If this is a function entry probe, we need
827                          * to skip the first entry on the stack as it
828                          * represents the return address rather than a
829                          * parameter to the function.
830                          */
831                         s0 = fasttrap_fuword32_noerr(&stack[0]);
832                         s1 = fasttrap_fuword32_noerr(&stack[1]);
833                         s2 = fasttrap_fuword32_noerr(&stack[2]);
834                         s3 = fasttrap_fuword32_noerr(&stack[3]);
835                         s4 = fasttrap_fuword32_noerr(&stack[4]);
836                         s5 = fasttrap_fuword32_noerr(&stack[5]);
837
838                         for (id = tp->ftt_ids; id != NULL; id = id->fti_next) {
839                                 fasttrap_probe_t *probe = id->fti_probe;
840
841                                 if (probe->ftp_type == DTFTP_ENTRY) {
842                                         /*
843                                          * We note that this was an entry
844                                          * probe to help ustack() find the
845                                          * first caller.
846                                          */
847                                         cookie = dtrace_interrupt_disable();
848                                         DTRACE_CPUFLAG_SET(CPU_DTRACE_ENTRY);
849                                         dtrace_probe(probe->ftp_id, s1, s2,
850                                             s3, s4, s5);
851                                         DTRACE_CPUFLAG_CLEAR(CPU_DTRACE_ENTRY);
852                                         dtrace_interrupt_enable(cookie);
853                                 } else if (probe->ftp_argmap == NULL) {
854                                         dtrace_probe(probe->ftp_id, s0, s1,
855                                             s2, s3, s4);
856                                 } else {
857                                         uint32_t t[5];
858
859                                         fasttrap_usdt_args32(probe, rp,
860                                             sizeof (t) / sizeof (t[0]), t);
861
862                                         dtrace_probe(probe->ftp_id, t[0], t[1],
863                                             t[2], t[3], t[4]);
864                                 }
865                         }

```

Now that we've properly invoked the DTrace framework, we have to make sure the program does the right thing -- rather than executing the instruction it needed, we forced it to execute a trap instruction; obviously we can't just return control to the user-level thread without doing something. Of course, all we have to do at this point is emulate the original instruction -- but if you feel like writing a complete x86 emulator and running it in the kernel, be my guest. Instead, I did something much lazier. There are some instructions we do emulate -- the ones that are _position-dependent_ like relative jumps, calls, etc. (see fasttrap\_tracepoint\_init() on both sparc and intel) -- but there are really only a few of these. For the rest we use a technique called **displaced execution**.

As the name suggests, rather than executing the original instruction at its original address, we relocate it into some reserved scratch space in the user-level thread structure (obviously a per-thread entity). We then arrange to continue execution with what would have normally been the subsequent instruction. On x86 we just have a jmp to the next instruction, on sparc we make clever use of the delay slot and the %npc. I've love to stick the code here, but there's really a lot of it; I suggest you open your favorite fasttrap\_isa.c file and search for 'case FASTTRAP\_T\_COMMON' which is where I handle generic instructions using displaced execution (the other cases deal with the instructions that need to be emulated).

Just a quick time out to compare this to other tracing techniques. Every other technique that I'm aware of either has the potential for losing data or can serialize the process or induce lock contention. A technique that loses data is to just replace the trap with the original instruction, single-step and reinstall the trap; if another thread comes along in the meantime, it didn't see the trap. Truss is a good example of tracer that induces serialization; to avoid the lossiness problem, it stops all other threads in the process when it single-steps. By serializing the processes execution, you can't gather meaningful data about data races or lock contention (see usr/src/cmd/plockstat/), but, of course, with a lossy tracer, you can't really gather any _meaningful_ data at all.

amd64 trickiness

While you're looking at the displaced execution code, I'd appreciate it if you'd spend some time looking at the code to deal with %rip-relative (program counter-relative) instructions on amd64. The basic premise of displaced execution is that the number of instructions that we need to execute is relatively small, but I got a big surprise when it turned out that pretty much any instruction on amd64 could potentially depend on its address for correct execution.

With a little help from the in-kernel disassembler, we detect if the instruction is %rip-relative:

```
469         if (p->p_model == DATAMODEL_LP64 && tp->ftt_type == FASTTRAP_T_COMMON) {
470                 /*
471                  * If the process is 64-bit and the instruction type is still
472                  * FASTTRAP_T_COMMON -- meaning we're going to copy it out an
473                  * execute it -- we need to watch for %rip-relative
474                  * addressing mode. See the portion of fasttrap_pid_probe()
475                  * below where we handle tracepoints with type
476                  * FASTTRAP_T_COMMON for how we emulate instructions that
477                  * employ %rip-relative addressing.
478                  */
479                 if (rmindex != -1) {
480                         uint_t mod = FASTTRAP_MODRM_MOD(instr[rmindex]);
481                         uint_t reg = FASTTRAP_MODRM_REG(instr[rmindex]);
482                         uint_t rm = FASTTRAP_MODRM_RM(instr[rmindex]);
483
484                         ASSERT(rmindex > start);
485
486                         if (mod == 0 && rm == 5) {
487                                 /*
488                                  * We need to be sure to avoid other
489                                  * registers used by this instruction. While
490                                  * the reg field may determine the op code
491                                  * rather than denoting a register, assuming
492                                  * that it denotes a register is always safe.
493                                  * We leave the REX field intact and use
494                                  * whatever value's there for simplicity.
495                                  */
496                                 if (reg != 0) {
497                                         tp->ftt_ripmode = FASTTRAP_RIP_1 |
498                                             (FASTTRAP_RIP_X *
499                                             FASTTRAP_REX_B(rex));
500                                         rm = 0;
501                                 } else {
502                                         tp->ftt_ripmode = FASTTRAP_RIP_2 |
503                                             (FASTTRAP_RIP_X *
504                                             FASTTRAP_REX_B(rex));
505                                         rm = 1;
506                                 }
507
508                                 tp->ftt_modrm = tp->ftt_instr[rmindex];
509                                 tp->ftt_instr[rmindex] =
510                                     FASTTRAP_MODRM(2, reg, rm);
511                         }
512                 }
513         }

```

Note that we've changed the instruction at line 509 to depend on %rax (or %r8) rather than %rip. When we hit that tracepoint, we move what would have normally been the %rip value into %rax (or %r8), and make sure to reset the value of %rax (or %r8) when we're done. Actually, as you might have noticed from the code above, it's a little more complicated because we want to avoid using %rax if the instruction already uses that register, but I'm sure you can figure it out from the code :-).

more

I was going to write more about the pid provider, user-level statically defined tracing (USDT), the plockstat provider and command, and some (I think) clever parts about user/kernel interactions, but this is already much longer than what I assume is the average attention span. More later.
