---
title: "ZFS fundamentals: transaction groups"
date: "2012-12-12"
categories: 
  - "zfs"
tags: 
  - "georgewilson"
  - "mattahrens"
  - "maxbruning"
  - "txg"
  - "zfs"
---

I've [continued](http://dtrace.org/blogs/ahl/2012/11/08/zfs-trivia-metaslabs/) to explore ZFS as I try to understand performance pathologies, and improve performance. A particular point of interest has been the ZFS write throttle, the mechanism ZFS uses to avoid filling all of system memory with modified data. I'm eager to write about the strides we're making in that regard at Delphix, but it's hard to appreciate without an understanding of how ZFS batches data. Unfortunately that explanation is literally nowhere to be found. Back in 2001 I had not yet started working on DTrace, and was talking to Matt and Jeff, the authors of ZFS, about joining them. They had only been at it for a few months; I was fortunate to be in a conference with them as the ideas around transaction groups formulated. Transaction groups are how ZFS batches up chunks of data to be written to disk ("groups" of "transactions"). Jeff stood at the whiteboard and drew the progression of states for transaction groups, from open, accepting new transactions, to quiescing, allowing transactions to complete, to syncing, writing data out to disk. As far as I can tell, that was both the first time that picture had been drawn and the last. If you search for information on ZFS transaction groups you'll find mention of those states... and not much else. The header comment in [usr/src/uts/common/fs/zfs/txg.c](http://src.illumos.org/source/xref/illumos-gate/usr/src/uts/common/fs/zfs/txg.c?r=ce636f8b38e8c9ff484e880d9abb27251a882860) isn't particularly helpful:

```
/*
 * Pool-wide transaction groups.
 */
```

I set out to write a proper description of ZFS transaction groups. I'm posting it here first, and I'll be offering it as a submission to illumos. Many thanks to [Matt Ahrens](https://twitter.com/mahrens1), [George Wilson](https://twitter.com/zfsdude), and [Max Bruning](https://twitter.com/mrbruning) for their feedback.

### ZFS Transaction Groups

ZFS transaction groups are, as the name implies, groups of transactions that act on persistent state. ZFS asserts consistency at the granularity of these transaction groups. Each successive transaction group (txg) is assigned a 64-bit consecutive identifier. There are three active transaction group states: open, quiescing, or syncing. At any given time, there may be an active txg associated with each state; each active txg may either be processing, or blocked waiting to enter the next state. There may be up to three active txgs, and there is always a txg in the open state (though it may be blocked waiting to enter the quiescing state). In broad strokes, transactions -- operations that change in-memory structures -- are accepted into the txg in the open state, and are completed while the txg is in the open or quiescing states. The accumulated changes are written to disk in the syncing state.

### Open

When a new txg becomes active, it first enters the open state. New transactions -- updates to in-memory structures -- are assigned to the currently open txg. There is always a txg in the open state so that ZFS can accept new changes (though the txg may refuse new changes if it has hit some limit). ZFS advances the open txg to the next state for a variety of reasons such as it hitting a time or size threshold, or the execution of an administrative action that must be completed in the syncing state.

### Quiescing

After a txg exits the open state, it enters the quiescing state. The quiescing state is intended to provide a buffer between accepting new transactions in the open state and writing them out to stable storage in the syncing state. While quiescing, transactions can continue their operation without delaying either of the other states. Typically, a txg is in the quiescing state very briefly since the operations are bounded by software latencies rather than, say, slower I/O latencies. After all transactions complete, the txg is ready to enter the next state.

### Syncing

In the syncing state, the in-memory state built up during the open and (to a lesser degree) the quiescing states is written to stable storage. The process of writing out modified data can, in turn modify more data. For example when we write new blocks, we need to allocate space for them; those allocations modify metadata (space maps)… which themselves must be written to stable storage. During the sync state, ZFS iterates, writing out data until it converges and all in-memory changes have been written out. The first such pass is the largest as it encompasses all the modified user data (as opposed to filesystem metadata). Subsequent passes typically have far less data to write as they consist exclusively of filesystem metadata.

To ensure convergence, after a certain number of passes ZFS begins overwriting locations on stable storage that had been allocated earlier in the syncing state (and subsequently freed). ZFS usually allocates new blocks to optimize for large, continuous, writes. For the syncing state to converge however it must complete a pass where no new blocks are allocated since each allocation requires a modification of persistent metadata. Further, to hasten convergence, after a prescribed number of passes, ZFS also defers frees, and stops compressing.

In addition to writing out user data, we must also execute synctasks during the syncing context. A synctask is the mechanism by which some administrative activities work such as creating and destroying snapshots or datasets. Note that when a synctask is initiated it enters the open txg, and ZFS then pushes that txg as quickly as possible to completion of the syncing state in order to reduce the latency of the administrative activity. To complete the syncing state, ZFS writes out a new uberblock, the root of the tree of blocks that comprise all state stored on the ZFS pool. Finally, if there is a quiesced txg waiting, we signal that it can now transition to the syncing state.

### What else?

Please let me know if you have suggestions for how to improve the descriptions above. There's more to be written on the specifics of the implementation, transactions, the DMU, and, well, ZFS in general. One thing that I'd note is that Matt mentioned to me recently that were he starting from scratch, he might eliminate the quiescing state. I didn't understand fully until I researched the subsystem. Typically transactions take a very brief amount of time to "complete", time measured by CPU latency as opposed, say, to I/O latency. Had the quiescing phase been merged into the syncing phase, the design would be slightly simpler, and it would eliminate the mostly idle intermediate phase where a bunch of dirty data can sit in memory relatively idle.

Next I'll write about the ZFS write throttle, it's various brokenness, and our ideas for how to fix it.
