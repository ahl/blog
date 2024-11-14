---
title: "Austin API Summit Wrap-up"
date: "2024-04-17"
---

I started the year thinking that we had done some neat work in the Rust / API / OpenAPI ecosystem and I wanted to talk about it. I found my people at the Nordic APIs Austin Summit; it’s a conference that had been on my radar for a while, but it was my first time attending. I was pleasantly surprised by how interesting the sessions and conversations were… and was pleased to share some stuff we’ve been up to.

**The Oxide API-a-matic universe**

I presented ([slides](https://speakerdeck.com/ahl/uniting-rust-servers-and-clients-through-openapi)) our experience with Rust as API server and client, generating an SDK, and generally how terrific all that has been. It’s worked out better—if differently—than I expected. Joining Oxide, I had some experience with OpenAPI. I thought we’d be able to avoid a bunch of work, relying instead on the strength of the ecosystem. Faith in the ecosystem turned out to be misplaced. However, the investment we made in our own web API framework ([dropshot](https://github.com/oxidecomputer/dropshot)), and SDK generation ([progenitor](https://github.com/oxidecomputer/progenitor)) turned out to be incredibly and surprisingly valuable.

<iframe title="YouTube video player" src="https://www.youtube.com/embed/UptsQuOlhBU?si=fD4128utReMR7NSP" width="560" height="315" frameborder="0" allowfullscreen="allowfullscreen"></iframe>

**SDK: The Next Generation**

Given my poor experience with open source SDK generators, I thought we might be alone in seeing the value of SDK generation. Instead, it was a discernible theme at the conference.

Microsoft was there (in numbers) talking about [Kiota](https://learn.microsoft.com/en-us/openapi/kiota/overview), their open source SDK generator (and [TypeSpec](https://nordicapis.com/sessions/apis-at-scale-with-typespec/)—also interesting). It’s very cool, very post-lost decade Microsoft. Part of the thesis (as far as I can tell) is that first-party API producers may not need to ship their own SDK; instead, they can share OpenAPI (or TypeSpec) and let consumers use Kiota to generate clients. Further, they can generate specifically the clients they need: not just by picking the relevant language, but the relevant subset of the API, or even by picking subsets of **several** APIs to have a consistent client interface for all the APIs they want to talk to. You don’t need various vendors to make consistent SDKs—you can generate the SDK you need for the integrations you want to build. It’s a neat concept I hadn’t seen before.

TypeSpec is pretty cool; it jives with some of my opinions on OpenAPI. TypeSpec is a higher-level, terser, opinionated, more expressive way to define an API. It’s a new TypeScript-familiar language that you can translate into OpenAPI. At Oxide, we define our APIs with Dropshot as annotations in the code itself, but it’s a similar idea. In both cases humans aren’t writing JSON by hand (Yes! Leave machine formats to the machines!). There can be some lossiness, e.g. TypeSpec allows you to express pagination semantics that can’t be expressed in OpenAPI (and doesn’t seem to be on the radar for the next OpenAPI release).

I was blindsided by SDK generation being a thesis for a whole batch of startups. [APIMatic](https://www.apimatic.io/) and [liblab](https://nordicapis.com/sessions/from-apis-to-sdks-elevating-your-developer-experience-with-automated-sdk-generation/) were at the conference. I came across [Speakeasy](https://www.speakeasyapi.dev/) previously; and since discovered [Stainless](https://www.stainlessapi.com/), [Fern](https://www.buildwithfern.com/), and [Konfig](https://konfigthis.com/). Except for APIMatic all of these seem to have been founded in 2022 with at least $65m invested by my count. This is wild to me… here we are just giving away our Rust SDK generator… like suckers! (Just kidding! Open source is central to our mission at Oxide and we’re happy to work almost entirely in the open. We don’t ask for copyright assignment that might allow for the [source-available](https://thenewstack.io/hashicorp-abandons-open-source-for-business-source-license/) [maliciousness](https://news.ycombinator.com/item?id=37299906) we've seen proliferating.)

Each of these companies charges $100s/month for SDK generation. I guess the pitch is something like: you, a large company, drank the API Kool-Aid and now you have N internal APIs. At a minimum you’re exerting N x (number of languages in use) effort, but in all likelihood, that work is being repeated in a bunch of places where groups aren’t aware or don’t trust the SDKs made by others. So with SDK generation, you just call our API with your latest API specs and out pops SDKs in all the languages you and your customers care about. It will be interesting to see if this turns out to be a sustainable model.

**AI / ML**

Can you go a tech conference in 2024 where AI / ML isn’t a theme? I doubt it. Lots of [uses of AI in the API](https://nordicapis.com/sessions/securely-boosting-any-product-with-generative-ai-apis/) space on display. Use of generative AI APIs, [writing docs through gen AI](https://nordicapis.com/sessions/democratizing-api-accessibility-how-ai-and-visual-tools-can-help-anyone-tackle-the-technical/), [training AI on docs](https://nordicapis.com/sessions/how-i-built-bill-the-ai-powered-chatbot-that-reads-our-docs-for-fun/), even writing SDKs through gen AI. Gen AI all the things.

**In-person Conferences… still a thing!**

I had not been to a conference in … a while. It was surprisingly great. In particular, it was a chance to go deep on topics I don’t usually discuss. I work with a ton of great folks at Oxide, but I think none of them cares—for example—about the [draft proposal for the next OpenAPI release](https://github.com/OAI/sig-moonwalk) or has opinions about it. A bunch of folks who **do** care were there in person, and we could debate topics I’d been chewing on. I got to hear about how others are approaching similar problems in ways that’s not always obvious from the talks that come out of conferences or online discussion.

I wasn’t sure if in-person conferences were still going to be valuable. It was for me; it might be for you.
