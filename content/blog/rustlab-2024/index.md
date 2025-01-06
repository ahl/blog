---
title: "DTrace at Rustlab 2024"
date: "2025-01-05"
permalink: /dtrace-at-rustlab-2024/
---

I gave a talk at [Rustlab](https://rustlab.it/) this past November about work my colleague, Ben Naecker, and I did to add [DTrace USDT](https://illumos.org/books/dtrace/chp-usdt.html) (user-land statically defined tracing) probes to our Rust code. I've given a lot of talks about DTrace over the years; I only realized as I got on stage how excited I was to talk about the confluence of DTrace and Rust... and to an audience that may have never seen DTrace in use. I got to go deep (maybe too deep) into the details of making our [`usdt`](https://crates.io/crates/usdt) crate have zero disabled probe-effect[**](#zero)&mdash;a principle of DTrace analogous to Rust's principle of zero-cost abstractions.

**Need an endorsement of the talk?** A C++ programmer in the audience remarked after the talk that he was inspired to learn Rust. (This was my son. He also offered the carefully constructed observation, "the audience seemed to enjoy your jokes." Which parents of college-aged students will recognize for the encomium that it is.)

https://www.youtube.com/watch?v=J10nd1UdSys

## Florence

This summer I traveled with my family to Florence to drop off my older boy at his study abroad program. I even got to record an [episode of Oxide and Friends](https://share.transistor.fm/s/7f163561) abroad.

Florence was wonderful! I idly started thinking if there might be a good reason to visit again during the semester. This quickly turned up Rustlab. A Rust conference in Florence? It didn't even take much imagination to justify the trip! The organizers even had a DTrace+Rust-sized hole in their agenda.

## Rustlab

The conference was great: good folks, well-run, good talks, and in a delightful location. In a recent Oxide and Friends, we [talked about conferences in tech](https://share.transistor.fm/s/081bb0ed). A lot of them have gone online, and&mdash;compared with decades ago&mdash;there are many other opportunities to congregate without traveling, but there's something delightful about meeting with folks in-person. Rustlab had a great hallway track. I got to hang out with [Tim McNamara](https://timclicks.dev/), Rust luminary and social media pal (Tim gave a great keynote and web programming intro); Orhun Parmaksız, who works on [`ratatui`](https://crates.io/crates/ratatui/), a textual user-interface crate that we use a **ton** at Oxide (and we'll be having Orhun on OxF soon!); Paolo Barbolini, who I recognized from the OxF Discord, up well past midnight to catch the live shows; ... to name a few.

Thanks to the organizers, speakers, and to everyone who attended my talk; I hope to make it back in the future.

<iframe class="speakerdeck-iframe" frameborder="0" src="https://speakerdeck.com/player/c9884a78a54b4f2cba79233012ae906e" title="Rustlab 2024: DTrace for Rust... and Everything Else Too" allowfullscreen="true" style="border: 0px; background: padding-box padding-box rgba(0, 0, 0, 0.1); margin: 0px; padding: 0px; border-radius: 6px; box-shadow: rgba(0, 0, 0, 0.2) 0px 5px 40px; width: 100%; height: auto; aspect-ratio: 560 / 315;" data-ratio="1.7777777777777777"></iframe>

<a name="zero">**</a> As someone astutely observed after my talk, the cost isn't actually zero since there's a register move, test, and branch; also there's a bunch of additional program text that can impact caching. All true, but it's the rare inner loop for which the impact would be measurable.
