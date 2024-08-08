+++
title = "This Month in Xilem, July 2024"
authors = ["Daniel McNab", "Olivier Faure"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

This month saw a lot of community contributions to Linebender projects.
Philipp Mildenberger especially is a major driver for progress on Xilem and especially the `xilem_web` crate.
Bruce Mitchener has been very active reviewing pull requests, which is an under-appreciated but essential task for any open-source project.

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI.

- [xilem#423][] integrates Tokio into Xilem, showing how the Xilem model can interact with async work.
- [xilem#428][] by Philipp adds explicit support for Flex parameters.
- Philipp ported the async work to Xilem Web, and experimented with some extensions:
  - [xilem#440][] is an experiment into async event handling.
  - [xilem#448][] added the `MemoizedAwait` view, which runs a task whenever the input data has a new value.
- [xilem#467][] by Jared O'Connell adds a calculator example.
  This example can also be run on Android ([xilem#474][]).  

<figure>

<img style="height: auto" src="calculator_example.png" alt="A calculator with display section showing the calculation 9994 + 3231 = 13225. Below this section is a grid of buttons 4 wide and 5 tall, containing the digits 0-9 and some other calculator operations." height="533" width = "931">

<figcaption>

The new `calc` example for Xilem.

</figcaption>
</figure>

## Masonry

Masonry is the widget system used by Xilem.
The most impactful change was the implementation of the ["Widgets in arenas" RFC](https://github.com/linebender/rfcs/blob/main/rfcs/0006-widgets-in-arenas.md) in [xilem#396][].
Widgets are now stored in a global arena, which has little impact on the public APIs but make future changes a lot easier to implement.

- [xilem#417][] exposes an unstable API for embedding Masonry and Xilem into an existing Winit application.
- [xilem#418][], [xilem#421][] improved our unofficial support for iOS.
- [rfcs#7][] specifies a clearer architecture for passes, including simplifications necessary for compositor use.
  Actually integrating with the compositor is still out of scope [for reasons Raph has laid out before](https://raphlinus.github.io/ui/graphics/2020/09/13/compositor-is-evil.html), but the new pass specification gives us a starting point when we get there.

## Vello

Vello is our GPU vector renderer.

- The [GPU-friendly Stroke Expansion](https://linebender.org/gpu-stroke-expansion-paper/) paper has been released.
  Follow the link for the outline, the full paper, demos, and a talk at the HPG 2024 conference.
  Note that the interactive demo is built with `xilem_web`.
- [Vello 0.2.1](https://github.com/linebender/vello/releases/tag/v0.2.1) has been released.
  This patch version resolves a longstanding crash around empty scenes.
- [vello#643][], [vello#647][] switch to Git LFS for storing snapshot test files.
  Git LFS is an open-source Git extension supported by Github, that lets users commit large files to a repository without permanently bloating it.
  Using it has trade-offs around bandwidth usage in CI, but otherwise matches our use-case perfectly.

<figure>

<img style="height: auto" src="stroke_expansion_demo.png" alt="A screenshot containing a single bezier curve, split into several differently coloured segments. The control points of the curve are visible. The expanded stroke for this curve is shown as a filled section, with the generated line segment end points visible as small circles. There are several controls for settings of the stroke expansion above the curve." height="755" width = "868">

<figcaption>

The demo for the stroke expansion paper, which was created using Xilem Web.
An interactive version can be found on the [paper's website](https://linebender.org/gpu-stroke-expansion-paper/#beztoy-container).

</figcaption>
</figure>

## Parley

Parley is a text layout library.
Nico Burns contributed quite a few improvements to Parley this month:

- [parley#67][] implements inline box layout, which lets us mix textual and non-textual content and have the non-textual content laid out in flow with the text.
- [parley#76][], [parley#84][] gets style types closer to CSS engines.

## Others

- [The Glazier repository](https://github.com/linebender/glazier/) has been archived.
  This is in continuity with our direction so far, as we've dropped support for it and committed to building our crates on top of Winit.
- We are at the end of our documented [May-July 2024 roadmap](@/blog/2024-06-17-roadmap-may-2024.md).
  We expect to do a full retrospective on it soon, including the items we shipped and those that slipped.
- Daniel and Olivier are now accepting "office hours" style appointments, for open-ended discussion of the ecosystem.
  - [See Daniel's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2).
  - [See Olivier's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2t767ZRETD_TkRI_VxK2ZTG0VrO9OZ4l7HvTxefhtJcg85iK0ZN7zWNnAEZtH0Dn7C1GKxrmYM).

[xilem#396]: https://github.com/linebender/xilem/pull/396
[xilem#417]: https://github.com/linebender/xilem/pull/417
[xilem#418]: https://github.com/linebender/xilem/pull/418
[xilem#421]: https://github.com/linebender/xilem/pull/421
[xilem#423]: https://github.com/linebender/xilem/pull/423
[xilem#428]: https://github.com/linebender/xilem/pull/428
[xilem#440]: https://github.com/linebender/xilem/pull/440
[xilem#448]: https://github.com/linebender/xilem/pull/448
[xilem#467]: https://github.com/linebender/xilem/pull/467
[xilem#474]: https://github.com/linebender/xilem/pull/474
[rfcs#7]: https://github.com/linebender/rfcs/pull/7
[vello#643]: https://github.com/linebender/vello/pull/643
[vello#647]: https://github.com/linebender/vello/pull/647
[parley#67]: https://github.com/linebender/parley/pull/67
[parley#76]: https://github.com/linebender/parley/pull/76
[parley#84]: https://github.com/linebender/parley/pull/84
