---
title: "Typify 2 WIP: A better JSON Schema to Rust compiler"
date: "2026-05-10"
permalink: /2026/05/10/typify2-wip/
---

JSON Schema isn't a great way to describe data... but it's both a popular and
pervasive way to describe data. For the past several years I've been working on
a Rust crate `typify` that compiles JSON Schema into Rust types. In a
[previous](/2024/01/22/rust-and-json-schema/)
post, I wrote about some of the frustrating complexities of JSON Schema, the
evolving specification, and my own increasing familiarity with its nuances.
Since then I started a re-architected version of typify; in this post I'll talk
about why and how I redesigned it, and provide an update for (increasingly
impatient) typify users.


# Building the wrong thing first

The only way I know how to build the right thing is to build the wrong thing
first. Maybe that's a bit hyperbolic, so: the only way to build something
better is to first build something worse. I think that's more or less universal
among software engineers I've worked with. Some are better at thinking through; others need to feel out the code (I'm much more in that latter camp); but no one gets it all right the first time.

Typify is the apotheosis of this: the first thing was definitely going to be
wrong. Why? JSON Schema is big and hairy, and if I had tried to build the
grand unified theory of everything I would have never gotten anything working.
Fortunately, I had a bunch of concrete constraints.

The motivating use case was as a subcomponent of an OpenAPI SDK generator
(`progenitor`). OpenAPI 3.0.x defines its types in a way that resembles an
older JSON Schema draft. It's intentionally similar, intentionally different,
and unintentionally annoying. For example, JSON Schema had/has/will have a way
to describe a schema as having `null` as a valid value; rather than using that,
OpenAPI shoved in a new `nullable` keyword. Why? I'm sure there were
reasons--reasons that have since been discarded or reassessed because more
modern versions of OpenAPI just use JSON Schema (a more recent revision).
Suffice to say: I just cared about the JSON Schema-like dialect that OpenAPI
3.0.x uses.

I didn't even care about *all* of that JSON Schema subset. JSON Schema is...
either flexible or chaotic depending on your perspective. Having started my
programming life in the chaos of Perl, I now gravitate to the inflexibility
(and precision) of Rust: I'd like there to be exactly one way to do a
particular thing (of course, Rust has oodles of flexibility--just saying 
restrictiveness can be a virtue). The purpose of the SDK generator was (and is,
primarily) to turn OpenAPI docs derived from Rust code into Rust SDKs. That
meant that typify needed to turn schemas--whose origins were Rust types--
back into Rust types. While there might be an infinity of ways to describe some
particular type in JSON Schema, the `schemars::JsonSchema` derive macro used a
consistent and narrow subset.

Sure JSON Schema is complicated, but I could safely ignore a big chunk of that complexity.


# So much wrong

With an OpenAPI SDK generator and JSON type compiler, folks (including folks at
Oxide) found other use cases for `progenitor` and `typify`. Plenty of those use
cases exposed ways in which we mishandled schemas or had a flawed understanding
of JSON Schema. I was proud of the utility people were getting, but also a
little embarrassed by failures. Progenitor could handle OpenAPI 3.0.x; what
about 3.1.x or 3.2.x, which use an incompatible JSON Schema specification?
Yeah, it breaks in all kinds of ways. (Also: Hello? Semver?) The JSON Schema
version that typify handles is (basically) Draft 7 (and that's an artifact of
using `schemars` to marshal data). On other versions (such as 2020-12 which
OpenAPI 3.1+ use), typify... sometimes works, and often breaks.

For example, a Rust type like `struct Foo(String, String)` might be represented in Draft 7 like this:

```json
{
  "type": "array",
  "items": [
    { "type": "string" },
    { "type": "string" }
  ],
  "minItems": 2,
  "maxItems": 2
}
```

In 2020-12, that becomes:

```json
{
  "type": "array",
  "prefixItems": [
    { "type": "string" },
    { "type": "string" }
  ],
  "minItems": 2,
  "maxItems": 2
}
```

