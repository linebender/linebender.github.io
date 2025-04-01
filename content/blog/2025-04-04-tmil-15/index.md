+++
title = "Linebender in March 2025"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

- [xilem#887][] ... by ....
- [xilem#]

## Masonry

Masonry is the widget system used by Xilem.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

Development continues on the properties feature we mentioned last month:

- [xilem#873][], implementing properties, was merged.
- [xilem#892][]

We have also made a major change to the internal design of Masonry, in [xilem#910][] and [xilem#914][].
This change splits Masonry into two packages, `masonry_core` and `masonry`.
The Masonry crate provides a runner for Masonry core using Winit, and should be used for an integrated app development experience.
Masonry Core contains the implementation of Masonry's widgets and passes, and is intended to be be embedded into existing apps which use wgpu.
This should be transparent to existing Masonry users.

We have also made significant progress outside of these highlighted areas:

- [xilem#882][] ... <!-- Screenshot? -->
- [xilem#897][] ... (includes optimisation?)
- [xilem#904][] ...
- [xilem#899][]

We hoped to release a new alpha of Xilem and Masonry to crates.io in March.
However, this was put on hold whilst the Properties experiment continues.

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

Work continues on the Sparse Strips renderer, introduced in full [in February's update][].

<!-- TODO: Whittle down the most important changes -->
<!-- - [xilem#879][]: TODO: Do we want to "announce" Android View yet? -->

<!-- TODO: Screenshot of WebGL example, in e.g. Firefox on Linux? -->

You can follow the progress in [#vello > Vello Hybrid](https://xi.zulipchat.com/#narrow/channel/197075-vello/topic/Vello.20Hybrid) and other threads in [#vello](https://xi.zulipchat.com/#narrow/channel/197075-vello).
<!-- We also are hosting weekly renderer office hours specifically aimed at developing this (? - do we want this) -->
<!-- TODO: Mention new mini roadmap? -->

We have also renamed the #gpu channel in Zulip to #vello, to better reflect the realities of what is discussed there.

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

## Resvg

<!-- ? maybe nothing to talk about -->

## Kurbo

Kurbo provides data structures and algorithms for curves and vector paths.

## Color

[Color][] provides functionality for representing, converting, parsing, serializing, and manipulating colors in a variety of color spaces.
It closely follows the [CSS Color Module Level 4][] draft spec.

## Velato

## Kompari

[Kompari][] is a tool for visual inspection of snapshot tests.

## Research and Future Directions

Linebender has an origin story in being a very research oriented group, looking to break new ground.
While we are focused on shipping code today, we still have an eye on the future and how to be prepared for the new opportunities and technologies that are coming.

## Rustweek

Many members of the Linebender community will be attending the [Rustweek 2025](https://rustweek.org/) conference, in Utrecht in May.
At the time of writing, there are still tickets available.
Raph will be giving a talk, titled [*Faster, easier 2D vector rendering*](https://rustweek.org/talks/raph/).
This will be covering a lot of the sparse strips work discussed in [the Vello section](#vello).
<!-- TODO: Write something here -->

## Get Involved

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

We host an hour long office hours meeting each week where we discuss what's going on in our projects.
See [#office hours in Zulip](https://xi.zulipchat.com/#narrow/channel/359642-office-hours) for details.
We've also started a separate office hours time dedicated to the renderer collaboration, details also available at that link.

- Daniel and Olivier's "office hours" appointments can still be booked by anyone for open-ended discussion of the ecosystem.
  - [See Daniel's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2).
  - [See Olivier's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2t767ZRETD_TkRI_VxK2ZTG0VrO9OZ4l7HvTxefhtJcg85iK0ZN7zWNnAEZtH0Dn7C1GKxrmYM).

[Color]: https://docs.rs/color/
[CSS Color Module Level 4]: https://www.w3.org/TR/css-color-4/

[Kompari]: https://github.com/linebender/kompari
