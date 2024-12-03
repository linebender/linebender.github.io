+++
title = "Linebender in November 2024: color!"
authors = ["Bruce Mitchener", "Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

This month's update contains some surprises, even to us!

## Color

When we published the update for [October, 2024](/blog/tmix-10/), we had already been discussing starting a new color-focused crate, but hadn't yet created a repository.

We may have a dedicated blog post about the Color crate in the future and laying out some of the motivations, but we'll go over it a bit here.

Linebender, among other things, believes in helping build solid foundations for people building user interfaces and working in graphics, in the Rust programming language (and on GPUs).
Upon seeing a problem, we would rather produce a focused crate that solves that problem and that can be used by others within the Rust ecosystem.
Color is one such area.
It is much harder than it may appear at first, with many complicating factors like what should be done by a basic color struct, what color spaces to support, and whether or not to support color formats used in print (like CMYK).

We looked around at a wide variety of crates in various stages of maintenance and development, their feature sets, the "weight" of those crates, and how closely they fit in with what we would need in Vello, tiny-skia, resvg, svgtypes, Masonry, and Xilem.
We decided to do a crate based on the [CSS Color Module Level 4] draft specification, with support for a limited number of additional color spaces.

We drew a line in the sand and released a preview of our crate as [color 0.1.0][] on November 21, 2024.
The documentation lays out more of the scope, goals, and main types provided by the crate.

Since then, we have begun the work of making Peniko and Vello use the Color crate and improving it to support that usage.
Early next year, we will use it within svgtypes, resvg, and tiny-skia.

We will be releasing a 0.2.0 release within the next month and encourage people to give it a try.
The 0.2.0 release will include some additional (non-CSS 4) color spaces like ACEScg, OkHsl, and OkHsv as well as things that we found useful when using it in Peniko and Vello.

Many thanks to [Brendan Zabarauskas][] for working with us to transfer his `color` crate name for our use.
His original crate was published as 0.0.1 on November 21, 2014, motivating us ten years later to ship our 0.1.0 on November 21, 2024.

We're very excited about the future of this crate and look forward to talking to other people within the Rust ecosystem about using it.

## Kurbo

Kurbo has seen some minor updates within the last month and we're planning to publish them in a minor patch release within the next month.

Internally, we're talking about the future direction of Kurbo and how we can improve upon it as a foundational crate for curves, shapes, and math.
Hopefully we'll be writing about that in more detail within the next couple of months.

## Peniko

Peniko will have a big release within the next month, once the new release of Color is out, that will remove the old `peniko::Color` type and replace it with the new create.
This release will be the start of a path towards wider color gamut support throughout the Linebender ecosystem.

We have long had a pain point with Peniko where it defines some resource management types, `Blob`, `Font`, and `WeakBlob`.
These are used within Parley and Vello as well as by anyone using those crates.
We plan to move these into a separate crate to ease versioning issues and to take advantage of this to improve the functionality.
This will probably be coming in the next big release following the one adding Color.

## Vello

Vello too will have a big release within the next month, building upon the new releases of Peniko and Color.

Additionally:

* [vello#735][], [vello#743][]: Update to wgpu 23.0.1, matching what is used by Bevy 0.15.
* [vello#722][]: Improving image sampling.
* [vello#740][]: Correcting support for PNG glyphs in fonts, important for emoji on macOS.


## Parley

***SOMEONE ELSE WRITE THIS***

## Masonry

***SOMEONE ELSE WRITE THIS***

## Xilem

***SOMEONE ELSE WRITE THIS***

## Piet

While we aren't yet planning on a new release of Piet as there have been no changes since last month, we did begin a discussion about a possible major revision to Piet and what a next-generation Piet might look like.

Something great about Piet was the idea that with a single rendering API, you could target multiple back-ends, mostly ones that were platform specific, but also SVG.
With the advent of Vello, this was left behind, with a focus on making Vello work.
Also, many things that used to be part of Piet are now separate crates, like Peniko, Parley, and Color.
These new crates exceed the capabilities that were present in Piet.

Nico Burns started a discussion about this that took off in our Zulip, [Low-level Piet].
We look forward to continuing to discuss this topic over the coming months.

## tiny-skia, resvg, simplecss, svgtypes

In the last month, we have made progress on converting the simplecss and svgtypes crates over to using the Linebender CI infrastructure.
Work will continue on the tiny-skia and resvg repositories.

The re-licensing of the crates within the resvg repository has been completed and these are now under the "Apache-2.0 OR MIT" license pair.

Additionally, a couple of exciting improvements have landed in the resvg repository:

* [resvg#843][]: Add support for the `!important` attribute.
* [resvg#850][]: Add support for embedded luma JPEG images.

We haven't yet set a timeline for doing the next release of the resvg family of crates as it will be the first that we have done after assuming stewardship of these crates.

## Velato, Vello SVG

[velato 0.4.0][] was released, updating it to use Vello 0.3. Additionally, 0.3.1 was released which correctly implemented non-linear easing.

[vello_svg 0.5.0][] was released, updating it to use usvg 0.44.

## Research and Future Directions

Linebender has an origin story in being a very research oriented group, looking to break new ground.
While we are focused on shipping code today, we still have an eye on the future and how to be prepared for the new opportunities and technologies that are coming.
This can be seen in our thinking about the future of Piet as previously discussed as well as the work that we're doing with Color.

We have a couple of active and ongoing research projects:

* Raph Levien on new rendering approaches for Vello.
* Joe Neeman on boolean path operations.
* ***IS THERE ANYTHING ELSE TO MENTION HERE?***

This last month also saw the transition of the Slang shader language from Nvidia to Khronos.
We don't have any immediate plans to adopt Slang in Vello (we're already pretty busy!), but we are looking at Slang and thinking about how we can best make use of it to improve the usefulness of our crates.
We'd love to [talk][] to people who are adopting Slang within the Rust ecosystem or interested in publishing Slang code for wider use.

## Get Involved!

We welcome collaboration on any of our crates.
This can include improving the documentation, implementing new features, improving our test coverage, or using them within your own code.

[CSS Color Module Level 4]: https://www.w3.org/TR/css-color-4/
[Low-level Piet]: https://xi.zulipchat.com/#narrow/channel/197075-gpu/topic/Low-level.20Piet
[talk]: https://xi.zulipchat.com/#narrow/channel/197075-gpu

[color 0.1.0]: https://docs.rs/color/0.1.0/color/

[resvg#843]: https://github.com/linebender/resvg/pull/843
[resvg#850]: https://github.com/linebender/resvg/pull/850

[vello#722]: https://github.com/linebender/vello/pull/722
[vello#735]: https://github.com/linebender/vello/pull/735
[vello#740]: https://github.com/linebender/vello/pull/740
[vello#743]: https://github.com/linebender/vello/pull/743

[velato 0.4.0]: https://github.com/linebender/velato/releases/tag/v0.4.0
[vello_svg 0.5.0]: https://github.com/linebender/vello_svg/releases/tag/v0.5.0
