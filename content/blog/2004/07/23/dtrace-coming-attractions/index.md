---
title: "DTrace: coming attractions"
date: "2004-07-23"
categories: 
  - "dtrace"
---

I haven't been as prolific a blog writer as I like for the last few days because I've been working morning, noon, and night on some pretty cool new stuff for DTrace. Here's a teaser, I promise I'll give you more later when I have it all working:

```
bash-2.05b# dtrace -l -n plockstat100694:::
ID     PROVIDER            MODULE                        FUNCTION NAME
37394 plockstat694         libc.so.1                mutex_lock_queue mutex-block
37395 plockstat694         libc.so.1                     rwlock_lock rw-block
37396 plockstat694         libc.so.1                 __mutex_trylock mutex-acquire
37397 plockstat694         libc.so.1                  __mutex_unlock mutex-release
37398 plockstat694         libc.so.1           mutex_unlock_internal mutex-release
37399 plockstat694         libc.so.1          mutex_trylock_adaptive mutex-spin
37400 plockstat694         libc.so.1                     rwlock_lock rw-block
37401 plockstat694         libc.so.1                 cond_wait_queue mutex-acquire
37402 plockstat694         libc.so.1           _pthread_spin_trylock mutex-acquire
37403 plockstat694         libc.so.1                     lmutex_lock mutex-acquire
37404 plockstat694         libc.so.1                 __mutex_trylock mutex-acquire
37405 plockstat694         libc.so.1                 mutex_lock_impl mutex-acquire
37406 plockstat694         libc.so.1               fast_process_lock mutex-acquire
...

```

In case you need a hint, note that `plockstat` rhymes with [lockstat(1m)](http://docs.sun.com/db/doc/816-5166/6mbb1kq58?a=view)...
