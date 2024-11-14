---
title: "DTrace OEL Dynamic Language Support"
date: "2014-12-27"
categories:
  - "dtrace"
permalink: /2014/12/27/dtrace-oel-dynamic-language-support/
---

We built DTrace to solve problems; at the start, the problems we understood best were our own. In the Solaris Kernel Group we started by instrumenting the kernel and system calls, the user/kernel boundary. Early use required detailed knowledge of kernel internals. As DTrace use grew—within the team, in Sun and then beyond—we extended DTrace to turn every function and every instruction in user programs into probes. We added stable points of instrumentation both in the kernel and in user-land so that no deep knowledge of program or kernel internals would be required.

Oracle has been evolving their port of DTrace to OEL, prioritizing the stable points of instrumentation most relevant for the widest group of users. While DTrace started with providers that unlocked tens of thousands of points of instrumentation, the Oracle port enables a small number of comprehensible probes. [Since I last tried out their port](http://dtrace.org/blogs/ahl/2012/02/23/dtrace-oel-update/) they’ve fixed some bugs, and added support for stable I/O and process probes, as well as user-land static probes.

```
[root@screven ~]# uname -a
Linux screven 3.8.13-16.el6uek.x86_64 #1 SMP Fri Sep 20 11:54:42 PDT 2013 x86_64 x86_64 x86_64 GNU/Linux
[root@screven ~]# cat test.d
provider test {
        probe foo(int);
};
[root@screven ~]# cat main.c
#include "test.h"

int
main(int argc, char **argv)
{
        TEST_FOO(100);
        return (0);
}
[root@screven ~]# dtrace -h -s test.d
[root@screven ~]# gcc -c main.c
[root@screven ~]# dtrace -G -s test.d main.o
[root@screven ~]# gcc -o main main.o test.o
[root@screven ~]# dtrace -c ./main -n 'test$target:::foo{ trace(arg0); }'
dtrace: description 'test$target:::foo' matched 1 probe
CPU     ID                    FUNCTION:NAME
  0    643                         main:foo               100

```

[USDT](http://dtrace.org/blogs/ahl/2006/05/08/user-land-tracing-gets-better-and-better/), as it’s called, was a relatively late addition in the initial development of DTrace. We added it initially to support probes in user-land locking primitives (the [plockstat(1M)](https://developer.apple.com/library/mac/documentation/Darwin/Reference/ManPages/man1/plockstat.1.html) command uses it just as the [lockstat(1M)](http://docs.oracle.com/cd/E19253-01/816-5166/lockstat-1m/index.html) command was converted to use kernel SDT probes). We were right in thinking that USDT would be useful for providing probes in infrastructure software such as [Apache](https://github.com/davepacheco/mod_usdt) and [MySQL](http://dev.mysql.com/doc/refman/5.5/en/dba-dtrace-mysqld-ref.html); we didn't anticipate how incredibly valuable it would be for supporting dynamic languages such as javascript (including Node), python, java, and bash.

USDT built on both the learning and code from years of DTrace development. By effectively starting there, OEL benefits from a decade of integrations and investigations. DTrace users on all platforms will benefit from the growth of our community. I look forward to seeing the new investigations on OEL and new integrations in all types of applications.
