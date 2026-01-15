+++
title = "Linebender in December 2025"
authors = ["Kaur Kuut"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Vello

Vello is our vector renderer with three different implementations.
There is the fully GPU compute based Vello, the fully CPU based Vello CPU, and the hybrid GPU/CPU one called Vello Hybrid.
They can draw large 2D scenes with high performance.

- [vello#1294][]: Added features to Vello CPU to switch between `u8` and `f32` pipelines.
- [vello#1327][]: Eliminated overdraw for opaque image fills.
- [vello#1325][]: Reduced the memory usage of wide tile commands.
- [vello#1303][]: Fixed filter expansion logic for transforms with scale/skew and for clipped layers.
- [vello#1313][]: Fixed gradients within a clip layer in Vello Hybrid.
- [vello#1323][]: Fixed non-deterministic GPU stroke artifacts.

<figure>
<img style="height: auto;" width="1280" height="960" src="image_overdraw.jpg" alt="A photo of a flower being layered on top of itself at decreasing size.">
<figcaption>
30% performance improvement in this flower's case with the new overdraw handling in Vello CPU / Vello Hybrid.
</figcaption>
</figure>

The work on rendering sparse strip alpha values in GPU compute shaders continued in [vello#1293][], which added tile intersection checking.
You can follow the progress in [#vello > Thoughts on GPU sparse strips](https://xi.zulipchat.com/#narrow/channel/197075-vello/topic/Thoughts.20on.20GPU.20sparse.20strips/with/567687539).

## Masonry and Xilem

Masonry is the widget system developed by Linebender.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

Xilem is our flagship GUI project, inspired by SwiftUI, which uses Masonry for its widgets.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

- [xilem#1519][]: Added `Canvas` widget, for 2d vector drawing.
- [xilem#1510][]: Added a new `CollectionWidget` trait to unify collection widget method naming.
- [xilem#1527][]: Added `Gap` property to `Flex` and `Grid`.
- [xilem#1528][]: Added alternative text to `Image` widget.
- [xilem#1526][]: Implemented `Into<BrushRef>` for `BorderColor`.
- [xilem#1534][]: Migrated to Kurbo's `Axis`.
- [xilem#1488][]: Renamed `map_message` to `map_message_result`.
- [xilem#1533][]: Renamed `Grid` attributes from `width` / `height` to `row_count`, `column_count`.
- [xilem#1484][]: Split Android examples into separate files.
- [xilem#1503][]: Split some Xilem code into new a new `xilem_masonry` package.
- [xilem#1504][]: Fixed `TextInput` placeholder alignment.
- [xilem#1537][]: Fixed `ScrollBar` behavior with large content sizes.
- [xilem#1540][]: Improved `Flex` child constraint accuracy.
- [xilem#1507][]: Wrote guidelines for writing Xilem doc examples.
- [xilem#1544][]: Documented node non-persistence in accessibility method.
- [xilem#1545][]: Now using third person more consistently in docs.

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

- [parley#436][]: Migrated text analysis and internationalization to ICU4X.
- [parley#479][], [parley#481]: Started work on a dedicated glyph rendering crate.
- [parley#475][]: Now using ICU4X `Script` type in `fontique`.
- [parley#473][]: We now bake composite properties data.
- [parley#487][]: Export `Tag` from HarfRust, to allow users to use it directly.
- [parley#490][]: Fixed bidi state leaking across layouts.
- [parley#493][]: Fixed crash with empty layout.
- [parley#498][]: Updated to `ui-events` 0.2.

## Kurbo

Kurbo provides data structures and algorithms for curves and vector paths.

- [kurbo#534][]: Optimized `RoundedRect::winding`.
- [kurbo#533][]: Clarified the order of `PathEl` points.

## Peniko

Peniko is a 2D graphics type library which provides a set of generic types that define styles for rendering and composition.

- [peniko#155][]: Updated to Kurbo 0.13.

## Fearless SIMD

Fearless SIMD is our SIMD infrastructure library.
It provides a solid way for writing SIMD operations portably across Wasm, AArch64, x86, and x86_64.

- [fearless_simd#141][]: Added `any_true`, `all_true`, `any_false`, and `all_false` methods for mask types.
- [fearless_simd#167][]: Added precise float-to-integer conversions, which saturate out-of-bounds results and convert `NaN` to `0` on all platforms.
- [fearless_simd#168][]: Added `Level::is_fallback`, which states whether the current SIMD level is the scalar fallback.
- [fearless_simd#155][]: Added a vectored shift left operation.
- [fearless_simd#154][]: Added documentation for operations and vector types.
- [fearless_simd#149][]: Reworked the API.
- [fearless_simd#158][]: Renamed `madd` / `msub` to `mul_add` / `mul_sub` for consistency with `std`.
- [fearless_simd#159][]: Now using native vector types to back SIMD types.
- [fearless_simd#170][]: Made `Element` an associated type on `SimdBase`.
- [fearless_simd#180][]: `SimdFrom::simd_from` now takes the SIMD token as the first argument instead of the second.

## Velato

Velato is our Lottie render library.
The goal is to provide coverage of the large Lottie spec, up to what Vello can render, for use in interactive graphics.

We released [Velato 0.8][] and [0.8.1][Velato 0.8.1]. 🎉

## Bevy Vello

Bevy Vello is our Bevy integration for Vello.
The goal is to provide support for rendering scenes, text, SVGs, and Lotties in the Bevy game engine.

We released [Bevy Vello 0.11][], [0.12][Bevy Vello 0.12], and [0.12.1][Bevy Vello 0.12.1]. 🎉

- [bevy_vello#185][]: Added support for `UiTransform` for raw Vello scenes.
- [bevy_vello#188][]: Finished adding Bevy 0.17 support.
- [bevy_vello#191][]: Renamed `SSRenderTarget` to `VelloRenderTarget`.
- [bevy_vello#195][], [bevy_vello#196][]: Fixed AABBs not being correctly calculated.

## Resvg

Resvg is our fast, small, portable SVG library.

- [resvg#980][], [resvg#981][]: No longer writing empty `<defs />`.
- [resvg#987][]: Now using checked arithmetic when computing bounding box.
- [resvg#990][]: Fixed panicking during pixmap creation.

## SimpleCSS

SimpleCSS is a basic CSS 2.1 parser and selector.

- [simplecss#39][]: Added support for parsing `@font-face`.

## Druid

Druid is a GUI library which was a predecessor to Xilem, and an ancestor of Masonry.

- [druid#2410][]: Updated to Piet 0.7, which is going to be the final Piet version compatible with Druid.

## Get Involved

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

We host an hour long office hours meeting each week where we discuss what's going on in our projects.
See [#office hours in Zulip](https://xi.zulipchat.com/#narrow/channel/359642-office-hours) for details.
We're also running a separate office hours time dedicated to the renderer collaboration, details also available at that link.

[vello#1293]: https://github.com/linebender/vello/pull/1293
[vello#1294]: https://github.com/linebender/vello/pull/1294
[vello#1303]: https://github.com/linebender/vello/pull/1303
[vello#1313]: https://github.com/linebender/vello/pull/1313
[vello#1323]: https://github.com/linebender/vello/pull/1323
[vello#1325]: https://github.com/linebender/vello/pull/1325
[vello#1327]: https://github.com/linebender/vello/pull/1327

[xilem#1484]: https://github.com/linebender/xilem/pull/1484
[xilem#1488]: https://github.com/linebender/xilem/pull/1488
[xilem#1503]: https://github.com/linebender/xilem/pull/1503
[xilem#1504]: https://github.com/linebender/xilem/pull/1504
[xilem#1507]: https://github.com/linebender/xilem/pull/1507
[xilem#1510]: https://github.com/linebender/xilem/pull/1510
[xilem#1519]: https://github.com/linebender/xilem/pull/1519
[xilem#1526]: https://github.com/linebender/xilem/pull/1526
[xilem#1527]: https://github.com/linebender/xilem/pull/1527
[xilem#1528]: https://github.com/linebender/xilem/pull/1528
[xilem#1533]: https://github.com/linebender/xilem/pull/1533
[xilem#1534]: https://github.com/linebender/xilem/pull/1534
[xilem#1537]: https://github.com/linebender/xilem/pull/1537
[xilem#1540]: https://github.com/linebender/xilem/pull/1540
[xilem#1544]: https://github.com/linebender/xilem/pull/1544
[xilem#1545]: https://github.com/linebender/xilem/pull/1545

[parley#436]: https://github.com/linebender/parley/pull/436
[parley#473]: https://github.com/linebender/parley/pull/473
[parley#475]: https://github.com/linebender/parley/pull/475
[parley#479]: https://github.com/linebender/parley/pull/479
[parley#481]: https://github.com/linebender/parley/pull/481
[parley#487]: https://github.com/linebender/parley/pull/487
[parley#490]: https://github.com/linebender/parley/pull/490
[parley#493]: https://github.com/linebender/parley/pull/493
[parley#498]: https://github.com/linebender/parley/pull/498

[kurbo#533]: https://github.com/linebender/kurbo/pull/533
[kurbo#534]: https://github.com/linebender/kurbo/pull/534

[peniko#155]: https://github.com/linebender/peniko/pull/155

[fearless_simd#141]: https://github.com/linebender/fearless_simd/pull/141
[fearless_simd#149]: https://github.com/linebender/fearless_simd/pull/149
[fearless_simd#154]: https://github.com/linebender/fearless_simd/pull/154
[fearless_simd#155]: https://github.com/linebender/fearless_simd/pull/155
[fearless_simd#158]: https://github.com/linebender/fearless_simd/pull/158
[fearless_simd#159]: https://github.com/linebender/fearless_simd/pull/159
[fearless_simd#167]: https://github.com/linebender/fearless_simd/pull/167
[fearless_simd#168]: https://github.com/linebender/fearless_simd/pull/168
[fearless_simd#170]: https://github.com/linebender/fearless_simd/pull/170
[fearless_simd#180]: https://github.com/linebender/fearless_simd/pull/180

[Velato 0.8]: https://github.com/linebender/velato/releases/tag/v0.8.0
[Velato 0.8.1]: https://github.com/linebender/velato/releases/tag/v0.8.1

[Bevy Vello 0.11]: https://github.com/linebender/bevy_vello/releases/tag/v0.11.0
[Bevy Vello 0.12]: https://github.com/linebender/bevy_vello/releases/tag/v0.12.0
[Bevy Vello 0.12.1]: https://github.com/linebender/bevy_vello/releases/tag/v0.12.1
[bevy_vello#185]: https://github.com/linebender/bevy_vello/pull/185
[bevy_vello#188]: https://github.com/linebender/bevy_vello/pull/188
[bevy_vello#191]: https://github.com/linebender/bevy_vello/pull/191
[bevy_vello#195]: https://github.com/linebender/bevy_vello/pull/195
[bevy_vello#196]: https://github.com/linebender/bevy_vello/pull/196

[resvg#980]: https://github.com/linebender/resvg/pull/980
[resvg#981]: https://github.com/linebender/resvg/pull/981
[resvg#987]: https://github.com/linebender/resvg/pull/987
[resvg#990]: https://github.com/linebender/resvg/pull/990

[simplecss#39]: https://github.com/linebender/simplecss/pull/39

[druid#2410]: https://github.com/linebender/druid/pull/2410
