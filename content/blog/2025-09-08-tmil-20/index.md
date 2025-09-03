+++
title = "Linebender in August 2025"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

- [vello#1096][]: Updated to Wgpu version 26.
- [vello#1161][]: Simplifies the API for rendering a wgpu `Texture` into a Vello `Scene`.
- [vello#1169][]: Increased the Skrifa version to v0.35.0. This was backported as the [v0.5.1](https://github.com/linebender/vello/releases/tag/v0.5.1) release.
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

## Masonry

Masonry is the widget system developed by Linebender.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

<!-- <figure>

<img style="height: auto" width="521" height="420" src="masonry_new_style.png" alt="A todo list app, with items referring to aspects of the new design language, namely 'New Colours', 'Increased Consistency', and 'More Rounded Corners'. The item labelled 'A full design system' is unchecked.">

<figcaption>

As of [xilem#1096][] Masonry's default styles have been improved.
This is not a full design system, but is a piecewise improvement.

</figcaption>
</figure> -->

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI, which uses Masonry for its widgets.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

Placehero:

## Anymore

<!-- We released version 1.0. -->

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

## Kurbo

<!-- TODO -->

## Get Involved

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

We host an hour long office hours meeting each week where we discuss what's going on in our projects.
See [#office hours in Zulip](https://xi.zulipchat.com/#narrow/channel/359642-office-hours) for details.
We're also running a separate office hours time dedicated to the renderer collaboration, details also available at that link.

If you wish to discuss the Linebender project individually, Daniel is offering ["office hours" appointments](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2), which are free to book.
It really helps us to learn what aspects our users care about the most.

[vello#1064]: https://github.com/linebender/vello/pull/1064

[Kurbo 0.11.3]: https://github.com/linebender/kurbo/releases/tag/v0.11.3
