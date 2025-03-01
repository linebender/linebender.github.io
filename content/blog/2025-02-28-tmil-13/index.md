+++
title = "Linebender in January 2025"
authors = ["Kaur Kuut", "Daniel McNab", "Tom Churchman", "Raph Levien"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

<!-- Docs contributed, pod flexibility, emoji picker landed,  -->

## Masonry

Masonry is the widget system used by Xilem.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

<!-- Docs, Box<dyn widget> cleanup, inspector, transforms, zstack -->

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

We released Vello 0.4.0

- [xilem#791][]: Update to wgpu 0.24
...

<!-- Screenshot of image extend modes -->

## Kurbo

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.
In January, a variety of layout edge cases have been fixed, support for bidirectional text was expanded, and testing of text layout and selection has improved.

- [parley#244][]: Adds screenshots to selection area and cursor positioning tests,
- [parley#241][]: Allows configuring the behavior of alignment for lines that overflow the container width,
- [parley#245][]: Enables detection of the base direction of text layouts (left-to-right or right-to-left),
- [parley#249][]: Fixes issues related to line breaking around inline boxes, by new Linebender member Wim de With,
- [parley#256][]: Corrects the calculation of trailing white space, also by Wim de With,
- [parley#254][]: Follows the [CSS white space model][] in stripping leading white space following a new line in white space collapsing mode; and
- [parley#239][]: Takes another step towards supporting `no_std`.

## Peniko

<!-- there was a release, but only to bump dependencies -->

## Color

[Color][] provides functionality for representing, converting, parsing, serializing, and manipulating colors in a variety of color spaces.
It closely follows the [CSS Color Module Level 4][] draft spec.

We released [Color 0.2.3][], which added easier methods to convert 8-bit colors from byte streams (for use with GPUs) and to other color representations in [color#135][] and [color#136][].
This release also saw the addition of the ACES2065-1 color space in [color#124][].

## Velato

<!-- Vello 0.4 https://github.com/linebender/velato/pull/49 -->

## Vello SVG

<!-- new release and stuff -->

## Kompari

<!-- Repository rework, interactive review, multiple diff methods -->

## Resvg

- [resvg#873][]: Added the stylesheet option to the C API.
- [resvg#878][]: Updated to `svgtypes` 0.15.3.

## SVG Types

We released [SVG Types 0.15.3] with a few minor fixes.
This is also the first release under the stewardship of Linebender.

## Tiny Skia

- [tiny-skia#146][]: Add `scale_by`/`scale_to` functions to `Size`.

## SimpleCSS

We released [SimpleCSS 0.2.2] with `no_std` support, updated docs clarifying Linebender involvement, and to test run the publishing workflow.

## Research and Future Directions

Linebender has an origin story in being a very research oriented group, looking to break new ground.
While we are focused on shipping code today, we still have an eye on the future and how to be prepared for the new opportunities and technologies that are coming.

While Vello has unmatched rendering speed thanks to its GPU-driven architecture, there are two practical tradeoffs: it can require unpredictable amounts of memory, and it doesn't work on downlevel GPUs with weak or nonexistent support for compute shaders.
Raph Levien has been exploring a possible hybrid CPU/GPU direction for Vello to address these issues.
Read the [design doc][Potato design doc], follow the [Zulip thread][Potato zulip thread], or stay tuned for more developments.

## Get Involved

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

We host an hour long office hours meeting each week where we discuss what's going on in our projects.
See [#office hours in Zulip](https://xi.zulipchat.com/#narrow/channel/359642-office-hours) for details.

* Daniel and Olivier's "office hours" appointments can still be booked by anyone for open-ended discussion of the ecosystem.
  * [See Daniel's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2).
  * [See Olivier's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2t767ZRETD_TkRI_VxK2ZTG0VrO9OZ4l7HvTxefhtJcg85iK0ZN7zWNnAEZtH0Dn7C1GKxrmYM).

[Color]: https://docs.rs/color/
[CSS Color Module Level 4]: https://www.w3.org/TR/css-color-4/
[Color 0.2.3]: https://github.com/linebender/color/releases/tag/v0.2.3
[color#124]: https://github.com/linebender/color/pull/124
[color#135]: https://github.com/linebender/color/pull/135
[color#136]: https://github.com/linebender/color/pull/136

[parley#239]: https://github.com/linebender/parley/pull/239
[parley#241]: https://github.com/linebender/parley/pull/241
[parley#244]: https://github.com/linebender/parley/pull/244
[parley#245]: https://github.com/linebender/parley/pull/245
[parley#249]: https://github.com/linebender/parley/pull/249
[parley#254]: https://github.com/linebender/parley/pull/254
[parley#256]: https://github.com/linebender/parley/pull/256
[CSS white space model]: https://www.w3.org/TR/CSS2/text.html#white-space-model

[resvg#873]: https://github.com/linebender/resvg/pull/873
[resvg#878]: https://github.com/linebender/resvg/pull/878

[SVG Types 0.15.3]: https://github.com/linebender/svgtypes/releases/tag/v0.15.3

[tiny-skia#146]: https://github.com/linebender/tiny-skia/pull/146

[SimpleCSS 0.2.2]: https://github.com/linebender/simplecss/releases/tag/v0.2.2

[Potato design doc]: https://docs.google.com/document/d/1gEqf7ehTzd89Djf_VpkL0B_Fb15e0w5fuv_UzyacAPU/edit?usp=sharing
[Potato zulip thread]: https://xi.zulipchat.com/#narrow/channel/197075-gpu/topic/Potato.20-.20a.20paper.20design.20for.20a.20CPU.2FGPU.20hybrid.20renderer
