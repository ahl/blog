---
title: "Open-Sourcing the JavaOne Keynote"
date: "2005-06-27"
categories:
  - "dtrace"
permalink: /2005/06/27/open-sourcing-the-javaone-keynote/
---

This morning I gave a demo of [DTrace](http://www.sun.com/bigadmin/content/dtrace/) with the [Java agents](https://solaris10-dtrace-vm-agents.dev.java.net) during the keynote at [JavaOne](http://java.sun.com/javaone). In the past few hours I've had a lot of great feedback from Java developers -- we've found a bunch of _big_ performance wins already, and I expect we'll find more this week (remember the [DTrace challenge](http://www.sun.com/software/solaris/javaone_challenge.jsp)). For the demo, I ran /usr/java/demo/jfc/Java2D/Java2Demo.jar with the Java DTrace agents enabled and ran a couple of scripts on it.

The first script just gathered a frequency count for each method invoked -- nothing too subtle:

**jmethod.d**

```
#!/usr/sbin/dtrace -s
dvm$1:::method-entry
{
@[copyinstr(arg0), copyinstr(arg1)] = count();
}
END
{
printa("%-10@u %s.%s()\n", @);
}

```

```
bash-3.00# dtrace -s jmethods.d `pgrep java`
...
574        sun/java2d/SunGraphics2D.getCompClip()
608        sun/java2d/pipe/Region.dimAdd()
648        java/lang/ref/Reference.get()
671        java/awt/geom/AffineTransform.transform()
685        java/awt/Component.getParent_NoClientCode()
685        java/awt/Component.getParent()
702        sun/misc/VM.addFinalRefCount()
798        java/lang/ref/ReferenceQueue.remove()
809        java/lang/ref/Reference.access$200()
923        java/awt/geom/RectIterator.isDone()
1228       sun/dc/pr/Rasterizer.nextTile()
1657       sun/dc/pr/Rasterizer.getTileState()
1692       sun/java2d/pipe/AlphaColorPipe.renderPathTile()
1692       sun/java2d/pipe/AlphaColorPipe.needTile()
1702       sun/java2d/SunGraphics2D.getSurfaceData()
3457       java/lang/Math.min()
^C

```

The second demo was a little more exciting: this guy followed a thread of control all the way from Java code through the native library code, the system calls, and all the kernel function calls to the lowest levels of the system. Each different layer of the stack is annotated with _color_ -- the first use of color in a DTrace script as far as I know.

**follow.d**

```
#!/usr/sbin/dtrace -s
/*
* This script was used for the DTrace demo during the JavaOne keynote.
*
* VT100 escape sequences are used to produce multi-colored output from
* dtrace(1M). Pink is Java code, red is library code, blue is system calls,
* and green is kernel function calls.
*/
#pragma D option quiet
dvm$1:::method-entry
/copyinstr(arg0) == "sun/java2d/pipe/AlphaColorPipe" &&
copyinstr(arg1) == "renderPathTile"/
{
self->interested = 1;
self->depth = 8;
}
dvm$1:::method-entry
/self->interested/
{
printf("33[01;35m%*.*s -> %s.%s33[0m\n",
self->depth, self->depth, "",
copyinstr(arg0), copyinstr(arg1));
self->depth += 2;
}
dvm$1:::method-return
/self->interested/
{
self->depth -= 2;
printf("33[01;35m%*.*s depth, self->depth, "",
copyinstr(arg0), copyinstr(arg1));
}
dvm$1:::method-return
/self->interested &&
copyinstr(arg0) == "sun/java2d/pipe/AlphaColorPipe" &&
copyinstr(arg1) == "renderPathTile"/
{
self->interested = 0;
self->depth = 0;
exit(0);
}
pid$1:::entry
/self->interested && probemod != "libdvmti.so"/
{
printf("33[01;31m%*.*s -> %s`%s33[0m\n",
self->depth, self->depth, "",
probemod, probefunc);
self->depth += 2;
}
pid$1:::return
/self->interested && probemod != "libdvmti.so"/
{
self->depth -= 2;
printf("33[01;31m%*.*s depth, self->depth, "",
probemod, probefunc);
}
syscall:::entry
/self->interested/
{
printf("33[01;34m%*.*s => %s33[0m\n",
self->depth, self->depth, "",
probefunc);
self->depth += 2;
}
syscall:::return
/self->interested/
{
self->depth -= 2;
printf("33[01;34m%*.*s depth, self->depth, "",
probefunc);
}
fbt:::entry
/self->interested/
{
printf("33[32m%*.*s -> %s33[0m\n",
self->depth, self->depth, "",
probefunc);
self->depth += 2;
}
fbt:::return
/self->interested/
{
self->depth -= 2;
printf("33[32m%*.*s depth, self->depth, "",
probefunc);
}

```

`bash-3.00# dtrace -s follow.d \`pgrep java\`  
     -> sun/java2d/pipe/AlphaColorPipe.renderPathTile       -> copyout  
       <- kcopy       -> sun/java2d/SunGraphics2D.getSurfaceData...  
           <- libc.so.1\`\_lwp\_cond\_signal  
         <- libjvm.so\`\_\_1cNObjectMonitorEexit6MpnGThread\_\_v\_  
       <- libjvm.so\`\_\_1cSObjectSynchronizerIjni\_exit6FpnHoopDesc\_pnGThread\_\_v\_  
     <- libjvm.so\`jni\_MonitorExit  
   <- libawt.so\`Java\_sun\_java2d\_loops\_MaskFill\_MaskFill<- sun/java2d/pipe/AlphaColorPipe.renderPathTile`

Now you can give a keynote demo of your very own.

* * *

Technorati Tags: [DTrace](http://technorati.com/tag/DTrace) [JavaOne](http://technorati.com/tag/JavaOne)
