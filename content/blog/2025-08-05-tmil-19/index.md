+++
title = "Linebender in July 2025"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

<!-- TODO -->
- [vello#...][]:

This month's progress on the sparse strips renderers, a collaborative evolution of Vello, has been centered around applying SIMD <!-- TODO -->.

<!-- TODO -->
- [vello#...][]:

Our [working roadmap](https://docs.google.com/document/d/1ZquH-53j2OedTbgEKCJBKTh4WLE11UveM10mNdnVARY/edit?tab=t.0#heading=h.j3duh9pgdm94) outlines the planned timeline for work on the renderers into next year.

An integration of Vello as the backend for Servo's canvas rendering has landed in <!-- TODO --> (and also Vello CPU in <!-- TODO -->).

<!-- TODO: Image of Canvas drawn with Vello? -->

### Fearless SIMD

TODO: A new blurb here?

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

## Anymore

We have factored a shared utility for use in both Xilem and Masonry...
<!-- TODO -->

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
