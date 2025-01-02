+++
title = "Linebender in December 2024"
authors = ["Bruce Mitchener", "Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

We'd like to wish everyone a happy New Year!

## Peniko

Peniko provides a set of shared types for concepts that are important for drawing/stroking paths, but excluding the path geometry itself (which can be found in kurbo).
It includes types for brush styles (including gradient) and color.

Last month, we continued down the path to supporting a richer color model and wider gamuts.
We released [Peniko 0.3.0] which replaces the old color code with our new [Color] crate.

* [peniko#63]: Use the Color crate
* [peniko#71]: `Gradient` now tracks a hue direction and interpolation color space
* [peniko#77]: Add x/y extend modes and quality hint to images

## Color

Color provides functionality for representing, converting, parsing, serializing, and manipulating colors in a variety of color spaces.
It closely follows the [CSS Color Module Level 4] draft spec.

Last month, we released [Color 0.2.0] and [Color 0.2.1].

The release of Color 0.2.0 contained a significant amount of work to enable use with Peniko and the rest of the Linebender stack.

Color 0.2.1 included improvements to parsing.

* [color#109]: Make color parsing case insensitive
* [color#111]: Add `FromStr` impl for `AlphaColor`, `DynamicColor`, `OpaqueColor`, and `PremulColor`

We encourage people to give [Color] a try and let us know how it goes.

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

Vello is now using the new Peniko 0.3 release along with the Color crate.

* [vello#757]: Remove `buffer_labels` feature as it is no longer optional
* [vello#761]: Remove work-around for `const` / `let` globals in WGSL
* [vello#768]: Migrate to notify-debouncer-full
* [vello#774]: Initialize wgpu `InstanceDescriptor` from environment

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

**TODO: Add lots of stuff here**

## Masonry

Masonry is the widget system used by Xilem.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

Updates to Parley integration and usage for IME and text editing:

* [xilem#766]: Update to upstream unbatched PlainEditor
* [xilem#767]: Stop IME just before shutting down
* [xilem#768]: Clear compose in `TextArea::reset_text`
* [xilem#785]: Improve IME area reporting and set IME area in `TextArea`

The new tree arena landed and saw some performance improvements:

* [xilem#752]: Tree arena
* [xilem#772]: Use hashmap to track tree arena node children
* [xilem#774]: Use `hashbrown` as drop-in replacement to optimize tree access

We updated to start using the Color crate:

* [xilem#798]: Update to git Parley and Vello (uses Color)
* [xilem#800]: Use `palette` and `Color` from top level Masonry and Xilem

Miscellaneous:

* [xilem#777]: Render one frame before showing the window to avoid flashing
* [xilem#795]: Sized Box: Use a brush for the border

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

* [xilem#797]: Button with customizable label (text)

## Kurbo

## Piet

Discussion continued about [Low-level Piet].

## Tiny Skia, Resvg, SimpleCSS, SVG Types

## Velato, Vello SVG

## Research and Future Directions

Linebender has an origin story in being a very research oriented group, looking to break new ground.
While we are focused on shipping code today, we still have an eye on the future and how to be prepared for the new opportunities and technologies that are coming.
This can be seen in our thinking about the future of Piet as previously discussed as well as the work that we're doing with Color.

We have a couple of active and ongoing research projects:

* Raph Levien on SIMD, stroke expansion, and new rendering approaches for Vello.
* Joe Neeman on boolean path operations.

November also saw the transition of the Slang shader language from Nvidia to Khronos.
We don't have any immediate plans to adopt Slang in Vello (we're already pretty busy!), but we are looking at Slang and thinking about how we can best make use of it to improve the usefulness of our crates.
We'd love to [talk][] to people who are adopting Slang within the Rust ecosystem or interested in publishing Slang code for wider use.

## Get Involved

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

We host an hour long office hours meeting each week where we discuss what's going on in our projects.
See [#office hours in Zulip](https://xi.zulipchat.com/#narrow/channel/359642-office-hours) for details.

* Daniel and Olivier's "office hours" appointments can still be booked by anyone for open-ended discussion of the ecosystem.
  * [See Daniel's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2).
  * [See Olivier's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2t767ZRETD_TkRI_VxK2ZTG0VrO9OZ4l7HvTxefhtJcg85iK0ZN7zWNnAEZtH0Dn7C1GKxrmYM).

[CSS Color Module Level 4]: https://www.w3.org/TR/css-color-4/
[Low-level Piet]: https://xi.zulipchat.com/#narrow/channel/197075-gpu/topic/Low-level.20Piet

[color#109]: https://github.com/linebender/color/pull/109
[color#111]: https://github.com/linebender/color/pull/111
[peniko#63]: https://github.com/linebender/peniko/pull/63
[peniko#71]: https://github.com/linebender/peniko/pull/71
[peniko#77]: https://github.com/linebender/peniko/pull/77
[vello#757]: https://github.com/linebender/vello/pull/757
[vello#761]: https://github.com/linebender/vello/pull/761
[vello#768]: https://github.com/linebender/vello/pull/768
[vello#774]: https://github.com/linebender/vello/pull/774
[xilem#752]: https://github.com/linebender/xilem/pull/752
[xilem#766]: https://github.com/linebender/xilem/pull/766
[xilem#767]: https://github.com/linebender/xilem/pull/767
[xilem#768]: https://github.com/linebender/xilem/pull/768
[xilem#772]: https://github.com/linebender/xilem/pull/772
[xilem#774]: https://github.com/linebender/xilem/pull/774
[xilem#777]: https://github.com/linebender/xilem/pull/777
[xilem#785]: https://github.com/linebender/xilem/pull/785
[xilem#795]: https://github.com/linebender/xilem/pull/795
[xilem#797]: https://github.com/linebender/xilem/pull/797
[xilem#798]: https://github.com/linebender/xilem/pull/798
[xilem#800]: https://github.com/linebender/xilem/pull/800

[Color]: https://docs.rs/color/
[Color 0.2.0]: https://github.com/linebender/color/releases/tag/v0.2.0
[Color 0.2.1]: https://github.com/linebender/color/releases/tag/v0.2.1
[Peniko 0.3.0]: https://github.com/linebender/peniko/releases/tag/v0.3.0
