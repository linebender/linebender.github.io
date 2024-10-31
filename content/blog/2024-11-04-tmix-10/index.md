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

<figure>

<!-- <img style="height: auto" src="http_cats.png" alt="A list of HTTP status codes, with 'Select' buttons. HTTP code 418 'I'm a teapot' is selected, with a picture of a kitten hiding in a teapot." height="962" width="600"> -->

<figcaption>

<!-- The HTTP cats example. -->

</figcaption>
</figure>

## Masonry

Masonry is the widget system used by Xilem.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

<!-- - [xilem#515][]: Removes unimplemented functionality from our text handling code. -->

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

The main update is the release of version 0.3.0...

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

Editor things?

## Druid/Piet

We are working towards a final release of Druid.

We are also planning on relicensing Druid from its current license of Apache 2.0 only to the Rust ecosystem standard Apache 2.0 OR MIT dual license.
This will greatly improve cross-compatibility will the rest of the ecosystem, allowing two-way movement of code between most Linebender projects.
This will also apply to Xilem, Masonry and the (archived) Glazier.

## Others

- Daniel and Olivier's "office hours" appointments can still be booked by anyone for open-ended discussion of the ecosystem.
  - [See Daniel's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2).
  - [See Olivier's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2t767ZRETD_TkRI_VxK2ZTG0VrO9OZ4l7HvTxefhtJcg85iK0ZN7zWNnAEZtH0Dn7C1GKxrmYM).

<!-- [xilem#515]: https://github.com/linebender/xilem/pull/515 -->
