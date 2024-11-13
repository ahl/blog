---
title: "the pid provider and &gt; 10 arguments"
date: "2005-04-04"
categories: 
  - "dtrace"
---

A long-time DTrace user was recently examining an ugly C++ application, and this obvious DTrace invocation to trace the 15th argument (zero-indexed) to a particularly ugly function:

> ```
> # dtrace -n pid123::foobar:entry'{ trace(arg15); }'
> dtrace: invalid probe specifier pid380863:::entry{ trace(arg15); }: in action list:
> failed to resolve arg15: Unknown variable name
> 
> ```

As described in the [Solaris Dynamic Tracing Guide](http://docs.sun.com/app/docs/doc/817-6223/6mlkidlfv?a=view) we actually only provide access to arguments 0-9. I suppose you could call this a design oversight, but really it reflects our bias about software -- no one's going to want to call your function if it has a bazillion arguments.

But -- as with most things pertaining to C++ -- sometimes you just have to hold your nose and get it working. If you need to trace arguments past `arg9` in functions you're observing with the pid provider, here's how you can do it:

<table><tbody><tr><td>x86</td><td><tt>this-&gt;argN = *(uint32_t *)copyin(uregs[R_SP] + sizeof (uint32_t) * (this-&gt;N + 1), sizeof (uint32_t));</tt></td></tr><tr><td>x86-64/AMD64</td><td><tt>this-&gt;argN = *(uint64_t *)copyin(uregs[R_SP] + sizeof (uint64_t) * (this-&gt;N - 5), sizeof (uint64_t));</tt></td></tr><tr><td>SPARC 32-bit</td><td><tt>this-&gt;argN = *(uint32_t *)copyin(uregs[R_SP] + sizeof (uint32_t) * (this-&gt;N + 17), sizeof (uint32_t));</tt></td></tr><tr><td>SPARC 64-bit</td><td><tt>this-&gt;argN = *(uint64_t *)copyin(uregs[R_SP] + 0x7ff + sizeof (uint64_t) * (this-&gt;N + 16), sizeof (uint64_t));</tt></td></tr></tbody></table>

Note that for SPARC (32-bit and 64-bit) as well as AMD64, these formulas only work for arguments past the sixth -- but then you should probably be using `arg0 .. arg9` when you can.

### UPDATE

The methods above only apply for integer arguments; while I think it will work for 32-bit x86, the other architectures can pass floating-point arguments in registers as well as on the stack. Perhaps a future entry will discuss floating-point arguments if anyone cares.

There are a couple of gotchas I neglected to mention. _On AMD64_ if the argument is less that 64-bits (e.g. an `int`), the compiler can leave garbage in the upper bits meaning that you have to cast the variable to the appropriate type in DTrace (e.g. trace((int)this->argN)). On both 32-bit architectures, 64-bit arguments are passed in 2 of these 32-bit arguments; to get the full 64-bit value, just _shift_ and _or_ the two arguments together (e.g. `((this->arg13 <arg14)` or `((this->arg14 <arg13)` for SPARC and x86 respectively). Even for arguments that you can get with the built-in variables, you will need to mash together 64-bit arguments on 32-bit architectures (except on the SPARCv8+ ABI which can pass the first 6 arguments in 64-bit registers).

* * *

Technorati tag: [DTrace](http://technorati.com/tag/DTrace)
