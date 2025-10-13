---
title: "User-land tracing gets better and better"
date: "2006-05-08"
categories:
  - "dtrace"
permalink: /2006/05/08/user-land-tracing-gets-better-and-better/
---

As I've mentioned in the past, developers can add their own DTrace probes using the [user-land statically defined tracing](http://docs.sun.com/app/docs/doc/817-6223/6mlkidlms?a=view) (USDT) mechanism. It's been used to instrument [Postgres](http://www.sun.com/software/solaris/postgres.jsp) and [Apache](http://daemons.net/~matty/code/apache.html), and to add observability into dynamic languages such as [Java](http://blogs.sun.com/roller/page/kamg?entry=built_in_dtrace_probes_in), [Ruby](http://blogs.sun.com/roller/page/bmc?entry=dtrace_and_ruby), and [php](http://blogs.sun.com/roller/page/bmc?entry=dtrace_and_php). I recently made a couple of improvements to USDT that I mentioned [here](http://www.opensolaris.org/jive/thread.jspa?messageID=23815&#23815) and [here](http://www.opensolaris.org/jive/thread.jspa?messageID=31314), but I think deserve a little more discussion.

Adding USDT probes (as described in the [DTrace manual](http://docs.sun.com/app/docs/doc/817-6223/6mlkidlms?a=view)) requires creating a file defining the probes, modifying the source code to identify those probe sites, and modifying the build process to invoke `dtrace(1M)` with the `\-G` option which causes it to emit an object file which is then linked into the final binary. Bart wrote up a [nice example](http://blogs.sun.com/roller/page/barts?entry=putting_user_defined_dtrace_probe) of how to do this. The mechanisms are mostly the same, but have been tweaked a bit.

#### USDT in C++

One of the biggest impediments to using USDT was its (entirely understandable) enmity toward C++. Briefly, the problem was that the modifications to the source code used a structure that was incompatible with C++ (it turns out you can only `extern "C"` symbols at the file scope -- go figure). To address this, I added a new `\-h` option that creates a header file based on the probe definitions. Here's what the new way looks like:

provider.d

```
provider database {
probe query__start(char *);
probe query__done(char *);
};

```

src.c or src.cxx

```
...
#include "provider.h"
...
static int
db_query(char *query, char *result, size_t size)
{
...
DATABASE_QUERY_START(query);
...
DATABASE_QUERY_DONE(result);
...
}

```

Here's how you compile it:

```
$ dtrace -h -s provider.d
$ gcc -c src.c
$ dtrace -G -s provider.d src.o
$ gcc -o db provider.o src.o ...

```

If you've looked at the old USDT usage, the big differences are the creation and use of `provider.h`, and that we use the `_PROVIDER_PROBE_()` macro rather than the generic `DTRACE_PROBE1()` macro. In addition to working with C++, this has the added benefit that it engages the compiler's type checking since the macros in the generated header file require the types specified in the provider definition.

#### Is-Enabled Probes

One of the tenets of DTrace is that the mere presence of probes can never slow down the system. We achieve this for USDT probes by only adding the overhead of a few no-op instructions. And while it's _mostly_ true that USDT probes induce no overhead, there are some cases where the overhead can actually be substantial. The actual probe site is as cheap as a no-op, but setting up the arguments to the probe can be expensive. This is especially true for dynamic languages where probe arguments such as the class or method name are often expensive to compute. As a result, some providers -- the one for Ruby, for example -- couldn't be used in production due to the _disabled_ probe effect.

To address this problem, [Bryan](http://blogs.sun.com/bmc) and I came up with the idea of what -- for lack of a better term -- I call **is-enabled** probes. Every probe specified in the provider definition has an associated is-enabled macro (in addition to the actual probe macro). That macro is used to check if the DTrace probe is currently enabled so the program can then only do the work of computing the requisite arguments if they're needed.

For comparison, Rich Lowe's prototype Ruby provider basically looked like this:

```
rb_call(...
{
...
RUBY_ENTRY(rb_class2name(klass), rb_id2name(method));
...
RUBY_RETURN(rb_class2name(klass), rb_id2name(method));
...
}

```

Where `rb_class2name()` and `rb_id2name` perform quite expensive operations.

With is-enabled probes, Bryan was able to [greatly reduce the overhead](http://blogs.sun.com/roller/page/bmc?entry=dtrace_on_rails) of the Ruby provider to essentially zero:

```
rb_call(...
{
...
if (RUBY_ENTRY_ENABLED())
RUBY_ENTRY(rb_class2name(klass), rb_id2name(method));
...
if (RUBY_RETURN_ENABLED())
RUBY_RETURN(rb_class2name(klass), rb_id2name(method));
...
}

```

When the source objects are post-processed by `dtrace -G`, each is-enabled site is turned into a simple move of 0 into the return value register (`%eax`, `%rax`, or `%o0` depending on your ISA and bitness). When probes are disabled, we get to skip all the expensive argument setup; when a probe is enabled, the is-enabled site changes so that the return value register will have a 1. (It's also worth noting that you can pull some compiler tricks to make sure that the program text for the uncommon case -- probes enabled -- is placed out of line.)

The obvious question is then "When should is-enabled probes be used?" As with so many performance questions the only answer is to measure both. If you can eke by without is-enabled probes, do that: is-enabled probes are incompatible with versions of Solaris earlier than Nevada build 38 and they incur a greater _enabled_ probe effect. But if acceptable performance can only be attained by using is-enabled probes, that's exactly where they were designed to be used.

* * *

Technorati Tags: [DTrace](http://technorati.com/tag/DTrace) [USDT](http://technorati.com/tag/USDT)
