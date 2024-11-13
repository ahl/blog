---
title: "a small ZFS hack"
date: "2007-01-28"
categories: 
  - "zfs"
---

I've been [dabbling](http://dtrace.org/blogs/ahl/double_parity_raid_z) a bit in [ZFS](http://www.opensolaris.org/os/community/zfs/) recently, and what's amazing is not just how well it solved the well-understood filesystem problem, but how its design opens the door to novel ways to manage data. Compression is a great example. An almost accidental by-product of the design is that your data can be stored compressed on disk. This is especially interesting in an era when we have CPU cycles to spare, many too few available IOPs, and disk latencies that you can measure with a stop watch (well, not really, but you get the idea). With ZFS can you trade in some of those spare CPU cycles for IOPs by turning on compression, and the additional latency introduced by decompression is dwarfed by the time we spend twiddling our thumbs waiting for the platter to complete another revolution.

### smaller and smaller

Turning on compression in zfs (`zfs compression=on <dataset>`) enables the so called LZJB compression algorithm -- a [variation](http://cvs.opensolaris.org/source/xref/onnv/onnv-gate/usr/src/uts/common/os/compress.c#34) on [Lempel-Ziv](http://en.wikipedia.org/wiki/Lempel-Ziv) tagged by its [humble author](http://blogs.sun.com/bonwick). LZJB is fast, reasonably effective, and quite simple (compress and decompress are [implemented in about a hundred lines of code](http://cvs.opensolaris.org/source/xref/onnv/onnv-gate/usr/src/uts/common/fs/zfs/lzjb.c)). But the ZFS architecture can support many compression algorithms. Just as users can choose from several different checksum algorithms (fletcher2, fletcher4, or sha256), ZFS lets you pick your compression routine -- it's just that there's only the one so far.

### putting the z(lib) in ZFS

I thought it might be interesting to add a **gzip** compression algorithm based on [zlib](http://www.zlib.net/). I was able to hack this up pretty quicky because the Solaris kernel already contains a complete copy of zlib (albeit scattered around a little) for [decompressing](http://cvs.opensolaris.org/source/xref/onnv/onnv-gate/usr/src/uts/common/zmod/zmod.c#42) [CTF data](http://cvs.opensolaris.org/source/xref/onnv/onnv-gate/usr/src/uts/common/sys/ctf.h#39) for [DTrace](http://www.opensolaris.org/os/community/dtrace/), and apparently for some sort of [compressed PPP streams module](http://cvs.opensolaris.org/source/xref/onnv/onnv-gate/usr/src/uts/common/io/ppp/spppcomp/zlib.c) (or whatever... I don't care). Here's what the ZFS/zlib mash-up looks like (for the curious, this is with the default compression level -- 6 on a scale from 1 to 9):

```
# zfs create pool/gzip
# zfs set compression=gzip pool/gzip
# cp -r /pool/lzjb/* /pool/gzip
# zfs list
NAME        USED  AVAIL  REFER  MOUNTPOINT
pool/gzip  64.9M  33.2G  64.9M  /pool/gzip
pool/lzjb   128M  33.2G   128M  /pool/lzjb

```

That's with a 1.2G crash dump (pretty much the most compressible file imaginable). Here are the compression ratios with a pile of ELF binaries (/usr/bin and /usr/lib):

```
# zfs get compressratio
NAME       PROPERTY       VALUE      SOURCE
pool/gzip  compressratio  3.27x      -
pool/lzjb  compressratio  1.89x      -

```

Pretty cool. Actually compressing these files with gzip(1) yields a _slightly_ smaller result, but it's very close, and the convenience of getting the same compression transparently from the filesystem is awfully compelling. It's just a prototype at the moment. I have no idea how well it will perform in terms of speed, but early testing suggests that it will be lousy compared to LZJB. I'd be very interested in any feedback: Would this be a useful feature? Is there an ideal trade-off between CPU time and compression ratio? I'd like to see if this is worth integrating into OpenSolaris.

* * *

Technorati Tags: [ZFS](http://technorati.com/tag/ZFS) [OpenSolaris](http://technorati.com/tag/OpenSolaris)
