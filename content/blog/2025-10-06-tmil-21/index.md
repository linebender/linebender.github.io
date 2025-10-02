+++
title = "Linebender in September 2025"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

<!-- TODO -->
- [][]:

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

- [vello#1183][]: Support for luminance masks.
- [vello#1187][]: Fix flattening tolerance calculation.
- [vello#1192][]: Add `push_clip_layer`, in preparation for removing `Mix::Clip`.
- [vello#1224][]: Update to the latest Peniko main.
- [vello#1229][]: Put wgpu's default features behind a `wgpu_default` feature flag, by Stefan Tammer.

We're planning to make a Vello release early in October for compatibility with Bevy v0.17.

Our sparse strip renderers are moving towards maturity.
In September, we have had some improvements in Vello Hybrid's capabilities, and further performance optimisation.

- [vello#1188][]: Make the aliasing threshold configurable.
- [vello#1196][]: Gradient rendering in Vello Hybrid.
- [vello#1206][]: Store data about filled areas in a strip.
- [vello#1209][]: Optimise the `estimate_num_squads` method.
- [vello#1215][]: Glyph caching for hinting instances and outline paths.
- [vello#1221][]: Unify our sweep gradient handling.
- [vello#1239][]: Release [Vello CPU v0.2.0](https://github.com/linebender/vello/releases/tag/sparse-stips-v0.0.2). This release was a final release for Peniko v0.4.x compatibility, and we are planning to release Vello CPU v0.1.0 within the first half of October.

We're planning to make the beta release of Vello CPU in early October.
<!-- TODO: What to say exactly here? -->
Its performance is now extremely competitive - according to [our benchmarking](https://laurenzv.github.io/vello_chart/) is likely the fastest CPU-only renderer in Rust.
<!-- TODO: We'd like to thank Laurenz, something about Master's project, etc. -->

<!-- TODO: This roadmap is a bit out of date.
Our [working roadmap](https://docs.google.com/document/d/1ZquH-53j2OedTbgEKCJBKTh4WLE11UveM10mNdnVARY/edit?tab=t.0#heading=h.j3duh9pgdm94) outlines the planned timeline for work on the renderers into next year. -->
<figure>

<!-- <img style="height: auto; width: 50%; margin-left: 25%;" width="666" height="673" src="hybrid-blending.png" alt="A series of overlapping shapes of various colours, variously composited."> -->

<figcaption>

<!-- TODO: What Image? Gradients in Vello Hybrid? Performance improvements since July? -->

</figcaption>
</figure>

We expect to have Linebender represented at the Graphite Community Meetup, details of which can be found in [their post](https://graphite.rs/blog/graphite-community-meetup-in-germany/).

### Peniko

<!-- TODO: Add screenshot. -->

### Linebender Resource Handle

<!-- Linebender -->
Talk about previous state, dependencies on Peniko, Cosmic text, etc.

## Masonry

Masonry is the widget system developed by Linebender.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

<!-- TODO -->
-[][]:

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI, which uses Masonry for its widgets.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

<!-- TODO -->
- [][]:

<!-- TODO: If there were Placehero updates, they go here. -->

<!-- TODO: Add screenshot. -->

## Kurbo

Kurbo provides data structures and algorithms for curves and vector paths.
<!-- TODO: We released v0.12 -->

<!-- TODO -->
- [][]:

## Fearless SIMD

Fearless SIMD is our SIMD infrastructure library.
It provides a solid way for writing SIMD operations portably across WASM, Aarch64, x86, and x86_64.
The improvements we made in September include:

- [][]:

We look forward to seeing ecosystem use cases develop alongside our maturing.

## Get Involved

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

We host an hour long office hours meeting each week where we discuss what's going on in our projects.
See [#office hours in Zulip](https://xi.zulipchat.com/#narrow/channel/359642-office-hours) for details.
We're also running a separate office hours time dedicated to the renderer collaboration, details also available at that link.

If you wish to discuss the Linebender project individually, Daniel is offering ["office hours" appointments](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2), which are free to book.
It really helps us to learn what aspects our users care about the most.
