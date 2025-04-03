+++
title = "Linebender in March 2025"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

- [xilem#887][]: Added a View for the already existing Split widget.
- [xilem#899][]: Added some more documentation.

## Masonry

Masonry is the widget system used by Xilem.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

We have made a major change to the internal design of Masonry, in [xilem#910][] and [xilem#914][].
This change splits Masonry into two packages, `masonry_core` and `masonry`.
The Masonry crate provides a runner for Masonry core using Winit, and should be used for an integrated app development experience.
Masonry Core contains the implementation of Masonry's widgets and passes, and is designed to be be embedded into existing apps (currently only those which use wgpu).
This should be transparent to existing Masonry users.

We hoped to release a new alpha of Xilem and Masonry to crates.io in March.
However, this was put on hold whilst the Properties experiment we mentioned last month continues:

- [xilem#873][]: Implements properties, which is additional data attached to each widget in a uniform way, intended to enable styling.
- [xilem#892][]: Uses properties for styling the built-in `Button` widget.

<!-- TODO: Screenshot of new capability from button styling. Deferred to Olivier. Non-blocking -->

We have also made significant progress outside of these highlighted areas:

- [xilem#897][]: Reduced the size of many of our screenshot tests, to better.
- [xilem#904][]: Implemented a new algorithm for screenshot testing, which better reflects the aspects of the screenshots we wish to test.
  It also implements automatic compression of blessed screenshots using the excellent [oxipng][] library.
- [xilem#899][]: Added documentation for lots of items and methods which were missing documentation.
- [xilem#882][]: Adds the core of a virtual scrolling widget, to be used with medium or large scroll areas.

 <!-- TODO: Screenshot of virtual scrolling? -->

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

- [vello#841][]: Fixed incorrect COLR Emoi rendering, which especially impacted users on Windows.
  We backported this to create [Vello 0.4.1](https://github.com/linebender/vello/releases/tag/v0.4.1).
- [vello#817][], [vello#877][]: Updates some of our GPU encoding code to not assume that allocators will give over-aligned allocations.

We have also renamed the #gpu channel on Zulip to [#vello](https://xi.zulipchat.com/#narrow/channel/197075-vello), which better reflects the purpose of the channel.
Progress on the sparse strips renderer has also been continuing at pace:

- [vello#827][]: Added some core API types.
- [vello#828][]: Imported the implementation of sparse strip production, without rasterisation (Vello Common).
- [vello#830][], [vello#832][]: Added on CPU rasterisation of sparse strips, with integration tests (Vello CPU).
- [vello#834][]: Added a tool (Vello Toy) for visualising different stages of the new pipeline.
- [vello#831][]: Added the fragment shader based GPU rasteriser for sparse strips (Vello Hybrid).
- [vello#872][]: Enable running Vello Hybrid using WebGL2.
- [vello#867][]: Benchmarking harness for Vello Common and Vello CPU.

<!-- TODO: Screenshot of WebGL example, in e.g. Firefox on Linux? -->

The current state of the sparse strip renderers is that they can reliably render scenes where all elements are a solid colour.
<!-- TODO: Mention new mini roadmap? -->
There are some PRs where you can track ongoing work:

- [vello#878][]: Add clipping to Vello CPU.
- [vello#883][]: Implements text/glyph rendering.
- [vello#879][]: Runs Vello Hybrid on Android. <!-- Is now the time to announce Android View? In what form? -->

<!-- Maybe? Screenshot relevant for clipping? -->

You can also follow the progress in [#vello > Vello Hybrid](https://xi.zulipchat.com/#narrow/channel/197075-vello/topic/Vello.20Hybrid) and other threads in [#vello](https://xi.zulipchat.com/#narrow/channel/197075-vello).
We also are hosting weekly renderer office hours specifically aimed at developing this collaboration.
Information about how to join these, follow [#office hours](https://xi.zulipchat.com/#narrow/channel/359642-office-hours).

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.
The biggest news in March was that an egui community member has opened [a PR][egui#5784] to replace egui's bespoke text handling with Parley, with buy-in from egui's maintainer.
The author of that change, valadaptive, has been doing fantastic work improving Parley, and has authored many of these changes.

- [parley#306][]: Updated `register_fonts` to take a `Blob`, which allows more efficient memory usage.
- [parley#312][]: Allows customising how registered fonts are matched, useful for implementing web fonts.
- [parley#296][]: Makes selected newline characters visible.
- [parley#316][], [parley#317][]: Provides greater access to the internals of `PlainEditor`.
- [parley#318][]: `is_done` method on `BreakLines`, for detecting whether linebreaking is finished.
- [parley#299][]: Fixed text editing for layouts containing inline boxes.

<!-- TODO: Screenshot of egui using Parley? -->

## Kurbo

Kurbo provides data structures and algorithms for curves and vector paths.

## Color

[Color][] provides functionality for representing, converting, parsing, serializing, and manipulating colors in a variety of color spaces.
It closely follows the [CSS Color Module Level 4][] draft spec.

## Kompari

[Kompari][] is a tool for visual inspection of snapshot tests.

- [kompari#26][]: Added support for optimising screenshots using `oxipng`.
- [kompari#27][]: Support detecting unoptimised screenshots (useful for CI).
- [kompari#34][]: Added parallel processing of compute intensive tasks.

## Research and Future Directions

Linebender has an origin story in being a very research oriented group, looking to break new ground.
While we are focused on shipping code today, we still have an eye on the future and how to be prepared for the new opportunities and technologies that are coming.

## Rustweek

Many members of the Linebender community will be attending the [Rustweek 2025](https://rustweek.org/) conference, in Utrecht in May.
At the time of writing, there are still tickets available.
Raph will be giving a talk, titled [*Faster, easier 2D vector rendering*](https://rustweek.org/talks/raph/).
This will be covering a lot of the sparse strips work discussed in [the Vello section](#vello).
<!-- TODO: Anything more to say here? Do we want to announce the Linebender time/day, or should it be kept to Zulip and office hours? -->

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

[egui#5784]: https://github.com/emilk/egui/pull/5784
