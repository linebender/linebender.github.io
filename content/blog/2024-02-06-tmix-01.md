+++
title = "This Month in Xilem, January 2024"
authors = ["Raph Levien"]
date = "2024-02-06 07:28:42"
+++

Welcome to the first in the “this month in Xilem” series, bringing updates on the [Xilem] UI toolkit.

There were two blog posts on the new [Linebender blog](https://linebender.org/blog).
The first, [Xilem 2024 plans](https://linebender.org/blog/xilem-2024/), sets out the goals for the project for the year.
The second, [Roadmap for the Xilem backend in 2024](https://linebender.org/blog/xilem-backend-roadmap/), by Olivier Faure, presents a roadmap for the Xilem native widget set.
A major feature of the roadmap is a clean boundary between the Xilem reactive framework and the lower level widget set.
We hope that enables experimentation with other reactive frameworks and possibly integration with other projects, rather than requiring everybody to buy into the Xilem stack.

On the planning front, a major decision was whether to continue work on our own window creation library, [Glazier](https://github.com/linebender/glazier), or whether to join forces with [winit](https://github.com/rust-windowing/winit).
Among other things, since Android is a primary target for our work, and winit already has a working Android back-end, it offers an appealing path to getting that up quickly.
We have decided to try joining forces with winit, and look forward to future collaboration.
Over the next few weeks, that will entail porting Xilem over to winit ([xilem#163](https://github.com/linebender/xilem/issues/163)) and also trying to land working Android IME (soft keyboard) support in winit ([winit#2993](https://github.com/rust-windowing/winit/pull/2993)).

In Vello, a major advance was porting the low level font handling code from the experimental swash crate to the production-quality [skrifa](https://github.com/googlefonts/fontations) ([vello#423](https://github.com/linebender/vello/pull/423)).
Skrifa is being developed by Google Fonts as a memory safe library for loading font files, and is integrated into Chromium builds.
Over the coming weeks, a goal is to release Vello (and supporting crates) on crates.io.

[Xilem]: https://github.com/linebender/xilem
