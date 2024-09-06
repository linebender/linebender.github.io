+++
title = "This Month in Xilem, August 2024"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

<figure>

<img style="height: auto; margin: 0 auto" src="../../linebender.svg" alt="Linebender logo; smoothly stroked overlapping lowercase letters, an l and a b." height="200" width = "200">

<figcaption>

Linebender has a new logo, which was contributed by [Markus Siglreithmaier](https://github.com/msiglreith).
We're seeking feedback on Zulip in [#linebender>logo](https://xi.zulipchat.com/#narrow/stream/419691-linebender/topic/logo).

</figcaption>
</figure>

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI.

- We integrated basic animated variable font rendering ([xilem#507][]).
- Philipp Mildenberger moved us back to using `Viewmarker` in [xilem#472][].
  This restores the ability to return `ViewSequence`s, by making a tradeoff to work around Rust's faulty trait ambiguity rules.
- We added Views for some existing and new widgets:
  - spinners, useful for showing indetermine progress ([xilem#497][]);
  - progress bars ([xilem#513][]); and
  - the `Portal` widget, which brings support for scroll areas to Xilem ([xilem#561][]).
- Xilem Web:
    <!-- TODO: -->
  - One or two most impactful items.

<figure>

<img style="height: auto" src="stopwatch.png" alt="A timer with the value '0:00:26.4' (26.4 seconds) above two buttons, reset and start. Below these are 5 laps, labelled 'lap 5' to 'lap 1', each with a lap time of approximately 5 seconds and a corresponding elapsed time. This is all contained in an OS window titled 'Stopwatch'." height="388" width = "559">

<figcaption>

Jared O'Connell additionally contributed a stopwatch example in [xilem#492][].

</figcaption>
</figure>

## Masonry

Masonry is the widget system used by Xilem.
This month has been dominated by Olivier's implementation of pass specification ([rfcs#7][]):

- [xilem#512][]: The new compose pass, which is used to minimise redraw when scrolling.
- [xilem#510][]: Adds the mutate pass, which allows later passes to assume the tree structure is stable by limiting when tree mutation can occur.
- [xilem#522][]: Implements paint and accessibility as passes, with follow-up by Tom Churchman in [xilem#557][].
- [xilem#488][] and [xilem#540]: updated event and pointer handling, and disabled handling respectively.
- In progress work includes layout in [xilem#529][], focus in [xilem#538][], animation in [xilem#539][] and scroll requests in [xilem#550][].

We also had some great community contributions, such as:

- Richard Dodd added a new progress Bar widget ([xilem#513][]).
- Fixes for Portal scrolling, by Tom Churchman ([#xilem#563][]).

<figure>

<img style="height: auto" src="progress_bar.png" alt="A horizontal progress bar which is 70% filled with blue, with white text overlaid saying 70%. Below this is a checkbox labelled 'set indetermine progress', and a button labelled 'change progress'." height="129" width = "431">

<figcaption>

A progress bar widget was contributed by Richard Dodd in ([xilem#513][]).

</figcaption>
</figure>

We continue to make progress on other key features.

- Snapshot tests now are cross-platform and run in CI ([xilem#233][]).
- Pan/flick gestures are being developed in ([xilem#562][]).
- Progress continues on text input for Android. This has shifted into creating our own activity type, with better support for accessibility and text input than `NativeActivity`. <!-- TODO: links? - cc @xorgy -->
- Progress is continuing well on Accessibility for Android. You can try this in [xilem#575][]<!-- , or see it in the below video -->.

<!-- TODO: Accessibility video. Embed? -->

## Vello

Vello is our GPU vector renderer.
We have made some really great progress in August<!-- , TODO: and are actively planning a 0.3.0 release? -->.

- We now support blurred rounded rectangles, contributed by Markus Siglreithmaier ([vello#665][])
- Blend stack spilling was implementing, which allows blends more than four layers deep ([vello#661][]).
- Fixes for some visual artifacts in [vello#651][], [vello#659] and [vello#673][].
- Embedding existing wgpu textures is now supported ([vello#636][]).
- We now support bitmap glyphs (i.e. emoji), in addition to our previous COLR support ([vello#641][]).
- Work has begun on sparse strips, which is tracked in [vello#670][].
- Frame pacing for Android is being investigated and developed in [vello#674][].
- Thomas Smith has been doing some deep-dives into parallel scan, which is the core operation in Vello.
  This work can be found in [#gpu>Progress for GPU's Without Forward Progress Guarantees](https://xi.zulipchat.com/#narrow/stream/197075-gpu/topic/Progress.20for.20GPU's.20Without.20Forward.20Progress.20Guarantees).

<figure>

<img style="height: auto" src="rounded_rectangles.png" alt="A vertically split image. On the left are three blurred shapes: a blue rectangle with rounded corners, and two black ellipses. The ellipses are partially occluded by the boundaries of the image. On the right is a web page, showing three blog post titles and summaries, each in a box with a black box shadow." height="644" width = "944">

<figcaption>

Vello now supports blurred rounded rectangles, based on the maths      from [Raph's blog](https://raphlinus.github.io/graphics/2020/04/21/blurred-rounded-rects.html).
Left: Vello's blurred rounded rectangles test scene.
Right: [Blitz](https://github.com/DioxusLabs/blitz/)'s `servo` example which renders box shadows using this feature.

</figcaption>
</figure>

## Parley

Parley is a text layout library.

- Primitives required for robust text editing were added in [parley#106][].

## Others

- Bruce Mitchener has started work on Interpoli, a... <!-- TODO: Finish -->
- We have performed a [retrospective](@/blog/2024-08-24-roadmap-may-2024-retro.md) on our last roadmap.
- Daniel and Olivier's "office hours" appointments can still be booked by anyone for open-ended time-limited discussion of the ecosystem.
  - [See Daniel's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2).
  - [See Olivier's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2t767ZRETD_TkRI_VxK2ZTG0VrO9OZ4l7HvTxefhtJcg85iK0ZN7zWNnAEZtH0Dn7C1GKxrmYM).

<!-- TODO: Add remaining links -->
[xilem#396]: https://github.com/linebender/xilem/pull/396
[rfcs#7]: https://github.com/linebender/rfcs/pull/7
