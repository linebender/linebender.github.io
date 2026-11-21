+++
title = "Advice for the next 100 Rust GUIs"
authors = ["Raph Levien"]
+++

This is a followup to [Advice for the next dozen Rust GUIs], published 4.5 years ago.
The world is different in many ways, but some of the advice there is still valid.
There have been roughly 20 Rust GUI toolkits announced since that blog post (see [2026 survey]), but also a recent rapid acceleration of interest in Rust GUI, fueled by AI.

## Rust GUI is still compelling

The motivation to build GUI apps in Rust remains strong.
Many shipping GUI apps suffer from poor performance, more so when built with cross-platform technology.
GUI toolkits shipped by platforms have largely stagnated, and system vendors remain uninterested in solving the cross-platform problem, so Electron has grown in popularity.
RAM remains expensive and the focus is shifting to mobile, so bloat has real consequences.
Building high-performance UI in Rust is a practical alternative.

Adoption of Rust UI in the past few years has been slow.
Few flagship apps use Rust for their UI.
One of the most notable is the Zed editor, built on their own GPUI framework.
It proves that shipping Rust UI is practical, and indeed, GPUI is emerging as a major choice for AI-driven rewrites, most notably Discord.
My personal sense is that GPUI is well suited to the Zed use case, and shipping in a real project boosts its visibility and credibility quite a bit, but it’s not necessarily the ideal choice for more general purpose applications.

The reasons for slow adoption may be melting away.
Writing Rust code is harder than languages more traditionally used for UI (TypeScript, Kotlin, Swift), and finding engineers skilled in both Rust and UI is challenging.
But AI is changing the game.
There’s [evidence][How does programming language affect token efficiency and correctness?] that the LLM cost of writing Rust code is not significantly higher than other languages, and there is reason to believe that in large codebases, Rust will be more maintainable because its type system helps enforce invariants.
So if all languages have roughly the same cost, then why not go with one that has a dramatically better combination of safety and performance?

For these reasons, Rust is rapidly becoming *the* language for AI rewrites of existing codebases.
But there is still no consensus on the strategy for the UI layer.
I predict there will be a small explosion of trying lots of different things.

## Native is complicated

My 2022 blog post had a large section on “native,” first pointing out that the term has no clear meaning, and more broadly pointing out that “native” UI toolkits are simply choices among many, and should be evaluated on the important quality criteria.

I think this is largely still true, but it depends greatly on which platform is being targeted.
Windows still has no clear native choice (see [Microsoft Hasn’t Had a Coherent GUI Strategy Since Petzold]) and on Mac there is a bit of fragmentation between AppKit and SwiftUI.
However, SwiftUI has performance issues (see [Michael Tsai blog from 2025]) and so it is clearly possible to do better on the performance front.

While desktop UI has remained stagnant, the situation on mobile is a bit different.
Platforms continue to invest, and there are many fine details that are important for an app to look and feel native.
One very notable example is Liquid Glass on iOS – if you use UIKit or SwiftUI, you can get that with little effort, but apps built with cross-platform toolkits won’t quite seem right.
In addition, making gesture recognition, scroll dynamics, and IME 100% match the platform are not yet fully solved problems.

Thus, for mobile, an extremely viable path is to use the native toolkit for UI but do the common “business logic” in Rust.
I’m not sure about the business logic, but for the UI, this is the decision that Shopify has made recently, abandoning React Native.
I believe 1Password has this architecture.
This is also the "Phoenix" plan for Discord on mobile, while using GPUI on desktop.

That said, there's been considerable progress in trying to build mobile UI entirely in Rust.
Below is a short screencast of the editor demo in [android-view].
You can see that a lot of IME functionality works, including swiping, moving the cursor by rolling the space bar.
Not shown in the video below, but composition regions (needed for CJK) also work, and there's basic accessibility through AccessKit.

{{ video(src="android-view-demo.mp4") }}

## Text layout and accessibility infrastructure is basically solved

One of the big unsolved problems as of the 2022 blog post was text layout.
It is of course a hard problem to get right.
I’m pleased to say that the ecosystem now has a good solution, in the form of [Parley].
It’s gotten there as a result of sustained investment from Google, Canva, and Dioxus (now Cognition), as well as contributions from the community.
It seems to be used in the majority of new Rust UI toolkits.
In addition, to help mitigate the risk of monoculture, Cosmic Text is a viable alternative.

