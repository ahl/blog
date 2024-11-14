---
title: "Flash news I wish I could read"
date: "2011-07-01"
categories: 
  - "flash-2"
tags: 
  - "compellent"
  - "emc"
  - "flash"
  - "hsp"
  - "netapp"
  - "oracle"
  - "ssd"
  - "sun"
  - "theregister"
  - "zfs"
---

For a short while, I ran the flash memory strategy at Sun and then Oracle, so I still keep my ear to the ground regarding flash news. That news is often frustratingly light — journalists in the space who are fully capable of providing analysis end up brushing the surface. With a tip of the hat to the [FJM](http://www.firejoemorgan.com/) crew, here's my commentary on a [recent article](http://www.theregister.co.uk/2011/07/01/netapp_hybrid_aggregates/).

> NetApp has Hybrid Aggregate drives coming, with data moved automatically in real time between flash located next to the spinning disks. The company now says that this is a better technology than PCIe flash approaches.

Sounds interesting. NetApp had previously stacked its chips on a PCIe approach for flash called the performance acceleration module (PAM); I read about it in the [same publication](http://www.theregister.co.uk/2008/09/26/netapp_pam_to_get_ssd/). This apparent change of strategy is significant, and I wish that the article would have explored the issue, but it was never mentioned.

> NetApp, presenting at an Analyst Day event in New York on 30 June, said that having networked storage move as it were into the host server environment was disadvantageous. This was according to Stifel Nicolaus analyst Aaron Rakers.

1\. So is this a quote from NetApp or a quote from an analyst or a quote from NetApp quoting an analyst? I'm confused.

2\. This is a dense and interesting statement so allow **me** to unpack it. Moving storage to the host server is code for Fusion-io. These guys make a flash-laden PCIe card that you put in your compute node for super-fast local data access, and they connect a bunch of them together with an IB backplane to share the contents of different cards between hosts. They recently went public, and customers love the performance they offer over traditional SANs. I assume the term "[disadvantageous](http://www.google.com//finance?chdnp=1&chdd=1&chds=1&chdv=1&chvs=maximized&chdeh=0&chfdeh=0&chdet=1309550400000&chddm=6256&chls=IntervalBasedLine&cmpto=NASDAQ:NTAP&cmptdms=0&q=NYSE:FIO&ntsp=0)" was left intentionally vague as those being disadvantaged may be NTAP shareholders rather than customers implementing such a solution.

> Manish Goel, NetApp's product ops EVP, said SSDs used as hard disk drive replacements were not as interesting as using flash at the disk layer in a Hybrid Aggregate drive approach – and this was coming.

An Aggregate is the term NetApp uses for a collection of drives. A Hybrid Aggregate — presumably — is some new thing that mixes HDDs and SSDs. Maybe it's like Sun's [hybrid storage pool](http://dtrace.org/blogs/ahl/2008/07/01/hybrid-storage-pools-in-cacm/). I would have liked to see Manish Goel's statement vetted or explained, but that's all we get.

> Flash Cache in the controller is a straightforward array read I/O accelerator. PCIe flash in host servers is a complementary technology but will not decentralise the storage market and move networked storage back into the host servers.

Is this still the NetApp announcement or is this back to the journalism? It's a new paragraph so I guess it's the latter. Fusion-io will be happy to learn that it only took a couple of lines to be upgraded from "disadvantageous" to "complementary". And you may be interested to know why NetApp says that host-based flash is complementary. There's a vendor out there working with NetApp on a host-based flash PCIe card that NetApp will treat as part of its caching tier, pushing data to the card for fast access by the host. I'd need to dig up my notes from the many vendor roadmaps I saw to recall who is building this, but in the context of a public blog post it's probably better that I don't.

> NetApp has a patent in this Hybrid Aggregate disk drive area called "Mechanisms for moving data in a Hybrid Aggregate".
> 
> ...

I won't bore you by reposting the except from the patent, but the broad language of the patent does recall to mind the many recent [invalidated NetApp patents...](http://www.theregister.co.uk/2008/10/07/sun_gets_netapp_patent_invalidated/)

> Surely this is what we all understand as auto-placement of data in a virtual storage pool comprising SSD and fast disk tiers, such as Compellent's block-level Data Progression? Not so, according to a person close to the situation: "It's much more automatic, real-time and granular. Compellent needs policies and is not real-time. \[NetApp\] will be automatic and always move data real-time, rather than retroactively."

What could have followed this — but didn't — was a response from a representative from Compellent or someone familiar with their technology. Compellent, EMC, Oracle, and others all have strategies that involve mixing flash memory with conventional hard drives. It's the rare article that discusses those types of connections. Oracle's [ZFS](http://dtrace.org/blogs/ahl/2010/11/19/delphix-welcomes-matt-ahrens-and-george-wilson/)\-[products](http://blogs.oracle.com/fishworks/entry/all_together_now) uses flash as a caching tier, automatically populating it with useful data. Compellent has a clever technique of moving data between storage tiers seamlessly — and customers seem to love it. EMC just [hucks a bunch of SSDs](http://www.enterprisestorageforum.com/hardware/news/article.php/3833141/STEC-Has-EMC-to-Thank-for-Its-Rapid-Growth.htm) into an array — and customers seem to grin and bear it. NetApp's approach? It's hard to decipher what it would mean to "move data in real-time, rather than retroactively." Does that mean that data is moved when it's written and then never moved again? That doesn't sound better. My guess is that NetApp's approach is very much like Compellent's — something they should be touting rather than parrying. And I'd love to read **that** article.
