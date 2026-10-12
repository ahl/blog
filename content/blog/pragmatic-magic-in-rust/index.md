---
title: "Pragmatic Magic in Rust"
date: "2026-10-11"
permalink: /2026/10/11/pragmatic-magic-in-rust/
image: images/thumb_tip.jpg
description: "How autoref specialization works, how progenitor uses it to alter client behavior, and the place for magic in code."
---

In Rust, there's this idea of *specialization*, roughly the ability to
*specialize*, say, a trait implementation based on whether or not a type
implements some other trait. Effectively, test whether a type implements a
trait and take action based on that. Amazing! Unfortunately, it's also
persistently out of reach! It's a nightly feature at no risk of stabilization
in the foreseeable future.

Which is why the change in behavior below is kind of surprising!

```rust
// #[derive(Default)]
struct A;

fn main() {
    let d = maybe_get_default_a();
    println!("{}", d.is_some());
}
```

```console
$ cargo run
false
```

(uncommenting the `derive` line...)

```rust
#[derive(Default)]
struct A;

fn main() {
    let d = maybe_get_default_a();
    println!("{}", d.is_some());
}
```

```console
$ cargo run
true
```

Clearly there's some magic afoot! What does the rest of the code look like?

```rust
fn maybe_get_default_a() -> Option<A> {
    MaybeDefaultHelper::<A>::new().maybe_default()
}

trait MaybeDefault<T> {
    fn maybe_default(self) -> Option<T>;
}

struct MaybeDefaultHelper<T>(std::marker::PhantomData<T>);

impl<T> MaybeDefaultHelper<T> {
    fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<T: Default> MaybeDefault<T> for MaybeDefaultHelper<T> {
    fn maybe_default(self) -> Option<T> {
        Some(T::default())
    }
}

impl<T> MaybeDefault<T> for &MaybeDefaultHelper<T> {
    fn maybe_default(self) -> Option<T> {
        None
    }
}
```

Note that there are two `impl` blocks for `MaybeDefault<T>`---one for
`MaybeDefaultHelper<T>` and the other for `&MaybeDefaultHelper<T>`. This relies
on a trick called *autoref specialization*. To make our lives easier, the
compiler will implicitly insert `&` to make code ergonomic.
Autoref-specialization relies on this behavior of the compiler to
*specialize* on the specific trait implementation that is resolved at compile
time.

As he does, David Tolnay has [written about this in
detail](https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md)
if you want to get into it.

If this implicit `&` is ringing a bell, it's because you've probably used a
very similar mechanism without realizing it! Ever seen something like this when
building with the CLI framework `clap`?

```plaintext
error[E0599]: the method `value_parser` exists for reference
`&&&&&&clap::builder::_infer_ValueParser_for<Color>`, but its trait bounds were not satisfied
  --> src/main.rs:12:5
   |
 4 | struct Color {
   | ------------ doesn't satisfy 7 bounds
...
12 |     #[arg(long)]
   |     ^ method cannot be called on `&&&&&&clap::builder::_infer_ValueParser_for<Color>`
   |       due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `Color: ValueEnum`
           which is required by `&&&&&_infer_ValueParser_for<Color>: _impls_ValueEnum`
           `Color: ValueParserFactory`
           which is required by `&&&&&&_infer_ValueParser_for<Color>: _impls_ValueParserFactory`
           `Color: From<OsString>`
           which is required by `&&&&_infer_ValueParser_for<Color>: _impls_From_OsString`
           `Color: From<&'s std::ffi::OsStr>`
           which is required by `&&&_infer_ValueParser_for<Color>: _impls_From_OsStr`
           `Color: From<String>`
           which is required by `&&_infer_ValueParser_for<Color>: _impls_From_String`
           `Color: From<&'s str>`
           which is required by `&_infer_ValueParser_for<Color>: _impls_From_str`
           `Color: FromStr`
           which is required by `_infer_ValueParser_for<Color>: _impls_FromStr`
```

That's right! Autoref-specialization. (Or---more
pedantically---auto**deref**-specialization.) In this case, the generated code
inserts `&&&&&&` and the compiler implicitly *dereferences* (`*`) to find code
that works. Clap uses the mechanism so that a parameter can implement any of a
number of traits that tell it how to convert something that's string-like into
the parameter's type. You can see it in the [clap
code](https://github.com/clap-rs/clap/blob/ccc89b83875474399e9d9d70841163f6319d1a41/clap_builder/src/builder/value_parser.rs#L2622-L2647):

{% raw %}
```rust
macro_rules! value_parser {
    ($name:ty) => {{
        use $crate::builder::impl_prelude::*;
        let auto = $crate::builder::_infer_ValueParser_for::<$name>::new();
        (&&&&&&auto).value_parser()
    }};
}

mod private {
    use super::*;

    #[allow(non_camel_case_types)]
    pub trait _impls_ValueParserFactorySealed {}
    impl<P: ValueParserFactory> _impls_ValueParserFactorySealed for &&&&&&_infer_ValueParser_for<P> {}

