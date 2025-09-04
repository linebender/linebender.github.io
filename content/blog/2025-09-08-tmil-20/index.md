+++
title = "Linebender in August 2025"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

- [parley#400][]: Migrates Parley to [HarfRust](https://github.com/harfbuzz/harfrust) for shaping, which is a Rust port of HarfBuzz.
- [parley#405][]: Adds a selection of benchmarks.
- [parley#406][], [parley#413][]: Adds internal caching, improving performance significantly.

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

- [vello#1096][]: Updated to Wgpu version 26.
- [vello#1161][]: Simplifies the API for rendering a wgpu `Texture` into a Vello `Scene`.
- [vello#1169][]: Increased the Skrifa version to v0.35.0. This was backported as [Vello v0.5.1](https://github.com/linebender/vello/releases/tag/v0.5.1).
- [vello#1182][]: Fixed a longstanding issue with our examples, where they would crash if the window was minimised on windows.

The biggest updates to the sparse strip renderers this month are the support for blending and caching in Vello Hybrid.
We have also had continued SIMD improvements, including x86 support.

- [vello#1136][]: Allows disabling anti-aliasing.
- [vello#1137][], [vello#1153][]: Add support for caching sparse strips.
- [vello#1149][]: Support for SSE4.2 and AVX2 SIMD (i.e. on x86 and x86_64).
  The [benchmark results viewer](https://laurenzv.github.io/vello_chart/) has been updated to include some x86_64 results.
- [vello#1177][]: Move to the new v0.2.0 release of Fearless SIMD.
- [vello#1155][], [vello#1178][]: Add full support for blending to Vello Hybrid.

<!-- TODO: This roadmap is a bit out of date.
Our [working roadmap](https://docs.google.com/document/d/1ZquH-53j2OedTbgEKCJBKTh4WLE11UveM10mNdnVARY/edit?tab=t.0#heading=h.j3duh9pgdm94) outlines the planned timeline for work on the renderers into next year. -->

We have additionally been doing further work on Vello's imaging model.
This is partly in service of the integrations of Vello in [Graphite](https://graphite.rs/) and [Servo](https://servo.org/).

- [color#191][], [peniko#115][] (in progress): Add support for interpolating gradients in unpremultiplied space.
  This is provided for web compatibility, and shouldn't be used by most users.
- [peniko#117][]: Split Image into the sampler and data.
- [peniko#120][], [peniko#121][]: Add `Brga` and premultiplied image formats.
- [vello#1183][] (in progress): Support luminance masks.

As of [servo#38962][], Vello and Vello CPU are the only canvas backends used in Servo.

<!-- TODO: Image for blending in Vello Hybrid? -->

### Fearless SIMD

Fearless SIMD is our SIMD infrastructure library.
We are developing it in concert with the Vello sparse strips renderers, particularly `vello_cpu`.
We released [Fearless SIMD v0.2.0](https://github.com/linebender/fearless_simd/releases/tag/v0.2.0), the first Fearless SIMD release since 2018.
This included the following changes from August:

- [fearless_simd#50][], [fearless_simd#52][]: Add SSE4.2 support.
- [fearless_simd#58][]: Add some initial docs.
- [fearless_simd#59][]: Add basic AVX2 support.
- [fearless_simd#61][]: Removes `f16` support to prepare for release.
- [fearless_simd#69][]: Fixes a soundness issue with unsafe SIMD functions.

<!-- TODO: With this release, Fearless SIMD is now ready for experimentation by other users.
We do have further planned architectural changes, so... -->

## Masonry

Masonry is the widget system developed by Linebender.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

- [xilem#1207][]: Make `ObjectFit`, `Linebreaking` and `SpinnerColor` properties.
- [xilem#1212][]: Add `Widget::Action`, which indicates the action type widgets return.
- [xilem#1226][]: Refactor tab focus.
- [xilem#1228][]: Add placeholder text to text input.
- [xilem#1250][]: Move widgets to use a `Length` type, which represents a number of logical pixels.
- [xilem#1269][]: Introduce "focus fallback" mechanism.
- [xilem#1273][]: Update to Anymore v1.0.
- [xilem#1282][], many others: Add unit tests for passes.
- [xilem#1295][]: Merge `TextColor` and `SpinnerColor` into `ContentColor`.
- [xilem#1310][]: Cleans up how the event loop handles being suspended, by Nixon.
- [xilem#1312][]: Remove the generic on VirtualScroll.
- [xilem#1314][]: Makes the focused indicator for checkboxes larger, by tannal.
- [xilem#1320][]: Add `NewWindow` struct to encapsulate attributes needed to create a window.
- [xilem#1321][]: When a new font is loaded, relayout everything.
- [xilem#1332][]: Restore base colour customisation.
- [xilem#1349][]: Move Masonry's examples from "Masonry Winit" to "Masonry".
- [xilem#1353][]: Limit the size of screenshot tests to 8KiB by default.
- [xilem#1366][]: Make the `Checkbox`'s checked state be controlled only by the driver.
- [xilem#1371][]: Fix and optimise window resizing and minimising.

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI, which uses Masonry for its widgets.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

- [xilem#1260][]: Make the virtual scroll implementation much saner.
- [xilem#1273][]: Update to Anymore v1.0.
- [xilem#1363][]: Fix the `declare_property_tuple` macro, by Alex Moon.
- [xilem#1278][]: Add `Prop`, which changes how Masonry `Properties` are applied to more closely match Xilem Web.
- [xilem#1318][]: Fix initial font loading in easy cases.
- [xilem#1333][]: Add docs about precise capturing, by Nils Martel.
- [xilem#1348][]: Add `flex_row` and `flex_col` as convenience functions, by Paul Xu.
- [xilem#1352][]: Add error handling to virtual cats example, by Nils Martel.
- [xilem#1357][]: Fix how the `disabled` property is handled for text input.
- [xilem#1364][]: Remove the `app_state` parameter from teardown, by Alex Moon.

Placehero:

- [xilem#1323][]: Use an Action for navigation (rather than directly setting fields on the app state).
- [xilem#1337][]: Use virtual scrolling for timelines.
- [xilem#1370][]: Show the blurhash of media attachments.

<!-- <figure>

TODO: Screenshot for blurhashes and virtual scrolling in Placehero?

<img style="height: auto" width="521" height="420" src="masonry_new_style.png" alt="A todo list app, with items referring to aspects of the new design language, namely 'New Colours', 'Increased Consistency', and 'More Rounded Corners'. The item labelled 'A full design system' is unchecked.">

<figcaption>

As of [xilem#1096][] Masonry's default styles have been improved.
This is not a full design system, but is a piecewise improvement.

</figcaption>
</figure> -->

## Anymore

On 4 August, we released [Anymore v1.0.0](https://github.com/linebender/anymore/releases/tag/v1.0.0).
This is our crate for the fundamental `AnyDebug` trait, which greatly aids debugging when using dynamically typed values.
Anymore is designed to be used in public APIs; it has a minimal API surface so that it can be extremely stable.
We recommend considering it for use cases where you would otherwise be returning (potentially wrapped) `Box<dyn Any>` values from your libraries.

## Kurbo

<!-- TODO: -->

## Get Involved

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

We host an hour long office hours meeting each week where we discuss what's going on in our projects.
See [#office hours in Zulip](https://xi.zulipchat.com/#narrow/channel/359642-office-hours) for details.
We're also running a separate office hours time dedicated to the renderer collaboration, details also available at that link.

If you wish to discuss the Linebender project individually, Daniel is offering ["office hours" appointments](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2), which are free to book.
It really helps us to learn what aspects our users care about the most.

[vello#1064]: https://github.com/linebender/vello/pull/1064

[servo#38962]: https://github.com/servo/servo/pull/38962
