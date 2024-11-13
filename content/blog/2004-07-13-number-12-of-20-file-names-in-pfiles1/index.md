---
title: "Number 12 of 20: file names in pfiles(1)"
date: "2004-07-13"
categories: 
  - "opensolaris"
---

[go to the Solaris 10 top 11-20 list for more](http://dtrace.org/blogs/ahl/the_solaris_10_top_11)

[Eric Schrock](http://blogs.sun.com/eschrock) has tagged in to talk about [file names in pfiles(1)](http://blogs.sun.com/roller/page/eschrock/20040712#nuts_and_bolts_of_pfiles). This is something we've wanted for forever; here's a teaser:

```
bash-2.05# pfiles 100354
100354: /usr/lib/nfs/mountd
Current rlimit: 256 file descriptors
0: S_IFCHR mode:0666 dev:267,0 ino:6815752 uid:0 gid:3 rdev:13,2
O_RDONLY
/devices/pseudo/mm@0:null
1: S_IFCHR mode:0666 dev:267,0 ino:6815752 uid:0 gid:3 rdev:13,2
O_WRONLY
/devices/pseudo/mm@0:null
...
11: S_IFCHR mode:0000 dev:267,0 ino:33950 uid:0 gid:0 rdev:105,45
O_RDWR
/devices/pseudo/tl@0:ticots
12: S_IFREG mode:0644 dev:32,0 ino:583850 uid:0 gid:1 size:364
O_RDWR|O_CREAT|O_TRUNC
/etc/rmtab

```
