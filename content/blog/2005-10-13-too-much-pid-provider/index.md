---
title: "Too much pid provider"
date: "2005-10-13"
categories: 
  - "dtrace"
---

Perhaps it's a bit Machiavellian, but I just love code that in some way tricks another piece of code. For example, in college I wrote some code that trolled through the address space of my favorite game to afford me certain advantages. Most recently, I've been working on some code that tricks other code into believing a complete fiction[\[1\]](#fearslut) about what operating system it's executing on. While working on that, I discovered an interesting problem with the [pid provider](http://docs.sun.com/app/docs/doc/817-6223/6mlkidlls?a=view) -- code that's all about deception and sleight of hand. Before you read further, be warned: I've already written two completely numbing accounts of the details of the pid provider [here](http://dtrace.org/blogs/ahl/pid_provider_exposed) and [here](http://dtrace.org/blogs/ahl/the_pid_provider_and_10), and this is going to follow much in that pattern. If you skip this one for fear of being bored to death[\[2\]](#bored), I won't be offended.

The problem arose because the traced process tried to execute an x86 instruction like this:

```
call    *0x10(%gs)

```

This instruction is supposed to perform a call to the address loaded from 0x10 bytes beyond the base of the segment described by the %gs selector. The neat thing about the pid provider (in case you've skipped those other posts) is that most instructions are executed natively, but some -- and `call` is one of them -- have to be emulated in the kernel. This instruction's somewhat unusual behavior needed to be emulated precisely; the pid provider, however, didn't know from selector prefixes and blithely tried to load from the absolute virtual address 0x10. Whoops.

To correct this, I needed to add some additional logic to parse the instruction and then augment the emulation code to know how to deal with these selectors. The first part was trivial, but the second half involved some digging into the x86 architecture manual. There are two kinds of descriptor tables, the LDT (local) and GDT (global). The value of %gs, in this case, tells us which table to look in, the index into that table, and the permissions associated with that selector.

Below is the code I added to [usr/src/uts/intel/dtrace/fasttrap\_isa.c](http://cvs.opensolaris.org/source/xref/usr/src/uts/intel/dtrace/fasttrap_isa.c) to handle this case. You can find the context [here](http://cvs.opensolaris.org/source/xref/usr/src/uts/intel/dtrace/fasttrap_isa.c#1092).

```
1145                         if (tp->ftt_code == 1) {
1146
1147                                 /*
1148                                  * If there's a segment prefix for this
1149                                  * instruction, first grab the appropriate
1150                                  * segment selector, then pull the base value
1151                                  * out of the appropriate descriptor table
1152                                  * and add it to the computed address.
1153                                  */
1154                                 if (tp->ftt_segment != FASTTRAP_SEG_NONE) {
1155                                         uint16_t sel, ndx;
1156                                         user_desc_t *desc;
1157
1158                                         switch (tp->ftt_segment) {
1159                                         case FASTTRAP_SEG_CS:
1160                                                 sel = rp->r_cs;
1161                                                 break;
1162                                         case FASTTRAP_SEG_DS:
1163                                                 sel = rp->r_ds;
1164                                                 break;
1165                                         case FASTTRAP_SEG_ES:
1166                                                 sel = rp->r_es;
1167                                                 break;
1168                                         case FASTTRAP_SEG_FS:
1169                                                 sel = rp->r_fs;
1170                                                 break;
1171                                         case FASTTRAP_SEG_GS:
1172                                                 sel = rp->r_gs;
1173                                                 break;
1174                                         case FASTTRAP_SEG_SS:
1175                                                 sel = rp->r_ss;
1176                                                 break;
1177                                         }
1178
1179                                         /*
1180                                          * Make sure the given segment register
1181                                          * specifies a user priority selector
1182                                          * rather than a kernel selector.
1183                                          */
1184                                         if (!SELISUPL(sel)) {
1185                                                 fasttrap_sigsegv(p, curthread,
1186                                                     addr);
1187                                                 new_pc = pc;
1188                                                 break;
1189                                         }
1190
1191                                         ndx = SELTOIDX(sel);
1192
1193                                         if (SELISLDT(sel)) {
1194                                                 if (ndx > p->p_ldtlimit) {
1195                                                         fasttrap_sigsegv(p,
1196                                                             curthread, addr);
1197                                                         new_pc = pc;
1198                                                         break;
1199                                                 }
1200
1201                                                 desc = p->p_ldt + ndx;
1202
1203                                         } else {
1204                                                 if (ndx >= NGDT) {
1205                                                         fasttrap_sigsegv(p,
1206                                                             curthread, addr);
1207                                                         new_pc = pc;
1208                                                         break;
1209                                                 }
1210
1211                                                 desc = cpu_get_gdt() + ndx;
1212                                         }
1213
1214                                         addr += USEGD_GETBASE(desc);
1215                                 }

```

The thing I learned by writing this is how to find the base address for those segment selectors which has been something I've been meaning to figure out. We (and most other operating systems) get to the thread pointer through a segment selector, so when debugging in mdb(1) I've often wondered how to perform the mapping from the value of %gs to the thread pointer that I care about. I haven't put that code back yet, so feel free to point out any problems you see. Anyway, if you made it here, congratulations and thanks.

* * *

[\[1\]](#fearslut_ref)Such is my love of the elaborate ruse that I once took months setting up a friend of mine for a very minor gag. [Lucas](http://imdb.com/name/nm1425096/) and I were playing scrabble and he was disappointed to hear that the putative word "fearslut" wasn't good. Later I conspired with a friend at his [company](http://pixar.com) to have a third party send mail to an etymology mailing list claiming that he had found the word "fearslut" in an old manuscript of an obscure Shakespear play. Three months later Lucas triumphantly announced to me that, lo and behold, "fearslut" was a word. I think I passed out I was laughing so hard.

[\[2\]](#bored_ref)My parents are fond of recounting my response when they asked what I was doing in my operating systems class during college: "If I told you, you wouldn't understand, and if I explained it, you'd be bored."
