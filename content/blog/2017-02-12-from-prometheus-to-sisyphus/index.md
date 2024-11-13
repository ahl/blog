---
title: "From Prometheus to Sisyphus"
date: "2017-02-12"
categories: 
  - "dtrace"
---

When Apple announced their new file system, APFS, in June, I hustled to be in the front row of the WWDC presentation, questions with the presenters, and then the open Q&A session. I took a week to write up my notes which turned into as 12 page behemoth of a blog post — longer than my college thesis. Despite [reassurances from the tweeps](https://twitter.com/ahl/status/743258869523087361), I was sure that the blog post was an order of magnitude longer than the modern attention span. I was wrong; so wrong that Ars Technica wanted to republish the blog post. [Never underestimate the interest in all things Apple](https://developers.slashdot.org/story/08/01/22/2156244/apple-crippled-its-dtrace-port).

In that piece I left one big thread dangling. Apple shipped APFS as a technology preview, but they left out access to one of the biggest new features: snapshots. Digging around I noticed that there was a curiously named new system call, “fs\_snapshot”, but explicitly didn’t investigate: the post was already too long (I thought), I had spent enough time on it, and someone else (surely! surely?) would want to pull on that thread.

#### Slow News Day

Every so often I’d poke around for APFS news, but there was very little new. Last month folks discovered that [APFS was coming to iOS](https://arstechnica.com/apple/2017/01/ios-10-3-will-be-apples-first-update-to-convert-storage-to-apfs/) sooner rather than later. But there wasn’t anything new to play with or any revelations on how APFS would work.

I would search for “APFS snapshots”, “fs\_snapshot”, anything I could think of to see if anyone had figured out how to make snapshots work on APFS. Nothing.

A few weekends ago, I decided to yank on that thread myself.

#### Prometheus

I started from the system call, wandered through Apple’s open source kernel, leaned heavily on DTrace, and eventually figured it out. Apple had shipped snapshots in APFS, they just hadn’t made it easy to get there. The folks at Ars were excited for a follow-up, and my investigation turned into this: [“Testing out snapshots in Apple’s next-generation APFS file system.”](https://arstechnica.com/apple/2017/02/testing-out-snapshots-in-apples-next-generation-apfs-file-system/)

Snapshots were there; the APIs were laid bare; I was going to bring fire to all the Mac fans; [John Siracusa](https://arstechnica.com/apple/2015/04/after-fifteen-years-ars-says-goodbye-to-john-siracusas-os-x-reviews/) and [Andy Ihnatko](http://www.macworld.com/author/Andy-Ihnatko/) would carry me on their shoulders down the streets of the Internet.

#### Sisyphus

On the eve that this new piece was about to run I was nervously scrolling through Twitter as I took the bus home from [work](http://www.transposit.com/). Now that I had invested the time to research and write-up APFS snapshots I didn’t want someone else beating me to the punch.

Then I found this and my heart dropped:

<blockquote class="twitter-tweet" data-lang="en"><p lang="en" dir="ltr">Slides from the "Storing our digital lives: Mac filesystems from MFS to APFS" session at MacTech Conference 2016: <a href="https://t.co/uJJuqLL8n9">https://t.co/uJJuqLL8n9</a></p>— Rich Trouton (@rtrouton) <a href="https://twitter.com/rtrouton/status/799339130643615744">November 17, 2016</a></blockquote> 

Skim past the craziness of MFS and the hairball of HFS, and start digging through the APFS section. Slide 49, “APFS Snapshots” and there it is “apfs\_snapshot” — not a tool that anyone laboriously reverse engineered, deciphering system calls and semi-published APIs — a tool shipped from Apple and included in macOS by default. F — .

Apple had secreted this utility away (along with some others) in `/System/Library/Filesystems/apfs.fs/Contents/Resources/`

#### What To Do?

The article that was initially about a glorious act of discovery had become an article about the reinvention of the wheel. Conversely, vanishingly few people would recognize this as rediscovery since the apfs\_snapshot tool was so obscure (9 hits on Google!).

We toned down the already modest chest-thumping and published the article this morning to a pretty nice response so far. I might have happier as an FAKE NEWS Prometheus, blissfully unaware of the pre-existence of fire, but I would have been mortified when the inevitable commenter, one of the few who had used apfs\_snapshot, crushed me with my own boulder.
