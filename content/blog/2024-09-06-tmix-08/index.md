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

<!-- TODO: Slightly better writing in these first two items -->
- Viewmarker [xilem#472][]
- Variable fonts experiments ([xilem#507][])
- We added Views for some existing and new widgets:
  - spinners, useful for showing indetermine progress ([xilem#497][]);
  - progress bars ([xilem#513][]); and
  - the `Portal` widget, which brings support for scroll areas to Xilem ([xilem#561][]).
- Xilem Web:
    <!-- TODO: -->
  - One or two most impactful items.

<figure>

<!-- TODO: Alt text -->
<img style="height: auto" src="stopwatch.png" alt="TODO" height="388" width = "559">

<figcaption>

Jared O'Connell additionally contributed a stopwatch example in [xilem#492][].

</figcaption>
</figure>

## Masonry

Masonry is the widget system used by Xilem.
This month has been dominated by Olivier's implementation of pass specification ([rfcs#7][]):

<!-- TODO: Rest of the pass specification PRs -->
- [xilem#522][]: Paint/Accessibility, with follow-up by Tom Churchman in [xilem#557][]

We also had some great community contributions, such as:

- A new progress Bar widget ([xilem#513][])
- Tom Churchman has contributed fixes for Portal scrolling ([#xilem#563][])
- Snapshot tests now are cross-platform and run in CI ([xilem#233][])

<figure>

<img style="height: auto" src="progress_bar.png" alt="A horizontal progress bar which is 70% filled with blue, with white text overlaid saying 70%. Below this is a checkbox labelled 'set indetermine progress', and a button labelled 'change progress'." height="129" width = "431">

<figcaption>

A progress bar widget was contributed by Richard Dodd in ([xilem#513][]).

</figcaption>
</figure>

We continue to make progress on other key features.

- Pan/flick gestures ([xilem#562][])
- Progress continues on text input for Android <!-- TODO: links? - cc @xorgy -->
- We were given a demonstration of initial [Android TalkBack](https://support.google.com/accessibility/android/answer/6283677) support in office hours. <!-- TODO: Link to PR? -->

<!-- TODO: Accessibility video. Embed? -->

## Vello

Vello is our GPU vector renderer.
We have made some really great progress in August<!-- , TODO: and are actively planning a 0.3.0 release -->.

- Blurred rounded rectangles, by Markus Siglreithmaier ([vello#665][])
- Blend stack spilling, allowing blends more than four layers deep ([vello#661][]).
- Fixes for some visual artifacts in [vello#651][], [vello#659] and [vello#673][].
- Embedding existing wgpu textures ([vello#636][]).
- Bitmap glyph (emoji) support in ([vello#641][]).
- Work has begun on sparse strips, which is tracked in [vello#670][].
- Frame pacing for Android is being investigated and developed in [vello#674][].
- <!-- TODO: Thomas Smith incredible deep-dives into parallel scan, which is the core operation in Vello. -->

<figure>

<img style="height: auto" src="..." alt="..." height=".." width = "..">

<figcaption>

Rounded rectangles!
Likely: Split image - Vello example and Blitz Web

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
