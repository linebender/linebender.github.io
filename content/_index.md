+++
title = "Homepage"
description = "Homepage for the Linebender organization"
+++

Welcome to the homepage for the [Linebender organization](https://github.com/linebender).

We are a friendly group of people who share an interest in 2D graphics and user interface design, with everything that entails.
Most of our projects are developed in [the Rust programming language][rust].
We hang out on the [Linebender Zulip][xi.zulip] and always welcome new people.

# Linebender projects

Below is a list of the main Linebender projects, and a short description of each project's purpose, as of 2024-12-07.

## Crates (actively developed)

 - [`xilem`][xilem] - Experimental Rust architecture for reactive UI.

   Xilem is a UI toolkit with a medium-grained reactive architecture strongly inspired by SwiftUI.
   It is currently in a pre-alpha state, with several significant issues, but is improving rapidly.

 - [`masonry`][masonry] - Foundational framework for Rust GUI libraries.

   Masonry is an evolution of Druid, designed to be driven by higher-level UI toolkits.
   It is currently used to power Xilem, but is designed to be usable by other toolkits as well.

 - [`vello`][vello] - GPU compute-centric 2D renderer.

   Vello is a 2D graphics rendering engine written in Rust, with a focus on GPU compute.
   It can draw large 2D scenes with interactive or near-interactive performance, using [`wgpu`][wgpu] for GPU access.
   The Vello project includes two additional in-progress renderers.

 - [`kurbo`][kurbo] - Create, manipulate, and interrogate 2D curve shapes.

   At its core, Kurbo is a library for constructing paths and splines out of straight lines and Bézier curves up to order 3 (known as cubic Béziers).
   It turns out that a series of cubic Bézier curves can be used to approximate any smooth curve with a very high degree of accuracy, compared to the number of curves required.
   They are also relatively easy to work with, and form the basis of the approach to curve rendering used in Vello.

 - [`color`][color] - Manipulate and represent colors.

   Color provides functionality for representing, converting, parsing, serializing, and manipulating colors in a variety of color spaces.
   It closely follows the [CSS Color Level 4] draft spec.

 - [`peniko`][peniko] - Non-geometric drawing primitives.

   Provides a set of shared types for concepts that are important for drawing/stroking paths, but excluding the path geometry itself (which can be found in [`kurbo`][kurbo]).
   It includes types for brush styles (including gradient) and color.

 - [`parley`][parley] - Rich text layout, backed by [`swash`][swash].
 - [`fontique`][fontique] - Font enumeration and fallback.
 - [`velato`][velato] - Convert Lottie animations to Vello scenes for rendering.
 - [`vello_svg`][vello_svg] - Convert SVG documents to Vello scenes for rendering.
 - [`interpoli`][interpoli] - Functionality for animating values.
 - [`kompari`][kompari] - A tool for detecting and reporting differences in images.
 - [`norad`][norad] - Read, write, and manipulate [Unified Font Object] files, a common font-design format.

## Crates (passively maintained)

 - [`druid`][druid] - GUI framework for Rust.

   Druid was the original Linebender UI framework, and was the main focus of development until early 2023.
   New development effort has moved on to [Xilem][xilem], which is quite different, but still heavily inherits from Druid via [Masonry][masonry].

   Druid's main goal was to offer a polished user experience.
   There were many factors to this goal, including performance, a rich palette of interactions, and playing well with the native platform.

   Druid is reasonably usable for [some subset of applications](https://github.com/linebender/druid/issues/1360) and has a significant testing history, which ensures some stability and correctness.
   However, there will not be any new features or bug fixes coming to Druid.
   As such we don't recommend using Druid for brand new applications.
   If you insist, then at least make sure your application doesn't require a feature that Druid doesn't have, e.g. accessibility or 3D support.

 - [`piet`][piet] - Abstraction layer over platform 2D rendering facilities.

   It provides an abstraction over the libraries used for 2D rendering on any particular platform ([Direct2D] on Windows, [Core Graphics] on Mac, and [Cairo]/[Pango] on Linux).

   Our goal is for Piet to be superseded by Vello, which will provide state-of-the-art GPU accelerated 2D rendering on any platform supported by [WebGPU].

 - [`druid-shell`][druid-shell] - The windowing library for Druid.

   It was created due to limitations with [`winit`][winit] and has features like file dialogs, system menus, and modal windows.
   When we started work on Xilem, we also forked `druid-shell` to bootstrap [Glazier][glazier].

 - [`druid-widget-nursery`][druid-widget-nursery] - Experimental widgets for Druid.

   Contains lots of Druid widgets that may be useful to UI developers.
   For example a widget to draw [Material Icons], a tree view, and a dropdown.
   It followed the very liberal [optimistic merging] strategy for handling PRs to minimize contributor friction, but the widgets contributed are all robust and high quality.

 - [`runebender`][runebender] - Experimental font editor.

   This was the motivating application for Druid development.
   It is not currently seeing active development, but this may be revisited in the future.

 - [`skribo`][skribo] - Text layout.

   This library aims to perform the conversion from text and font attributes to glyph runs, similar to [harfbuzz].
   It's not recommended to use this library as better alternatives exist, for example [`parley`] or [`cosmic-text`].

## Non-code repos

 - [`2d.graphics`][2d.graphics] - A work-in-progress book explaining the current state of the art of different aspects of 2D graphics.

   Currently the book mostly contains annotated reference lists, although some sections have written content.
   Pull requests welcome!
   The goal is to cover topics like color, shapes/curves, 2D GPU rendering, text, etc.

 - [`linebender.github.io`][linebender.github.io] - This website.

   If you see anything that looks incorrect, please submit a PR to [its repo][linebender.github.io].

## Archived repos

 - [`glazier`][glazier] - Windowing library.

   Glazier was a fork of [`druid-shell`][druid-shell].
   We pursued this due to limitations with [`winit`][winit], however eventually decided to work together with the `winit` team to improve `winit` instead.
   We recommend using `winit` over Glazier.

[xi.zulip]: https://xi.zulipchat.com
[rust]: https://rust-lang.org
[piet]: https://github.com/linebender/piet
[kurbo]: https://github.com/linebender/kurbo
[color]: https://github.com/linebender/color
[druid]: https://github.com/linebender/druid
[druid-shell]: https://github.com/linebender/druid/tree/master/druid-shell
[glazier]: https://github.com/linebender/glazier
[vello]: https://github.com/linebender/vello
[wgpu]: https://wgpu.rs/
[runebender]: https://github.com/linebender/runebender
[xilem]: https://github.com/linebender/xilem
[masonry]: https://github.com/linebender/xilem/tree/main/masonry
[2d.graphics]: https://github.com/linebender/2d.graphics
[velato]: https://github.com/linebender/velato
[vello_svg]: https://github.com/linebender/vello_svg
[interpoli]: https://github.com/linebender/interpoli
[kompari]: https://github.com/linebender/kompari
[parley]: https://github.com/linebender/parley
[fontique]: https://github.com/linebender/parley/tree/main/fontique
[swash]: https://github.com/dfrg/swash
[norad]: https://github.com/linebender/norad
[peniko]: https://github.com/linebender/peniko
[druid-widget-nursery]: https://github.com/linebender/druid-widget-nursery
[skribo]: https://github.com/linebender/skribo
[Unified Font Object]: http://unifiedfontobject.org/
[winit]: https://github.com/rust-windowing/winit
[Direct2D]: https://learn.microsoft.com/en-us/windows/win32/direct2d/direct2d-portal
[Core Graphics]: https://developer.apple.com/documentation/coregraphics
[Cairo]: https://www.cairographics.org/
[Pango]: https://www.pango.org/
[WebGPU]: https://www.w3.org/TR/webgpu/#intro
[Material Icons]: https://fonts.google.com/icons
[optimistic merging]: http://hintjens.com/blog:106
[harfbuzz]: https://github.com/harfbuzz/harfbuzz
[`parley`]: https://github.com/dfrg/parley
[`cosmic-text`]: https://github.com/pop-os/cosmic-text
[linebender.github.io]: https://github.com/linebender/linebender.github.io
[CSS Color Level 4]: https://www.w3.org/TR/css-color-4/
