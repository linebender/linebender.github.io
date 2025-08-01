+++
title = "Linebender in July 2025"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

- [vello#785][]: Strokes with a width of zero are no longer treated as fills.
- [vello#908][]: Updates to wgpu 25 release.
- [vello#1093][]: Disabled runtime checks in shaders using wgpu's new `create_shader_module_trusted` API, by sagudev.

This month's progress on the sparse strips renderers, a collaborative evolution of Vello, has been centered around adding support for NEON and WASM SIMD, as well as making further improvements to multi-threaded rendering.

- [vello#1064][], [vello#1086][]: Image rendering in Vello Hybrid.
- [vello#1078][]: Rewrote Vello CPU to be more SIMD-friendly.
- [vello#1092][]: Adds support for SIMD flattening.
- [vello#1103][]: Optimises alpha coverage calculation in strip rendering.
- [vello#1105][]: Ignores paths containing NaN points, giving a warning.
- [vello#1122][]: Adds opacity layers to Vello Hybrid.
- [vello#1134][]: Reuses FlattenCtx for paths.
- [kurbo#427][]: Contains a new stroke expander with significant performance improvements.

<!-- TODO: Clean up long sentences, hedge, ensure link text is meaningfuly, etc. -->
In order to get a better understanding of where we lie performance-wise, we included `tiny-skia` and `vello-cpu` in the [Blend2D benchmark harness](https://blend2d.com/performance.html), a comprehensive benchmarking tool that tests the performance of different parts of a 2D renderer and compares it against other renderers.
In order to do so, a [fork](https://github.com/LaurenzV/blend2d-apps/tree/benching) of the benchmark harness was created that uses C bindings to both libraries to include them in the harness.
To visualize the results, we created a chart, similar to how it is available on the official Blend2D website.
We do want to emphasize again that full credit for building this benchmark tool goes to the Blend2D team, and we merely extended it to also include `tiny-skia` and `vello-cpu`.

The results can be viewed [here](https://laurenzv.github.io/vello_chart/), the source code for generating the charts is available [here](https://github.com/LaurenzV/vello_chart).

Some things that should be noted here:

- Note that these are only preliminary results, and there are plans for making further improvements, especially for multi-threading.
- We currently do not support x86 SIMD, which is why they are currently not included in the chart.
  It should be noted that the [README](https://github.com/linebender/tiny-skia?tab=readme-ov-file#performance) of `tiny-skia` explicitly mentions that performance on ARM is worse than on x86, so we expect the performance gap to be smaller on x86.

Nevertheless, by looking at this chart, it is clear that `vello-cpu` has very impressive performance and on track to become the fastest CPU-based renderer in the Rust ecosystem! When taking all renderers into consideration, Blend2D is still the clear winner in terms of raw performance, but `vello-cpu` does end up taking the second place in many of the benchmarks and beating other renderers such as Skia and Cairo, especially as the size of the geometry gets larger. Similarly to Blend2D, `vello-cpu` also offers a multi-threaded rendering mode, which is especially effective when drawing larger geometries with curves or when using complex paints such as gradients or patterns.

Our [working roadmap](https://docs.google.com/document/d/1ZquH-53j2OedTbgEKCJBKTh4WLE11UveM10mNdnVARY/edit?tab=t.0#heading=h.j3duh9pgdm94) outlines the planned timeline for work on the renderers into next year.

An integration of Vello as the backend for Servo's canvas rendering has landed in [servo#36821][] (and also Vello CPU in [servo#38282][]).

<!-- TODO: Screenshot of chart, maybe? -->

### Fearless SIMD

<!-- TODO: Clean up this blurb -->
<!-- TODO: Do we need to say "experimental" for literally everything? -->
Fearless SIMD is our experimental SIMD infrastructure library, developed for Vello sparse strips.

- [fearless_simd#24][], [fearless_simd#26][], [fearless_simd#27][]: Implement all WASM SIMD methods.
<!-- TODO: Any others which are relevant? -->

## Masonry

Masonry is the widget system developed by Linebender.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

- [xilem#...][]:
<!-- TODO -->

<!-- <figure>

<img style="height: auto" width="1464" height="955" src="multiple_windows.png" alt="Three overlapping windows on a black background. The frontmost window is titled Second Window, has text displaying a count of 11, a plus button, and a minus button, stacked vertically. Behind it is First Window, which is the same with a count of 13. At the back is a window titled Multiple Windows, which shows a map from the aforementioned windows to their values above a textbox and Add button. The textbox contains the text Next Window.">

<figcaption>

As of [xilem#1038][] Masonry (and Xilem) support multiple windows.

</figcaption>
</figure> -->

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI, which uses Masonry for its widgets.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.
Our work on Placehero, which is the working name for our Mastodon client example, has inspired several significant architectural improvements in Xilem.

- [xilem#...][]:
<!-- TODO -->

<!-- Image of Xilem Chess GUI, if we have permission. -->

## Anymore

You might have noticed mentions of [Anymore](https://github.com/linebender/anymore) in both the Xilem and Masonry sections.
This is a new crate which we've created for the `AnyDebug` trait.
This allows creating dynamically typed values which can be inspected, making debugging downcasting failures much easier.
This crate is designed for stability, so that it can be used for interoperability between projects (without allocation).
We plan to release version 1.0 in early August.

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

- [parley#...][]:

<!-- ## TODO: The other project news items? -->

## Get Involved

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

We host an hour long office hours meeting each week where we discuss what's going on in our projects.
We're also running a separate office hours time dedicated to the renderer collaboration, details also available at that link.
See [#office hours in Zulip](https://xi.zulipchat.com/#narrow/channel/359642-office-hours) for details.

If you wish to discuss the Linebender project individually, Daniel is offering ["office hours" appointments](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2), which are free to book.
It really helps us to learn what aspects our users care about the most.

<!-- [Fearless SIMD]: https://github.com/linebender/fearless_simd
[#simd channel]: https://xi.zulipchat.com/#narrow/channel/514230-simd -->
