+++
title = "Linebender in June 2025"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

- [vello#...][]: ...

<!-- ... Sparse strips -->
This month we continued seeing a massive amount of activity on Vello's sparse strips renderers (see Raph's video above for details), thanks in part to the tireless contributions of Canva developers Alex Gemberg, Taj Pereira and Andrew Jakubowicz, and to the continued work of Laurenz Stampfl as part of his master's project.

- [vello#...][]: ...

## Masonry

Masonry is the widget system developed by Linebender.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

- [xilem#987][]: Route text events to the root widget if there is no focused widget.
- [xilem#1038][]: Support multiple (top-level) desktop windows in Masonry, by Martin Fischer.
- [xilem#1009][]: Add properties which only take effect when a widget is hovered/disabled/active.
- [xilem#1043][], [xilem#1044][]: Split out a Masonry Core crate, which can increase parallelism in compilation.
- [xilem#1048][]: Split testing into a Masonry Testing crate, allowing certainty that test code isn't inadvertently included in release binaries.
- [xilem#1053][], [xilem#1054][], [xilem#1055][], [xilem#1056][]: Add properties to `Textbox`, `Checkbox`, `Flex`, `Grid`, and `SizedBox`.
- [xilem#1086][]: Validate accessibility tree updates in tests.


<!-- TODO: Image for multiple windows? -->

<!-- 
<figure>

<img style="height: auto" width="300" height="300" src="button-shadows.png" alt="Screenshot of the new shadows.">

<figcaption>

[xilem#960][]: Adds new shadow property to buttons.

</figcaption>
</figure> -->

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI, which uses Masonry for its widgets.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

- [xilem#608][]: A `ViewSequence` called `WithoutElements`, to allow including side-effects in lists of element children.
- [xilem#992][]: Add support for multiple (top-level) desktop windows in Xilem, by Martin Fischer.
- [xilem#1071][]: Split Xilem into modules.
- [xilem#1079][], [xilem#1085][]: Provide the app's state to each `View` during more phases. This will allow many future architecture explorations.
- [xilem#1078][]: Allow accessing the channel of new requests for a `worker` directly.
- [xilem#1094][]: Refactor `lens` into its own view so that the state doesn't need to passed when constructing the view.
- [xilem#1097][]: Small example to teach variable length lists, by Nils Martel.
- [xilem#1102][]: Add a helper for flex rows, to improve readability of layout code.

In June, we started a new initiative for Xilem, which is a "hero" application.
That is, an application which we are developing alongside Xilem to focus its development.

- [xilem#1087][]: Display a single scrollable timeline.
- [xilem#1089][], [xilem#1116][]: Add simple HTML processing to make posts more readable.
- [xilem#1091][]: Load and display the avatar of post authors.
- [xilem#1092][]: Improve the styling of posts.
- [xilem#1108][]: Correctly display boosted posts.
- [xilem#1114][]: Allow viewing replies to a post.

<!-- TODO: Image of Placehero -->

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

- [parley#...][]: ...

## Android View

Andrdoid View was handed over to the Rust Mobile organisation.
<!-- TODO: Help wanted? Any actual updates? -->

## Get Involved

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

We host an hour long office hours meeting each week where we discuss what's going on in our projects.
See [#office hours in Zulip](https://xi.zulipchat.com/#narrow/channel/359642-office-hours) for details.
We've also started a separate office hours time dedicated to the renderer collaboration, details also available at that link.

- Daniel and Olivier's "office hours" appointments can still be booked by anyone for open-ended discussion of the ecosystem.
  - [See Daniel's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2).
  - [See Olivier's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2t767ZRETD_TkRI_VxK2ZTG0VrO9OZ4l7HvTxefhtJcg85iK0ZN7zWNnAEZtH0Dn7C1GKxrmYM).