AccessKit was just getting started in 2022, but is now solid infrastructure as well, and used by most UI toolkits.
I should note, though, there is a difference between a *good* integration and a box ticking exercise.
And there is more work to be done, for example to make rich text editing fully accessible.

## System compositor is still unsolved

One topic I talked about extensively in the 2022 blog post is leveraging the system compositor.
This issue is even more pressing today, especially with the increased importance of video.
You *really* want a video playing in a hardware accelerated layer when possible, and on low end devices it’s the only hope for getting smooth frame rates.

There has been some progress, notably the [subduction] library, but it certainly isn’t commonly integrated into UI toolkits yet.
To do it well is a challenging problem, as the capabilities provided by the platform vary widely.
Ideally there’s a rich and high level interface, something like CALayer on Apple platforms, and polyfills to implement that interface as well as possible on other platforms.
On Android, when the task is something simple, like overlaying some GUI controls on a video, it uses SurfaceFlinger layers, but when it involves more sophisticated compositing, it would run shaders on the GPU.

This is absolutely something we should build, ideally as a high quality community project rather than something with dozens of vibe-coded forks.
But even in the latter case it’s better for it to exist than not.

Discussion question: talk about winit stalling out?

## Immediate vs retained mode

One of the challenges of the Rust UI ecosystem is that different applications have different needs.
The idea of a single, unified UI for completely general purpose use may be behind us, especially now that LLMs can optimize and fine tune for specific use cases.

That’s been playing out architecturally in the choice between immediate mode and retained mode.
Immediate mode is very appealing because it’s less total code and seems quite a bit simpler.
And for some applications (most especially development support for games) it works well.
However, I am concerned it won’t scale especially well.
In short, it’s good at the first 90%, but it makes the second 90% harder.
To be considered really production-ready, it has to establish stable widget identities to build and update the accessibility tree, it has to reliably cache expensive intermediate artifacts such as text layouts (simple immediate mode often gets around this by cutting corners, which is fine for gamedev).
And of course, it pulls strongly towards a model of re-rendering everything every frame rather than exploiting layers in the compositor.

For this reason among others, I am not impressed by the choice of the [ArtCraft] projects to use egui.
I do think egui is a really good implementation of the immediate mode paradigm in Rust, and it does solve a number of problems (including accessibility, something Iced has yet to tackle), but it wouldn’t be my first choice for productivity and design applications.

## Clearly defined modules

One theme that’s emerging from trying to do LLM-assisted software engineering *well* is having clearly scoped modules, ideally with solid specifications of what the module does.
Within a module, given clear objectives such as performance and robustness against adverse inputs, an LLM can do very well.

This philosophy is in tension with what has historically worked well.
People reasonably choose a UI toolkit based on how good it looks.
In particular, Apple UI frameworks have an immense amount of design knowledge baked in, especially in the defaults.
With very little effort, it’s possible for a developer to make an app that looks and feels like a Mac app, even if not very polished.

I think in the future we should be moving toward a separation of *design* and *execution.*
Both are hard problems, but very different.
I’m especially excited about how much room there is to improve the execution side when really optimizing.
As discussed in the 2022 blog post, the computer science core of a UI toolkit is an incremental computation engine.

Having a good spec at the interface between design and execution is critically important.
As a general strategy, I suggest adapting web standards, but not following them slavishly.
Web standards are an impressive and important artifact.
A huge amount of time and energy has gone into producing them, and there’s also a lot of care to making things work cross-platform.
There are also extensive test suites and multiple high quality open source implementations.
All this goes a long way to explaining why Electron is popular.

