---
title: "Number 13 of 20: Core file improvements"
date: "2004-07-15"
categories: 
  - "opensolaris"
---

[go to the Solaris 10 top 11-20 list for more](http://dtrace.org/blogs/ahl/the_solaris_10_top_11)

### core files

Core files are snapshots of a process's state. They contain some of the memory segments (e.g. the stack and heap) as well as some of the in-kernel state associated with the process (e.g. the signal masks and register values). When a process gets certain signals, the kernel, by default, kills the process and produces a core file. You can also creat core files from running processes -- without altering the process -- using Solaris's [gcore(1)](http://docs.sun.com/db/doc/817-0689/6mgfkpcp7?a=view) utility.

So when your application crashed in the field, you could just take the core file and debug it right? Well, not exactly. Core files contained a _partial_ snap-shot of the process's memory mappings -- in particular they omitted the read-only segments which contained the program text (instructions). As a result you would have to recreate the environment from the machine where the core file was produce _exactly_ -- identical versions of the libraries, application binary and loadable modules. Consequently, core files were mostly useful for developers in development (and even then, an old core file could be useless after a recompilation). And this isn't just Solaris -- every OS I've every worked with has omitted program text from core files making those core files of marginal utility once they've left the machine that produced them.

### coreadm(1M)

In Solaris 7 we introduced [coreadm(1M)](http://docs.sun.com/db/doc/817-0690/6mgflnt7l?a=view) to let users and system administrators control the location and name of core files. Previously , core files had always been named "core" and resided in the current working directory of the process that dumped the core. With coreadm(1M) you can name core files whatever you want including meta characters that expand when the core is created; for example, "core.%f.%n" would expand to "core.staroffice.dels" if staroffice were to dump core on my desktop (named [dels](http://dels.com/)). System administrators can also set up a global repository for all cores produced on the system to keep an eye on programs unexpectedly dumping core (naturally in Solaris 10, [zone](http://www.sun.com/bigadmin/content/zones/) administrators can set up per-zone core file repositories).

In Solaris 10, coreadm(1M) becomes an even more powerful tool. Now you can specify which parts of the processes image go into the core file. Program text is there by default, and you can also choose to omit or include the stack, heap, anonymous data, mapped files, system V shared memory segments, ISM, DISM, etc. Let's say you've got some multi-processed database that contains a big DISM segment; rather than having each process include the shared segment in its core file, you can set up just one of the processes (or none of them) to include the segment in the core file.

### debugging core files from the field

Now that program text is included by default, core files from failures in the field can be useful without the incredibly arduous task of exactly replicating the original environment. The program text also includes a partial symbol table -- the dynsym -- so you can get accurate stack back traces, and correctly disassemble functions in your [favorite post-mortem debugger](http://docs.sun.com/db/doc/806-6545). If the dynsym doesn't cut it, you can use coreadm(1M) to configure your process to include the full symbol table in its core dumps as well -- so don't strip those binaries!

Also new to Solaris 10, we've started building many libraries with embedded type information in a compressed format. This is more of a teaser, since we're not quite ready to ship the tools to generate that type information, but that type information is included in core files by default. So now not only can we in Solaris actually make headway on core files we get from customers, but we can make progress much more quickly.

If you've installed Solaris Express, go check out the man page for coreadm(1m) and figure out how to get the right content in your core files. Once you get your first core file from a Solaris 10 machine in the field I hope you'll appreciate how much easier it was to debug.
