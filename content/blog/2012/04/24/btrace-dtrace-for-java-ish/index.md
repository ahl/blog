---
title: "BTrace: DTrace for Java... ish"
date: "2012-04-24"
categories: 
  - "dtrace"
tags: 
  - "btrace"
  - "dtrace"
  - "java"
---

DTrace [first peered into Java](http://dtrace.org/blogs/ahl/2005/04/18/dtracing-java/) in early 2005 thanks to an early prototype by Jarod Jenson that led eventually to the inclusion of USDT probes in the [HotSpot JVM](http://en.wikipedia.org/wiki/HotSpot). If you want to see where, say, the java.net.SocketOutputStream.write() method is called, you can simply run this DTrace script:

```
hotspot$target:::method-entry
/copyinstr(arg1, arg2) == "java/net/SocketOutputStream" &&
 copyinstr(arg3, arg4) == "write"/
{
        jstack(50, 8000);
}
```

And that will work as long as you rememember to start your JVM with the -XX:+ExtendedDTraceProbes option or you use the jinfo utility to enable it after the fact. And as long as you don't mind a crippling performance penalty (hint: you probably do).

Inspired by [dtrace.conf](http://dtrace.org/blogs/ahl/2012/04/09/dtrace-conf12-wrap-up/) a few weeks ago, I wanted to sketch out what the real Java provider would look like:

```
java$target:java.net.SocketOutputStream:write:entry
{
        jstack(50,8000);
}
```

And check it out:

```
# jdtrace.pl -p $(pgrep java) -n 'java$target:java.net.SocketOutputStream::entry{ jstack(50,8000); }'
dtrace: script '/tmp/jdtrace.19092/jdtrace.d' matched 0 probes
CPU     ID                    FUNCTION:NAME
0  64991 Java_com_sun_btrace_BTraceRuntime_dtraceProbe0:event
libbtrace.so`Java_com_sun_btrace_BTraceRuntime_dtraceProbe0+0xbb
com/sun/btrace/BTraceRuntime.dtraceProbe0(Ljava/lang/String;Ljava/lang/String;II)I
com/sun/btrace/BTraceRuntime.dtraceProbe(Ljava/lang/String;Ljava/lang/String;II)I
com/sun/btrace/BTraceUtils$D.probe(Ljava/lang/String;Ljava/lang/String;II)I
com/sun/btrace/BTraceUtils$D.probe(Ljava/lang/String;Ljava/lang/String;)I
java/net/SocketOutputStream.$btrace$jdtrace$probe1(Ljava/lang/String;Ljava/lang/String;)V
java/net/SocketOutputStream.write([BII)V
sun/nio/cs/StreamEncoder.writeBytes()V
sun/nio/cs/StreamEncoder.implFlushBuffer()V
sun/nio/cs/StreamEncoder.implFlush()V
sun/nio/cs/StreamEncoder.flush()V
java/io/OutputStreamWriter.flush()V
java/io/BufferedWriter.flush()V
java/io/PrintWriter.newLine()V
java/io/PrintWriter.println()V
java/io/PrintWriter.println(Ljava/lang/String;)V
com/delphix/appliance/server/ham/impl/HAMonitorServerThread.run()V
java/util/concurrent/ThreadPoolExecutor$Worker.runTask(Ljava/lang/Runnable;)V
java/util/concurrent/ThreadPoolExecutor$Worker.run()V
java/lang/Thread.run()V
StubRoutines (1)
libjvm.so`__1cJJavaCallsLcall_helper6FpnJJavaValue_pnMmethodHandle_pnRJavaCallArguments_pnGThread__v_+0x21d
libjvm.so`__1cCosUos_exception_wrapper6FpFpnJJavaValue_pnMmethodHandle_pnRJavaCallArguments_pnGThread__v2468_v_+0x27
libjvm.so`__1cJJavaCallsMcall_virtual6FpnJJavaValue_nGHandle_nLKlassHandle_nMsymbolHandle_5pnGThread__v_+0x149
libjvm.so`__1cMthread_entry6FpnKJavaThread_pnGThread__v_+0x113
libjvm.so`__1cKJavaThreadDrun6M_v_+0x2c6
libjvm.so`java_start+0x1f2
libc.so.1`_thrp_setup+0x9b
libc.so.1`_lwp_start
```

Obviously there's something fishy going on. First, we're using perl -- the shibboleth of fake-o-ware -- and there's this BTrace stuff in the output.

### Faking it with BTrace

[BTrace is a dynamic instrumentation tool for Java](http://kenai.com/projects/btrace); it is both inspired by DTrace and contains some DTrace integration. The perl script above takes the DTrace syntax and generates a DTrace script and a BTrace-enabled Java source file.

Like DTrace, BTrace lets you specify the points of instrumentation in your Java program as well as the actions to take. Here's what our generated source file looks like.

```
import com.sun.btrace.annotations.*;
import static com.sun.btrace.BTraceUtils.*;
@BTrace
public class jdtrace {
        @OnMethod(clazz="java.net.SocketOutputStream", method="write", location=@Location(Kind.ENTRY))
        public static void probe1(@ProbeClassName String c,
            @ProbeMethodName String m) {
                String name = "entry";
                String p = Strings.strcat(c, Strings.strcat(":",
                    Strings.strcat(m, Strings.strcat(":", name))));
                D.probe(p, "");
        }
}
```

Note that we specify where to trace (this can be a regular expression), and then take the action of joining the class, method, and "entry" string into a single string that we pass to the D.probe() method that causes a BTrace USDT probe to fire.

Here's what the D script looks like:

```
btrace$target:::event
{
        this->__jd_arg = copyinstr(arg0);
        this->__jd_mod = strtok(this->__jd_arg, ":");
        this->__jd_func = strtok(NULL, ":");
        this->__jd_name = strtok(NULL, ":");
}

btrace$target:::event
/((this->__jd_mod == "java.net.SocketOutputStream" &&
 this->__jd_func == "write" &&
 this->__jd_name == "entry"))/
{
        jstack(50,8000);
}
```

It's pretty simple. We parse the string that was passed to D.probe(), and disassemble it into the DTrace notion of module, function, and name. We then use that information so that the specified actions are executed as appropriate (we could have specified different Java methods to probe, and different actions to take for each). [Here's the code](https://github.com/adamleventhal/jdtrace/blob/master/jdtrace/jdtrace.pl) if you're interested.

This isn't the real Java provider, but is it close enough? Unfortunately not. The most glaring problem is that BTrace sometimes renders my Java process unresponsive. Other times it leaves instrumentation behind with no way of extracting it. The word "safe" appears as the third word on the BTrace website ("BTrace is safe"), but apparently there's still some way to go to achieve the requisite level of safety.

### A Better BTrace

BTrace is an interesting tool for examining Java programs, but one obvious obstacle is that the programs are pretty cumbersome to write. With BTrace, we should be able to write a simple one-liner to see where we are when the java.net.SocketOutputStream.write() method is called, but instead we have to write a fairly cumbersome program:

```
import com.sun.btrace.annotations.*;
import static com.sun.btrace.BTraceUtils.*;
@BTrace
public class TraceWrite {
        @OnMethod(clazz="java.net.SocketOutputStream", method="write", location=@Location(Kind.ENTRY))
        public static void onWrite() {
                jstack();
        }
}
```

DTrace-inspired syntax would let users iterate much more quickly:

```
$ dbtrace -p $(pgrep -n java) -n 'java.net.SocketOutputStream:write:entry{ jstack(); }'
java.net.SocketOutputStream.write(SocketOutputStream.java)
sun.nio.cs.StreamEncoder.writeBytes(StreamEncoder.java:202)
sun.nio.cs.StreamEncoder.implFlushBuffer(StreamEncoder.java:272)
sun.nio.cs.StreamEncoder.implFlush(StreamEncoder.java:276)
sun.nio.cs.StreamEncoder.flush(StreamEncoder.java:122)
java.io.OutputStreamWriter.flush(OutputStreamWriter.java:212)
java.io.BufferedWriter.flush(BufferedWriter.java:236)
java.io.PrintWriter.newLine(PrintWriter.java:438)
java.io.PrintWriter.println(PrintWriter.java:585)
java.io.PrintWriter.println(PrintWriter.java:696)
com.delphix.appliance.server.ham.impl.HAMonitorServerThread.run(HAMonitorServerThread.java:56)
java.util.concurrent.ThreadPoolExecutor$Worker.runTask(ThreadPoolExecutor.java:886)
java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:908)
java.lang.Thread.run(Thread.java:662)
```

With BTrace, you can trace nearly arbitrary information about a program's state, but instead of doing something like this:

```
dbtrace -p $(pgrep -n java) -n 'java.net.SocketOutputStream:write:entry{ printFields(this.impl); }'
```

You have to do this:

```
import com.sun.btrace.annotations.*;
import com.sun.btrace.AnyType;
import static com.sun.btrace.BTraceUtils.Reflective.*;
@BTrace
public class TraceWrite {
        @OnMethod(clazz="java.net.SocketOutputStream", method="write", location=@Location(Kind.ENTRY))
        public static void onWrite(@Self Object self) {
                Object impl = get(field(classOf(self), "impl"), self);
                printFields(impl);
        }
}
```

```
$ ./bin/btrace $(pgrep -n java) TraceWrite.java
{server=null, port=1080, external_address=null, useV4=false, cmdsock=null, cmdIn=null, cmdOut=null, applicationSetProxy=false, timeout=0, trafficClass=0, shut_rd=false, shut_wr=false, socketInputStream=java.net.SocketInputStream@9993a1, fdUseCount=0, fdLock=java.lang.Object@ab5443, closePending=false, CONNECTION_NOT_RESET=0, CONNECTION_RESET_PENDING=1, CONNECTION_RESET=2, resetState=0, resetLock=java.lang.Object@292936, fd1=null, anyLocalBoundAddr=null, lastfd=-1, stream=false, socket=Socket[addr=/127.0.0.1,port=38832,localport=8765], serverSocket=null, fd=java.io.FileDescriptor@50abcc, address=/127.0.0.1, port=38832, localport=8765, }
```

BTrace needs a language that enables rapid iteration -- piggybacking on Java is holding it back -- and it needs some hard safety guarantees. With those, many developers and support engineers would use BTrace as part of their daily work -- we certainly would here at [Delphix](http://www.delphix.com).

Back to DTrace. Even with a useable solution for Java only, the ability to have lightweight and focused tracing for Java (and other dynamic languages) could be highly valuable. We'll see how far BTrace can take us.