That said, strictly adhering to web standards has serious downsides.
DOM is slow and memory-intensive; a lighter weight tree representation with a bulk mutation interface might serve the need of GUI apps better.
Full CSS style resolution and layout is also expensive, though CSS is a rich and powerful way to describe styling.
A subset of CSS carefully curated for efficiency might be useful.
I’ve done [experiments](https://github.com/linebender/xilem/pull/75) in implementing selectors as an efficient state machine that supports incremental evaluation, at the expense of fancier CSS features such as sibling selectors.

The Web is also lacking a solid way to represent virtualized lists, a core feature of every efficient UI toolkit.
A spec here requires nailing down the interface with the data model, preserving incrementality.

Blitz, as a modular browser engine, is well placed to serve as the cornerstone of hybrid approaches borrowing heavily from Web specs, but also adapting them to be higher performance and more tightly integrated with platform capabilities.
As just one example, it’s possible to start with Stylo as the CSS selector engine, and then replace it with a leaner, more efficient CSS engine if that proves to be practical.

One good example of separation of concerns into smaller modules is the Xilem/Masonry split.
The Xilem reactive layer was designed to be independent of the actual elements it controls, and indeed it has instantiations both for DOM (which, among other things, is the infrastructure for the [squircle] interactive demo) and Masonry widgets.
Masonry, on the other hand, was designed to be independent of the reactive layer and indeed the language for the layer above it.

By contrast, SwiftUI mashes together the low level widget implementation, the reactivity framework, and the language as well; LLVM reflection from the Swift language is part of the execution engine.
Obviously SwiftUI has had a lot farther reach than Xilem, but my argument today is that we should reconsider assumptions as things are rapidly changing.

### Optimization opportunities

Given a UI design encoded in a certain way – which can be code, data, or a combination of the two in a DSL – an engine has a number of opportunities to optimize the execution of that design.
Perhaps a lot of this section is gilding the lily, as a straightforward evaluation of the design in optimized Rust code will outperform almost any shipping GUI.
But I’m interested in the limits of what performance is possible.
Can we get silky smooth UI even on low end Android phones?
I think the answer is yes, and the question worth exploring.

One useful tactic may be separating out the static from the dynamic parts of the design, and precomputing the static parts.
That in no way precludes the design having dynamic parts; it's just that it may be possible to avoid the cost of a fully dynamic implementation.

Another fruitful avenue for optimization is optimizing the use of layers, to be composited either using the system compositor or a lightweight GPU-accelerated polyfill.
Using layers effectively can dramatically improve performance, mostly by avoiding re-rendering of parts of the interface that haven't changed.
However, layers have a RAM cost, and using too many layers can actually slow things down, as allocating new layers has to go through calls to the underlying system.
Getting this right is a tricky optimization problem, and a good use of either measurement or analysis.
UI toolkits generally either use heuristics, which are not always perfect, or explicit hints (like [.drawingGroup()](https://developer.apple.com/documentation/swiftui/view/drawinggroup(opaque:colormode:)) in SwiftUI or [RepaintBoundary](https://api.flutter.dev/flutter/widgets/RepaintBoundary-class.html) in Flutter).

For a deeper dive into performance questions, see [A Critique of GPUI].
Ideally, there aren't huge performance differences between UI toolkits, as all low hanging fruit should be picked.
We expect implementations of system languages to be quite comparable in performance, but still tolerate wide discrepancies in GUI performance.

## Advice

* Be bold.
  Now is the time.
  Rust UI has the potential to deliver vastly higher performance than the status quo.
* Build on solid foundations.
* Use well-defined specs when available.
* Find ways to work well with open source.
  Work with upstream but don’t spam them with slop PRs.

## Acknowledgements

Thanks to Nico Burns for reviewing an earlier draft of this post.
The current draft hasn't been carefully reviewed and checked, so may very well contain some mistakes.

[Advice for the next dozen Rust GUIs]: https://raphlinus.github.io/rust/gui/2022/07/15/next-dozen-guis.html
[2026 survey]: https://blog.wybxc.cc/blog/rust-gui-survey-2026/
[A Critique of GPUI]: https://blog.zortax.de/posts/a-critique-of-gpui
[How does programming language affect token efficiency and correctness?]: https://danluu.com/pl-tokens/
[Microsoft Hasn’t Had a Coherent GUI Strategy Since Petzold]: https://www.jsnover.com/blog/2026/03/13/microsoft-hasnt-had-a-coherent-gui-strategy-since-petzold/
[Michael Tsai blog from 2025]: https://mjtsai.com/blog/2025/06/18/swiftui-at-wwdc-2025/
[subduction]: https://github.com/forest-rs/subduction
[ArtCraft]: https://getartcraft.com/
[squircle]: https://linebender.org/wiki/curves/squircle/
[android-view]: https://github.com/rust-mobile/android-view
[Parley]: https://github.com/linebender/parley
