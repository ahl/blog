---
title: "DTrace and JavaOne: The End of the Beginning"
date: "2008-04-10"
categories: 
  - "dtrace"
---

It was a good run, but [Jarod](http://www.forsythesunsolutions.com/jarod) and I didn't make the cut for JavaOne this year...

### 2005

In 2005, Jarod came up with what he described as a jacked up way to use DTrace to get inside Java. This became the basis of the Java provider (first dvm for the 1.4.2 and 1.5 JVMs and now the hotspot provider for Java 6). That year, I got to [stand up on stage at the keynote](http://dtrace.org/blogs/ahl/dtrace_in_the_javaone_keynote) with John Loiacono and present DTrace for Java for the [first time](http://dtrace.org/blogs/ahl/open_sourcing_the_javaone_keynote) (to 10,000 people -- I was nervous). John was then the EVP of software at Sun. Shortly after that, he parlayed our keynote success into a [sweet gig at Adobe](http://www.adobe.com/aboutadobe/pressroom/executivebios/johnloiacono.html) (I was considered for the job, but ultimately rejected, they said, because their door frames couldn't accommodate my [fro](http://flickr.com/photos/astros/24456640/) -- legal action is pending).

That year we also started [the DTrace challenge](http://dtrace.org/blogs/ahl/crazy_dtrace_java_challenge). The premise was that if we chained up Jarod in the exhibition hall, developers could bring him their applications and he could use DTrace to find a performance win -- or he'd fork over a free iPod. In three years Jarod has given out one iPod and that one deserves a Bondsian asterisk.

After the excitement of the keynote, and the frenetic pace of the exhibition hall (and a haircut), Jarod and I anticipated at least fair interest in our talk, but we expected the numbers to be down a bit because we were presenting in the afternoon on the last day of the conference. We got to the room 15 minutes early to set up, skirting what we thought must have been the line for lunch, or free beer, or something, but turned out to be the line for our talk. Damn. It turns out that in addition to the 1,000 in the room, there was an overflow room with another 500-1,000 people. That [first DTrace for Java](http://dtrace.org/blogs/ahl/dtrace_presentation_at_javaone) talk had only the most basic features like tracing method entry and return, memory allocation, and Java stack backtraces -- but we already knew we were off to a good start.

### 2006

No keynote, but the DTrace [challenge](http://flickr.com/photos/jimgris/148199546/) was on again and [our talk](http://dtrace.org/blogs/ahl/javaone_dtrace_session_2006) reprised its primo slot on the last day of the conference after lunch (yes, that's sarcasm). That year the Java group took the step of including DTrace support in the JVM itself. It was also possible to dynamically turn instrumentation of the JVM off and on as opposed to the start-time option of the year before. In addition to our talk, there was a DTrace hands-on lab that was quite popular and got people some DTrace experience after watching what it can do in the hands of someone like Jarod.

### 2007

The [DTrace talk in 2007](http://dtrace.org/blogs/ahl/dtrace_javaone_2007) (again, last day of the conference after lunch) was actually one of my favorite demos I've given because I had never seen the technology we were presenting before. Shortly before JavaOne started, Lev Serebryakov from the Java group had built a way of embedding static probes in a Java program. While this isn't required to trace Java code, it does mean that developers can expose the higher level semantics of their programs to users and developers through DTrace. Jarod hacked up an example in his hotel room about 20 minutes before we presented, and amazingly it all went off without a hitch. How money is that?

JSDT -- as the Java Statically Defined Tracing is called -- is in development for the next version of the JVM, and is the next step for DTrace support of dynamic languages. Java was the first dynamic language that we first considered for use with DTrace, and it's quite a tough environment to support due to the incredible sophistication of the JVM. That support has lead the way for other dynamic languages such as Ruby, Perl, and Python which all now have built-in DTrace providers.

### 2008

For DTrace and Java, this is not the end. It is not even the beginning of the end. Jarod and I are out, but Jon, Simon, Angelo, Raghavan, Amit, and others are in. At JavaOne 2008 next month there will be a talk, a BOF, and a hands-on lab about DTrace for Java **and** it's not even all Java: there's some php and JavaScript mixed in and both also have their own DTrace providers. I've enjoyed speaking at JavaOne these past three years, and while it's good to pass the torch, I'll miss doing it again this year. If I have the time, and can get past security I'll try to sneak into Jon and Simon's talk -- though it will be a departure from tradition for a DTrace talk to fall on a day other than the last.