    #[allow(non_camel_case_types)]
    pub trait _impls_ValueEnumSealed {}
    impl<E: crate::ValueEnum> _impls_ValueEnumSealed for &&&&&_infer_ValueParser_for<E> {}

    #[allow(non_camel_case_types)]
    pub trait _impls_From_OsStringSealed {}
    impl<FromOsString> _impls_From_OsStringSealed for &&&&_infer_ValueParser_for<FromOsString> where
        FromOsString: From<std::ffi::OsString> + std::any::Any + Send + Sync + 'static
    {
    }
```
{% endraw %}

## Progenitor and its problematic hooks

One of the first things I built at Oxide was progenitor, an OpenAPI to Rust SDK
code generation tool, often invoked as a macro. (I've talked about it
[here](https://www.youtube.com/watch?v=UptsQuOlhBU),
[here](https://share.transistor.fm/s/db64d619), and
[here](https://www.youtube.com/watch?v=1ArFfBT9GBo).) An early addition was the
ability for consumers to modify the generated client e.g. to insert logging. We
did this with `pre_hook` and `post_hook`:

```rust
progenitor::generate_api!(
    spec = "../../openapi/installinator/installinator-1.0.0-c0ed87.json",
    inner_type = slog::Logger,
    pre_hook = (|log: &slog::Logger, request: &reqwest::Request| {
        slog::debug!(log, "client request";
            "method" => %request.method(),
            "uri" => %request.url(),
            "body" => ?&request.body(),
        );
    }),
    post_hook = (|log: &slog::Logger, result: &Result<_, _>| {
        slog::debug!(log, "client response"; "result" => ?result);
    }),
);
```

Consumers specify arbitrary code that's blasted into the generated client.

It's fine, but kind of gross, and quite inscrutable when you get something
wrong.

## Now, with Autoref-specialization

One of our generated clients is the public SDK for interacting with the Oxide
Cloud Computer API, and one of the key consumers of that SDK is our CLI. We
wanted to add more comprehensive debugging output to both to make it easier to
see what API calls were going out and what responses were coming back.

The easy answer was more hooks. It was a mess.

We wanted to let consumers alter the behavior of the generated client with
normal Rust code (e.g. a trait `impl`), but we didn't want to *require* clients
to do so. Changing behavior based on the presence of a trait impl? Sounds
familiar!

We defined an ordinary trait whose methods would be called at appropriate times by generated code:

```rust
pub trait ClientHooks<Inner> {
    /// Runs prior to the execution of the request.
    async fn pre<E>(&self, request: &mut reqwest::Request, info: &OperationInfo)
        -> std::result::Result<(), Error<E>>
    {
        Ok(())
    }

    /// Runs after completion of the request.
    async fn post<E>(&self, result: &reqwest::Result<reqwest::Response>, info: &OperationInfo)
        -> std::result::Result<(), Error<E>>
    {
        Ok(())
    }
}
```

For each operation, the generated code would call those methods:

```rust
            client.pre(&mut request, &info).await?;
            let result = client.execute(request).await;
            client.post(&result, &info).await?;
```

Then the autoref-specialization trick: `Client` needed an implementation of
`ClientHooks` that would satisfy those calls:

```rust
impl ClientHooks<slog::Logger> for &Client {}
```

Note the `&`---it's very important! For the code above the compiler says
(roughly), well, `client.pre` doesn't quite work, but let's try
`(&client).pre`. So clients who aren't interested in modifying behavior work
out of the box.

Clients that do want to do something extra now get to do so with an ordinary
trait implementation that takes precedence over the one for `&Client`.

```rust
impl ClientHooks<slog::Logger> for Client {
    async fn pre<E>(
        &self,
        request: &mut reqwest::Request,
        _info: &progenitor::progenitor_client::OperationInfo,
    ) -> Result<(), progenitor::progenitor_client::Error<E>> {
        slog::debug!(self.inner(), "client request";
            "method" => %request.method(),
            "uri" => %request.url(),
            "body" => ?&request.body(),
        );
        Ok(())
    }

    async fn post<E>(
        &self,
        result: &reqwest::Result<reqwest::Response>,
        _info: &progenitor::progenitor_client::OperationInfo,
    ) -> Result<(), progenitor::progenitor_client::Error<E>> {
        slog::debug!(self.inner(), "client response"; "result" => ?result);
        Ok(())
    }
}
```

(Note that the inner type is configurable; here it's `slog::Logger` but it can
be customized for each generated Client.)

## Magic for Good

I've encountered a lot of disappointing magic in my years of programming,
abstractions keep me an extremely convenient distance from the horrible
details... right up until the moment they stop working or are in the midst of
some pathological bug. Experiences like these drove me away from C++ towards C
early in my career; sure, you had to do everything yourself, but at least it
was all right there to see when it stopped working.

For sure, Rust has magic. Oddly, its most magical feature---macros---are one of
the features I found most tantalizing when I was first learning Rust. And for
sure, there are terrible, spurious, infuriating applications of macros that are
great when they work and diabolical when they don't. As with macros,
autoref-specialization has its pragmatic applications whose ergonomic benefits
far outweigh any inscrutable downsides. Go forth and autoref!
