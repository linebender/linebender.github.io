+++
title = "This Month in Linebender (Xilem), September 2024"
authors = ["Daniel McNab"]
+++

Linebender is an informal open-source organization working on various projects to advance the state of the art in GUI for [the Rust programming language](https://rust-lang.org).

## Xilem

Xilem is our flagship GUI project, inspired by SwiftUI.

- HTTP cats example ([xilem#571][]).
- `lens`, a shorthand for a common component pattern ([xilem#587][]).
- We added Views for some existing and new widgets:
  - Grid layout ([xilem#570][]).
  - Images ([xilem#571][]).

<!-- TODO: Xilem Web seems to not have any especially relevant updates? -->

<figure>

<!-- <img style="height: auto" src="TODO" alt="TODO" height="388" width = "559"> -->

<figcaption>

<!-- Caption -->

</figcaption>
</figure>

## Masonry

Masonry is the widget system used by Xilem.
Our ongoing pass specification project has been largely implemented.
That work is now in a wrapping-up stage, which has allowed Olivier to work on improving the codebase's quality in general.
Some highlights of work from September:

- [xilem#515][]: Removes unimplemented functionality from our text handling code.
- [xilem#570][]: An implementation of a grid layout, by Jared O'Connell.
- [xilem#550][]: Restores scroll requests, as used by text inputs.
- [xilem#565][]: Allows widgets to implement more efficient pointer position dispatching, by new Linebender member Tom Churchman.

<!-- Grid example? -->

Some ongoing work includes:

- Text input and prose accessibility in [xilem#615][].
- Custom client side decorations for Wayland by Marco Melorio in [xilem#606][].
- [xilem#605][] by failingprovince which enables greater flexibility in the layout of the `Image` widget.
- An svg like widget for absolute positioning, by Muhammad Ragib Hasin ([xilem#591][]).

<!-- TODO: Figure for another video, of the talkback with keyboard actions? -->

## Vello

Vello is our GPU vector renderer.
Progress towards the upcoming 0.3.0 release can be tracked in [the milestone](https://github.com/linebender/vello/milestone/2).

- We now no longer consider Vello to be experimental ([vello#691][]). Please note that Vello is still an alpha, and has several known issues.
- An implementation of the single pass scan described last month is being created by Thomas Smith in [vello#685][].

## Parley

Parley is a text layout library.

- Nico Burns contributed a new way to assign layout properties in [xilem#76][].

## Others

- Daniel and Olivier's "office hours" appointments can still be booked by anyone for open-ended discussion of the ecosystem.
  - [See Daniel's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ32eQYJ9DtZ_wJaYNtT36YioETiloZDIdImFpBFRo5-XsqGzpikgkg47LPsiHhpiwiQ1orOwwW2).
  - [See Olivier's schedule here](https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ2t767ZRETD_TkRI_VxK2ZTG0VrO9OZ4l7HvTxefhtJcg85iK0ZN7zWNnAEZtH0Dn7C1GKxrmYM).
