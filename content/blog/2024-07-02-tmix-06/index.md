+++
title = "This Month in Xilem, June 2024"
authors = ["Daniel McNab"]
+++

Following on from May, Olivier wrote and published a [report](@/blog/2024-06-15-rustnl-2024-unconference.md) on our experience of the RustNL unconference.
This led to some useful discussion online - see [the zulip thread](https://xi.zulipchat.com/#narrow/stream/181284-blogging/topic/Draft.20-.20Report.20on.20the.20RustNL.202024.20Conference/near/444974910) corresponding with that post for links.

We released [Vello 0.2.0][] at the start of the month, which included some key improvements discussed in previous months' updates.
In June, we landed some initial handling of Emoji ([vello#615][]), which supports glyphs which use the COLR specification.
Some in-flight work includes an enhanced testing setup ([vello#610][]), GPU memory allocation ([vello#606][]) robustness, and improved correctness for some unusual strokes ([vello#607][]).

<figure>

<img src="Xilem Emoji Picker.png" alt="Screenshot of an app, containing a 4x4 grid of labelled Emoji, zoom in and out buttons and pagination controls. 'cat face with wry smile' is selected" height="1132" width = "863">

<figcaption>

A new Emoji viewer example ([xilem#420][]) running on Android.
This example currently only works for systems with COLR system emoji, such as some Android systems.

</figcaption>
</figure>

[xilem#310][] brought in some new ideas about how Xilem Core can be structured, to move away from the macro heavy design used previously.
I believe that this has additional advantages for specialised `View` types, such as OS windows, although this is yet to be integrated.
Support for the `Adapt` views built on this has enabled an elm<!-- TODO: Capitalisation? --> -inspired example for Xilem in [xilem#401][], showing the flexibility of the Xilem architecture.
Our observability support has also been improved, with automatic tracing to a file in debug builds added in [xilem#384][].
In the near term, our focus will be on integration with async Rust, for features such as timers and network interaction.

However, there has also been excellent community engagement with Xilem in June.
Veniamin Ilmer has created a [Minesweeper game](https://github.com/veniamin-ilmer/minesweeper_xilem/) using Xilem.
The discussion on Zulip (in [#xilem > Minesweeper converted from Iced to Xilem](https://xi.zulipchat.com/#narrow/stream/354396-xilem/topic/Minesweeper.20converted.20from.20Iced.20to.20Xilem)) has been very useful in guiding areas for improvement.
<!-- Screenshot of Minesweeper example -->

Philipp Mildenberger has been prolific in their work on Xilem, as a co-author of the new Xilem Core, and in adapting Xilem Web to use it.
They also worked to bring support for `OneOf` views to Xilem Core in [xilem#394][] - these were previously only supported in Xilem Web.

<!-- Screenshot of a web example? -->

We are now in the final month of the [May-July 2024 roadmap](@/blog/2024-06-15-rustnl-2024-unconference.md).
In the intervening time much of the work has been started or completed, although there have been some changes of priorities.
I expect that we will soon be looking at prioritising items on our [long-term roadmap](@/wiki/long-term-roadmap.md).

[xilem#420]: https://github.com/linebender/xilem/pull/420
[xilem#394]: https://github.com/linebender/xilem/pull/394
[vello 0.2.0]: https://github.com/linebender/vello/releases/tag/v0.2.0

[vello#606]: https://github.com/linebender/vello/pull/606
[vello#607]: https://github.com/linebender/vello/pull/607
[vello#610]: https://github.com/linebender/vello/pull/610
[vello#615]: https://github.com/linebender/vello/pull/615