Before 2020-12 `items` could either be a single schema or an array of schemas
(with `additionalItems` meaning "and this schema for everything past that array
of items"). In 2020-12, `items` can **only** be a single schema and
`prefixItems` enumerates the discrete array items. It's a highly sensible, and
also incompatible change (Note: also an example of how OpenAPI 3.1 isn't
backward compatible with 3.0). Typify? It just ignores `prefixItems` and tries
its best, so you get something like:

```rust
struct Foo(pub [::serde_json::Value; 2usize]);
```

We see that there's an array with exactly two items; there aren't the
enumerated items (because we don't know about `prefixItems`) so we don't
generate a tuple. In fact there's no `items` type at all so we just use the
yolo type of `serde_json::Value` which lands us with `[Value; 2]`.

There are also plenty of normal OpenAPI 3.1 / JSON Schema Draft 7 constructions
that I just didn't anticipate or didn't understand. I already [wrote about some
of them](/2024/01/22/rust-and-json-schema/#anyof), but there are others. For
example, the normal place to put a type to which you refer is under `$defs`,
but a reference can be (and sometimes is) to literally any other part of the
document. It can be to other documents entirely! What happens with schemas with
references like this? Mostly panics.

Structurally, there's a lot that typify has a hard time with or handles
inefficiently. It's a compiler, sure, but it could reasonably be described as a
single-pass transformer. There's no real IR, and as a result, things get messy;
complex work can be repeated many many times.

Finally, when it fails (see all examples above and many many more) it often
does so inscrutably. People don't like panics; they really don't like them when
there are no clues regarding the cause.


# Goals for better

There's no better instructor for the better version than the mistakes and
inadequacies of the first version. My own experience and that of other users made the goals for a new version clear:

- Multiple versions of JSON Schema (and OpenAPI)

- Coverage for as many cockamamie schema constructions as possible:
  `if`/`then`/`else`, `patternProperties`, `$dynamicRef`, etc.

- Schemas spanning multiple documents (more common in JSON Schema in 2019-09
  and 2020-12)

- Flexible references: `$defs`, `definitions`, random JSON paths, named
  `$anchor`s and `$id`s (i.e. however JSON Schema lets you refer to things)

- Errors that provide the JSON path and context (where in the doc did it fail?
  why did it fail?)

- More generally, a flexible structure to be able to handle schema
  constructions as we encounter them in the wild


# First, a better architecture

As I noted before, typify currently has a pretty simple structure: see a
schema, emit a Rust type. As its complexity has grown, so has my understanding
of a reasonable decomposition:

## Bundler

A schema (or OpenAPI description) can span documents. It's quite annoying. The
"bundler" abstraction wraps up that pile of JSON (or YAML) documents, indexing internal anchors and identifiers. Other layers can examine content and chase references by looking up documents (in whole or part) in a bundle.

## IR

Code specific to each version (e.g. OpenAPI 3.0.x or JSON Schema 2020-12)
interprets structures and produces a generic internal representation (IR). This
is a big missing piece of the current design which makes multi-version support
particularly challenging.

## Normalizer

As noted above, there are an infinity of different, but equivalent schemas.
Trying to somehow handle them all and wind up with the same type proved to be
intractable. Instead, the normalizer transforms the IR into a canonical form.

A schema can have an arbitrarily deep nesting of "subschemas":

```json
{ 
  "allOf": [
    {
      "oneOf": [
        {
          "anyOf": [
            { ... }
          ]
        }
      ]
    }
  ]
}
```

That's very (very) annoying to deal with. It leads to a lot of repeated work,
and recursive complexity.

Instead we want to land in a kind of disjunctive normal form where a
"normalized" schema is either a flat schema (no complex subschemas) or an
exclusive OR of flat schemas.

Note that the exclusivity of various arms is important: in JSON Schema, a
`oneOf` means that *exactly* one subschema must match; any values in the
intersection between subschemas are therefore invalid. Consider this example:

```json
{
  "oneOf": [
    {
      "type": "integer",
      "maximum": 4
    },
    {
      "type": "integer",
      "minimum": 2
    }
  ]
}
```

A value of, say, `3` would be invalid because it doesn't match *exactly* one
subschema of the `oneOf`. Normalization transforms the `oneOf` into an
`exclusiveOneOf`:

```json
{
  "exclusiveOneOf": [
    {
      "type": "integer",
      "maximum": 4,
      "not": {
        "type": "integer",
        "minimum": 2
      }
    },
    {
      "type": "integer",
      "minimum": 2,
      "not": {
        "type": "integer",
        "maximum": 4
      }
    }
  ]
}
```

... and then simplifies it to this (the canonical IR; not JSON Schema):


```json
{
  "exclusiveOneOf": [
    {
      "type": "integer",
      "exclusiveMaximum": 2
    },
    {
      "type": "integer",
      "exclusiveMinimum": 4
    }
  ]
}
```

Note that 2 through 4 (inclusive) satisfy both of the original subschemas so we
need to snip that range out of both.

The `exclusiveOneOf` can then cleanly map to a Rust `enum` without concerns
about ambiguity (i.e. a given value in Rust should round-trip
(serialize/deserialize) back into the same Rust value).

## Converter

Conversion in `typify` is pretty complex because we have to handle all kinds of
inputs. Conversion of normalized schemas can be both much simpler and more
obviously exhaustive. Previously, this was mind-bending and complex. In the new
architecture, it's fun to carve out particular patterns of canonical schemas
and map them to Rust type representations.

## Type representation

Another discrete, seemingly generic layer to fall out is that of type
representation. "Build me an enum with this name and these variants." or "Make
a struct with these properties." It's nice, generic stuff that can be separately
developed and tested. I can write tests that construct types, generate the code, and then use the types to validate its construction.
This layer pulls out some of the type-specific complexity and configurability
into its own entity. Do you want to use `BTreeMap`, `HashMap` or something
custom? Do you prefer `String` or `::std::string::String`?

We can also get really precise (read: excessively fussy) about the modeling of
absent vs. null object properties. These are typically (accidentally) conflated
in Rust:

```rust
struct Foo {
    bar: Option<String>,
}
```

Here `bar` could be absent or have a value of `null`. Some folks add a
`#[serde(default)]` annotation, thinking it affects this... when, in fact, it
does nothing.

If you care about the distinction between `null` and absent, or if you want to
precisely model schemas where one is permitted but the other is not, this layer
isolates this kind of fastidiousness, and allows testing independent of
anything related to JSON Schema.


# How's it going?

This decomposition has been extremely empowering. The general shape *feels*
really good, and has let me make progress in a bunch of different areas. Having
the right abstractions has kept the project on track.

As expected, the normalizer is the gnarliest chunk of code. It's burdened with
literally all the complexities of JSON Schema, but has well-defined outcome
criteria. I've gone through several significant iterations just of this layer,
and it still needs work. The great thing is that even with a crummy version, I
can make progress on all the other parts.

Other layers are crystal clear. When my brain feels broken by the normalizer, I
can do some satisfying turd-polishing on the type representation. For example,
how would we represent a schema like this in Rust?

```json
{
  "type": "array",
  "prefixItems": [
    { "type": "string" },
    { "type": "string" }
  ],
  "items": { "type": "integer" },
  "minItems": 2
}
```

So that's an array with at least two strings, followed by zero or more
integers. We'd like to do something like this in Rust:

```rust
struct Foo(String, String, #[serde(flatten)] Vec<i64>);
```

... but `serde` doesn't flatten one sequence into another (though it *can*
flatten one object into another ¯\(ツ)/¯). Our type representation layer can
support that kind of structure by generating custom `serde` implementations.

Further, by having a strong foundation with all these layers in a basically
functional state, I can advance them all by introducing new input schemas that
force new development in each successive layer.


# Do you even AI?

Can you write a blog post in 2026 without mentioning your use of AI? I've
started using Claude more recently to try to help out; it hasn't been the
miracle I had hoped for. I've found it much too eager to chase bad ideas well
past the point where I would have identified it as a bad idea. It seems unable
to distinguish (or ask) the difference between constraints that are important
and those that can be relaxed. I've tried embracing the vibes and letting it do
its thing; the results have been consistently in the wrong direction. Its
greatest utility in this project has been as a discussion partner to (mostly)
validate my existing plans, and to document work I've done. I have higher hopes
for its assistance on some of the more repetitive tasks (e.g. an expressive,
builder interface for the type representation), and in some areas where I have
more confidence that I can specify the design completely (as opposed to using
implementation as a means to discover the shape of various parts).


# So... typify 2 when?

I've been working on this re-write inconsistently for around 2 years now. It's
not a high priority because the benefits to Oxide aren't immediate. For
example, it would be great to move all our servers and clients to a more recent
version of OpenAPI... but being on an older version doesn't have any acute
downsides. There are newer OpenAPI documents from which--sure--we'd like to
make SDKs... but there aren't that many and often we can convert, say, OpenAPI
3.1 to 3.0 without losing fidelity. It's firmly in the nice-to-have bucket, not
the hair-on-fire bucket.

That said, each new issue keeps me motivated; each new bolt-on work-around for
the structural inadequacies makes me want to work on the new version more. I
have a plan for incrementally rolling it out (crash landing a re-write is
famously tough to do smoothly). In the meantime, it's satisfying having the
right bones, the right structure, building the better version from what I've
learned from the first version.
