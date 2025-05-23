---
title: "dtrace.conf(24)"
date: "2025-01-05T17:00:00"
permalink: /dtrace-conf-2024/
---

A few weeks ago, Oxide hosted dtrace.conf(24), the (mostly) quadrennial DTrace conference. We started the conference in 2008, continuing in 2012 and 2016. When I joined Oxide in 2020; it felt like stars aligning for dtrace.conf(20)... ultimately canceled due to Covid. This year, we moved the conference online. As an unconference, we're **very** reliant on folks showing up ready to present. Would we even have enough speakers to get us through lunchtime? It turned out that Bryan and my hand-wringing was unwarranted: we had a great, *full* day of presentations both from folks we knew (invited, and cajoled) and some new faces.

We recorded the livestream and I assembled each talk into a video (with mostly light, and sometimes less-light editing to cover up AV glitches). Enjoy!

* * *

Bryan kicked us off with the State of the Union. The evolution of DTrace and of dtrace.conf. I share his appreciation that 20+ years after releasing DTrace, there's still a community of users, developers, and speakers!

https://youtu.be/8jBu1A-ol-Q

Ben and I then presented our work on the Rust `usdt` crate for adding USDT probes to Rust code. This turned out not only to be incredibly useful for us at Oxide, but also foundational for several of the talks that followed.

https://youtu.be/7J1mVrRlsNg

Kyle showed innovative of in-kernel SDT with Rust at Oxide.

https://youtu.be/5ki7xQYRBEg

Alan showed the instrumentation and monitoring he and the Oxide storage team built for understanding how data flow through the system.

https://youtu.be/VeTldIjfCEc

Aapo, relatively new to DTrace, had spent some time unearthing the history and mechanisms of DTrace USDT... presented in delightful narrative fashion!

https://youtu.be/eGG69xkyQ-k

Nahum shared his work integrating OpenTelemetry with Dropshot.

https://youtu.be/P81LhhtIsQ0

Dave (the man, the myth, the legend) provided resources for users learning DTrace. This is a great talk for anyone looking to get started.

https://youtu.be/UM0YYB7McMw

Kris, our invited speaker from Oracle, presented the work he and his team did porting DTrace to an eBPF backend on Linux. This is an incredible advancement for both DTrace and eBPF users.

https://youtu.be/rMOmaWQ18d4

Ry gave an amazing talk on integrating P4 and DTrace.

https://youtu.be/3GZQSwZyoOs

Robert presented a future for user-land tracing with DTrace that's both exciting and tantalizingly close at-hand.

https://youtu.be/yK7xs2SmfFY

Eliza&mdash;inspired by the proceedings&mdash;gave a last-minute talk on the confluence of Tokio Tracing and DTrace.

https://youtu.be/_ApAZ9F_dHI

Luqman found a long-standing but in *kernel* SDT in the presence of probes in tail-call context. He walked us through the discovery and the fix.

https://youtu.be/P7-iIpW8jUU

And finally, Bryan and I closed it out with a recap of the day.

https://youtu.be/qqLJtu7SmhA

Thanks to everyone who helped make the day possible, especially Kevin, Ben, and Matthew!
