+++
title = "Linebender in October 2024: resvg Adoption"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

This month's update is very significant, featuring several crate releases and the adoption of resvg.

## Resvg

We would like to thank [Yevhenii Reizner (RazrFalcon)](https://github.com/RazrFalcon) for all their work on resvg.
The project is...

The biggest update from this month is that .

### Relicense

We are in the process of relicensing the resvg crate from their current MPL-2.0.
This allows us to more easily integrate existing Linebender code, and enables greater compatibility.
This was inspired by a similar move made by wgpu (TODO: Link).
This process can be followed at [resvg#838][].

### Plans

We have no immediate plans for changes in resvg or tiny skia.
We are currently serving.

---

<p></p>

Alongside the work on resvg, we have been continuing work on our existing projects.

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

- [xilem#681][]: `View::rebuild` was updated to reflect the ability to use reborrowing.
- [xilem#669][]: The font weight for `label`s was exposed.

<figure>

<!-- <img style="height: auto" src="http_cats.png" alt="A list of HTTP status codes, with 'Select' buttons. HTTP code 418 'I'm a teapot' is selected, with a picture of a kitten hiding in a teapot." height="962" width="600"> -->

<figcaption>

<!-- The HTTP cats example. -->

</figcaption>
</figure>

Work on Xilem Web continues.

## Masonry

Masonry is the widget system used by Xilem.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

- [xilem#632][]: Starts a Masonry book, including a guide on creating a To-Do List app.
- [rfcs#7][]: The pass specification RFC has been fully implemented, and the RFC has been merged.
- [xilem#705][]: Moved methods for mutating widgets to free functions.
  - This decreases ergonomics in favour of increasing uniformity between Masonry and crates which create their own widgets.
  - This change is intended to improve rustdoc output, and the ergonomics should be restored once Rust has Arbitrary Self Types V2.
- [xilem#615][]: Our accessibility for text has been greatly improved, allowing by-character and by-word movement through accessibility tools.
- [xilem#660][]: `request_paint` has been removed in favour of a new `request_render`, so that accessibility updates are less likely to be missed.
- [xilem#718][], [xilem#720][]: restored the ability to embed Masonry in external projects by re-exported required items.
- [xilem#612][]: A built-in feature-gated integration with [Tracy](https://github.com/wolfpld/tracy).

Daniel also provided a talk to [GOSIM China 2024](https://china2024.gosim.org/) about Masonry.
The recording of this presentation is not yet available.

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

The main update is the release of version 0.3.0...

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

Editor things?

## Piet

Piet is a 2d graphics library which abstracts over platform-native graphics APIs.
During October, we released [Piet 0.7.0](https://github.com/linebender/piet/releases/tag/v0.7.0).

Some key updates from October included in this release are:
- [piet#572][]: Use of `Arc` over `Rc` for stroke dashing, by anesthetice.
- [piet#585][]: An update to the Rust 2018 edition, from Bruce Mitchener.
- [piet#578][]: Improved robustness of `copy_raw_pixels` on Direct2D by Kaur Kuut.

## Druid

Druid is a GUI library which was a predecessor to Xilem, and an ancestor of Masonry.

We are in the process of relicensing Druid from its current license of Apache 2.0 only to the Rust ecosystem standard Apache 2.0 OR MIT dual license.
If you have previously contributed to Druid, please read and respond to [druid#2414][].
This will greatly improve cross-compatibility will the rest of the Rust ecosystem.
We will also apply this to Xilem, Masonry and Glazier (which is currently archived).

## Others

- Daniel and Olivier's "office hours" appointments can still be booked by anyone for open-ended discussion of the ecosystem.
  - [See Daniel's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2).
  - [See Olivier's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2t767ZRETD_TkRI_VxK2ZTG0VrO9OZ4l7HvTxefhtJcg85iK0ZN7zWNnAEZtH0Dn7C1GKxrmYM).

<!-- [xilem#515]: https://github.com/linebender/xilem/pull/515 -->
