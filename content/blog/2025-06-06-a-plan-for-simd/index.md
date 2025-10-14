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
SIMD results in massive speedups for workloads operating on bulk data, especially image processing, media codec, audio.
It is also possible to exploit SIMD for speedups in other applications such as string processing.
The primary goal of this library is to make SIMD programming ergonomic and safe for Rust programmers, making it as easy as possible to achieve near-peak performance across a wide variety of CPUs.
These goals are very similar to those of [Highway], a mature and capable SIMD library for C++.

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
For example, here is the current way to express dispatch:

```rust
#[inline(always)]
fn to_srgb_impl<S: Simd>(simd: S, rgba: [f32; 4]) -> [f32; 4] {
    ...
}

simd_dispatch!(to_srgb(level, rgba: [f32; 4]) -> [f32; 4] = to_srgb_impl);
```

But with an attribute, it could be simplified to something like this (exact syntax may vary):

```rust
#[simd_dispatch]
fn to_srgb<S: Simd>(level: Level, rgba: [f32; 4]) -> [f32; 4] {
    let simd: S = level.get_simd();
    ...
}
```

While the current fearless\_simd\#2 prototype implements a handful of types, the goal is to support a full cartesian product of SIMD width 64 to 512, signed and unsigned integer types from 8 to 32 (possibly 64), f32 (possibly f64), and f16 where available (primarily newer ARM chips, though Sapphire Rapids does support it).
For widths greater than native, the library will polyfill with arrays of the native SIMD width.

