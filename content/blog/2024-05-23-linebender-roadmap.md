+++
title = "May-July roadmap for Linebender"
authors = ["Olivier Faure"]
date = "2024-05-23 18:00:00"
+++

On May 7 and 8, 2024, folks from the Linebender team went to [the RustNL conference](https://2024.rustnl.org/) for two days, followed by two days of Unconference.

By that point, we all felt fairly exhausted, but Raph Levien being the cruel taskmaster that he is, he still made us work one extra day, and so we came together in our hotel's conference room to discuss the Linebender project, and what we wanted from it.

We'd initially thought of that this day would be an informal day of unstructured chatting about the ecosystem, but it fairly quickly turned into a semi-formal planning exercise, where we tried to figure out a roadmap for the next few months.

What follows is a comprehensive summary of what exactly we're planning to do.
We've tried to focus on blockers, where a missing feature in one project is stalling progress on other projects, because those are the hardest barriers to coordination, and we wanted to take advantage of being in the same room together to knock them down.


## Vello

### Robust dynamic memory

- **Priority:** Highest.
- **Assigned:** Daniel McNab.
- **Description:** Allocating dynamic memory in a robust way is essential to display large scenes without graphical artifacts. We have several possible solutions, we need to implement one.
- **Issue:** [vello#366](https://github.com/linebender/vello/issues/366)

### Reliable snapshot tests

- **Priority:** High.
- **Assigned:** Daniel McNab.
- **Description:** Snapshot tests are essential to test regressions in graphic software. Masonry's test suite heavily depends on them. But it's flaky right now because even when the fonts and inputs are exactly the same, renderers can have subtle differences that make the snapshots slightly different. We either need to create a fully deterministic renderer or add testing infrastructure that can tolerate small differences. [`nv-flip`](https://docs.rs/nv-flip/latest/nv_flip/) might have what we want.
- **Issue:** [vello#608](https://github.com/linebender/vello/issues/608)

### Sparse strips

- **Priority:** High.
- **Assigned:** Raph Levien.
- **Description:** Sparse strips are a new research direction that might significantly improve our rendering pipeline and our memory footprint. We want to write a prototype implementation so we can confirm the performance benefits.
- **Zulip thread:** [Sparse strip path rendering](https://xi.zulipchat.com/#narrow/stream/197075-gpu/topic/Sparse.20strip.20path.20rendering)

### Add tests

- **Priority:** Medium.
- **Assigned:** Olivier Faure.
- **Description:** Adding tests is often a "we'll do it later" item, but it has short-term benefits as well. Adding tests would help Vello contributors check that their modifications don't add regressions to the crate, and help us catch hidden bugs. As a first step, we'd want to at least add some extremely basic tests, such as checking that Vello doesn't crash when we run a sample image through the CPU shaders.
- **PR:** [vello#439](https://github.com/linebender/vello/pull/439)


## Masonry

### Complete text input

- **Priority:** Highest.
- **Assigned:** Olivier Faure, Aaron Muir Hamilton.
- **Description:** We'd like to bring up Masonry's textbox widget to be best-in-class among Rust frameworks. We want to support native-feeling interactions out of the box, such as cursor movement (go to next word, go to beginning of line), selection (select to end of text) and edition (IME, deleting an entire word, etc). This is something other framework developers have expressed interest in, so we want to implement it in a composable way.
- **Issue:** [xilem#388](https://github.com/linebender/xilem/issues/388)

### Better tracing for debugging

- **Priority:** High.
- **Assigned:** Olivier Faure.
- **Description:** Masonry currently uses the `tracing` crate to produce logs usable for debugging. These logs could be improved, so that a user finding a bug can quickly narrow in on information relevant to the bug, without having to restart the app with changed log levels. The bulk of the improvement would be to implement a new subscriber with multiple layers.
- **Issue:** [xilem#250](https://github.com/linebender/xilem/issues/250)

### Fix glaring documentation problems

- **Priority:** High.
- **Assigned:** Olivier Faure.
- **Description:** Masonry's documentation hasn't been updated since the port to Winit. While a lot of the code is still in flux, and thus hard to document, we should at least do a quick rewrite pass to remove references to Piet, Glazier, etc.
- **PR:** [xilem#329](https://github.com/linebender/xilem/pull/329)

### Finish repository port

- **Priority:** High.
- **Assigned:** Daniel McNab. (?)
- **Description:** We've ported the `masonry-rs` repository to the `linebender` organization, but that repository isn't where the crate's code lives anymore. We should update the README to reflect that information, port the issues to `linebender/xilem`, and archive `linebender/masonry`.
- **Zulip thread:** [Finishing the Masonry repository transition](https://xi.zulipchat.com/#narrow/stream/317477-masonry/topic/.E2.9C.94.20Finishing.20the.20Masonry.20repository.20transition)

### Write "Widgets in Arena" RFC

- **Priority:** High.
- **Assigned:** Olivier Faure.
- **Description:** This was something alluded to in the January blog posts. We want to move Masonry from storing widgets in WidgetPod to storing them in a slotmap-like arena. This is a large enough change to write an RFC about it.
- **PR:** [rfcs#6](https://github.com/linebender/rfcs/pull/6)

### Write "Pass order" RFC

- **Priority:** High.
- **Assigned:** Olivier Faure.
- **Description:** Right now widgets can use context methods in their different passes to request that other passes be ran. What passes can run what other passes isn't clearly documented, especially around edge cases. We should write an RFC to nail down the passes, how they can communicate, and the rationale around them.
- **PR:** TBD.

### Rewrite documentation

- **Priority:** Medium.
- **Assigned:** Olivier Faure.
- **Description:** Masonry's documentation hasn't been updated since the port to Winit. Once other major changes are implemented, the documentation should be brought up to date. We should avoid procrastinating on documentation too much just because the architecture is still in progress.
- **Issue:** [xilem#389](https://github.com/linebender/xilem/issues/389)

### Improve focus handling

- **Priority:** Medium.
- **Assigned:** Olivier Faure.
- **Description:** We should improve tab focus in Masonry. Right now tab focus has some bugs, doesn't cover buttons, isn't updated by clicks, and uses architecture inherited from Druid that could be strongly simplified once other architectural changes are completed.
- **Issue:** [xilem#390](https://github.com/linebender/xilem/issues/390)

### Improve pointer status handling

- **Priority:** Medium.
- **Assigned:** Olivier Faure.
- **Description:** We should centralize and improve how Masonry handles pointers. In particular, we should add an explicit "pointer capture" API that makes other widgets not get events from that pointer until the pointer is released or lost.
- **Issue:** [xilem#312](https://github.com/linebender/xilem/issues/312)


## Parley

### Create abstractions for text selection and editing

- **Priority:** Highest.
- **Assigned:** Chad Brokaw, Aaron Muir Hamilton.
- **Description:** We want Parley to implement types and methods that Masonry and other editors will be able to use to create a text-editing widget, or to handle selection in non-editable text. The initial implementation may be located in Masonry.
- **Issue:** [parley#52](https://github.com/linebender/parley/issues/52)

### Inline block layout

- **Priority:** High.
- **Assigned:** Chad Brokaw.
- **Description:** Parley should support laying out inline content the way web browsers do, where blocks are splices into your text and should be laid out according to available inline space the same way words are.
- **Issue:** [parley#25](https://github.com/linebender/parley/issues/25)
- **PRs:** [parley#67](https://github.com/linebender/parley/pull/67), [parley#76](https://github.com/linebender/parley/pull/76)

### Write adequate documentation

- **Priority:** High.
- **Assigned:** Nico Burns.
- **Description:** Parley currently has virtually no documentation. If we want it to become a fundational crate, we need to explain to people how to use it.
- **PR:** [parley#26](https://github.com/linebender/parley/pull/26)

### Software renderer

- **Priority:** Low.
- **Assigned:** Chad Brokaw.
- **Description:** Create a software renderer for the glyphs created by Parley, so that users can easily import Parley in their library without depending on a graphics API like Vello. The software renderer should be black-and-white for simplicity. We'll call for volunteers to implement COLRv1 and other advanced features.
- **PRs:** [parley#54](https://github.com/linebender/parley/pull/54), [parley#55](https://github.com/linebender/parley/pull/55)


## Xilem

### Android bringup

- **Priority:** Highest.
- **Assigned:** Daniel McNab.
- **Description:** We want Xilem to reliably work on Android. Running your Xilem app should require a small number of steps. Ideally, it should be as simple as running `cargo some-command` in your project.
- **PR:** [xilem#309](https://github.com/linebender/xilem/pull/309)

### Update repository layout.

- **Priority:** High.
- **Assigned:** Olivier Faure.
- **Description:** We've agreed on a layout for Linebender repositories. We should move projects in the Xilem repo to match that layout.
- **PRs:** [xilem#302](https://github.com/linebender/xilem/pull/302), [vello#590](https://github.com/linebender/vello/pull/590)

### Rework xilem_core to be compatible with Masonry backend

- **Priority:** High.
- **Assigned:** Daniel McNab.
- **Description:** The version of Xilem that was ported to Masonry used a different `View` trait that is incompatible with the current `xilem_core` crate. We need to rework `xilem_core` to get traits that can be used by both Masonry and `xilem_web`. Doing so while keeping readable code is non-trivial.
- **PR:** [xilem#235](https://github.com/linebender/xilem/pull/235), [xilem#310](https://github.com/linebender/xilem/pull/310)

### Write Documentation

- **Priority:** High.
- **Assigned:** Olivier Faure.
- **Description:** Xilem's lack of documentation is one of the problems new users complain about the most often. We need to avoid staying in that state for too long.
- **Issue:** [xilem#392](https://github.com/linebender/xilem/issues/392)

### Re-implement async integration

- **Priority:** Medium.
- **Assigned:** Daniel McNab.
- **Description:** One of Xilem's selling point is the ability to handle virtual lists and async sources of data. That ability was temporarily removed for the Masonry port, but we'd like to add it back it. We don't want Masonry to be aware of async stuff, but the AppDriver trait should at least provide a method which lets Xilem access Masonry internals on its own initiative.
- **Issue:** [xilem#340](https://github.com/linebender/xilem/issues/340)

### Add unit tests

- **Priority:** Low.
- **Assigned:** Olivier Faure.
- **Description:** We need to figure out an architecture for doing unit tests in Xilem. Tests can then be added by volunteer contributors.
- **Issue:** [xilem#393](https://github.com/linebender/xilem/issues/393)

### Add basic benchmarks

- **Priority:** Low.
- **Assigned:** Olivier.
- **Description:** Right now we're mostly working off vibes to know whether or not any of our work improved performance or introduced regressions.
We should write at least *one* benchmark that would test a basic Xilem use-case, so we get a general idea how our performance is progressing.
- **Issue:** [xilem#362](https://github.com/linebender/xilem/issues/362)


## AccessKit

Matt Campbell's contract with Google hasn't started yet.
When it does, top priority will likely be the Android backend.

The plan is to study existing Android accessibility implementations for non-Java toolkits, including Flutter, Chromium, and Gio, to get a better understanding of what's required to implement Android accessibility from scratch.

Beyond Android support, we've only had cursory discussions of what work Matt will do for the rest of the year.
Possible themes include scrollable containers, virtualized lists, richer text support (e.g. exposing font and style attributes) across platforms, and better integration into Xilem/Masonry.


## Android-specific work

Getting the Android port right involves a lot of small changes and making things feel non-broken.

Some notable sub-goals:

### Subclassing View/Application/Activity

- **Priority:** Highest.
- **Assigned:** Aaron Muir Hamilton, Raph Levien.
- **Description:** The Java classes of an Android application are where the rubber meets the road. We're investigating what we want our boilerplate to look like.

### Handling IME

- **Priority:** Highest.
- **Assigned:** Aaron Muir Hamilton, Raph Levien.
- **Description:** We need to extend Winit to be compatible with Android IME interfaces. An Android application without IME is a non-starter. Our design choices here will have influence on accessibility as well.

### Gesture recognition

- **Priority:** Low.
- **Assigned:** Aaron Muir Hamilton.
- **Description:** When the user swipes on the screen, we want to send some kind of scrolling events instead of a normal pointer event. Ideally there should be first-class support for that event to handle momentum scrolling, but a dumber solution would just emulate mouse wheel ticks.


## Conclusion

Whew! We have our work cut out for us there.

Overall, I'm feeling very optimistic about the coming months.
In the weeks since we first drafted this roadmap, we've already made a lot of progress on our respective tasks (for instance Daniel is now running on what async experts call a "work-stealing" algorithm), so it does look like we'll meet our deadlines.

What comes after that is a bit hazier.
Our three areas of focus are likely to be Android support, accessibility, and developer experience.

As we fill out gaps in our feature set, Xilem is moving closer to becoming a stable, mature framework for writing GUI applications.
We're not GUI yet, but we hope we'll be in the near future.

In the meantime, stay tuned!
