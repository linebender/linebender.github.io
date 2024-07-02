+++
title = "This Month in Xilem, June 2024"
authors = ["Daniel McNab"]
+++

We released [Vello 0.2.0][] at the start of the month, which included some key improvements discussed in previous months' updates.
In June, we merged initial handling of Emoji ([vello#615][]), which supports glyphs which use the COLR specification (so this does not, for example, support Apple's system Emoji).
In-flight work includes an enhanced testing setup ([vello#610][]), GPU memory allocation robustness ([vello#606][]), and improved correctness for some unusual strokes ([vello#607][]).

<figure>

<img src="Xilem Emoji Picker.png" style="height: auto" alt="Screenshot of an app, containing a 4x4 grid of labelled Emoji, zoom in and out buttons and pagination controls. 'cat face with wry smile' is selected" height="1132" width = "863">

<figcaption>

An in-progress Emoji viewer example ([xilem#420][]) running on Android.
This example currently only works for systems with COLR system emoji, such as some Android systems.

</figcaption>
</figure>

[xilem#310][] brought in some new ideas about how Xilem Core can be structured, to move away from the macro heavy design used previously.
I believe that this has additional advantages for specialised `View` types, such as OS windows, although this is yet to be integrated.
Support for the `Adapt` views built on this has enabled an [Elm](https://elm-lang.org/)-inspired example for Xilem in [xilem#401][], showing the flexibility of the Xilem architecture.
Our observability support has also been improved, with automatic tracing to a file in debug builds added in [xilem#384][]; [xilem#396][] will lay the foundation for more advanced debugging capabilities.
My immediate focus will be on integration with async Rust, for features such as timers and network interaction.

Philipp Mildenberger has been prolific in his work on Xilem, as a co-author of the new Xilem Core, and in adapting Xilem Web to use it.
He also worked to bring support for `OneOf` views to Xilem Core in [xilem#394][] - these were previously only supported in Xilem Web.

There was also some excellent community engagement with Xilem in June.
Veniamin Ilmer has created an open source [Minesweeper game](https://github.com/veniamin-ilmer/minesweeper_xilem/) using Xilem.
The discussion on Zulip (in [#xilem > Minesweeper converted from Iced to Xilem](https://xi.zulipchat.com/#narrow/stream/354396-xilem/topic/Minesweeper.20converted.20from.20Iced.20to.20Xilem)) has been very useful in guiding areas for improvement.      

<figure>

<img style="height: auto" src="Minesweeper.png" alt="A window titled Minesweeper, with text in the top-left 'Mines: 82', a top-center button with a neutral ASCII emoticon, and a board represented by a grid of buttons 30 wide and 16 tall, where some buttons are replaced with colour labels containing digits, and some buttons contain an exclamation mark." height="589" width = "931">

<figcaption>

Screenshot by Veniamin Ilmer of their Minesweeper game, used under the [Apache 2.0](https://github.com/veniamin-ilmer/minesweeper_xilem/blob/main/LICENSE) license.

</figcaption>
</figure>

Following on from May, Olivier wrote and published a [report](@/blog/2024-06-15-rustnl-2024-unconference.md) on our experience of the RustNL unconference.
This led to some useful discussion online - see [the zulip thread](https://xi.zulipchat.com/#narrow/stream/181284-blogging/topic/Draft.20-.20Report.20on.20the.20RustNL.202024.20Conference/near/444974910) corresponding with that post for links.
We are now in the final month of the [May-July 2024 roadmap](@/blog/2024-06-15-rustnl-2024-unconference.md).
In the intervening time much of the work has been started or completed, although there have been some changes of priorities.
I expect that we will soon be looking at prioritising items on our [long-term roadmap](@/wiki/long-term-roadmap.md).

[xilem#310]:https://github.com/linebender/xilem/pull/310
[xilem#384]: https://github.com/linebender/xilem/pull/384
[xilem#394]: https://github.com/linebender/xilem/pull/394
[xilem#396]: https://github.com/linebender/xilem/pull/396
[xilem#401]:https://github.com/linebender/xilem/pull/401
[xilem#420]: https://github.com/linebender/xilem/pull/420
[vello 0.2.0]: https://github.com/linebender/vello/releases/tag/v0.2.0

[vello#606]: https://github.com/linebender/vello/pull/606
[vello#607]: https://github.com/linebender/vello/pull/607
[vello#610]: https://github.com/linebender/vello/pull/610
[vello#615]: https://github.com/linebender/vello/pull/615
