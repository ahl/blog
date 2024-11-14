---
title: "DTrace probes in Rust"
date: "2023-12-29"
categories:
  - "dtrace"
coverImage: "Untitled.jpg"
permalink: /2023/12/29/dtrace-probes-in-rust/
---

DTrace's User-land Statically-Defined Tracing (USDT) was... kind of an accident. [Bryan](https://mastodon.social/@bcantrill) has (kindly) retconned the genesis of USDT as a way to understand dynamic languages\[citation needed\]. Indeed, it's been essential for that, but its origins were much less ambitious or prescient.

Way back in the **1990s(!)** Jeff Bonwick had created a program called [`lockstat(1M)`](https://docs.oracle.com/cd/E86824_01/html/E54764/lockstat-1m.html) for live instrumentation of kernel locking primitives ("live" as distinct from "dynamic" in that while the instrumentation could be turned on and off, the data payloads were pretty much static). This was incredibly useful! What locks were hot? Where were they contended? New observability led to new performance fixes. When we built DTrace, we [incorporated its instrumentation as the `lockstat` provider](https://www.usenix.org/conference/2004-usenix-annual-technical-conference/dynamic-instrumentation-production-systems).  After building user-land tracing with the `pid` provider, it seemed like an obvious step to build a `plockstat(1M)` command to understand user-land locking primitives. So I built it. And, my goodness, was the first iteration of that a disaster. A total mess. Special cases on special cases with unholy knowledge sprinkled everywhere. We yanked that out of the first integration of DTrace and went back to the drawing board. What we came up with was USDT, the first provider of which was `plockstat` whose probes are consumed by the eponymous command.

Bryan and I touched on this somewhere in the [2+ hours of DTrace history](https://oxide-and-friends.transistor.fm/episodes/dtrace-at-20) we recorded back in September:

<iframe src="https://share.transistor.fm/e/bdfd0524" width="100%" height="180" frameborder="no" scrolling="no" seamless=""></iframe>

Years passed as they do, and USDT turned out to be very very very useful. At Oxide, we wanted that usefulness in the Rust code we're writing, so my colleague, Ben Naecker, and I built a [`usdt` crate](https://crates.io/crates/usdt). While we have probes in lots of places, knowledge had passed more or less word-of-mouth. Shocking! To remedy this, at the last Oxide all-company event, Ben and I put together a little slide deck on inserting USDT probes in Rust code, using those probes (spoiler: here's a **new** cause for frustration with `async` Rust), and an exercise to try it out. Enjoy!

<iframe class="speakerdeck-iframe" frameborder="0" src="https://speakerdeck.com/player/34c4980d89b74fe8b3be7f9032781169" title="DTrace USDT for Rust" allowfullscreen="true" style="border: 0px; background: padding-box padding-box rgba(0, 0, 0, 0.1); margin: 0px; padding: 0px; border-radius: 6px; box-shadow: rgba(0, 0, 0, 0.2) 0px 5px 40px; width: 100%; height: auto; aspect-ratio: 560 / 315;" data-ratio="1.7777777777777777"></iframe>

(tl;dr add USDT probes where you have log statements and you'll probably thank yourself later.)
