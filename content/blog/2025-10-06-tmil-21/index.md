+++
title = "Linebender in September 2025"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Parley

Parley is a text layout library.
It handles text layout, mostly at the level of line breaking and resolving glyph positions.

<!-- TODO -->
- [][]:

## Vello

Vello is our GPU vector renderer.
It can draw large 2D scenes with high performance, using GPU compute shaders for most of the work.

<!-- TODO -->
- [][]:

<!-- TODO: Intro for Sparse Strips -->

- [][]:

<!-- TODO: This roadmap is a bit out of date.
Our [working roadmap](https://docs.google.com/document/d/1ZquH-53j2OedTbgEKCJBKTh4WLE11UveM10mNdnVARY/edit?tab=t.0#heading=h.j3duh9pgdm94) outlines the planned timeline for work on the renderers into next year. -->
<figure>

<!-- <img style="height: auto; width: 50%; margin-left: 25%;" width="666" height="673" src="hybrid-blending.png" alt="A series of overlapping shapes of various colours, variously composited."> -->

<figcaption>

<!-- TODO -->

</figcaption>
</figure>

<!-- TODO: Graphite community meetup? -->

### Peniko

<!-- TODO: Add screenshot. -->

### Linebender Resource Handle

<!-- Linebender  -->
Talk about previous state, dependencies on Peniko, Cosmic text, etc.

## Masonry

Masonry is the widget system developed by Linebender.
It provides a non-opinionated retained widget tree, designed as a base layer for high-level GUI frameworks.

<!-- TODO -->
-[][]:

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI, which uses Masonry for its widgets.
It lets you build user interfaces declaratively by composing lightweight views together, and will diff them to provide minimal updates to a retained layer.

<!-- TODO -->
- [][]:

<!-- TODO: If there were Placehero updates, they go here. -->

<!-- TODO: Add screenshot. -->

## Kurbo

Kurbo provides data structures and algorithms for curves and vector paths.
<!-- TODO: We released v0.12 -->

<!-- TODO -->
- [][]:

## Fearless SIMD

Fearless SIMD is our SIMD infrastructure library.
It provides a solid way for writing SIMD operations portably across WASM, Aarch64, x86, and x86_64.
The improvements we made in September include:

- [][]:

We look forward to seeing ecosystem use cases develop alongside our maturing.

## Get Involved

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

We host an hour long office hours meeting each week where we discuss what's going on in our projects.
See [#office hours in Zulip](https://xi.zulipchat.com/#narrow/channel/359642-office-hours) for details.
We're also running a separate office hours time dedicated to the renderer collaboration, details also available at that link.

If you wish to discuss the Linebender project individually, Daniel is offering ["office hours" appointments](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2), which are free to book.
It really helps us to learn what aspects our users care about the most.

[servo#38962]: https://github.com/servo/servo/pull/38962

[color#191]: https://github.com/linebender/color/pull/191

[fearless_simd#50]: https://github.com/linebender/fearless_simd/pull/50
[fearless_simd#52]: https://github.com/linebender/fearless_simd/pull/52
[fearless_simd#58]: https://github.com/linebender/fearless_simd/pull/58
[fearless_simd#59]: https://github.com/linebender/fearless_simd/pull/59
[fearless_simd#61]: https://github.com/linebender/fearless_simd/pull/61
[fearless_simd#69]: https://github.com/linebender/fearless_simd/pull/69

[kurbo#476]: https://github.com/linebender/kurbo/pull/476
[kurbo#483]: https://github.com/linebender/kurbo/pull/483
[kurbo#485]: https://github.com/linebender/kurbo/pull/485
[kurbo#486]: https://github.com/linebender/kurbo/pull/486
[kurbo#487]: https://github.com/linebender/kurbo/pull/487
[kurbo#488]: https://github.com/linebender/kurbo/pull/488
[kurbo#489]: https://github.com/linebender/kurbo/pull/489
[kurbo#490]: https://github.com/linebender/kurbo/pull/490
[kurbo#493]: https://github.com/linebender/kurbo/pull/493

[parley#400]: https://github.com/linebender/parley/pull/400
[parley#405]: https://github.com/linebender/parley/pull/405
[parley#406]: https://github.com/linebender/parley/pull/406
[parley#413]: https://github.com/linebender/parley/pull/413
[peniko#115]: https://github.com/linebender/peniko/pull/115
[peniko#117]: https://github.com/linebender/peniko/pull/117
[peniko#120]: https://github.com/linebender/peniko/pull/120
[peniko#121]: https://github.com/linebender/peniko/pull/121

[vello#1096]: https://github.com/linebender/vello/pull/1096
[vello#1136]: https://github.com/linebender/vello/pull/1136
[vello#1137]: https://github.com/linebender/vello/pull/1137
[vello#1149]: https://github.com/linebender/vello/pull/1149
[vello#1153]: https://github.com/linebender/vello/pull/1153
[vello#1155]: https://github.com/linebender/vello/pull/1155
[vello#1161]: https://github.com/linebender/vello/pull/1161
[vello#1169]: https://github.com/linebender/vello/pull/1169
[vello#1177]: https://github.com/linebender/vello/pull/1177
[vello#1178]: https://github.com/linebender/vello/pull/1178
[vello#1182]: https://github.com/linebender/vello/pull/1182
[vello#1183]: https://github.com/linebender/vello/pull/1183

[xilem#1207]: https://github.com/linebender/xilem/pull/1207
[xilem#1212]: https://github.com/linebender/xilem/pull/1212
[xilem#1226]: https://github.com/linebender/xilem/pull/1226
[xilem#1228]: https://github.com/linebender/xilem/pull/1228
[xilem#1250]: https://github.com/linebender/xilem/pull/1250
[xilem#1260]: https://github.com/linebender/xilem/pull/1260
[xilem#1269]: https://github.com/linebender/xilem/pull/1269
[xilem#1273]: https://github.com/linebender/xilem/pull/1273
[xilem#1278]: https://github.com/linebender/xilem/pull/1278
[xilem#1282]: https://github.com/linebender/xilem/pull/1282
[xilem#1295]: https://github.com/linebender/xilem/pull/1295
[xilem#1310]: https://github.com/linebender/xilem/pull/1310
[xilem#1312]: https://github.com/linebender/xilem/pull/1312
[xilem#1314]: https://github.com/linebender/xilem/pull/1314
[xilem#1318]: https://github.com/linebender/xilem/pull/1318
[xilem#1320]: https://github.com/linebender/xilem/pull/1320
[xilem#1321]: https://github.com/linebender/xilem/pull/1321
[xilem#1323]: https://github.com/linebender/xilem/pull/1323
[xilem#1332]: https://github.com/linebender/xilem/pull/1332
[xilem#1333]: https://github.com/linebender/xilem/pull/1333
[xilem#1337]: https://github.com/linebender/xilem/pull/1337
[xilem#1348]: https://github.com/linebender/xilem/pull/1348
[xilem#1349]: https://github.com/linebender/xilem/pull/1349
[xilem#1352]: https://github.com/linebender/xilem/pull/1352
[xilem#1353]: https://github.com/linebender/xilem/pull/1353
[xilem#1357]: https://github.com/linebender/xilem/pull/1357
[xilem#1363]: https://github.com/linebender/xilem/pull/1363
[xilem#1364]: https://github.com/linebender/xilem/pull/1364
[xilem#1366]: https://github.com/linebender/xilem/pull/1366
[xilem#1370]: https://github.com/linebender/xilem/pull/1370
[xilem#1371]: https://github.com/linebender/xilem/pull/1371
