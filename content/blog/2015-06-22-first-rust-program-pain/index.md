---
title: "First Rust Program Pain (So you can avoid it...)"
date: "2015-06-22"
categories: 
  - "software"
tags: 
  - "rust"
  - "scrabble"
---

![](http://www.rust-lang.org/logos/rust-logo-blk.svg)Like many programmers I like to try out new languages. After lunch with Alex Crichton, one of the Rust contributors, I started writing my favorite program in Rust. Rust is a “safe” systems language that introduces concepts of data ownership and mutability to semantically prevent whole categories of problems. It’s primarily developed at Mozilla Research in service of a next generation rendering engine, and while I presume that the name is a poke in the eye of Google’s Chrome, no one was brave enough to confirm that lest their next Uber ride reroute them to [Bagram](https://en.wikipedia.org/wiki/Black_jail).

My standard “hello world” is a anagrammer / Scrabble cheater. Why? In most languages you can get it done in a few dozen lines of code, and it uses a variety of important language and library features: lists, maps, file IO, console IO, strings, sorting, etc. Rust is great, interesting in the way that I found objected-oriented or functional programming interesting when I first learned about them. It's notions of data ownership, borrowing, and mutability I think lead to some of the same aha moments as closures for example. I found Rust to be quirky enough though that I thought I might be able to save others the pain of their first program, advancing them to the glorious, safe efficiency of their second by relating my experience.

So with the help of Stack Overflow I wrote the first chunk:

```
  1 use std::fs::File;
  2 use std::path::Path;
  3 use std::error::Error;
  4 use std::io::BufReader;
  5 use std::io::BufRead;
  6
  7 fn main() {
  8         let path = Path::new("../word.lst");
  9         let file = match File::open(&path) {
 10                 Err(why) => panic!("failed to open {}: {}", path.display(),
 11                     Error::description(&why)),
 12                 Ok(f) => f,
 13         };
 14
 15         let mut b = BufReader::new(file);
 16         let mut s = String::new();
 17
 18         while b.read_line(&mut s).is_ok() {
 19                 println!("{}", s);
 20         }
 21 }

```

So far so good? Well I ran it and it didn’t seem to be terminating...

```
$ time ./scrabble >/dev/null
<time passes>

```

What’s happening?

```
$ ./scrabble | head
aa

aa
aah

aa
aah
aahed

aa
thread '<main>' panicked at 'failed printing to stdout: Broken pipe (os error 32)', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libstd/io/stdio.rs:404

```

Okay — first lesson: String::clear(). As the documentation clearly states, BufReader::read\_line() appends to an existing string; my own expectations and preconceptions are beside the point.

```
  1 use std::fs::File;
  2 use std::path::Path;
  3 use std::error::Error;
  4 use std::io::BufReader;
  5 use std::io::BufRead;
  6
  7 fn main() {
  8         let path = Path::new("../word.lst");
  9         let file = match File::open(&path) {
 10                 Err(why) => panic!("failed to open {}: {}", path.display(),
 11                     Error::description(&why)),
 12                 Ok(f) => f,
 13         };
 14
 15         let mut b = BufReader::new(file);
 16         let mut s = String::new();
 17
 18         while b.read_line(&mut s).is_ok() {
 19                s.pop();
 20                 println!("{}", s);
 21                 s.clear();
 22         }
 23 }

```

Better? Yes:

```
$ ./scrabble | head
aa
aah
aahed
aahing
aahs
aal
aalii
aaliis
aals
aardvark
thread '<main>' panicked at 'failed printing to stdout: Broken pipe (os error 32)', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libstd/io/stdio.rs:404

```

Correct? No:

```
$ time ./scrabble >/dev/null
<time passes>

```

It turns out that BufReader::read\_line() indeed is\_ok() even at EOF. Again, documented but—to me—counter-intuitive. And it turns out that this is a [somewhat divisive topic](https://github.com/rust-lang/rust/issues/22588). No matter; how about something else? Well it works, but the ever persnickety rustc finds ‘while true’ too blue-collar of a construct:

```
$ rustc scrabble.rs
scrabble.rs:18:2: 25:3 warning: denote infinite loops with loop { ... }, #[warn(while_true)] on by default
scrabble.rs:18     while true {
scrabble.rs:19         if !b.read_line(&mut s).is_ok() || s.len() == 0 {
scrabble.rs:20             break;
scrabble.rs:21         }
scrabble.rs:22         s.pop();
scrabble.rs:23         println!("{}", s);
                ...

```

Trying to embrace the fastidious methodology (while ever temped to [unsafe](https://doc.rust-lang.org/book/unsafe.html)\-and-let-execution-be-the-judge) I gave up on read\_line() and its controversial EOF and error semantics to try out BufReader::lines():

```
 18         for s in b.lines() {
 19                 println!("{}", s);
 20         }

```

```
$ rustc scrabble2.rs
scrabble2.rs:18:18: 18:19 error: the trait `core::fmt::Display` is not implemented for the type `core::result::Result<collections::string::String, std::io::error::Error>` [E0277]
scrabble2.rs:18         println!("{}", s);
                                       ^
note: in expansion of format_args!
<std macros>:2:25: 2:58 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
scrabble2.rs:18:3: 18:21 note: expansion site
scrabble2.rs:18:18: 18:19 note: `core::result::Result<collections::string::String, std::io::error::Error>` cannot be formatted with the default formatter; try using `:?` instead if you are using a format string
scrabble2.rs:18         println!("{}", s);
                                       ^
note: in expansion of format_args!
<std macros>:2:25: 2:58 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
scrabble2.rs:18:3: 18:21 note: expansion site
error: aborting due to previous error

```

Okay; that was apparently very wrong. The BufReader::lines() iterator gives us Result<String>s which we need to unwrap(). No problem.

```
 18         for line in b.lines() {
 19                 let s = line.unwrap();
 20                 println!("{}", s);
 21         }

```

```
scrabble.rs:15:6: 15:11 warning: variable does not need to be mutable, #[warn(unused_mut)] on by default
scrabble.rs:15     let mut b = BufReader::new(file);

```

Fine, rustc, you’re the boss. Now it's simpler and it’s cranking:

```
  1 use std::fs::File;
  2 use std::path::Path;
  3 use std::error::Error;
  4 use std::io::BufReader;
  5 use std::io::BufRead;
  6 use std::collections::HashMap;
  7
  8 fn main() {
  9         let path = Path::new("../word.lst");
 10         let file = match File::open(&path) {
 11                 Err(why) => panic!("failed to open {}: {}", path.display(),
 12                     Error::description(&why)),
 13                 Ok(f) => f,
 14         };
 15
 16         let b = BufReader::new(file);
 17
 18         for line in b.lines() {
 19                 let s = line.unwrap();
 20                 println!("{}", s);
 21         }
 22 }

```

Now let’s build up our map. We’ll create a map from the sorted characters to the list of anagrams. For that we’ll use matching, another handy construct.

```
 23                 let mut v: Vec<char> = s.chars().collect();
 24                 v.sort();
 25                 let ss: String = v.into_iter().collect();
 26
 27                 match dict.get(&ss) {
 28                         Some(mut v) => v.push(s),
 29                         _ => {
 30                                 let mut v = Vec::new();
 31                                 v.push(s);
 32                                 dict.insert(ss, v);
 33                         },
 34                 }

```

What could be simpler? I love this language! But not so fast...

```
scrabble.rs:28:19: 28:20 error: cannot borrow immutable borrowed content `*v` as mutable
scrabble.rs:28             Some(mut v) => v.push(s),
                                           ^
scrabble.rs:32:5: 32:9 error: cannot borrow `dict` as mutable because it is also borrowed as immutable
scrabble.rs:32                 dict.insert(ss, v);
                                ^~~~
scrabble.rs:27:9: 27:13 note: previous borrow of `dict` occurs here; the immutable borrow prevents subsequent moves or mutable borrows of `dict` until the borrow ends
scrabble.rs:27         match dict.get(&ss) {
                              ^~~~
scrabble.rs:34:4: 34:4 note: previous borrow ends here
scrabble.rs:27         match dict.get(&ss) {
...
scrabble.rs:34         }
                        ^
error: aborting due to 2 previous errors

```

This is where in C I’d start casting away const. Not an option here. Okay, but I remember these notions of ownership, borrowing, and mutability as concepts early in the Rust overview. At the time it seemed like one of those explanations of git that sounds like more of a functional analysis of cryptocurrency. But perhaps there were some important nuggets in there...

Mutability, check! The Hashmap::get() yielded an immutable borrow that would exist for as long as its return value was in scope. Easily solved by changing it to a get\_mut():

```
scrabble.rs:32:5: 32:9 error: cannot borrow `dict` as mutable more than once at a time
scrabble.rs:32                 dict.insert(ss, v);
                               ^~~~
scrabble.rs:27:9: 27:13 note: previous borrow of `dict` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `dict` until the borrow ends
scrabble.rs:27         match dict.get_mut(&ss) {
                             ^~~~
scrabble.rs:34:4: 34:4 note: previous borrow ends here
scrabble.rs:27         match dict.get_mut(&ss) {
...
scrabble.rs:34         }
                       ^
error: aborting due to previous error

```

Wrong again. Moving me right down the Kübler-Ross model from anger into bargaining. You’re saying that I can’t mutate it because I can already mutate it? What do I have, rustc, that you want? How about if I pull the insert() out of the context of that get\_mut()?

```
 27                 let mut bb = false;
 28
 29                 match dict.get_mut(&ss) {
 30                         Some(mut v) => v.push(s),
 31                         _ => {
 32                                 bb = true;
 33                         },
 34                 }
 35                 if bb {
 36                         let mut v = Vec::new();
 37                         v.push(s);
 38                         dict.insert(ss, v);
 39                 }

```

Inelegant, yes, but Rust was billed as safe-C, not elegant-C, right?

```
scrabble.rs:37:11: 37:12 error: use of moved value: `s`
scrabble.rs:37             v.push(s);
                                  ^
scrabble.rs:30:26: 30:27 note: `s` moved here because it has type `collections::string::String`, which is non-copyable
scrabble.rs:30             Some(mut v) => v.push(s),
                                                 ^
error: aborting due to previous error

```

So by pushing the anagram into the list at line 30 we lost ownership, and even though that definitely didn’t happen in the case of us reaching line 37, rustc isn’t having it. Indeed there doesn’t seem to be a way to both get an existing value and to insert a value in one lexical vicinity. At this point I felt like I was in some bureaucratic infinite loop, doomed to shuttle to and fro between windows at the DMV, always holding the wrong form. Any crazy person will immediately be given an mutable map, but asking for a mutable map immediately [classifies you a sane](https://en.wikipedia.org/wiki/Catch-22_(logic)).

After walking away for day to contemplate, here’s the compromise I came to:

```
 27                 if dict.contains_key(&ss) {
 28                         dict.get_mut(&ss).unwrap().push(s);
 29                 } else {
 30                         let mut v = Vec::new();
 31                         v.push(s);
 32                         dict.insert(ss, v);
 33                 }

```

And everyone was happy! But it turns out that there’s an even Rustier way of doing this (thanks to Delphix intern, John Ericson) with a very specific API:

```
                let mut v = dict.entry(sort_str(&s)).or_insert(Vec::new());
                v.push(s);

```

This is starting to look at lot less like safe C and a lot more like the stacking magic of C++. No matter; I’m just trying to cheat at Scrabble, not debate philosophy. Now that I’ve got my map built, let’s prompt the user and do the lookup. We’ll put the string sorting logic into a function:

```
  8 fn sort_str(s: String) -> String {
  9         let mut v: Vec<char> = s.chars().collect();
 10         v.sort();
 11         let ss: String = v.into_iter().collect();
 12         ss
 13 }

```

```
scrabble.rs:32:36: 32:37 error: use of moved value: `s`
scrabble.rs:32             dict.get_mut(&ss).unwrap().push(s);
                                                           ^
scrabble.rs:29:21: 29:22 note: `s` moved here because it has type `collections::string::String`, which is non-copyable
scrabble.rs:29         let ss = sort_str(s);
                                         ^
scrabble.rs:35:11: 35:12 error: use of moved value: `s`
scrabble.rs:35             v.push(s);
                                  ^
scrabble.rs:29:21: 29:22 note: `s` moved here because it has type `collections::string::String`, which is non-copyable
scrabble.rs:29         let ss = sort_str(s);
                                         ^
error: aborting due to 2 previous errors

```

This was wrong because we need to pass s as a reference or else its borrowed and destroyed; this needs to happen both in the function signature and call site.

```
  8 fn sort_str(s: &String) -> String {
  9         let mut v: Vec<char> = s.chars().collect();
 10         v.sort();
 11         let ss: String = v.into_iter().collect();
 12         ss
 13 }

```

As an aside I’d note how goofy I think it is that the absence of a semi-colon denotes function return. And that using an explicit return is sneered at as “un-idiomatic”. I’ve been told that this choice enables deeply elegant constructs with closures and that I’m simply behind the times. Fair enough. Now we’ll read the user-input:

```
 41         for line in stdin().lock().lines() {
 42                 let s = line.unwrap();
 43
 44                 match dict.get(&sort_str(&s)) {
 45                         Some(v) => {
 46                                 print!("anagrams for {}: ", s);
 47                                 for a in v {
 48                                         print!("{} ", a);
 49                                 }
 50                                 println!("");
 51                         },
 52                         _ => println!("no dice"),
 53                 }
 54         }

```

```
scrabble.rs:43:14: 43:21 error: borrowed value does not live long enough
scrabble.rs:43     for line in stdin().lock().lines() {
                               ^~~~~~~
scrabble.rs:43:2: 57:2 note: reference must be valid for the destruction scope surrounding statement at 43:1...
scrabble.rs:43     for line in stdin().lock().lines() {
scrabble.rs:44         let s = line.unwrap();
scrabble.rs:45
scrabble.rs:46         match dict.get(&sort_str(&s)) {
scrabble.rs:47             Some(v) => {
scrabble.rs:48                 print!("anagrams for {}: ", s);
               ...
scrabble.rs:43:2: 57:2 note: ...but borrowed value is only valid for the statement at 43:1
scrabble.rs:43     for line in stdin().lock().lines() {
scrabble.rs:44         let s = line.unwrap();
scrabble.rs:45
scrabble.rs:46         match dict.get(&sort_str(&s)) {
scrabble.rs:47             Some(v) => {
scrabble.rs:48                 print!("anagrams for {}: ", s);
               ...
scrabble.rs:43:2: 57:2 help: consider using a `let` binding to increase its lifetime
scrabble.rs:43     for line in stdin().lock().lines() {
scrabble.rs:44         let s = line.unwrap();
scrabble.rs:45
scrabble.rs:46         match dict.get(&sort_str(&s)) {
scrabble.rs:47             Some(v) => {
scrabble.rs:48                 print!("anagrams for {}: ", s);
               ...
error: aborting due to previous error

```

Okay! Too cute! Got it. Here’s the final program with some clean up here and there:

```
  1 use std::fs::File;
  2 use std::path::Path;
  3 use std::error::Error;
  4 use std::io::BufReader;
  5 use std::io::BufRead;
  6 use std::collections::HashMap;
  7 use std::io::stdin;
  8
  9 fn sort_str(s: &String) -> String {
 10         let mut v: Vec<char> = s.chars().collect();
 11         v.sort();
 12         v.into_iter().collect()
 13 }
 14
 15 fn main() {
 16         let path = Path::new("../word.lst");
 17         let file = match File::open(&path) {
 18                 Err(why) => panic!("failed to open {}: {}", path.display(),
 19                     Error::description(&why)),
 20                 Ok(f) => f,
 21         };
 22
 23         let b = BufReader::new(file);
 24
 25         let mut dict: HashMap<String, Vec<String>> = HashMap::new();
 26
 27         for line in b.lines() {
 28                 let s = line.unwrap();
 29                 dict.entry(sort_str(&s)).or_insert(Vec::new()).push(s);
 30         }
 31
 32         let sin = stdin();
 33
 34         for line in sin.lock().lines() {
 35                 let s = line.unwrap();
 36
 37                 match dict.get(&sort_str(&s)) {
 38                         Some(v) => {
 39                                 print!("anagrams for {}: ", s);
 40                                 for a in v {
 41                                         print!("{} ", a);
 42                                 }
 43                                 println!("");
 44                         },
 45                         _ => println!("no dice"),
 46                 }
 47         }
 48 }

```

## Lessons

Rust is not Python. I knew that Rust wasn’t Python… or Java, or Perl, etc. But it still took me a while to remember and embrace that. You have to think about memory management even when you get to do less of it explicitly. For programs with messy notions of data ownership I can see Rust making for significantly cleaner code, easier to understand, and more approachable to new engineers. The concepts of ownership, borrowing, and mutability aren’t “like” anything. It took the mistakes of that first program to teach me that. Hopefully you can skip straight to your second Rust program.

## Postscript

Before I posted this I received some suggestions from my colleagues at Delphix about how to improve the final code. I resolved to focus on the process—the journey if you will—rather than the result. That said I now realize that I was myself a victim of learning from some poor examples (from stack overflow in particular). There's nothing more durable than poor but serviceable examples; we've all seen inefficient copy/pasta littered throughout a code base. So with the help again from John Ericson and the Twitterverse at large here's my final version as a github gist (if I was going to do it over again I'd stick each revision in github for easier navigation). Happy copying!

<script src="https://gist.github.com/ahl/8b7831a7ce601b461b45.js"></script>
