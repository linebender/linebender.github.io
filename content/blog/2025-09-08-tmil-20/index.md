+++
title = "Linebender in August 2025"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

<!-- - [vello#xxx][]: Some content. -->

This month's progress on the sparse strips renderers, a collaborative evolution of Vello, has been centered around adding support for NEON and WASM SIMD, as well as making further improvements to multi-threaded rendering.

<!-- - [vello#xxx][]: Some content. -->

Our [working roadmap](https://docs.google.com/document/d/1ZquH-53j2OedTbgEKCJBKTh4WLE11UveM10mNdnVARY/edit?tab=t.0#heading=h.j3duh9pgdm94) outlines the planned timeline for work on the renderers into next year.

<!-- Servo update -->

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
