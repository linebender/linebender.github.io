+++
title = "fearless_simd v0.7: 64-bit integers, improved generics, SSE2, and upcoming v1.0"
authors = ["Shnatsel"]
+++

[`fearless_simd`](https://crates.io/crates/fearless_simd) takes `unsafe` out of SIMD.

No matter what level of abstraction you're after, be it autovectorization and multiversioning, portable SIMD, or safe access to raw intrinsics and nothing more, `fearless_simd` has you covered!

It features zero dependencies, short build times, safe public APIs, and [very little](https://gist.github.com/Shnatsel/61fc294987a1e051ce3835c97dc0fc19) `unsafe` under the hood - orders of magnitude less than the alternatives!

The major additions in v0.7 are support for 64-bit integers, an explicit SSE2 level replacing scalar fallback on x86, improved support for generic programming, and more implemented SIMD operations.

This is also the last major release prior to Fearless SIMD v1.0, if no concerns about the API are raised.

## 64-bit integers

The entire API surface of `fearless_simd` is now also exposed for `u64` and `i64` vector types.

Earlier releases didn't support 64-bit integer vectors due to hardware support being spotty. For example, AVX2 lacks hardware support for many operations on 64-bit integers, so it would require emulation using the available SIMD instructions for those operations to get decent performance.

Keeping track of which intrinsics are part of which instruction set was also challenging, and getting it wrong would be a memory safety violation. However, in v0.5 we [made the compiler keep track of it for us](https://shnatsel.github.io/safe-simd-in-rust-even-on-the-inside/), which removed the vast majority of `unsafe` blocks from `fearless_simd` and made implementing operations with uneven hardware support much easier.

All the other integer vector types (`i8`,`u8`,`i16`,`u16`,`i32`,`u32`) and `f32`/`f64` were already supported by previous releases, so 64-bit integers were the last missing piece for full type coverage. We will investigate supporting `f16` once the type is stabilized in the standard library.

## More operations

 - `swizzle_dyn` is now implemented for all widths to allow arbitrary byte shuffles. I've also [contributed performance improvements](https://shnatsel.github.io/improving-std-simd-swizzle-dyn/) for this operation to `std::simd`. Unlike `std::simd`, Fearless SIMD supports both zeroing out-of-bounds indices and returning implementation-defined (but memory-safe) results for when you're sure all indices are in bounds, which is cheaper than zeroing on some platforms.
 - All types can now be widened/narrowed; e.g. you can convert vectors of `u8` to `u16`, or `u16` to `u8` in SIMD code. Narrowing conversions can wrap, like `as` operator, or saturate, at your option.
 - Added convenience functions `shift_elements_left`, `shift_elements_right`, `rotate_elements_left`, and `rotate_elements_right` for better compatibility with `std::simd` API. They could already be implemented in terms of `slide`, but this makes the intent more clear.

## Improved generic programming

Support for generic programming - writing functions that are generic over the vector type - has been substantially improved. Here are just a few highlights:

 - The `SimdBase` trait now abstracts over both integer and float vectors, and implements all methods available on both integers and floats.
 - Improvements to the `Bytes` trait allow generic bitcasts (safe transmutes) between SIMD vectors.
 - Every single operation on SIMD types is now available through a trait. There are no remaining operations implemented only for concrete types.
 - Associated types such as `SimdBase::Element` and `SimdBase::Array` now encode a lot of generic bounds to allow generic operations on them.

See the full changelog for details. All in all, generic SIMD programming is now much more pleasant, and allows expressing more algorithms generically.

These improvements also benefit users who abstract over SIMD vector types using macros, writing e.g. `$type::from_slice` instead of `T::from_slice`. They no longer need additional crates such as [`paste`](https://crates.io/crates/paste) to inject types into function names, since all operations are now available on the types themselves.

## Explicit SSE2 support

These days x86 systems without SSE4.2 are [very rare](https://firefoxgraphics.github.io/telemetry/#view=system). However, since SSE2 is part of the baseline instruction set in both x86_64 and i686 Rust targets, the presence of SSE2 can be assumed, without any runtime dispatch or multiversioning. Certain crates only need a very limited set of vector instructions and don't benefit from later extensions, so forgoing runtime dispatch can simplify the code and reduce binary size.

To better serve this use case, Fearless SIMD now has an explicit `Sse2` level with operations expressed in terms of SIMD intrinsics, rather than relying on autovectorization of the `Fallback` level when SSE4.2 is not available.

SSE2 remains a runtime-detected level on the [tier-2 i586 targets](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-2-without-host-tools), and can be disabled there using [the usual multiversioning controls](https://github.com/linebender/fearless_simd/tree/main/fearless_simd#multiversioning-on-x86).

## Build time improvements

Despite the addition of 64-bit integer vectors, more supported operations, and an entirely new SSE2 SIMD level, the compilation time of `fearless_simd` when used as a dependency stayed the same as v0.6: 2 seconds from scratch for x86 and 1 second from scratch for Aarch64. This is measured via `cargo clean && cargo build --release --timings`.

Keeping compilation time unchanged despite the additions required a build profiling and optimization effort, without which the x86 build time would have increased to 3.4 seconds on my machine. It's still not that much for a from-scratch release build, and would have been entirely invisible for crates that have other dependency chains that take longer than 3.4 seconds to compile. But I believe that keeping build times low is important for Fearless SIMD to become a foundational SIMD abstraction. This is also a big part of why `fearless_simd` doesn't have any dependencies itself.

The vast majority of `fearless_simd` API is made up of generic functions. They emit no machine code unless instantiated, so you don't pay for them in build time or binary size if you don't use them. The compiler only needs to type-check and borrow-check them, so the build time is almost entirely frontend-bottlenecked. On nightly Rust, using `RUSTFLAGS=-Zthreads=8` to enable the parallel frontend halves the compilation time, so we have further improvements to look forward to once the parallel frontend is enabled in the stable toolchain.

## API cleanups prior to 1.0

This release saw the type-specific `reinterpret_*` methods removed. These methods only existed for a handful of types, and each pair of types would need a separate method, which is impractical. So these were simply dropped in favor of the generic `bitcast` method. The `load_interleaved_128` operation was renamed to `load_four_interleaved`, to make the intent more clear.

These are fairly minor API cleanups, and the API has been stable for nearly a year. Barring the recent changes to generics, the API has been proven in real-world projects, with over a dozen direct dependents on crates.io, and over a thousand github repositories using it directly or indirectly.

We are confident in the design of the crate, and it's time to make it official. If no concerns about the API are raised, we are going to **ship v1.0 of Fearless SIMD in early September.**

There are no further changes planned for v1.0, and we are postponing this release purely to allow some time for feedback from the community before finalizing the API.

## A call for feedback

We believe `fearless_simd` can become _the_ foundational SIMD crate for the Rust ecosystem, whether you want [automatic vectorization](https://github.com/linebender/fearless_simd/tree/main/fearless_simd#automatic-vectorization), [portable SIMD](https://github.com/linebender/fearless_simd/tree/main/fearless_simd#portable-simd), [safe access to intrinsics](https://github.com/linebender/fearless_simd/tree/main/fearless_simd#explicit-intrinsics), or all of the above. This goal is now within sight.

Now more than ever, we'd like to hear from you! Are any APIs awkward or missing? Or is everything working smoothly? Please let us know either way!

You can either open an issue on the [Github repository](https://github.com/linebender/fearless_simd/issues), or talk to us [on Zulip](https://xi.zulipchat.com/#narrow/channel/514230-simd).
