---
title: "Double-Parity RAID-Z"
date: "2006-06-18"
categories:
  - "zfs"
tags:
  - "galoisfield"
  - "opensolaris"
  - "raid-z"
  - "raidz2"
  - "solaris"
  - "zfs"
permalink: /2006/06/18/double-parity-raid-z/
---

When ZFS first started, it was just [Jeff](http://blogs.sun.com/bonwick) trying to pair old problems with new solutions in margins too small to contain either. Then [Matt](http://blogs.sun.com/ahrens) joined up to bring some young blood to the project. By the time the project putback, the team had grown to more than a dozen. And now I've been pulled in -- if only for a cameo.

When ZFS first hit the streets, Jeff wrote about [RAID-Z](http://blogs.sun.com/roller/page/bonwick?entry=raid_z), an implementation of RAID designed for ZFS. RAID-Z improves upon previous RAID schemes primarily in that it eliminates the so-called "write hole" by using a full (and variable-sized) stripe for all write operations. It's worth noting that RAID-Z exploits the fact that ZFS is an end-to-end solution such that metadata (traditionally associated with the filesystem layer) is used to interpret the RAID layout on disk (an operation usually ascribed to a volume manager). In that post, Jeff mentioned that a double-parity version of RAID-Z was in the works. What he actually meant is that he had read a paper, and thought it might work out -- you'd be forgiven for inferring that actual code had been written.

Over lunch, [Bill](http://blogs.sun.com/bill) -- yet another elite ZFS hacker -- mentioned double-parity RAID-Z and their plans for implementing it. I pressed for details, read the paper, got interested in the math, and started yakking about it enough for Bill to tell me to put up or shut up.

### RAID-6

The basic notion behind double-parity RAID or RAID-6 is that a stripe can survive _two_ failures without losing data where RAID-5 can survive only a single failure. There are a number of different ways of implementing double-parity RAID; the way Jeff and Bill had chosen (due to its computational simplicity and lack of legal encumbrance) was one described by H. Peter Anvin in [this paper](http://kernel.org/pub/linux/kernel/people/hpa/raid6.pdf). It's a nice read, but I'll attempt to summarize some of the math (warning: this summary is going to be boring and largely unsatisfying so feel free to [skip it](#raidz2)).

For a given stripe of _n_ data blocks, **D0 .. Dn-1**, RAID-5 computes the contents of the parity disk **P** by taking the bitwise **XOR** of those data blocks. If any **D_n_** is corrupted or missing, we can recover it by taking the **XOR** of all other data blocks with **P**. With RAID-6, we need to compute another prity disk **Q** using a different technique such that **Q** alone can reconstruct any **D_n_** and **P** and **Q** together can reconstruction any _two_ data blocks.

To talk about this, it's easier -- believe it or not -- to define a Galois field (or a finite field as I learned it) over the integers \[0..255\] -- the values that can be stored in a single byte. The addition field operation (+) is just bitwise **XOR**. Multiplication (x) by 2 is given by this bitwise operation for _x_ **x** 2 = _y_:

<table><tbody><tr><td><em>y</em><sub>7</sub></td><td>=</td><td><em>x</em><sub>6</sub></td></tr><tr><td><em>y</em><sub>6</sub></td><td>=</td><td><em>x</em><sub>5</sub></td></tr><tr><td><em>y</em><sub>5</sub></td><td>=</td><td><em>x</em><sub>4</sub></td></tr><tr><td><em>y</em><sub>4</sub></td><td>=</td><td><em>x</em><sub>3</sub> + <em>x</em><sub>7</sub></td></tr><tr><td><em>y</em><sub>3</sub></td><td>=</td><td><em>x</em><sub>2</sub> + <em>x</em><sub>7</sub></td></tr><tr><td><em>y</em><sub>2</sub></td><td>=</td><td><em>x</em><sub>1</sub> + <em>x</em><sub>7</sub></td></tr><tr><td><em>y</em><sub>1</sub></td><td>=</td><td><em>x</em><sub>0</sub></td></tr><tr><td><em>y</em><sub>0</sub></td><td>=</td><td><em>x</em><sub>7</sub></td></tr></tbody></table>

A couple of simple things worth noting: addition (+) is the same as subtraction (-), 0 is the additive identity and the multiplicative annihilator, 1 is the multiplicative identity. Slightly more subtle: each element of the field except for 0 (i.e. \[1..255\]) can be represented as 2_n_ for some _n_. And importantly: _x_\-1 = _x_254. Also note that _x_ x _y_ can be rewritten as 2log _x_ x 2log _y_ or 2log _x_ + log _y_ (where _+_ in that case is normal integer addition).

We compute **Q** as 2n-1 D0 + 2n-2 D1 ... + Dn-1 or equivalently ((...(((D0 x 2 + D1 + ...) x 2 + Dn-2) x 2 + Dn-1. Computing **Q** isn't much slower than computing **P** since we're just dealing with a few simple bitwise operations.

With **P** and **Q** we can recover from any two failures. If **D_x_** fails, we can repair it with **P**. If **P** also fails, we can recover **D_x_** by computing **Q_x_** where **Q_i_** = **Q** + 2n - 1 - x x **D_x_** (easily done by performing the same computation as for generating **Q** but with **D_x_** set to 0); **D_x_** is then (**Q_x_** + **Q**) / 2n - 1 - x = (**Q_x_** + **Q**) x 2x + 1 - n. Once we solve for **D_x_**, then we recompute **P** as we had initially.

When two data disks are missing, **D_x_** and **D_y_**, that's when the rubber really meets the road. We compute **Pxy** and **Qxy** such that **Pxy** + **Dx** + **Dy** = **P** and **Qxy** + 2n - 1 - x x **Dx** + 2n - 1 - y x **Dy** = **Q** (as before). Using those two expressions and some basic algebra, we can solve for **Dx** and then plug that in to solve for **Dy**. The actual expressions are a little too hairy for HTML, but you can check out equation 16 in the paper or [the code](http://cvs.opensolaris.org/source/xref/on/usr/src/uts/common/fs/zfs/vdev_raidz.c#518) for the gory details.

### Double-Parity RAID-Z

As of build 42 of [OpenSolaris](http://opensolaris.org), RAID-Z comes in a double-parity version to complement the existing single-parity version -- and it only took about 400 additional lines of code. Check out the code [here](http://cvs.opensolaris.org/source/xref/on/usr/src/uts/common/fs/zfs/vdev_raidz.c). Of particular interest are the [code](http://cvs.opensolaris.org/source/xref/on/usr/src/uts/common/fs/zfs/vdev_raidz.c#322) to generate both parity blocks and the [code](http://cvs.opensolaris.org/source/xref/on/usr/src/uts/common/fs/zfs/vdev_raidz.c#474) to do double block reconstruction. What's especially cool about ZFS is that we don't just blithely reconstruct data, but we can verify it against the known checksum. This means, for example, that we could get back seemingly valid data from all disks, but fail the checksum; in that case we'd first try reconstructing each individual block, and then try reconstructing every pair of blocks until we've found something that checksums. You can see the code for combinatorial reconstruction [here](http://cvs.opensolaris.org/source/xref/on/usr/src/uts/common/fs/zfs/vdev_raidz.c#989).

### Using raidz2

To make a double-parity RAID-Z vdev, specify `raidz2` to [zpool(1M)](http://docs.sun.com/app/docs/doc/819-2240/6n4htdnpp?a=view):

```
# zpool create pool raidz2 c1t0d0 c1t0d1 c1t0d2 c1t0d3 c1t0d4
```

This will create a pool with a double-parity RAID-Z vdev of width 5 where all data can sustain up to two failures be they corrupt data coming off the drives or drives that are failed or missing. The `raidz` vdev type continues to mean single-parity RAID-Z as does the new alias `raidz1`.

Double-parity RAID-Z is probably going to supplant the use of its single-parity predecessor in many if not most cases. As [Dave Hitz](http://blogs.netapp.com/dave/) of NetApp helpfully noted in a [recent post](http://blogs.netapp.com/dave/2006/05/why_double_prot.html) double-parity RAID doesn't actually cost you any additional space because you'll typically have wider stripes. Rather than having two single-parity stripes of 5 disks each, you'll have one double-parity stripe with 10 disks -- the same capacity with extra protection against failures. It also shouldn't cost you in terms of performance because the total number of disk operations will be the same and the additional math, while slightly more complex, is still insignificant compared with actually getting bits on disk. So enjoy the extra parity.

* * *

Technorati Tags: [OpenSolaris](http://technorati.com/tag/OpenSolaris) [ZFS](http://technorati.com/tag/ZFS)
