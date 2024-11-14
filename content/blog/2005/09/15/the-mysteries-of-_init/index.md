---
title: "The mysteries of _init"
date: "2005-09-15"
categories: 
  - "opensolaris"
---

I hadn't been fully aware that I felt this way, but I recently had a realization: I love the linker. It's a technology that's amazing in both its simplicity and its complexity. I'm sure my feelings are influenced in no small way by the caliber of the engineers working on it -- [Rod](/rie) and [Mike](/msw) are always eager to explain how the some facet of the linker works or to add something new and whizzy if it can't quite do what I need.

Over the course of developing [user-level statically defined tracing](http://docs.sun.com/app/docs/doc/817-6223/6mlkidlms?a=view) USDT, I've worked (and continue to work) with the linker guys to figure out the best way to slot the two technologies together. Recently, some users of USDT have run into a problem where binaries compiled with USDT probes weren't actually making them available to the system. We eventually tracked it down to incorrect use of the linker. I thought it would be helpful to describe the problem and the solution in case other people bump into something similar.

First a little bit on initialization. In a C compiler, you can specify an initialization function like this: `#pragma init(my\_init)`. The intention of this is to have the specified function (e.g. `my\_init`) called when the binary is loaded into the program. This is a good place to do initialization like memory allocation or other set up used in the rest of the binary. What the compiler actually does when you specify this is create a ".init" section which contains a call to the specified function.

As a concrete example (and the example relevant to this specific manifestation of the problem), take a look at this code in [usr/src/lib/libdtrace/common/drti.c](http://cvs.opensolaris.org/source/xref/usr/src/lib/libdtrace/common/drti.c#88):

```
88 #pragma init(dtrace_dof_init)
89 static void
90 dtrace_dof_init(void)
91 {

```

When we compile this into an object file (which we then deliver in /usr/lib/dtrace/drti.o), the compiler generates a .init ELF section that contains a call to `dtrace\_dof\_init()` (actually it contains a call with a relocation that gets filled into to be the address of `dtrace\_dof\_init()`, but that's a detail for another blog entry).

The linker doesn't really do anything special with .init ELF sections -- it just concatenates them like it does all other sections with the same name. So when you compile a bunch of object files with .init sections, they just get crammed together -- there's still nothing special that causes them to get executed with the binary is loaded.

Here's the clever part, when a compiler invokes the linker, it provides two special object files: crti.o at the beginning, and crtn.o at the end. You can find those binaries on your system in /usr/lib/ or in /usr/sfw/lib/gcc/... for the gcc version. Those binaries are where the clever part happens; crti.o's .init section contains effectively an open brace and crtn.o contains the close brace (the function prologue and epilogue respectively):

```
$ dis -t .init /usr/lib/crti.o
section .init
_init()
_init:                  55                 pushl  %ebp
1:                      8b ec              movl   %esp,%ebp
3:                      53                 pushl  %ebx
4:                      e8 00 00 00 00     call   +0x5
9:                      5b                 popl   %ebx
a:                      81 c3 03 00 00 00  addl   $0x3,%ebx
$ dis -t .init /usr/lib/crtn.o
section .init
0:                      5b                 popl   %ebx
1:                      c9                 leave
2:                      c3                 ret

```

By now you may see the punch-line: by bracketing the user-generated object files with these crti.o and crtn.o the resulting .init section is the concatenation of the function prologue, all the calls in the user's object files, and finally the function epilogue. All of this is contained in the symbol called \_init.

The linker then has some magic that identifies the \_init function as special and includes a dynamic entry (DT\_INIT) that causes \_init to be called by the the run-time linker (ld.so.1) when the binary is loaded. In the binary that was built with USDT but wasn't working properly, there was a .init section with the call to dtrace\_dof\_init(), but no \_init symbol. The problem was, of course, that crti.o and crtn.o weren't being specified in the linker invocation resulting in a .init section, but no \_init symbol so no DT\_INIT section, so no initialization and no USDT.
