+++
title = "Linebender in February 2025"
authors = ["Daniel McNab", "Raph Levien"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.
February was a slow month for Xilem specific changes, although many changes further down the stack will be relevant to Xilem users.

- [xilem#861][]: makes a method on `Pod` public to make creating widgets easier, by Evgeny.

## Masonry

Masonry is the widget system used by Xilem.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

- [xilem#872][]: Exported the scale factor for limited use cases (such as single-pixel lines).

There is some important work upcoming:

- [xilem#873][]: Olivier started to develop his "arbitrary properties" feature, which is intended for styling.
- [xilem#875][]: Adds an image widget containing arbitrary vector graphics content, by Richard Dodd.

As Parley has now been released to crates.io (discussed later in this update), we're hoping to release a new alpha of Xilem and Masonry to crates.io in March.

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

- [vello#803][]: Removed the `render_to_surface` API, as `wgpu`'s new [`TextureBlitter`][] utility makes it unnecessary.
- [vello#832][] (in review): Jared Moulton added support for font embolden to our text handling.

Last month we mentioned a research exploration into a CPU/GPU hybrid 2D rendering mode.
This hybrid mode works well on downlevel GPUs without strong support for compute shaders, and is also more memory efficient.
There has been a lot of interest from the community, and we are now moving forward with building out this rendering mode in Vello.
We are very excited that Alex Gemberg and Taj Pereira, two engineers from Canva, are joining the collaboration on this renderer.

In February, Raph opened [vello#818][], which provided the first implementation of this hybrid pipeline.
We are planning to refactor this to share most of its code with an ongoing CPU-only implementation, which is being developed by Laurenz Stampfl as his Masters project.
To that aim, we have landed [vello#826][], which provided the repository-layout scaffolding, in the `sparse_strips` folder.
This initial PR is just a stub, but we are planning on filling these crates out imminently, allowing collaborative work on both pipelines (hybrid and CPU-only).
You can follow the progress in [#gpu > Vello Hybrid](https://xi.zulipchat.com/#narrow/channel/197075-gpu/topic/Vello.20Hybrid) and other threads in [#gpu](https://xi.zulipchat.com/#narrow/channel/197075-gpu).

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

We released [Parley and Fontique 0.3.0][] to crates.io.
These contain many of the features we have discussed over the previous months, including significant improvements to `PlainEditor`.

<!-- TODO: If we want to say anything more here, someone else will have to do it. -->

## Resvg

We released [Resvg 0.45.0][] in February.
This is the first release under the stewardship of Linebender.
We'd like to reiterate our many thanks to Yevhenii Reizner for the years of hard work that he has poured into this and other crates.
The largest change in this release is the relicense to dual Apache-2.0 MIT.
It also includes support for the `!important` CSS flag.

<!-- TODO: If we want to say anything more here, someone else will have to do it. -->

## Kurbo

Kurbo provides data structures and algorithms for curves and vector paths.

- [kurbo#412][], [kurbo#413][]: Added utility methods to `Size`, by [@nils-mathieu][].

## Color

[Color][] provides functionality for representing, converting, parsing, serializing, and manipulating colors in a variety of color spaces.
It closely follows the [CSS Color Module Level 4][] draft spec.

The biggest update to Color in February was the addition of absolute color conversions, in [color#139][].
This won't be visible for most users, but will be useful for those who need to convert between color spaces without changing the white point.
This will also be used in [color#137][], which allows creating a theoretical black-body color from a temperature in Kelvin.

## Velato

We released [Velato 0.5.0][], bringing compatibility with Vello 0.4.0.

## Kompari

[Kompari][] is a tool for visual inspection of snapshot tests.
[parley#272][] (in review) begins the exciting process of integrating Kompari into our existing projects.

## Research and Future Directions

Linebender has an origin story in being a very research oriented group, looking to break new ground.
While we are focused on shipping code today, we still have an eye on the future and how to be prepared for the new opportunities and technologies that are coming.

While we didn't publish much in the way of artifacts in February, there was quite a bit of experimentation with computational geometry ideas related to [#kurbo > Two-point shape control](https://xi.zulipchat.com/#narrow/channel/260979-kurbo/topic/Two.20point.20shape.20control), a technique that holds promise for faster and more accurate stroke expansion of cubic Béziers.
Stay tuned for more updates.

## Get Involved

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

We host an hour long office hours meeting each week where we discuss what's going on in our projects.
See [#office hours in Zulip](https://xi.zulipchat.com/#narrow/channel/359642-office-hours) for details.
We've also started a separate office hours time dedicated to the renderer collaboration, details also available at that link.

* Daniel and Olivier's "office hours" appointments can still be booked by anyone for open-ended discussion of the ecosystem.
  * [See Daniel's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2).
  * [See Olivier's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2t767ZRETD_TkRI_VxK2ZTG0VrO9OZ4l7HvTxefhtJcg85iK0ZN7zWNnAEZtH0Dn7C1GKxrmYM).

[Color]: https://docs.rs/color/
[CSS Color Module Level 4]: https://www.w3.org/TR/css-color-4/
