+++
title = "Retrospective on the May-July roadmap"
authors = ["Daniel McNab"]
+++

At the start of May, we created a [roadmap](@/blog/2024-06-17-roadmap-may-2024.md) for the following three months.
We recently went through a process of analysing how well our progress was guided by the roadmap; this report summarises those results.

## Vello [(original)](@/blog/2024-06-17-roadmap-may-2024.md#vello)

### Robust dynamic memory [(original)](@/blog/2024-06-17-roadmap-may-2024.md#robust-dynamic-memory)

- **Priority:** Highest.
- **Status**: Experimental PR ([vello#606](https://github.com/linebender/vello/pull/606)), but hasn’t landed.
  Work required on the CPU side API, but the GPU operations are well-understood.
  This PR could not be released in its current form.

### Reliable snapshot tests [(original)](@/blog/2024-06-17-roadmap-may-2024.md#reliable-snapshot-tests)

- **Priority:** High.
- **Status**: This was implemented in [vello#610](https://github.com/linebender/vello/pull/610).
  This has already been extremely useful in our review process, as it allows changes to be validated in the files changed view.
  We also implemented an experiment to use [Git LFS](https://git-lfs.com/), which has been working really well to avoid increasing repository size.

### Sparse strips [(original)](@/blog/2024-06-17-roadmap-may-2024.md#sparse-strips)

- **Priority:** High.
- **Status**: Not much progress. We have deferred this.

### Add tests [(original)](@/blog/2024-06-17-roadmap-may-2024.md#add-tests)

- **Priority:** Medium
- **Status**: This item had already been completed at the time the roadmap was written.
  This was also expanded upon in.
  During our retrospective meeting, we did determine some potential future expansions to testing, namely:
  - Running individual stages in isolation, to enable property testing (TODO: Make Vello issue).
  - Testing for additional platforms, such as DirectX on Windows and Android (TODO: Make Vello issue).

### Progress beyond roadmap {#vello-extras}

We have also landed several important features which were not part of our planned roadmap.
These include:

- Blending which uses more than four deep layers ([vello#661](https://github.com/linebender/vello/pull/661)).
- Preliminary support for embedding pre-existing `wgpu` textures ([vello#636](https://github.com/linebender/vello/pull/636)).
  This was driven by user needs.
  Further work in this direction is tracked in [vello#664](https://github.com/linebender/vello/issues/664).
- Support for Emoji (COLRv1 in [vello#615](https://github.com/linebender/vello/pull/615), bitmap in [vello#641](https://github.com/linebender/vello/pull/641)).

The [stroke expansion paper](https://linebender.org/gpu-stroke-expansion-paper/) documents a key part of Vello's pipeline.
This was presented at ACM High Performance Graphics (HPG) 2024, placing 3rd Place in the Best Paper Award.

## Masonry [(original)](@/blog/2024-06-17-roadmap-may-2024.md#masonry)

### Complete text input [(original)](@/blog/2024-06-17-roadmap-may-2024.md#complete-text-input)

- **Priority**: Highest
- **Status**: Not that much progress.
  There were some community contributions such as

### Better tracing for debugging [(original)](@/blog/2024-06-17-roadmap-may-2024.md#better-tracing-for-debugging)

- **Priority**: High
- **Status**: [xilem#384](https://github.com/linebender/xilem/pull/384) implements most of this, the rest is deferred.
  There are known issues with the size of the log files created by default in very large apps.

### Fix glaring documentation problems [(original)](@/blog/2024-06-17-roadmap-may-2024.md#fix-glaring-documentation-problems)

- **Priority**: High
- **Status**: The issues mentioned in this item were addressed in [xilem#329](https://github.com/linebender/xilem/pull/329), and.

### Finish repository port [(original)](@/blog/2024-06-17-roadmap-may-2024.md#fix-glaring-documentation-problems)

- **Priority**: High
- **Status**: This was completed.
  We moved all Masonry issues into the Xilem repository, and updated all references in code to use full links.
  This move was coordinated in [#linebender > Moving Masonry into Xilem](https://xi.zulipchat.com/#narrow/stream/419691-linebender/topic/Moving.20Masonry.20into.20Xilem).

### Write “widgets in arena” RFC [(original)](@/blog/2024-06-17-roadmap-may-2024.md#write-widgets-in-arena-rfc)

- **Priority**: High
- **Status**: The RFC was [rfcs#6](https://github.com/linebender/rfcs/pull/6), which was mostly implemented in [xilem#396](https://github.com/linebender/xilem/pull/396).

### Write “pass order” RFC [(original)](@/blog/2024-06-17-roadmap-may-2024.md#write-pass-order-rfc)

- **Prioriy**: High
- **Status**: [rfcs#7](https://github.com/linebender/rfcs/pull/7) is in draft, and several important parts of the RFC have been completed.
  We have also learned several important lessons about the RFC process, including.

### Rewrite documentation [(original)](@/blog/2024-06-17-roadmap-may-2024.md#rewrite-documentation)

- **Priority**: Medium
- **Status**: This work has been deferred until the "pass order" refactor is complete, as we expect Masonry's core to be quite stable at that point.

### Improve focus handling [(original)](@/blog/2024-06-17-roadmap-may-2024.md#improve-focus-handling) ?

- **Priority**: Medium
- **Status**: Not done.

### Improve pointer status handling [(original)](@/blog/2024-06-17-roadmap-may-2024.md#improve-pointer-status-handling) ?

- **Priority**: Medium
- **Status**: Basic translation of touch events was added in ([xilem#313](https://github.com/linebender/xilem/pull/313)).
  This unblocked using Masonry on devices where pointer input is not expected (primarily on Android).
  We have split the remainder of this task into two tasks, with a priority placed on completing the parts which unblock other work (such as scrolling on Android).

### Progress beyond roadmap {#masonry-extras}

Masonry is in a state of flux, however is starting to see a few exploratory users.
To unblock these users, some additional important features were added outside of the roadmap, such as:

- Support for embedding Masonry inside a pre-existing event loop ([xilem#417](https://github.com/linebender/xilem/pull/417)).
- Support for a `gap` property in the flexbox ([xilem#437](https://github.com/linebender/xilem/pull/437)).
- Fixes on iOS, including [xilem#418](https://github.com/linebender/xilem/pull/418) and [xilem#421](https://github.com/linebender/xilem/pull/421).

Additionally, some of the items tracked under [Xilem](#xilem) below also impacted Masonry, especially for testing.

## Parley [(original)](@/blog/2024-06-17-roadmap-may-2024.md#parley)

### Create abstractions for text selection and editing [(original)](@/blog/2024-06-17-roadmap-may-2024.md#write-pass-order-rfc#create-abstractions-for-text-selection-and-editing) ?

- **Priority**: Highest
- **Status**: There have been useful discussions in [parley#52](https://github.com/linebender/parley/issues/52).
  Work has started on this in [parley#106](https://github.com/linebender/parley/pull/106) in response to this retrospective.

### Inline block layout [(original)](@/blog/2024-06-17-roadmap-may-2024.md#inline-block-layout)

- **Priority**: High
- **Status**: A first cut of this feature was added by Nico Burns in ([parley#67](https://github.com/linebender/parley/pull/67)).
  [parley#76](https://github.com/linebender/parley/pull/76) expands upon this, and has been approved in principle, but is awaiting thorough review.

### Write adequate documentation [(original)](@/blog/2024-06-17-roadmap-may-2024.md#write-adequate-documentation)

- **Priority**: High
- **Status**: Some work was done towards this in [parley#26](https://github.com/linebender/parley/pull/26), which unfortunately could not be merged.
  [parley#97](https://github.com/linebender/parley/pull/97) added some documentation to Fontique.
  Further documentation work from Nico is currently blocked on the tree-like styles PR [parley#76](https://github.com/linebender/parley/pull/76) being merged (to avoid conflicts).

### Software renderer [(original)](@/blog/2024-06-17-roadmap-may-2024.md#software-renderer)

- **Priority**: Low
- **Status**: This was not completed in the form envisioned.
  However, Nico contributed two examples, showing how to use Parley with Swash ([parley#54](https://github.com/linebender/parley/pull/54)) and tiny-skia ([parley#55](https://github.com/linebender/parley/pull/55)).
  We have decided not to carry this forward into the next roadmap.

### Progress beyond roadmap {#parley-extras}

[parley#56](https://github.com/linebender/parley/pull/56) added support for Emoji, which is used with the [Vello PRs](#vello-extras).

<!-- TODO: Accurate sentence on Harfruzz -->

## Xilem

Android bringup
Status: basic bringup done
Update repository layout
Status: done
Rework xilem_core to be compatible with Masonry backend
Status: done
Write documentation
Status: pending.
Re-implement async integration
Status: largely done (todo: link relevant PRs, as xilem#340 is not updated). xilem#423 was the main implementation
Add unit tests
Status: pending
Add basic benchmarks
Status: pending

AccessKit
Proof-of-concept Android adapter in progress (<https://github.com/AccessKit/accesskit/tree/android-basics>)

Android specific work
Injecting custom logic into View/Application/Activity
Status: in progress (we also spent some time investigating DEX injection, which Matt has got implemented; it’s now looking like a custom Activity and View is the way forward)
IME
Status: progress (winit#3787 opens keyboard, real IME involves content view subclassing and therefore requires the Activity to support it directly)
Gesture recognition
Status: not started

## Conclusions

Overall, the roadmap provided a useful guide of what to work on, and .
One clear recurring issue is around documentation.
We have consistently found.
