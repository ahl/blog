---
title: "My New DTrace Favorite"
date: "2012-07-27"
categories:
  - "dtrace"
permalink: /2012/07/28/my-new-dtrace-favorite/
---

The mantra as we initially developed DTrace was to make impossible things possible, not easy things easier. Since codifying that, the tendency toward the latter in developer tools has been apparent. Our focus on the former however has left certain usability burrs that stymie newbies, and annoy vets. Much of the DTrace development of late has focused on a middle category: simplifying hard things that should be simple.

### The `print()` action

In that vein, my colleague, Eric Schrock, [added the `print()` action to DTrace](http://dtrace.org/blogs/eschrock/2011/10/26/your-mdb-fell-into-my-dtrace/) back in November. Before then, my workflow used to look like this:

- "I want to trace `xdr_bytes()`"
- Drop into mdb(1) to print out the structure I want to examine
- Write a D script to emit the members I'm interested in:

```
fbt::xdr_bytes:entry
{
        trace(args[0]->x_base);
        trace(args[0]->x_handy);
}
```

Repeat times a thousand, allow for errors, iterate on chased pointers, and sum up the time. With Eric's fix, DTrace is a lot easier to use:

- "I want to trace `xdr_bytes()`"
- Boom:

```
fbt::xdr_bytes:entry
{
        print(args[0]);
}
```

### `print()` for translated types

Of course, in addition to tracing any kernel function, DTrace has stable probes that identify points of well-known, (reasonably) well-documented activity. Those probes don't correspond to kernel functions so mdb isn't as useful. The workflow is a little more annoying:

- "I want to trace IO"
- either

- google "dtrace io provider", find the argument and the type definitions

- or (more often)

- `dtrace -lvn io:::` to see the type names
- `less /usr/lib/dtrace/io.d` to find the type definitions

- write the D script:

```
io:::start
{
        trace(args[1]->dev_name);
        trace(args[1]->dev_pathname);
}
```

Repeat another thousand, much more annoying times.

Unfortunately, `print()` wasn't as helpful in this case:

```
# dtrace -n 'io:::start{ print(*args[1]); }'
dtrace: invalid probe specifier io:::start{ trace(*args[1]); }: print( ) may not be applied to a dynamic expression
```

Stable probes such as the io:::start probe can use translated arguments, synthetic types that DTrace populates with stable data from the unstable underlying implementation. For example, despite very different implementations, the io:::start provider exposes the same data on illumos, FreeBSD, Mac OS X, and Oracle Solaris. Parameters are effectively translated one at a time; the \* (dereference) operator was invalid for these expressions.

In a [recent push to illumos](https://github.com/illumos/illumos-gate/commit/e5803b76927480e8f9b67b22201c484ccf4c2bcf), I added this support:

```
# dtrace -n 'io:::start{ print(*args[1]); }'
dtrace: description 'io:::start' matched 6 probes
CPU ID FUNCTION:NAME
0 11307 bdev_strategy:start devinfo_t {
    int dev_major = 0x62
    int dev_minor = 0x40
    int dev_instance = 0x1
    string dev_name = [ "sd" ]
    string dev_statname = [ "sd1" ]
    string dev_pathname = [ "/devices/pci@0,0/pci15ad,1976@10/sd@0,0:a" ]
}
```

Between Eric's addition and my own, my most commonly encountered DTrace annoyances are no more.

### Behind the scenes

For the DTrace super-nerds out there, I thought I'd share a bit of the implementation. In order to `trace()` or `print()` an expression, it needs to exist in memory somewhere. Translated types don't exist in memory, rather individual members are translated statically. We can see this in the output of the DTrace DIF (D intermediate form) disassembler:

```
# dtrace -n 'io:::start{ trace(args[1]->dev_name); }' -Se
DIFO 0x75e940 returns string (unknown) by ref (size 256)
OFF OPCODE INSTRUCTION
00: 25000001 setx DT_INTEGER[0], %r1 ! 0x0
01: 28000101 ldga DT_VAR(0), %r1, %r1
02: 0e010002 mov %r1, %r2
03: 25000103 setx DT_INTEGER[1], %r3 ! 0xe0
04: 07020302 add %r2, %r3, %r2
05: 22020002 ldx [%r2], %r2
06: 25000003 setx DT_INTEGER[0], %r3 ! 0x0
07: 0f020300 cmp %r2, %r3
08: 1200000b be 11
09: 0e000002 mov %r0, %r2
10: 1100000c ba 12
11: 25000202 setx DT_INTEGER[2], %r2 ! 0x1
12: 10020000 tst %r2
13: 12000011 be 17
14: 26000102 sets DT_STRING[1], %r2 ! "nfs"
15: 0e020002 mov %r2, %r2
16: 1100001e ba 30
17: 25000302 setx DT_INTEGER[3], %r2 ! 0xfffffffffc031110
18: 22020002 ldx [%r2], %r2
19: 0e010003 mov %r1, %r3
20: 25000404 setx DT_INTEGER[4], %r4 ! 0xa8
21: 07030403 add %r3, %r4, %r3
22: 22030003 ldx [%r3], %r3
23: 33000000 flushts
24: 31000003 pushtv DT_TYPE(0), %r3 ! DT_TYPE(0) = D type
25: 2f001403 call DIF_SUBR(20), %r3 ! getmajor
26: 25000504 setx DT_INTEGER[5], %r4 ! 0x70
27: 08030403 mul %r3, %r4, %r3
28: 07020302 add %r2, %r3, %r2
29: 22020002 ldx [%r2], %r2
30: 23000002 ret %r2
```

In this case, this logic comes from /usr/lib/io.d, and -- in particular -- this translation:

```
        dev_name = B->b_dip == NULL ? "nfs" :
            stringof(`devnamesp[getmajor(B->b_edev)].dn_name);
```

To implement allow `trace()` and `print()` to work on translated types, we now generate code to first use the DTrace build-in alloca() function to get some scratch space, and then generate the translation for each member of the translated type. For example:

```
# dtrace -n 'io:::start{ print(*args[1]); }' -Se
DIFO 0x9466b0 returns D type (struct) by ref (size 780)
OFF OPCODE INSTRUCTION
00: 25000001 setx DT_INTEGER[0], %r1 ! 0x0
01: 28000101 ldga DT_VAR(0), %r1, %r1
02: 25000102 setx DT_INTEGER[1], %r2 ! 0x30c
03: 33000000 flushts
04: 31000002 pushtv DT_TYPE(0), %r2 ! DT_TYPE(0) = D type
05: 2f000f02 call DIF_SUBR(15), %r2 ! alloca
06: 0e010003 mov %r1, %r3
07: 25000204 setx DT_INTEGER[2], %r4 ! 0xe0
08: 07030403 add %r3, %r4, %r3
09: 22030003 ldx [%r3], %r3
10: 25000004 setx DT_INTEGER[0], %r4 ! 0x0
11: 0f030400 cmp %r3, %r4
12: 1300000f bne 15
13: 0e000003 mov %r0, %r3
14: 11000010 ba 16
15: 25000303 setx DT_INTEGER[3], %r3 ! 0x1
...
316: 2f001603 call DIF_SUBR(22), %r3 ! ddi_pathname
317: 25001204 setx DT_INTEGER[18], %r4 ! 0x20c
318: 07020404 add %r2, %r4, %r4
319: 25000e05 setx DT_INTEGER[14], %r5 ! 0x100
320: 3b030504 copys %r3, %r5, %r4
321: 23000002 ret %r2
```

### More to come

Usability was a big topic at [dtrace.conf](http://dtrace.org/blogs/ahl/2012/04/09/dtrace-conf12-wrap-up/) a few months ago. Expect to see more contributions along this theme.
