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

<!-- TODO: Slightly better writing here -->
- Viewmarker [xilem#472][]
- Variable fonts experiments ([xilem#507][])
- Stopwatch example [xilem#]
- Views for existing and new widgets: spinner ([xilem#497][]), portal ([xilem#561][]), progress bar ([xilem#513][])
- Xilem Web:
    <!-- TODO: -->
  - One or two most impactful items.

<figure>

<img style="height: auto" src="..." alt="..." height=".." width = "..">

<figcaption>

...
<!-- TODO: Is there a relevant image for here? Maybe the stopwatch? -->

</figcaption>
</figure>

## Masonry

Masonry is the widget system used by Xilem.
This month has been dominated by Olivier's implementation of pass specification ([rfcs#7][]):

<!-- TODO: Rest of the pass specification PRs -->
- [xilem#522][]: Paint/Accessibility, with follow-up by Tom Churchman in [xilem#557][]

Other work from August includes:

- Tom Churchman has contributed fixes for Portal scrolling ([#xilem#563][])
- Snapshot tests now are cross-platform and run in CI ([xilem#233][])
- Progress Bar widget ([xilem#513][])
- Pan/flick gestures ([xilem#562][]).
- Progress continues on text input for Android <!-- TODO: links? - cc @xorgy -->
- We were given a demonstration of initial [Android TalkBack](https://support.google.com/accessibility/android/answer/6283677) support in office hours.

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

</figcaption>
</figure>

## Parley

Parley is a text layout library.

- Primitives required for robust text editing added in [parley#106][].

## Others

- Bruce Mitchener has started work on Interpoli, a... <!-- TODO: Finish -->
- We have performed a [retrospective](@/blog/2024-08-24-roadmap-may-2024-retro.md) on our last roadmap.
- Daniel and Olivier's "office hours" appointments can still be booked by anyone for open-ended time-limited discussion of the ecosystem.
  - [See Daniel's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2).
  - [See Olivier's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2t767ZRETD_TkRI_VxK2ZTG0VrO9OZ4l7HvTxefhtJcg85iK0ZN7zWNnAEZtH0Dn7C1GKxrmYM).

<!--  -->
<!-- TODO: Add remaining links -->
[xilem#396]: https://github.com/linebender/xilem/pull/396
[rfcs#7]: https://github.com/linebender/rfcs/pull/7
