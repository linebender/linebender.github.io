+++
title = "Linebender in April 2025"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Masonry

Masonry is the widget system developed by Linebender.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

- [xilem#913][], [xilem#938][]: Ongoing work on properties, in preparation for adding more styling options.

<!-- Virtual Scroll -->

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI, which uses Masonry for its widgets.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

<!-- Virtual Scroll, again -->

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

<!-- 0.5.0, with wgpu 24 -->

We have also renamed the #gpu channel on Zulip to [#vello](https://xi.zulipchat.com/#narrow/channel/197075-vello), which better reflects the purpose of the channel.
Progress on the sparse strips renderer has also been continuing at pace:

<!-- Need a lot more plans -->
<!-- Link to the new renderer roadmap document  -->

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

<!-- TODO -->

## Kurbo

Kurbo provides data structures and algorithms for curves and vector paths.

<!-- Kurbo 0.11.2 -->

## Color

[Color][] provides functionality for representing, converting, parsing, serializing, and manipulating colors in a variety of color spaces.
It closely follows the [CSS Color Module Level 4][] draft spec.

<!-- Color release, other details -->

## Kompari

[Kompari][] is a tool for visual inspection of snapshot tests.

<!-- TODO -->

## Android View

[Android View](https://github.com/mwcampbell/android-view) is a platform integration for Rust code in Android apps.

<!-- TODO: Text, etc. -->

## Research and Future Directions

Linebender has an origin story in being a very research oriented group, looking to break new ground.
While we are focused on shipping code today, we still have an eye on the future and how to be prepared for the new opportunities and technologies that are coming.

<!-- UI Events -->

## RustWeek

Many members of the Linebender community will be attending the [RustWeek 2025](https://rustweek.org/) conference, in Utrecht in May.
<!-- TODO: check this At the time of writing, there are still tickets available. -->
Raph Levien will be giving a talk, titled [*Faster, easier 2D vector rendering*](https://rustweek.org/talks/raph/).
This will be covering a lot of the sparse strips work discussed in [the Vello section](#vello).

Matt Campbell will give a talk, titled [*AccessKit: reusable UI accessibility*](https://rustweek.org/talks/matt/).

<!-- Mention unconf? It's a ticketed and published event, so should be reasonably safe -->

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

[xilem#913]: https://github.com/linebender/xilem/pull/913
[xilem#938]: https://github.com/linebender/xilem/pull/938