Note: since that blog post, f16 and Neon f16 instructions are supported in Rust nightly.
It would be possible to polyfill in a hacky way (as is done in fearless\_simd\#2), but my inclination is to have these depend on Rust support, and similarly for AVX-512.
With luck, these will stabilize soon (and obviously that’s one of our big asks of the Rust project).
\[Update: AVX-512 will land in [1.89](https://github.com/rust-lang/rust/pull/138940) if things go well; still gathering more info about fp16 but that might also be very soon\]

## Explicit vs variable width

One of the big decisions in writing SIMD code is whether to write code with types of explicit width, or to use associated types in a trait which have chip-dependent widths.
The design of [pulp] strongly favors the latter; the [Simd](https://docs.rs/pulp/latest/pulp/trait.Simd.html) trait has only “natural width” types, and, in particular, there is no implementation of, for example, 256 bit wide operations on Neon.
A key departure from pulp, then, is support for portable explicit width programming.

For Linebender work, I expect 256 bits to be a sweet spot.
Obviously, it’s the natural width of AVX-2, which is a pretty big majority of x86\_64 chips.
On Neon, code written for 256 bits will be unrolled somewhat, but there are 32 registers (as opposed to 16 for AVX-2), so register pressure should not be a problem, and high end chips (such as Apple Silicon) have very wide issue, meaning that the number of elements that can be processed in a single clock cycle is similar to other chips with 256 bit vectors but narrower issue.
The main reason to go smaller is when the loop has a high probability of consuming less than 256 bits of input.

In the other direction, the majority of shipping AVX-512 chips are double-pumped, meaning that a 512 bit vector is processed in two clock cycles (see [mersenneforum post] for more details), each handling 256 bits, so code written to use 512 bits is not significantly faster (I assert this based on some serious experimentation on a Zen 5 laptop).
I also expect this state of affairs to continue for a while; AMD won this design war and Intel is struggling to catch up after flailing for many years.
If and when a significant number of true 512 bit chips ship, that would justify more work to write variants optimized for 512 bits.

For simpler (map-like) workloads performing the same scalar computation for each element separately, I propose adding pulp-like associated natural-width types and operations to the `Simd` trait.
Following Highway, I think it also makes sense to have some more operations that work in 128 bit blocks.
A motivating use case is f32 color space conversion, where the alpha channel is passed through unmodified; a very reasonable implementation strategy is to do the nonlinear conversion for all channels, then do a blend operation on lane 3 of 128 bit blocks.
(Additional note: in experiments, trying to do this absolutely broke autovectorization).

## Light use of macros

I am quite concerned about the compile time.
A cold build of fearless\_simd\#2 on an M1 Max is 3.25s.
Considering that building out the full cartesian product of sizes and types will cause about an order of magnitude increase in size, it’s clear that the library has the potential for major impact on compile times.

Using `-Z self-profile` to investigate, 87.6% of the time is in expand\_crate, which I believe is primarily macro expansion \[an expert in rustc can confirm or clarify\].
This is not hugely surprising, as (following the example of pulp), declarative macros are used very heavily.
A large fraction of that is the safe wrappers for intrinsics (corresponding to [core\_arch](https://docs.rs/pulp/latest/pulp/core_arch/x86/index.html) in pulp).

I believe that using codegen to expand out the macros before crate publish time will help greatly with compile times, but this needs to be experimentally validated.
A possible downside is the size of the crate (especially uncompressed), but I expect zlib compression to be very effective given the repetitive, boilerplate nature of the contents.

One use of macros will remain: `simd_dispatch` as a declarative macro to generate the dispatch wrappers.
Likely the proposed [declarative macro improvements](https://rust-lang.github.io/rust-project-goals/2025h1/macro-improvements.html) could help a lot here.
I’m especially positive about the ability to write attributes as a declarative macro, as that would reduce the stuttering in the existing syntax.

## Topics discussed in the blog

These topics are discussed in the [towards fearless SIMD, 7 years later] blog, but I'll touch on them here as they are quite important.

Dispatch is done by doing runtime detection once at the beginning of the application, resulting in a `Level` enum, each variant of which is a zero-sized token type representing CPU capability.
This choice (same as pulp) minimizes cost of runtime detection.

It is possible to write code in a generic SIMD style, and this will work well in some use cases, but we also support downcasting the generic `Simd` bound to a specific level, at which point that level's chip-specific capabilities are available.

Here's an example of downcasting:

```rust
#[inline(always)]
fn copy_alpha<S: Simd>(a: f32x4<S>, b: f32x4<S>) -> f32x4<S> {
    #[cfg(target_arch = "x86_64")]
    if let Some(avx2) = a.simd.level().as_avx2() {
        return avx2
            .sse4_1
            ._mm_blend_ps::<8>(a.into(), b.into())
            .simd_into(a.simd);
    }
    #[cfg(target_arch = "aarch64")]
    if let Some(neon) = a.simd.level().as_neon() {
        return neon
            .neon
            .vcopyq_laneq_f32::<3, 3>(a.into(), b.into())
            .simd_into(a.simd);
    }
    let mut result = a;
    result[3] = b[3];
    result
}
```

In addition, the SIMD types all support `core::ops`, including implicit splat so it is easy to, say, multiply a vector by a scalar.
This ergonomic feature is present in [simdeez] and [std::simd], but not [pulp].

## Alternatives considered

I started prototyping a DSL that compiles into Rust, using proc macro infrastructure.
This would operate in one of two modes, either as a proc macro run at build time, or as codegen to generate .rs files which would typically be checked into a repo, to minimize build time impact.
(I’ll note that this is a particular instance of a much more general need for better staged compilation, which we’re feeling particularly acutely for shader compilation).
At some point I'll post the prototype, as I think it's worthy of being considered if we're going to be doing a full exploration.

While I think this approach has some advantages, it approaches the cost of building a real programming language, with associated needs for tooling etc.
In addition, the proc macro approach really shows seams when it comes to cross-module interactions.

Another possibility is to evolve [pulp] in the direction we need.
That’s still not out of the question, but the changes proposed are quite extensive, and this could be disruptive to the existing user base, particularly [faer].
One goal in publishing this plan is to gather feedback from the pulp community about what they’d like to see happen.

## On RVV and SVE

One topic I didn’t cover in my blog post is RVV and SVE, which are pretty marginal these days but will become more important.
There are some pretty big challenges, and for the most part we’re blocked on Rust support for the intrinsics.
There is work in this direction, including a [project goal](https://rust-lang.github.io/rust-project-goals/2025h1/arm-sve-sme.html) and an [RFC for sized traits](https://github.com/rust-lang/rfcs/pull/3729).

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
In some ways, this makes sense, as it avoids pretty much all of the difficulties of multiversioning (including potential binary size impact), but it does require attention to build and deployment, especially as the number of cases will grow as [relaxed-simd] and other extensions ship.
In WASM, since SIMD capabilities are determined at compile time, the `Level` enum will compile to nothing.
When writing code portable to other targets that require runtime detection, it still makes sense to write code using the `Level` enum, but on WASM it has zero runtime cost.

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
There's also a post on [/r/rust](https://www.reddit.com/r/rust/comments/1l5yf3b/a_plan_for_simd/) open for discussion.

I'm posting this now to the Linebender blog to encourage more discussion in the Rust community.
The best place for serious technical discussion is the Zulip thread.
We expect development to continue in the [fearless_simd repo].
The library is not yet usable for broad applications, but it might be possible to start implementing SIMD speedups.
We will be implementing speedups in Vello's CPU and hybrid renderers in the coming weeks, and that will guide our priorities.
We are very interested in feedback about which features are missing, or any other friction; these can be filed as issues against the repo, or raised on the Zulip thread.

[Towards fearless SIMD, 7 years later]: https://linebender.org/blog/towards-fearless-simd/
[fearless_simd#2]: https://github.com/raphlinus/fearless_simd/pull/2
[fearless_simd#3]: https://github.com/raphlinus/fearless_simd/pull/3
[fearless_simd repo]: https://github.com/raphlinus/fearless_simd
[Firefox hardware survey]: https://firefoxgraphics.github.io/telemetry/#view=system
[simdeez]: https://docs.rs/simdeez/latest/simdeez/
[pulp]: https://docs.rs/pulp/latest/pulp/
[std::simd]: https://doc.rust-lang.org/std/simd/index.html
[relaxed-simd]: https://github.com/WebAssembly/relaxed-simd
[Highway]: https://github.com/google/highway
[mersenneforum post]: https://web.archive.org/web/20250526102842/https://www.mersenneforum.org/node/21615#post614191
[Vello]: https://github.com/linebender/vello
[faer]: https://docs.rs/faer/latest/faer/
