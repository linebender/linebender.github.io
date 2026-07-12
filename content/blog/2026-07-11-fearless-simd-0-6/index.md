+++
title = "fearless_simd v0.6 brings AVX-512 and ways to disable it"
authors = ["Shnatsel"]
+++

[`fearless_simd`](https://crates.io/crates/fearless_simd) takes `unsafe` out of SIMD.

No matter what level of abstraction you're after, be it autovectorization and multiversioning, portable SIMD, or safe access to raw intrinsics and nothing more, `fearless_simd` has you covered!

It features zero dependencies, from-scratch build time under 1 second, safe public APIs, and [very little](https://gist.github.com/Shnatsel/61fc294987a1e051ce3835c97dc0fc19) `unsafe` under the hood - orders of magnitude less than the alternatives!

The major addition in v0.6 is [AVX-512](https://en.wikipedia.org/wiki/AVX-512) support for all operations, including support for safely invoking AVX-512 intrinsics via the [kernel!](https://docs.rs/fearless_simd/latest/fearless_simd/macro.kernel.html) macro.

## Why AVX-512

AVX-512 is rather poorly named.
It would be better to call it AVX3, because extending the vector width to 512 bits is the least interesting addition, and many implementations process 512-bit vectors on 256-bit execution units anyway.

The real meat of AVX-512 is in the new instructions it brings, as well as in doubling the number of registers for _all_ vector widths, or even more if you use masks.
So you can see significant performance gains from AVX-512 without ever touching 512-bit vectors.

AVX-512 has a bad rep due to the early implementations from Intel being quite poor.
[Intel Skylake](https://en.wikipedia.org/wiki/Skylake_(microarchitecture)) processors would [immediately reduce the frequencies](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#Downclocking) of all CPU cores upon encountering a single AVX-512 instruction anywhere.
This usually hurts performance more than it helps.

The downclocking issue was fixed by Intel in [Ice Lake](https://en.wikipedia.org/wiki/Ice_Lake_(microprocessor)), and was never present on AMD.
Because of that `fearless_simd` **only enables AVX-512 on Ice Lake and later** on Intel.
Using Ice Lake as a baseline also gives us access to more instructions, enabling more efficient implementations of some operations.

We've already seen a [15% end-to-end runtime improvement](https://github.com/linebender/fearless_simd/pull/231#issuecomment-4760357890) in [Vello CPU](https://github.com/linebender/vello/tree/main/sparse_strips/vello_cpu) from enabling AVX-512 compared to the previous AVX2 path.
This was measured on AMD Zen 4, which only has 256-bit execution units; the gains on CPUs with native 512-bit vectors are likely greater.
And it was achieved without any code changes, simply by upgrading fearless_simd from v0.5 to v0.6.

## Why not AVX-512

Hardware has quirks.
For example, on AMD Zen4, AVX-512 scatter/gather instructions are slower than their scalar equivalent, but LLVM will sometimes emit them when automatically vectorizing loops, resulting in a [dramatic performance drop](https://github.com/llvm/llvm-project/issues/91370) on AVX-512 compared to AVX2 in some cases.
Intel has same scatter/gather performance pothole on AVX2 in Haswell.

## Controlling the instruction sets

Since the entire point of SIMD is performance, it is crucial for any SIMD abstraction to provide enough control to work around hardware quirks.

In addition, every x86 instruction set increases the binary size, because each SIMD function needs to be separately compiled for every instruction set and the final one selected at runtime.
This can be mitigated by compiling with e.g.
`-C target-cpu=x86-64-v3`, which will make the compiled program assume that AVX2 is always present, and omit the SSE4.2 and fallback code.
But such a program will crash on CPUs without AVX2.
This also would still compile the AVX-512 implementations, but getting rid of them can be desirable if you know the hardware you target doesn't have AVX-512 or doesn't benefit from it for your workload.
So the `target-cpu` flag is useful but doesn't provide enough flexibility.

That's why fearless_simd now provides controls for disabling certain instruction sets, both [for select functions](https://github.com/linebender/fearless_simd/blob/3d10e36bae31987855e784de907de308803f90e5/fearless_simd/examples/disable_avx2_for_one_function.rs#L1) and [for an entire binary](https://github.com/linebender/fearless_simd/tree/main/fearless_simd#multiversioning-on-x86).
The defaults are fine for most users, but the flexibility is there if you need it.

It's also worth noting that if binary size is a concern, you should try setting [`codegen-units=1`](https://nnethercote.github.io/perf-book/build-configuration.html#codegen-units) or [`lto=true`](https://nnethercote.github.io/perf-book/build-configuration.html#link-time-optimization) in your Cargo.toml before resorting to disabling SIMD instruction sets.

### Why not Cargo features?

We have considered using Cargo features to control which instruction sets get compiled, but they turned out to be a poor fit.
Cargo features are a library-level switch, but the decision on which instruction sets are to be included should be made by whoever compiles the final binary, not the library authors.
(If you are a library author and you really want to get rid of a certain instruction set for parts of the code, you can still [opt out in code rather than config](https://github.com/linebender/fearless_simd/blob/3d10e36bae31987855e784de907de308803f90e5/fearless_simd/examples/disable_avx2_for_one_function.rs)).
Also, if any dependency anywhere in your tree enables a Cargo feature, you cannot get rid of it without forking and patching the dependency.

We instead use `cfg` flags, which turned out to be the perfect fit.
They can be set by whoever builds the final binary and apply to the entire dependency tree.
You can run `RUSTFLAGS=--cfg=disable_dispatch_avx512 cargo build --release` for a one-off build without AVX-512, or set the configuration via [`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html) if you want to use it for all builds of your project.
See [the README](https://github.com/linebender/fearless_simd/tree/main/fearless_simd#multiversioning-on-x86) for details.

This design is somewhat unconventional, but we believe it provides the best solution for this particular problem.
You can also easily combine it with `-C target-cpu` flags, and the configuration for both is very similar!

## Let us know what you think!

This release closes the last major gap in `fearless_simd` in AVX-512 support and expands on the crate's flexibility.
We believe `fearless_simd` can become _the_ foundational SIMD crate for the Rust ecosystem, whether you want [automatic vectorization](https://github.com/linebender/fearless_simd/tree/main/fearless_simd#automatic-vectorization), [portable SIMD](https://github.com/linebender/fearless_simd/tree/main/fearless_simd#portable-simd), [safe access to intrinsics](https://github.com/linebender/fearless_simd/tree/main/fearless_simd#explicit-intrinsics), or all of the above.

Are you building something exciting with it? Is there something `fearless_simd` is missing? Please let us know [on Zulip](https://xi.zulipchat.com/#narrow/channel/514230-simd/topic/v0.2E6.2E0/with/609515457)!
