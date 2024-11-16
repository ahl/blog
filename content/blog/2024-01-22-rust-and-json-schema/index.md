---
title: "Rust and JSON Schema: odd couple or perfect strangers"
date: "2024-01-21"
permalink: /2024/01/22/rust-and-json-schema/
---

A bit over two years ago, I started work on [typify](https://crates.io/crates/typify), a library to generate Rust types from JSON Schema. It took me a while to figure out it was a compiler, but I’ll call it that now: it’s a compiler! It started life as a necessary component of an OpenAPI SDK generator—a pretty important building block for the control plane services at Oxide. Evolving the compiler has become somewhere between a hobby and an obsession, trying to generate increasingly idiomatic Rust from increasingly esoteric schemas.

## Get in, loser; we’re writing a compiler

Why did I start building this? I came to Oxide with a certain amount of OpenAPI optimism (from my previous company), optimism that was in some cases well-founded (it has earned its place as the de facto standard for describing HTTP-based APIs), and in other cases profoundly misplaced (the ecosystem was less mature than expected). On the back of that optimism, [Dave](https://www.davepacheco.net/blog/about/) and I (but mostly Dave) built a server framework, [dropshot](https://github.com/oxidecomputer/dropshot), that emits OpenAPI from the code. We gave a [pretty good talk](https://www.youtube.com/watch?v=EmSjZbSzA3A) about it in 2020 about using the code as the source of truth for interface specification.

As we built out services of the control plane we wanted service-specific clients. Ideally these would be derived from the OpenAPI documents emitted from dropshot. We couldn’t find what we wanted in the ecosystem (read: we tried them and they didn’t work) so we built our own. Before we could invoke APIs and understand their responses we needed to generate types. Since OpenAPI uses JSON Schema to define types\*\*, I started there.

\*\*: sort of; and it's actually quite annoying but I'll save my grousing for later.

## Sum types

Pretty uncontroversial take for Rust programmers: sum types are great. We use `enum`s a bunch in the API types because they let us express precise constraints. (They **do** make for tricky SDK generation in languages that **don’t** support sum types, but that’s not important here.) How are enums represented in JSON serialization or JSON schema? The answer, with some irony, is “variously". The ubiquitous Rust de/serialization framework gives 4 different choices (that I’ll show below).

My woodworking mentor as a kid observed that I start projects in the middle. That’s exactly what happened here. Reconstructing `enum`s from their generated schemas seemed tricky and interesting, so that’s where I started. Generally, an `enum` turns into a `oneOf` construction ("data must conform to exactly one of the given subschemas"). I try to apply heuristics that correspond to each of the [serde enum formats](https://serde.rs/enum-representations.html):

```rust
 let ty = self
    .maybe_option(type_name.clone(), metadata, subschemas)
    .or_else(|| self.maybe_externally_tagged_enum(type_name.clone(), metadata, subschemas))
    .or_else(|| self.maybe_adjacently_tagged_enum(type_name.clone(), metadata, subschemas))
    .or_else(|| self.maybe_internally_tagged_enum(type_name.clone(), metadata, subschemas))
    .or_else(|| self.maybe_singleton_subschema(type_name.clone(), subschemas))
    .map_or_else(|| self.untagged_enum(type_name, metadata, subschemas), Ok)?;
```

Externally tagged enums have this basic shape:

```json
{
  "<variant-name>": { .. }
}
```

Internally tagged enums look like this:

```json
{
  "<tag-name>": { "const": ["<variant-name>"] },
  … other properties …
}
```

Externally tagged enums:

```json
{
  "<tag-name>": { "const": ["<variant-name>"] },
  "<content-name>": { .. }
}
```

Unlike other formats, the final format, “untagged", doesn’t include any indication of the variant name—it just dumps the raw type data (and one needs to be careful that the subschemas are mutually exclusive).

Seeing enums traverse JSON Schema and turn back into the same Rust code was very satisfying. While I basically got enum generation right, there are a couple of JSON Schema constructs that I really screwed up.

## allOf

In JSON Schema an “allOf" indicates that a data value needs to conform to all subschemas… to no one’s surprise. So you see things like this:

```json
{
  "title": "Doodad",
  "allOf": [
    { "$ref": "#/$defs/Thingamajig" },
    { "$ref": "#/$defs/Whosiewhatsit" },
  ]
}
```

Serde has a `#[serde(flatten)]` annotation that takes the contents of a struct and, effectively, dumps it into the container struct. This seemed to match the `allOf` construct perfectly; the above schema would become:

```rust
// ⬇️ This is wrong; don’t do this ⬇️
struct Doodad {
    #[serde(flatten)]
    thingamajig: Thingamajig,
    #[serde(flatten)]
    whosiewhatsit: Whosiewhatis,
}

```

This is wrong! Very very wrong. So wrong it often results in structs for which no data results in valid deserialization or serializations that don’t match the given schema. In particular, imagine if both Thingamajig and Whosiewhatis have a fields of the same name with incompatible types.

Perhaps more precisely: the code above is only right under the narrow conditions that the subschemas are all fully orthogonal. In the wild (as we JSON Schema wranglers refer to its practical application), `allOf` is most commonly used to apply constraints to existing types.

Here’s an example from a github-related schema I found:

```json
"allOf": [
  { "$ref": "#/definitions/issue" },
  {
    "type": "object",
    "required": ["state", "closed_at"],
    "properties": {
      "state": { "type": "string", "enum": ["closed"] },
      "closed_at": { "type": "string" }
    }
  }
]
```

The “issue" type is an object with non-required properties like:

```json
{
  "state": {
    "type": "string",
    "enum": ["open", "closed"],
    "description": "State of the issue; either 'open' or 'closed'"
  },
  "closed_at": { "type": ["string", "null"], "format": "date-time" },
}
```

The result of this allOf is a type where state is required and must have the value “closed" and “closed\_at" must be a date-time string (and not null). (closed\_at was already required by the base type, so I’m not sure why the allOf felt the need to reassert that constraint.)

This is very very different than what #\[serde(flatten)\] gives us. Originally I was generating a broken type like this:

```rust
struct ClosedIssue {
    #[serde(flatten)]
    type_1: Issue,
    #[serde(flatten)]
    type_2: ClosedIssueType2,
}

struct ClosedIssueType2 {
    state: ClosedIssueType2State; // enum { Closed }
    closed_at: String,
}
```

Wrong and not actually useful. More recently I’ve applied merging logic to these kinds of constructions, but it’s tricky and opens the door to infinite recursion (one of the many sins the JSON Schema spec condemns albeit with merely its second sternest form of rebuke).

## `anyOf`

I got `allOf` wrong. I got `anyOf` much wronger. `AnyOf` says that a valid value should conform to any of the given subschemas. So if an `allOf` is just a struct with a bunch of flattened members then it would make sense that an `anyOf` is a struct with a bunch of optional members. It makes sense! especially if you don’t think about it!

```rust
// ⬇️ This is wrong; don’t do this ⬇️
struct Doodad {
    #[serde(flatten)]
    thingamajig: Option<Thingamajig>,
    #[serde(flatten)]
    whosiewhatsit: Option<Whosiewhatis>,
}
```

But if you do think about it even briefly, you realize that a type like carries only the most superficial relationship with the JSON Schema. For example, at least one of the subschemas needs to be valid and this type would be fine with an empty object (`{}`) turning into a bunch of `None`s.

So what’s a valid representation of `anyOf` as a Rust type? In a way I’m glad I went with this quick, clever, and very very wrong approach because a robust approach is a huge pain the neck! Consider an `anyOf` like this:

```json
{
  "title": "Something",
  "anyOf": [
    { "$ref": "#/$defs/A" },
    { "$ref": "#/$defs/B" },
    { "$ref": "#/$defs/C" },
  ]
}
```

Bear in mind, my goal is to allow only valid states to be represented by the generated types. That is, I want type-checking at build time rather than, say, validation by a runtime builder. Consequentially, I think we need a Rust type that’s effectively:

```rust
enum Something {
    A,
    B,
    C,
    A ∪ B,
    A ∪ C,
    B ∪ C,
    A ∪ B ∪ C,
}
```

You need the power set of all sub-types. Sort of. Some of those are going to produce unsatisfiable combinations (i.e. if the types are orthogonal). We’d ideally exclude those. And we need to come up with reasonable names for the enum variants (`AAndBAndC`?). Ugh. It’s awful. While I've cleaned up `allOf`, typify's `anyOf` implementation is still based on that original, wrong insight.

## JSON kvetching

I used to abstractly dislike JSON Schema. My dislike has become much more concrete. With a big caveat: I'm considering only the use cases I care about, which assuredly bear little-to-no resemblance to the use cases envisioned by the good folks who designed and evolve the standard. By way of a terrible analogy here's the crux of the issue: I think about product concept documents (PCDs) and product requirement documents (PRDs) which are vaguely common product management terms (that I’ll now interpret for my convenience). A PCD tells you about the thing. What is it? How’s it work? How might you build it? A PRD provides criteria for completion. Can it do this? Can it do that? JSON Schema is much better at telling you if the thing you've built is valid than it is at telling you how to build the intended values.

What I want is a schema definition for affirmative construction, describing the shape of types. What are the properties? What are the variants? What are the constraints? JSON Schema seems to have a greater emphasis on validation: does this value conform?

As an example of this, consider JSON Schema’s `if`/`then`/`else` construction.

```json
{
  "if": { "$ref": "#/$defs/A" },
  "then": { "$ref": "#/$defs/B" },
  "else": { "$ref": "#/$defs/C" }
}
```

If the value conforms to a schema, then it must conform to another schema… otherwise it must conform to a third schema. Why does JSON Schema even support this? I think (but am deeply unsure) that this is equivalent to:

```json
{
  "oneOf": [
    {
      "allOf": [
        { "$ref": "#/$defs/A" },
        { "$ref": "#/$defs/B" }
      ]
    },
    {
      "allOf": [
        { "not": { "$ref": "#/$defs/A" } },
        { "$ref": "#/$defs/C" }
      ]
    }
  ]
}

```

In other words, `{ A ∪ B, ¬A ∪ C }`. Perhaps it's a purely academic concern: I haven’t encountered `if`/`then`/`else` in an actual schema.

More generally: there are often many ways to express equivalent constructions. This is, again, likely a case of my wanting JSON Schema to be something it isn’t. There’s an emphasis on simplicity for human, hand-crafted authorship (e.g. `if`/`then`/`else`) whereas I might prefer a format authored and consumed by machines. The consequence is a spec that's broad, easy to misimplement or misinterpret, and prone to subtle incompatibilities from version to version.

## Typify to the future

As much as it’s been a pain in the neck, this JSON Schema compiler has also been a captivating puzzle, reminiscent of the annual untangling of Christmas tree lights (weirdly enjoyable… just me?). How to translate these complex, intersecting, convoluted (at times) schema into neat, precise, idiomatic Rust types. I feel like I kick over some new part of the spec every time I stare at it ([dependentRequired](https://json-schema.org/understanding-json-schema/reference/conditionals#dependentRequired)? Who knew!). There are plenty of puzzles left: schemas with no simple Rust representation, unanticipated constructions, weirdo anchor and reference syntex, and—to support OpenAPI 3.1—a new (subtly incompatible) JSON Schema revision to untangle.
