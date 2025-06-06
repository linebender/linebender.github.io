+++
title = "Towards fearless SIMD, 7 years later"
authors = ["Raph Levien"]
+++

Seven years ago I wrote a blog post [Towards fearless SIMD], outlining a vision for Rust as a compelling language for writing fast SIMD programs.
Where are we now?

Unfortunately, the present-day experience of writing SIMD in Rust is still pretty rough, though there has been progress, and there are promising efforts underway.
As in the previous post, this post will outline a possible vision.

Up to now, Linebender projects have not used SIMD, but that is changing.
As we work on CPU/GPU hybrid rendering techniques, it's clear that we need SIMD to get maximal performance of the CPU side.
We also see opportunities in faster color conversion and accelerated 2D geometry primitives.

This blog post is also a companion to a [podcast] I recorded recently with André Popovitch.
That podcast is a good introduction to SIMD concepts, while this blog post focuses more on future directions.

## A simple example

As a running example, we'll compute a [sigmoid] function for a vector of 4 values.
The scalar version is as follows:

```rust
fn sigmoid(x: [f32; 4]) -> [f32; 4] {
    x.map(|y| y / (1.0 + y * y).sqrt())
}
```

This particular simple code autovectorizes nicely ([Godbolt link](https://rust.godbolt.org/z/TfThE5r33)), but more complex examples often fail to autovectorize, often because of subtle differences in floating point semantics.
(Editorial note: a previous version of this post didn't autovectorize ([Godbolt](https://rust.godbolt.org/z/GoTEK3KT3)) because optimization level was set at `-O`, which is less aggressive than `-C opt-level=3`, the latter of which is the default for release builds)

## Safety

One of the biggest problems with writing SIMD in Rust is that all exposed SIMD intrinsics are marked as `unsafe`, even in cases where they can be used safely.
The reason is that support for SIMD features varies widely, and executing a SIMD instruction on a CPU that does not support it is undefined behavior – the chip can crash, ignore the instruction, or do something unexpected.
To be used safely, there must be some other mechanism to establish that the CPU does support the feature.

Here's the running example in hand-written intrinsic code, showing the need to write `unsafe` to access SIMD intrinsics at all:

```rust
#[cfg(target_arch = "aarch64")]
fn sigmoid_neon(x: [f32; 4]) -> [f32; 4] {
    use core::arch::aarch64::*;
    unsafe {
        let x_simd = core::mem::transmute(x);
        let x_squared = vmulq_f32(x_simd, x_simd);
        let ones = vdupq_n_f32(1.0);
        let sum = vaddq_f32(ones, x_squared);
        let sqrt = vsqrtq_f32(sum);
        let ratio = vdivq_f32(x_simd, sqrt);
        core::mem::transmute(ratio)
    }
}

#[cfg(target_arch = "x86_64")]
fn sigmoid_sse2(x: [f32; 4]) -> [f32; 4] {
    use core::arch::x86_64::*;
    unsafe {
        let x_simd = core::mem::transmute(x);
        let x_squared = _mm_mul_ps(x_simd, x_simd);
        let ones = _mm_set1_ps(1.0);
        let sum = _mm_add_ps(ones, x_squared);
        let sqrt = _mm_sqrt_ps(sum);
        let ratio = _mm_div_ps(x_simd, sqrt);
        core::mem::transmute(ratio)
    }
}
```

This is quite a simplified example.
For one, the SIMD width is fixed at 4 lanes (128 bits).
Most likely, in practice you'd iterate over a larger slice, taking chunks equal to the natural SIMD width.

## Multiversioning

A central problem important for SIMD is multiversioning and runtime dispatch.
In some cases, you know the exact CPU target, for example when compiling a binary you'll run only on your machine (in which case `target-cpu=native` is appropriate).
But when distributing software more widely, there may be a range of capabilities.
For highest performance, it's necessary to compile multiple versions of the code, and do runtime detection to dispatch to the best SIMD code the hardware can run.
This problem was expressed in the original fearless SIMD blog post, and there hasn't been significant advance at the Rust language level since then.

In the C++ world, the [Highway] library provides excellent SIMD support for a very wide range of targets, and also solves the multiversioning problem.
Among other uses are the codecs for the JPEG-XL image format.
Such codecs are an ideal use case for SIMD programming in general, and shipping them in a browser requires a good solution to multiversioning.
Highway has a really good explanation of [their approach to multiversioning][The Multi-SIMD-ISA Dilemma].
It will be useful to study it carefully to see how they've solved various problems.
And a concise way of saying what I'd like to see is "Highway for Rust."

One possible approach is a crate called [multiversion], which uses macros to replicate the code for multiple versions.
A more recent macro-based approach is [rust-target-feature-dispatch].
It is generally a similar approach to multiversion, and the specific differences are set out in that crate's [README](https://github.com/a4lg/rust-target-feature-dispatch/blob/main/src/README.md).

Another approach, as I believe first advocated in my 2018 blog post, is to write functions polymorphic on a zero-sized type representing the SIMD capabilities, then rely on monomorphization to create the various versions.
One motivation for this approach is to encode safety in Rust's type system.
Having the zero-sized token is proof of the underlying CPU having a certain level of SIMD capability, so calling those intrinsics is safe.
A major library that uses this approach is [pulp], which also powers the [faer] linear algebra library.

I started putting together a pulp version of the running example, but ran into the immediate problem that it lacks a `sqrt` intrinsic (this would be easy enough to add, however).
It also works a bit differently, in that it only supports vectors of the natural width, not ones of a fixed width.
For general linear algebra, that's fine, but for some other applications it adds friction, for example colors with alpha are naturally chunks of 4 scalars.
To see an example of pulp code, as well as some discussion, see this [Zulip thread](https://xi.zulipchat.com/#narrow/channel/255911-rust/topic/Rust.20SIMD.20thoughts/near/489370476).

In [fearless_simd#2] I propose a prototype of reasonably-ergonomic SIMD multiversioning.
Like the original fearless_simd prototype, vector data types are polymorphic on SIMD level.
The new prototype goes beyond that in several important ways.
For one, arithmetic traits in std::ops are implemented for vector types, so it's possible to add two vectors together, multiply vectors by scalars, etc.

Here's what the running example looks like in that prototype:

```rust
#[inline(always)]
fn sigmoid_impl<S: Simd>(simd: S, x: [f32; 4]) -> [f32; 4] {
    let x_simd: f32x4<S> = x.simd_into(simd);
    (x_simd / (1.0 + x_simd * x_simd).sqrt()).into()
}

simd_dispatch!(sigmoid(level, rgba: [f32; 4]) -> [f32; 4] = sigmoid_impl);
```

An advantage of the fearless_simd#2 prototype over pulp is a feature for downcasting based on SIMD level, so it's possible to write different code optimized for different chips.
See the [srgb example] in that pull request for more detail.
Though there are clear advantages, at this point I'm not sure whether this is the direction to go.
It would be a lot of work to build out all the needed types and operations, with potentially a large amount of repetitive boilerplate code in the library, which in turn may cause issues with compile time.
Another possible direction is a smarter, compiler-like proc macro which synthesizes the SIMD intrinsics as needed based on the types and operations in the source program.

One additional consideration for Rust is that the implementation of runtime feature detection is [slower than it should be][slow feature detection].
Thus, feature detection and dispatch shouldn't be done at every function call.
A good working solution is to do feature detection once, at the start of the program, then pass that token down through function calls.
It's workable but definitely an ergonomic paper cut.

## FP16 and AVX-512

A general trend in parallel computation, really fueled by AI workloads, is smaller scalars with higher throughputs.
While not yet common on x86_64, the FP16 extension is supported on all Apple Silicon desktop CPUs and most recent high-ish end ARM-based phones.
Since Neon is only 128 bits wide, having 8 lanes is welcome.
I find the f16 format to be especially useful for pixel values, as it can encode color values with more than enough precision to avoid visual artifacts (8 bits is not quite enough, though it is good enough for some applications, as long as you're not trying to do HDR).

Native Rust support for the `f16` type has not yet landed (tracked in [rust#125440]), which makes use of this scalar size harder.
However, there is some support in the [half] library, and also the [fearless_simd#2] prototype exports a number of FP16 Neon instructions through inline assembly.
When true f16 support lands, it will be possible to switch over to intrinsics, which will have better optimization and ergonomics (for example, the same method will splat constants converted to `f16` at compile time and `f32` variables to be converted at runtime).

AVX-512 is a somewhat controversial SIMD capability.
It first appeared in the ill-fated Larrabee project, which shipped in limited numbers as the Xeon Phi starting in 2010, and has since appeared in scattered Intel CPUs, but with compromises.
In particular, sprinkling even a small amount of AVX-512 code into a program could result in downclocking, reducing performance for all workloads (see [Stack Overflow thread on throttling] for more details).
These days, the most likely way to get a CPU with AVX-512 is an AMD Zen 4 or Zen 5; it is on their strength that AVX-512 makes up about 16% of computers in the Steam hardware survey.

The increased width is not the main reason to be enthusiastic about AVX-512.
Indeed, on Zen 4 and most Zen 5 chips, the datapath is 256 bits so full 512 bit instructions are "double pumped." The most exciting aspect is predication based on masks, a common implementation technique on GPUs.
In particular, memory load and store operations are safe when the mask bit is zero, which is especially helpful for using SIMD efficiently on strings.
Without predication, a common technique is to write two loops, the first handling only even multiples of the SIMD width, and a second, usually written as scalars, to handle the odd-size "tail".
There are lots of problems with this - code bloat, worse branch prediction, inability to exploit SIMD for chunks slightly less than the natural SIMD width (which gets worse as SIMD grows wider), and risks that the two loops don't have exactly the same behavior.

Going forward, Intel has proposed AVX10, and will hopefully ship AVX 10.2 chips in the next few years.
This extension has pretty much all of the features of AVX-512, with some cleanups and new features (until recently, AVX10 was defined has having a 256 bit base width and optionally 512, but 512 is now the baseline).
In addition, AVX10.2 will include 16-bit floats (currently available only in the Sapphire Rapids high-end server and workstation chips).

## About std::simd

The "portable SIMD" work has been going on for many years and currently has a home as the nightly [std::simd].
While I think it will be very useful in many applications, I am not personally very excited about it for my applications.
For one, because it emphasizes portability, it encourages a "lowest common denominator" approach, while I believe that for certain use cases it will be important to tune algorithms to best use the specific quirks of the different SIMD implementations.
For two, std::simd does not itself solve the multiversion problem.
From my perspective, it's probably best to consider it as a souped-up version of autovectorization.

## Language evolution

Rust's out of the box support for SIMD is still quite rough, especially the need to use `unsafe` extensively.
While some of the gap can be filled with libraries, arguably it should be a goal of the language itself to support safe SIMD code.
There is progress in this direction.

First, the original version of `target_feature` requires `unsafe` to call into *any* function annotated with `#[target_feature]`.
A proposal to relax that so that functions already under a target_feature gate can call safely call into another function with the same gate is called "[target_feature 1.1]" and is scheduled to ship in 1.86.
Closely related, once inside the suitable target_feature gate, the majority of SIMD intrinsics (broadly, those that don't do memory access through pointers) should be considered safe by the compiler, and that feature (safe intrinsics in core::arch) is also in flight.

There's more that can be done to help the Rust compiler recognize when SIMD use is safe, in particular to allow target_features when a concrete witness to the SIMD level is passed in as a function argument.
The "struct target_features" proposal ([RFC 3525]) enables target_feature in such cases, and is one of the proposals considered in the proposed Rust project goal [Nightly support for ergonomic SIMD multiversioning].

In general, improving Rust SIMD support will require both libraries and support in the Rust language.
Different approaches at the library level may indicate different language features to best support them.

## Looking forward

My main goal in putting these prototypes forward, as well as writing these blog posts, is to spark conversation on how best to support SIMD programming in Rust.
If done well, it is a great opportunity for the language, and fits in with its focus on performance and portability.

As we build out the [Vello hybrid CPU/GPU renderer], performance of the CPU components will rely heavily on SIMD, so we need to invest in writing a lot of SIMD code.
The most conservative approach would be hand-writing unsafe intrinsics-based code for all targets, but that's a lot of work and the use of unsafe is unappealing.
I'd love for the Rust ecosystem can come together and build good infrastructure, competitive with Highway.
For now, I think it's time to carefully consider the design space and try to come to consensus on what that should look like.

[Towards fearless SIMD]: https://raphlinus.github.io/rust/simd/2018/10/19/fearless-simd.html
[pulp]: https://docs.rs/pulp/latest/pulp/
[faer]: https://docs.rs/faer/latest/faer/
[multiversion]: https://docs.rs/multiversion/latest/multiversion/
[half]: https://docs.rs/half/latest/half/
[Nightly support for ergonomic SIMD multiversioning]: https://rust-lang.github.io/rust-project-goals/2025h1/simd-multiversioning.html
[Highway]: https://github.com/google/highway
[The Multi-SIMD-ISA Dilemma]: https://github.com/kfjahnke/zimt/blob/multi_isa/examples/multi_isa_example/multi_simd_isa.md
[slow feature detection]: https://internals.rust-lang.org/t/better-codegen-for-cpu-feature-detection/22083
[target_feature 1.1]: https://rust-lang.github.io/rfcs/2396-target-feature-1.1.html
[safe intrinsics in core::arch]: https://github.com/rust-lang/libs-team/issues/494
[RFC 3525]: https://github.com/rust-lang/rfcs/pull/3525
[fearless_simd#2]: https://github.com/raphlinus/fearless_simd/pull/2
[srgb example]: https://github.com/raphlinus/fearless_simd/pull/2/files#diff-be8aece917a9235076ff8ec42749b1f1a803d2a3cbc2ccdd5425b405c74f7436
[rust#125440]: https://github.com/rust-lang/rust/issues/125440
[std::simd]: https://doc.rust-lang.org/std/simd/index.html
[Stack Overflow thread on throttling]: https://stackoverflow.com/questions/56852812/simd-instructions-lowering-cpu-frequency#comment100256395_56852812
[rust-target-feature-dispatch]: https://github.com/a4lg/rust-target-feature-dispatch
[sigmoid]: https://raphlinus.github.io/audio/2018/09/05/sigmoid.html
<!-- TODO: not sure if this is the best link, we don't really have a project page for this -->
[Vello hybrid CPU/GPU renderer]: https://xi.zulipchat.com/#narrow/channel/197075-vello/topic/Potato.20-.20a.20paper.20design.20for.20a.20CPU.2FGPU.20hybrid.20renderer
[podcast]: https://www.youtube.com/watch?v=y0WcCUKxk50
