---
title: "I Love Go; I Hate Go"
date: "2016-08-01"
categories:
  - "software"
permalink: /2016/08/02/i-love-go-i-hate-go/
---

I liked Go right away. It was close enough to C and Java to be instantly familiar, the examples and tutorials were straightforward, and I was quickly writing real code. I've wanted to learn Go since its [popularity was surging](http://redmonk.com/sogrady/2016/07/20/language-rankings-6-16/#advanced_iframe) few years ago. In no danger of being judged an early adopter, I happily found a great project that—as it happened—had to be in Go (more in a future post).

### I Love Go

My first priority was not looking stupid. The folks I’d be doing this for are actual Go developers; I wanted my code to fit in without imposing with too many questions. They had no style guide. Knowing that my 80-column width sensibilities expose an unattractive nostalgia, I went looking for a max line length. There wasn’t one. Then I discovered `gofmt`, the simple tool that Go employs to liberate developers from the tyranny of stylistic choice. It takes Go code and spits it back out in the One True Style. I love it. I was raised in an engineering culture with an [exacting style guide](https://www.cis.upenn.edu/~lee/06cse480/data/cstyle.ms.pdf), but any style guide has gaps. Factions form; style-originalists face off against those who view (incorrectly!) the guide as a living document. I updated my decades-old [.vimrc](https://github.com/ahl/dotfiles/blob/master/.vimrc) to `gofmt` on save. These Go tyrants were feeling like my kind of tyrant.

One of the things that turned me off of C++ (98, 11, and 14) is its increasing amount of magic. Go is comparatively straightforward. When I reach for something I find that it's where I expect it to be. Libraries and examples aren’t mysterious. Error messages are non-mysterious (other than quickly resolved confusion about methods with “[pointer receivers](http://stackoverflow.com/questions/33936081/golang-method-with-pointer-receiver)”). Contrast this with [Rust whose errors read like mind-bindingly inscrutable tax forms](http://dtrace.org/blogs/ahl/2015/06/22/first-rust-program-pain/).

Go’s containment-based inheritance is easy enough to reason about. Its interfaces are similarly no-nonsense and pragmatic. You don’t have to define a structure as implementing an interface. You can use an interface to describe anything that implements it. Simple. This is particularly useful for testing. In a messy environment with components beyond your control you can apply interfaces to other people’s code, and then mock it out.

The toolchain is, again, simple to use—the benefit of starting from scratch—and makes for fast compilation, quick testing, and easy integration with a good-sized ecosystem. I stopped worrying about dependencies, rebuilding, etc. knowing that `go run` would find errors wherever I had introduced them and do so remarkably quickly.

### I Hate Go

Go is opinionated. Most successful products have that strong sense of what they are and what they aren’t; Go draws as sharp a line as any language. I was seduced by its rightheadedness around style, but with anything or anyone that opinionated you’ll find some of those opinions weird and others simply off-putting.

Reading the official documentation, I found myself in the middle of a section prefixed with the phrase “if GOOS is set to plan9”. Wow. I’m a few standard deviations from the norm in terms of being an OS nerd, but I’ve never even seen Plan 9. I knew that the Plan 9 folks got the band back together to create Go; it's great that their pop audiences don’t dissuade them from playing off their old B-sides. Quirky, but there’s nothing wrong with that.

I wanted to check an invariant; how does Go do assertions? [Fuck you, you’re a bad programmer.](https://golang.org/doc/faq#assertions) That’s how. The Go authors feel so strongly about asserts being typically misused that they refuse to provide them. So folks use one (or more) of some workable libraries.

I created an infinite recursion, overflowing the stack. Go produces the [first 100 stack frames and that’s it](https://golang.org/src/runtime/traceback.go#L581). Maybe you can change that, but I couldn’t figure out how. (“go stackoverflow” is about the most useless thing you can search for; chapeau, [Go](https://twitter.com/ahl/status/753254488123727873) and Stackoverflow respectively.) I could be convinced that I only want 100 stack frames, but not the last 100, not the same 4 over and over again. I ended up limiting the stack size with [`runtime.debug.SetMaxStack()`](https://golang.org/pkg/runtime/debug/#SetMaxStack), Goldilocks-ing it between too big to catch the relevant frames and too small to allow for normal operation.

I tried using other tools (ahem, DTrace) to print the stack, but, of course, the Go compiler omits frame pointers rendering the stacks unobservable to debuggers. Ditto arguments due to ABI-non-compliant calling conventions, but that’s an aside. The environment variable `GOEXPERIMENT=framepointer` is supposed to compile with frame pointers, but it was a challenge to rebuild the world. All paths seemed to lead me to my former-colleague’s scathing synopsis: [Golang is Trash](http://dtrace.org/blogs/wesolows/2014/12/29/golang-is-trash/).

As fun as it is to write code in Go, debugging in Go is no fun at all. I may well just be ignorant of the right tooling. But there sure isn’t a debugger with the simple charm of `go build` for compilation, `go test` for testing, or `go run` for execution.

### Immaturity and Promise

Have you ever been in a relationship where minor disagreements immediately escalated to “maybe we should break up?” The Go documentation even seems ready to push you to some other language at the slightest affront. Could I have asserts? Sure, if you’re a bad programmer. Perhaps ABI compliance has its merits? I’m sure you could find that in some other language. Could you give me the absolute value of this int? Is something wrong with your 'less than' key?

I’m sure time will, as it tends to, bring pragmatism. I appreciate that Go does have strong opinions (it’s just hard to remember that when I disagree with them). Weak opinions are what turn languages into unreadable mishmashes of overlapping mechanism. My favorite example of this is Perl. My first real programming job was in Perl. I was the most avid reader teenage of the Perl llama and camel books. Ask me about my chat with Larry Wall, Perl’s creator, if you see a beer in my hand. [In an interview](https://www.amazon.com/Masterminds-Programming-Conversations-Creators-Languages/dp/0596515170), Larry said, "In Perl 6, we decided it would be better to fix the language than fix the user”. Contrast this with [Go’s statement on assertions](https://golang.org/doc/faq#assertions):

> Go doesn't provide assertions. They are undeniably convenient, but our experience has been that programmers use them as a crutch to avoid thinking about proper error handling and reporting.

Perl wants to be whatever the user wants to be. The consumate pleaser. Go sees the user as flawed and the strictures of the language as the cure. It’s an authoritarian, steadfast in its ideals, yet too sensitive to find compromise (sound like anyone we all know?).

Despite my frustrations I really enjoy writing code in Go. It’s clean and supported by a great community and ecosystem. I’m particularly heartened that [Go 1.7 will compile with frame pointers by default](https://github.com/golang/go/issues/15840). Diagnosing certain types of bugs is still a pain in the neck, but I’m sure it will improve and I’ll see where I can pitch in.
