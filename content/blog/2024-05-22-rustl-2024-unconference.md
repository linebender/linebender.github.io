+++
title = "Report on the RustNL 2024 Unconference"
authors = ["Olivier Faure"]
date = "2024-05-23 18:00:00"
+++

On May 7 and 8, 2024, folks from the Linebender team went to [the RustNL conference](https://2024.rustnl.org/) in Delft.
We listened to talks, Raph gave one, Rik Arends did the "hot reloading my entire editor live" thing that makes every other Rust GUI developer jealous, it was an all-around good time.

(Videos for individual talks aren't up yet, but there are livestream recordings of [Day 1](https://www.youtube.com/watch?v=XLefuzE-ABU&t=23721s) and [Day 2](https://www.youtube.com/watch?v=521NfGf7AR0&t=19109s) on Youtube.)

After RustNL, though, another less-documented event called [The Unconference](https://2024.rustnl.org/unconf/). It was from roughly the same organizers and took place in the same city, but the format was fairly different.

The Unconference wasn't livestreamed, and I haven't found any public discussion of it, so I thought I'd take the time to describe a bit of what was going.


## The format

The "Unconference" was two days of loosely-structured chats between members of various Rust project teams where people talked about ecosystem collaboration, complained to Lang Team members that their favourite feature wasn't implemented yet, [went to pester everyone else about variadic generics](https://poignardazur.github.io/2024/05/25/report-on-rustnl-variadics/), and other good fun.

Attendees were split into three teams:

- The Rust project.
- GUI and Applications.
- Embedded development.

Discussions inside of these teams was mostly self-organized (more on that later), with cross-team discussion mostly taking place during lunch and recesses.


## The GUI team

As a member of the Linebender project, I was mostly exposed to the GUI team's proceedings.

The main topics of interest were:

- Build tooling (especially for Android and other non-desktop platforms).
- Text layout and edition.
- Winit adoption.
- Accessibility.
- Potential for further collaboration.
- Requests for new Rust features.

Note that I'm skipping a lot topics on the agenda, and other side discussions.
It feels like those topics were the most prominent during those two days.

There was also a fair amount of informal chatter around LogLog Games' incendiary [Leaving Rust gamedev after 3 years](https://loglog.games/blog/leaving-rust-gamedev/) article.
I think those discussions mostly reflected the Bevy community's consensus, that the article had some flaws but overall pointed at very clear weaknesses in the Rust GUI / game engine ecosystem.

### Build tooling

Everyone agreed that a better solution was needed to ship non-TUI Rust apps.

There was discussion about various platforms and build system, and the difficulty of having to eg write Java code for an Android port, and whether anyone wanted to take responsibility for maintaining that glue layer for the rest of the ecosystem.

Unfortunately the discussions on this subject didn't really go anywhere, as far as I could tell.
Packaging Rust apps is still very much an open problem.

### Text layout and edition

There was some widespread interest in the Linebender ecosystem's text handling crates.
From what I remember, Rik Arends and others had concerns about whether eg Parley and Swash added dependencies to our Vello renderer (they don't).

Some people also wanted Parley to be decoupled from Swash.

People we especially interested in the idea of having access to a common text-editing widget, or at least infrastructure to create one.
That widget would handle IME, text selection, accessibility actions, etc, in a way that would feel native to different platforms with their own text-editing quirks (mobile, MacOS, Linux with vim mode, etc).

### Winit

By now all of the Rust ecosystem has firmly converged on Winit.

(Well, not all! [One small project](https://github.com/makepad/makepad) with indomitable maintainers still holds out against the invaders.)

The consensus during this discussion seemed to be that Winit was there to stay, and that any changes we wanted would probably come in the form of upstream contributions, not competing projects.

Glazier is still shelved and we're planning to port its features to Winit until it reaches feature parity.

A discussion that came up was the idea of splitting more "vocabulary types" from Winit into standalone crates.
Vocabulary crates, eg crates that only define types and very little code, have a very low cost in memory and build types, while allowing projects to interop with eg Winit without actually adding a dependency on Winit.

This was connected with the discussions about text edition: having a vocabulary crate that would list Winit's events and also Parley's cursor movements would allow for a very low-dependency text edition widget.

### Accessibility

*Note: This section is very much my personal opinion and doesn't represent the Linebender project as a whole.*

The subject of Accessibility came up informally, but was in my opinion underexplored.

While the subject was on the agenda, by the time we reached it the Unconference had reached a point where everyone was tired and ready to leave or get to the catering.
This is an unfortunate pattern that I think is worth pointing out: if projects don't make a deliberate effort to prioritize discussions of accessibility, the subject will be left out.

Matt Campbell, the maintainer of AccessKit, could not attend the Unconference.
I'm concerned that, in his absence, other maintainers might have felt a dilution of responsibilty: since the accessibility guy wasn't there, nobody felt empowered to bring up accessibility.

One point that did come up during the discussion is one I'd like to hammer in over the next few weeks, is that the Rust ecosystem should adopt a holistic view of accessibility. Accessibility isn't just about screen readers, and making a framework accessible doesn't stop at adding a dependency to AccessKit, though doing so is a great first step.

Rik asked if there was a minimal set of the AccessKit elements that people could be expected to port their UI to.
The AccessKit framework is rich, and it's not obvious at first glance how much is needed versus nice-to-have.
The current reference in the Rust ecosystem is [the AccessKit implementation in eGUI](https://github.com/emilk/egui/pull/2294).

### Requests for Rust features

Jon Kelley, the creator of Dioxus, had a laundry list of features he wanted from the Rust language.

The requests were well-documented, with motivating examples in existing Dioxus code that would clearly be improved by the features.
Most of them felt both self-contained and like they would tremendously help beginners working on their first Rust project.

Overall people were very enthusiastic about them, and the lang team members in the room seemed pretty optimistic that those features could be added.

My favorite proposals were:

- `Capture` trait for auto-cloning reference-counted types in closures.
- Automatic partial borrows.
- Optional struct args.

Jon also wanted support for a remote cache of pre-built crates, so that users wouldn't suffer from having to rebuild dozens of crates every time you download your crate's dependencies.
The consensus from Rust maintainers was that such a cache would be essentially impossible with Rust's current architecture.


## Collaboration in the Rust GUI ecosystem

TODO

Overall the Unconference went well, and felt like a symptom of improving ecosystem collaboration.

- List of backers
- List of projects
- Things have gotten much better since last year


## On the tyranny of struturelessness

TODO

- GUI team had some trouble with discussions.
- Rust team stayed on rails, reportedly mostly thanks to Alice Cecile.
- Embedded team took well to it, from what I saw.


## Conclusion

Many thanks to RustNL organizers
We were exhausted after four days, but these folks had to run the darn thing.
