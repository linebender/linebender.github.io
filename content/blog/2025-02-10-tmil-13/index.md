+++
title = "Linebender in January 2025"
authors = ["Bruce Mitchener", "Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

<!-- ## Peniko - Elided in TMIL-13, nothing relevant to discuss. Yes there was a release, but only to bump dependencies -->

## Color

[Color][] provides functionality for representing, converting, parsing, serializing, and manipulating colors in a variety of color spaces.
It closely follows the [CSS Color Module Level 4][] draft spec.

We released [Color 0.2.3][], which added easier methods to convert 8-bit colors from byte streams (for use with GPUs) and to other color representations in [color#135][] and [color#136][].
This release also saw the addition of the ACES2065-1 color space in [color#124][].

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

We released Vello 0.4.0

- [xilem#791][]: Update to wgpu 0.24
...

<!-- Screenshot of image extend modes -->

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

<!-- Testing improvements, whitespace improvements, no_std improvement -->

## Kompari

<!-- Repository rework, interactive review, multiple diff methods -->

## Masonry

Masonry is the widget system used by Xilem.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

<!-- Docs, Box<dyn widget> cleanup, inspector, transforms, zstack -->

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

<!-- Docs contributed, pod flexibility, emoji picker landed,  -->

## Research and Future Directions

Linebender has an origin story in being a very research oriented group, looking to break new ground.
While we are focused on shipping code today, we still have an eye on the future and how to be prepared for the new opportunities and technologies that are coming.

We have a couple of active and ongoing research projects:

- Raph Levien is working on a version of Vello which has bounded GPU memory usage. <!-- Codenamed? Potato. Is that the final name? -->

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
