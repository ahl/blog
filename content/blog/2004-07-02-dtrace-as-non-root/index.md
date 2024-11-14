---
title: "DTrace as non-root"
date: "2004-07-01"
categories:
  - "dtrace"
permalink: /2004/07/02/dtrace-as-non-root/
---

fintanr's [weblog](http://blogs.sun.com/fintanr) has a nice [entry](http://blogs.sun.com/roller/page/fintanr/20040629#giving_a_user_privileges_to) on how to configure Solaris 10 to give privileges to individual users so they can run DTrace as non-root. By default, users require additional privileges to run DTrace because even providers that don't expose kernel state explicitly (like the `syscall` provider) can impact performance on the entire box. The privileges, what each permits and the implications are described in excruciating detail in the [Solaris Dynamic Tracing Guide](http://www.sun.com/bigadmin/content/dtrace/d10_latest.pdf) (Chapter 31 Security).
