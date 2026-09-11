+++
title = "Fearless SIMD v1.0 is here"
authors = ["Shnatsel"]
+++

[`fearless_simd`](https://crates.io/crates/fearless_simd) takes `unsafe` out of SIMD.

It has come a long way since the [original prototype 8 years ago](https://raphlinus.github.io/rust/simd/2018/10/19/fearless-simd.html). We are now confident that whatever it is you need, be it just autovectorization and multiversioning, or full-blown portable SIMD abstractions, or safe access to intrinsics and nothing more, Fearless SIMD will serve you well.

Instead of paraphrasing the [changelog](https://github.com/linebender/fearless_simd/blob/main/CHANGELOG.md), I'd like to take this opportunity to reflect on the goals of Fearless SIMD, how it achieves them, and what sets it apart from other SIMD abstractions.

# Performance

A common criticism leveled at portable SIMD abstractions is that they aren't performant enough, so we've put a lot of effort into making sure that Fearless SIMD never holds you back.

For example, when implementing portable abstractions for operations with different behavior in edge cases on different platforms, such as [swizzles](https://docs.rs/fearless_simd/latest/fearless_simd/trait.SimdBase.html#tymethod.swizzle_dyn) or [floating-point maximum](https://docs.rs/fearless_simd/latest/fearless_simd/trait.SimdBase.html#tymethod.max), we provide both a precise variant that's the same on all platforms, and a fast variant that returns a platform-dependent result, for use when you expect the edge cases to never happen.

We also made it easy to [express SIMD algorithms in terms of the hardware's native vector size](https://github.com/linebender/fearless_simd/blob/main/fearless_simd/examples/sigmoid.rs), so that your code always takes full advantage of the hardware, no matter where it runs. Fixed vector sizes are also supported for algorithms that need them.

We also put a lot of effort into making sure the implementations of our portable SIMD operations are state-of-the-art, and even contributed improvements upstream - both to [Rust](https://shnatsel.github.io/improving-std-simd-swizzle-dyn/) and [LLVM](https://gist.github.com/valadaptive/e37cc66c721b987935ef4d74cab6f3d6).

But if you need an instruction that isn't covered by portable abstractions, or want even more control, you can [safely drop down to platform intrinsics](https://github.com/linebender/fearless_simd/blob/main/fearless_simd/examples/srgb.rs) with no overhead for the parts of your code that need it, and keep the rest simple and portable.

With safe access to intrinsics, there *is* no performance ceiling.

# Safety

If you look up the source code of any other SIMD abstraction, you will find that it is full of `unsafe` code. Something like `rg unsafe` will turn up several *thousands* of `unsafe` blocks. 

But not in Fearless SIMD! The crate is engineered not to require ad-hoc `unsafe` code.

One piece of the puzzle is the [kernel! macro](https://docs.rs/fearless_simd/latest/fearless_simd/macro.kernel.html), which leans on [target feature v1.1](https://rust-lang.github.io/rfcs/2396-target-feature-1.1.html) in the compiler to invoke most SIMD intrinsics without `unsafe`. I have described the design in detail [in an earlier blog post](https://shnatsel.github.io/safe-simd-in-rust-even-on-the-inside/), so check this out if you'd like to learn more.

That removes most of ad-hoc `unsafe`, but doesn't cover SIMD load/store operations which operate on raw pointers. That's where our [safe transmute module](https://github.com/linebender/fearless_simd/blob/850adcae4996e6584fa77443a652c666f2aab126/fearless_simd/src/transmute.rs), inspired by crates such as `bytemuck` and `zerocopy`, comes into play.

SIMD intrinsics like `_mm_loadu_epi32` may seem special, but actually turn into plain loads and stores behind the scenes. So you can fully replicate their functionality with a [single, reusable wrapper](https://github.com/linebender/fearless_simd/blob/850adcae4996e6584fa77443a652c666f2aab126/fearless_simd/src/transmute.rs#L252-L354).

Thanks to the power of Rust's type system, we only need to audit these two small, self-contained building blocks. As long as they are memory-safe, the entire rest of the codebase is guaranteed to be memory-safe as well.

At last, SIMD in Rust can be truly fearless.

We are so confident in this design that we will be providing [3 years of security updates](https://github.com/linebender/fearless_simd/blob/main/fearless_simd/SECURITY.md) for v1.0 and all later versions.

# Ergonomics

Function multiversioning is tricky.

Previous solutions either [require adding `#[inline(always)]` annotations](https://shnatsel.github.io/safe-simd-in-rust-even-on-the-inside/) and understanding their implications, or [impose a small overhead on every function call](https://docs.rs/multiversion/0.9.0/multiversion/). The latter is fine most of the time, but degrades performance on very small functions, and still requires you to surgically add `#[inline(always)]` to get around that.

Both are leaky abstractions - you still need to think of what is happening under the hood.

Alongside `fearless_simd` v1.0, we are launching `fearless_simd_macros` v0.1, which provides a non-leaky abstraction: the `#[simd]` macro. With it, you don't have to think about what's happening under the hood at all! Put it on any SIMD function and it Just Works.


That said, while this is a big step forward for the ecosystem, there is still some boilerplate involved. We are keen to reduce it further, either with compiler support via the [Struct Target Features RFC](https://github.com/rust-lang/rfcs/pull/3525) to get rid of the `#[simd]` annotation entirely, or perhaps through [other tricks](https://github.com/linebender/fearless_simd/pull/375) we will explore in the future.

And if you don't like procedural macros, the old way of doing things is still available, if less convenient.

Ergonomics are the one area we expect may still evolve. But this does not compromise the stability guarantees of the core `fearless_simd` crate, and the code written today will continue working indefinitely.

# Evolution

The current API should be easy to evolve without breaking changes.

While we cannot see the future, there are viable paths to supporting both near-term Rust features, such as the `f16` type, and longer-term features such as [SVE](https://en.wikipedia.org/wiki/AArch64#Scalable_Vector_Extension_(SVE)) and [RISC-V Vector Extension](https://rvv-isadoc.readthedocs.io/en/latest/index.html) if/when these hardware extensions become relevant.

## Relation to std::simd

We would love to see `std::simd` stabilized, but it would not make Fearless SIMD obsolete.

The Rust standard library implements only the parts that absolutely have to be in it, and the rest (multiversioning, hardware-width vectors, etc) is left up to the ecosystem crates.

`fearless_simd` includes an equivalent of `std::simd` that works on stable Rust, but that is just one part of the bigger whole.

Once `std::simd` is stabilized, we will port Fearless SIMD to it to delete a lot of custom code and gain support all sorts of obscure platforms. But the need for ecosystem crates such as `fearless_simd` will remain.

# Adoption

It doesn't matter how brilliant your crate is if nobody is using it.

Fearless SIMD is already used by [30 other crates](https://crates.io/crates/fearless_simd/reverse_dependencies) as a direct dependency, and is undirectly relied on by [over a thousand crates](https://lib.rs/crates/fearless_simd/rev)!

It already underpins a nontrivial fraction of the Rust ecosystem, and we hope that v1.0 will take this even further.

If you'd like to use Fearless SIMD in your project, check out the [documentation](https://docs.rs/fearless_simd/latest/fearless_simd/) and [examples](https://github.com/linebender/fearless_simd/tree/main/fearless_simd/examples), and feel free to ask questions [on Zulip](https://xi.zulipchat.com/#narrow/channel/514230-simd)!
