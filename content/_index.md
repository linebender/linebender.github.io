+++
title = "Homepage"
description = "Homepage for the Linebender organization"
+++

Welcome to the homepage for the [Linebender organization](https://github.com/linebender). We are a friendly group of people who share an interest in 2D graphics and user interface design, with everything that entails. We hang out on [the Xi Zulip instance][xi.zulip] and always welcome new people.

# Linebender projects

Below is a list of the main linebender projects, and a short description of each project's purpose, as of 2023-04-22.

## Crates (actively developed)

These crates are under active development.

 - [`xilem`][xilem] - An experimental Rust architecture for reactive UI.

Xilem is a new UI toolkit with a medium-grained reactive architecture strongly inspired by SwiftUI. It is an evolution of Druid, aiming to keep what is good while improving on Druid's shortcomings.
   improving on Druid's shortcomings. It uses a reactive architecture built on
   top of view trees (this aproach is sometimes called 'virtual-DOM' or 'VDOM', which
   comes from its roots in single-page web apps).

 - [`vello`][vello] - An experimental GPU compute-centric 2D renderer.

   This crate contains a set of compute shaders and framework for rasterizing 2D
   geometry on the GPU. The goal is high performance, low resource usage and easy
   interoperability between platforms.

 - [`glazier`][glazier] - An experimental windowing library.

   This crate provides access to the platform API, in order to create windows and contexts to draw to, and to feed platform events like keypresses and mouse/touch events into an application. It serves a similar purpose to [`winit`][winit], and indeed either `winit` or `glazier` can be used as the windowing layer with `vello`. It exists because we have slightly different goals to `winit`: we want to provide rich desktop integration including things like menus, but there is a chance in the future that we can merge the functionality that we need into `winit` and use that crate instead.

 - [`kurbo`][kurbo] - A library for creating, manipulating and interrogating 2D curve shapes.

   At its core, `kurbo` is a library for contructing paths and splines out of straight lines and Bézier curves up to order 3 (known as cubic Béziers). It turns out that a series of cubic Bézier curves can be used to approximate any smooth curve with a very high degree of accuracy, compared to the number of curves required. They are also relatively easy to work with, and form the basis of the approach to curve rendering used in `vello`. The key abstraction is [`kurbo::Shape`](https://docs.rs/kurbo/latest/kurbo/trait.Shape.html), which provides the `path_elements` method. This method returns an interator over Bézier curves that approximate the type implementing the `Shape` trait, which `vello` can then draw.

 - [`peniko`][peniko] - A library for non-geometric drawing primitives.

   This crate provides a set of shared types for concepts that are important for drawing/stroking paths, but excluding the path geometry itself (which can be found in [`kurbo`][kurbo]). It includes types for brush styles (including gradient) and color.

The following crates are not part of the `xilem`/`vello` stack, but are under active development.

 - [`velato`][velato] - A crate that renders lottie animations to vello.
 - [`norad`][norad] - A crate for reading, writing, and manipulating [Unified Font Object] files, a common font-design format.

## Crates (passively maintained)

 - [`druid`][druid] - A GUI framework for Rust

   `druid` is the original Linebender UI framework, and was the main focus of development until recently. There are many good things about `druid`, but the data model it uses (primarily `Lens`es) gets complicated in Rust, and doesn't compose as well as it would in a [GC'd][garbage collection] functional programming language. However, a lot of the things that work will will be copied straight over to `xilem`, and today `druid` is a solid choice for a new UI app projected to have low to medium complexity. The [#druid-help channel on Zulip](https://xi.zulipchat.com/#narrow/stream/255910-druid-help) is still monitored and you will likely get good answers to any questions you have.

 - [`piet`][piet] - An abstraction layer over platform 2D rendering facilities.

   The purpose of the `piet` library is to provide abstraction over the libraries used for 2D rendering on any particular platform ([Direct2D] on Windows, [Core Graphics] on Mac, and [cairo]/[pango] on Linux). The goal is for `piet` to be superseeded by the `vello` crate, which will provide state-of-the-art 2D rendering on any platform supported by [WebGPU].

 - [`druid-shell`][druid-shell] - The old name for `glazier`.

   Previously, the windowing library `glazier` lived within the `druid` repository, and was called `druid-shell`. When work moved over to `vello`/`xilem`, there was debate over whether to split this code into its own crate, or move over to `winit`. We decided to keep using our own windowing library for now, to give us the flexibility to add or change what we need to, but we may try to unify with `winit` in the future.

 - [`druid-widget-nursery`][druid-widget-nursery] - A crate with widgets for `druid`.

   This crate contains lots of `druid` widgets that may be useful to UI developers. It includes things like widgets for [Material Icons], and a tree view (a way to draw data stored in a tree). It followed the very liberal [optimistic merging] strategy for handling PRs to minimize contributor friction, but the widgets contributed are all robust and high quality.

 - [`runebender`][runebender] - An experimental font editor.

   This was the motivating application for `druid` development. It is not currently seeing active development, but this may be revisited in the future.

 - [`skribo`][skribo] - A library for text layout.

   This library aims to perform the conversion from text and font attributes to glyph runs, similar to [harfbuzz]. It's not recommended to use this library as better alternatives exist, for example [`parley`] or [`cosmic-text`].

## Non-code repos

 - [`2d.graphics`][2d.graphics] - A work-in-progress book explaining the current state of the art of different aspects of 2D graphics.

   Currently the book mostly contains annotated reference lists, although some sections have written content. Pull requests welcome! The goal is to cover topics like color, shapes/curves, 2D GPU rendering, text, etc.

[xi.zulip]: https://xi.zulipchat.com
[piet]: https://github.com/linebender/piet
[kurbo]: https://github.com/linebender/kurbo
[druid]: https://github.com/linebender/druid
[druid-shell]: https://github.com/linebender/druid/tree/master/druid-shell
[glazier]: https://github.com/linebender/glazier
[vello]: https://github.com/linebender/vello
[runebender]: https://github.com/linebender/runebender
[xilem]: https://github.com/linebender/xilem
[2d.graphics]: https://github.com/linebender/2d.graphics
[velato]: https://github.com/linebender/velato
[norad]: https://github.com/linebender/norad
[peniko]: https://github.com/linebender/peniko
[druid-widget-nursery]: https://github.com/linebender/druid-widget-nursery
[skribo]: https://github.com/linebender/skribo
[Unified Font Object]: http://unifiedfontobject.org/
[winit]: https://github.com/rust-windowing/winit
[garbage collection]: https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)
[Direct2D]: https://learn.microsoft.com/en-us/windows/win32/direct2d/direct2d-portal
[Core Graphics]: https://developer.apple.com/documentation/coregraphics
[cairo]: https://www.cairographics.org/
[pango]: https://pango.gnome.org/
[WebGPU]: https://www.w3.org/TR/webgpu/#intro
[Material Icons]: https://fonts.google.com/icons
[optimistic merging]: http://hintjens.com/blog:106
[harfbuzz]: https://github.com/harfbuzz/harfbuzz
[`parley`]: https://github.com/dfrg/parley
[`cosmic-text`]: https://github.com/pop-os/cosmic-text

