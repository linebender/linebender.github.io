+++
title = "A plan for SIMD"
authors = ["Raph Levien"]
+++

# A plan for SIMD

This document is a followup to the blog post, “[Towards fearless SIMD, 7 years later]”.
My goal in publishing that was to start a conversation about the best way for Rust to support SIMD programming.
I’ve also done a bit more exploration and had some discussions at RustWeek, and am now prepared to offer a plan.
Ideally, I’d like some level of buy-in from the Rust ecosystem, but in any case it feels Good Enough to proceed with writing a large quantity of SIMD code, which is required for Linebender projects to meet their performance goals.
There's also a companion PR, [fearless_simd#3], which moves this proposal forward.

## Goals

The main goals are spelled out in the above-linked blog.
After some more experimentation and reflection, I’d like to explicitly add the following goals:

* Lightweight dependency.
The library itself should be quick to build.
It should have no expensive transitive dependencies.
In particular, it should not require proc macro infrastructure.
 
* Fine-grained levels.
I’ve spent more time looking at CPU stats, and it’s clear there is value in supporting at least SSE 4.2 – in the [Firefox hardware survey], AVX-2 support is only 74.2% (previously I was relying on Steam, which has it as 94.66%).

## Summary

We build in the direction of [fearless_simd#2], but with a few course corrections.
In particular, instead of manually curating the library and using (declarative) macros to try to reduce repetition and boilerplate, we rely heavily on code generation.

While the current fearless\_simd\#2 prototype implements a handful of types, the goal is to support a full cross-product of SIMD width 64 to 512, signed and unsigned integer types from 8 to 32 (possibly 64), f32 (possibly f64), and f16 where available (primarily newer ARM chips, though Sapphire Rapids does support it).
For widths greater than native, the library will polyfill with arrays of the native SIMD width.

Note: since that blog post, f16 and Neon f16 instructions are supported in Rust nightly.
It would be possible to polyfill in a hacky way (as is done in fearless\_simd\#2), but my inclination is to have these depend on Rust support, and similarly for AVX-512.
With luck, these will stabilize soon (and obviously that’s one of our big asks of the Rust project).
\[Update: AVX-512 will land in [1.89](https://github.com/rust-lang/rust/pull/138940) if things go well; still gathering more info about fp16 but that might also be very soon\]

## Explicit vs variable width

One of the big decisions in writing SIMD code is whether to write code with types of explicit width, or to use associated types in a trait which have chip-dependent widths.
The design of pulp strongly favors the latter; the [Simd](https://docs.rs/pulp/latest/pulp/trait.Simd.html) trait has only “natural width” types, and, in particular, there is no implementation of, for example, 256 bit wide operations on Neon.
A key departure from pulp, then, is support for portable explicit width programming.

For Linebender work, I expect 256 bits to be a sweet spot.
Obviously, it’s the natural width of AVX-2, which is a pretty big majority of x86\_64 chips.
On Neon, code written for 256 will be unrolled somewhat, but there are 32 registers (as opposed to 16 for AVX-2), so register pressure should not be a problem, and high end chips (such as Apple Silicon) have very wide issue.
The main reason to go smaller is when the loop has a high probability of consuming less than 256 bits of input.

In the other direction, the majority of shipping AVX-512 chips are double-pumped, so code written to use 512 bits is not significantly faster (I assert this based on some serious experimentation on a Zen 5 laptop).
I also expect this state of affairs to continue for a while; AMD won this design war and Intel is struggling to catch up after flailing for many years.
If and when a significant number of true 512 bit chips ship, that would justify more work to write 512-optimized variants.

For map-like workloads, I propose adding pulp-like associated natural-width types and methods to the Simd trait.
Following Highway, I think it also makes sense to have some more methods that work in 128 bit blocks.
A motivating use case is f32 color space conversion, where the alpha channel is passed through unmodified; a very reasonable implementation strategy is to do the nonlinear conversion for all channels, then do a blend operation on lane 3 of 128 bit blocks.
(Additional note: in experiments, trying to do this absolutely broke autovectorization).

## Light use of macros

I am quite concerned about the compile time.
Cold build of fearless\_simd\#2 on an M1 Max is 3.25s.
Considering that building out the full cross-product of sizes and types will cause about an order of magnitude increase in size, it’s clear that the library has the potential for major impact on compile times.

Using \-Z self-profile to investigate, 87.6% of the time is in expand\_crate, which I believe is primarily macro expansion \[an expert in rustc can confirm or clarify\].
This is not hugely surprising, as (following the example of pulp), declarative macros are used very heavily.
A large fraction of that is the safe wrappers for intrinsics (corresponding to [core\_arch](https://docs.rs/pulp/latest/pulp/core_arch/x86/index.html) in pulp).

I believe that using codegen to expand out the macros before crate publish time will help greatly with compile times, but this needs to be experimentally validated.
A possible downside is the size of the crate (especially uncompressed), but I expect zlib compression to be very effective given the repetitive, boilerplate nature of the contents.

One use of macros will remain: simd\_dispatch as a declarative macro to generate the dispatch wrappers.
Likely the proposed [declarative macro improvements](https://rust-lang.github.io/rust-project-goals/2025h1/macro-improvements.html) could help a lot here.
I’m especially positive about the ability to write attributes as a declarative macro, as that would reduce the stuttering in the existing syntax.

## Topics discussed in the blog

Some of these may be expanded, but I’ll list them as they’re relevant:

* dispatch done by passing around \`Level\` enum; minimize cost of detection  
* downcasting to access chip-specific capabilities  
* ergonomic std::ops, including implicit splat

## Alternatives considered

I started prototyping a DSL that compiles into Rust, using proc macro infrastructure.
This would operate in one of two modes, either as a proc macro run at build time, or as codegen to generate .rs files which would typically be checked into a repo, to minimize build time impact.
(I’ll note that this is a particular instance of a much more general need for better staged compilation, which we’re feeling particularly acutely for shader compilation).
At some point I'll post the prototype, as I think it's worthy of being considered if we're going to be doing a full exploration.

While I think this approach has some advantages, it approaches the cost of building a real programming language, with associated needs for tooling etc.
In addition, the proc macro approach really shows seams when it comes to cross-module interactions.

Another possibility is to evolve pulp in the direction we need.
That’s still not out of the question, but the changes proposed are quite extensive, and this could be disruptive to the existing user base, particularly faer.
One goal in publishing this plan is to gather feedback from the pulp community about what they’d like to see happen.

## On RVV and SVE

One topic I didn’t cover in my blog post is RVV and SVE, which are pretty marginal these days but will become more important.
There are some pretty big challenges, and for the most part we’re blocked on Rust support for the intrinsics.

Taking a longer view, I think there are two modes for “scalable vector” SIMD.
One is to generate asm which operates correctly no matter the SIMD width.
It’s fairly easy to see how to do this for map-like workloads, and I think it’s reasonable to consider this mostly a codegen problem for autovectorization, as opposed to a good use case for explicit SIMD.

The other is to treat scalable vectors as an annoying instruction encoding for SIMD width not known at compile time, and with partial support for operating on fixed-width (128 bit) blocks.
This is essentially the same concept as “[Fixed Width Vector Workloads on Variable Width Vectors](https://gist.github.com/zingaburga/805669eb891c820bd220418ee3f0d6bd#fixed-width-vector-workloads-on-variable-width-vectors)”.
The entire concept is a bit frustrating because nearly all ARM chips are 128 bits anyway.

See also the paper [SIMD Everywhere Optimization from ARM NEON to RISC-V Vector Extensions](https://arxiv.org/pdf/2309.16509) for an approach to making explicit SIMD code portable to RVV.

## Considerations for WASM SIMD

There’s a [Zulip thread](https://xi.zulipchat.com/#narrow/channel/197075-vello/topic/WASM.20SIMD) on this.
One tricky bit is that WASM doesn’t have runtime feature detection, rather they expect feature detection to be done as part of the negotiation for deciding which WASM blob to serve.
In some ways, this makes sense, as it avoids pretty much all of the difficulties of multiversioning (including potential binary size impact), but on the other hand it’s a pain, as it complicates build and deployment.

## Very small SIMD

In the embedded world, there are a variety of SIMD extensions for small widths.
I believe the standard RVV extension (intended for application class chips) requires a width of at least 128 bits, but there is a Zve64 profile intended for embedded.
Additionally, embedded ARM processors have a SIMD extension called “[Helium](https://en.wikichip.org/wiki/arm/helium)”, which is still 128 bit but with a smaller register count and more limited instructions.
Given that Linebender is pushing more into `no_std`, it’s possible we’ll want to support these.
However, I’m not able to find much evidence of Rust support for these.
At this point, probably best to consider it an open question and potential future work.

## Discussion

There’s a [discussion thread](https://xi.zulipchat.com/#narrow/channel/255911-rust/topic/A.20plan.20for.20SIMD/with/520769933) on Linebender Zulip.
I’ve also opened a [thread](https://rust-lang.zulipchat.com/#narrow/channel/219381-t-libs/topic/is.20.60arch.3A.3Aaarch64.3A.3Afloat16x8_t.60.20supposed.20to.20be.20stable.3F/with/520762685) on Rust Zulip with some gnarly details about stabilizing fp16 on Neon.

I'm posting this now to the Linebender blog to encourage more discussion in the Rust community.
The best place for serious technical discussion is the Zulip thread.
We expect development to continue in the [fearless_simd repo].

[Towards fearless SIMD, 7 years later]: https://linebender.org/blog/towards-fearless-simd/
[fearless_simd#2]: https://github.com/raphlinus/fearless_simd/pull/2
[fearless_simd#3]: https://github.com/raphlinus/fearless_simd/pull/3
[fearless_simd repo]: https://github.com/raphlinus/fearless_simd
[Firefox hardware survey]: https://firefoxgraphics.github.io/telemetry/#view=system
