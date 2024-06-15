+++
title = "Report on the RustNL 2024 Unconference"
authors = ["Olivier Faure"]
+++

On May 7 and 8, 2024, folks from the Linebender team went to [the RustNL conference](https://2024.rustnl.org/) in Delft.
We listened to [some Rust-related talks](https://www.youtube.com/playlist?list=PL8Q1w7Ff68DBZZbJt3ie5MUoJV5v2HeA7), Raph [gave one on Xilem](https://www.youtube.com/watch?v=OvfNipIcRiQ&list=PL8Q1w7Ff68DBZZbJt3ie5MUoJV5v2HeA7&index=15), Rik Arends did the "hot-reload my entire editor live" thing that makes every other Rust GUI developer jealous, it was an all-around great time.

After RustNL, though, another less-documented event called [The Unconference](https://2024.rustnl.org/unconf/).
It was from roughly the same organizers and took place in the same city, but the format was fairly different.

The Unconference wasn't livestreamed, and I haven't found any public discussion of it, so I thought I'd take the time to describe a bit of what was going on.


## The format

The "Unconference" was two days of loosely structured chats between members of various Rust project teams where people talked about ecosystem collaboration, complained to lang team members that their favorite feature wasn't implemented yet, [went to pester everyone else about variadic generics](https://poignardazur.github.io/2024/05/25/report-on-rustnl-variadics/), and other good fun.

Attendees were split into three teams:

- The Rust project.
- GUI and Applications.
- Embedded development.

Discussions inside these teams were mostly self-organized, with cross-team discussions mostly taking place during lunch and recess.


### On the tyranny of structurelessness

*Note: This section is very much my personal opinion and doesn't represent the Linebender project as a whole.*

I'm not sure the self-organized format worked well.

From what I saw, the Embedded team took well to it, and the Rust team was kept productive thanks to Alice Cecile's efforts in marshaling everyone.

In the case of the GUI team, people were spread in a very large room, which should have been conducive to small side discussions and people splitting up to talk about the problems that interested them.
Instead, there was an unspoken accord to progress through agenda items one by one, with a few people dominating the discussions on these items.

Because those people we spread around a large room, they had to talk loudly to address each other, which left little room for side-discussions.
Because the people talking were naturally the most confident and extroverted, more introverted people ended up taking a passive role in the discussion.

To me, this feels like a strategic mistake.
It was a setup that encouraged bikeshedding and long back-and-forths and discouraged plurality of opinions.
It's no coincidence that the most interesting conversations of the Unconference happened at lunch: lunch was the point of the Unconference where people were most mixed, had the most spontaneous conversations, and were least constrained by having to follow what someone else was saying.

Our discussions were still productive, I just feel like the format could have been improved.


## The GUI team

As a member of the Linebender project, I was mostly exposed to the GUI team's proceedings.

The main topics of interest were:

- Build tooling (especially for Android and other non-desktop platforms).
- Text layout and editing.
- Winit adoption.
- Accessibility.
- Potential for further collaboration.
- Requests for new Rust features.

Note that I'm skipping a lot of topics from the agenda and other side discussions.
The topics above were the most prominent during those two days.

There was also a fair amount of informal chatter around LogLog Games' strongly critical [Leaving Rust gamedev after 3 years](https://loglog.games/blog/leaving-rust-gamedev/) article.
I think those discussions mostly reflected the Bevy community's consensus, that the article had some flaws but overall pointed at very clear weaknesses in the Rust GUI / game engine ecosystem.

### Build tooling

Everyone agreed that a better solution was needed to ship non-TUI Rust apps.

There was discussion about various platforms and build systems, and the difficulty of having to e.g. write Java code for an Android port, and whether anyone wanted to take responsibility for maintaining that glue layer for the rest of the ecosystem.

No actionable decisions were made, but the consensus seems to be that although integrated build tools are desirable, external build tooling for complex platforms (primarily Apple and Android) is likely to remain necessary.
Nobody seems to have stepped up to work on multi-target builds in Cargo, and since external build tools are required as it is, most people are okay with building everything several times when they release a multi-target package.
Most users are comfortable invoking platform-specific tools for bundling, signing, and manifest generation, so there is not much urgency for integrated tooling for these.

### Text layout and editing

There was some widespread interest in the Linebender ecosystem's text-handling crates.
From what I remember, Rik Arends and others had concerns about whether e.g. Parley and Swash depend on our Vello renderer (they don't).

Some people also wanted Parley to be decoupled from Swash.

People were especially interested in the idea of having access to a common text-editing widget, or at least infrastructure for creating one.
That widget would handle IME, text selection, accessibility actions, etc, in a way that would feel native to different platforms with their own text-editing quirks (mobile, macOS, Linux with vim mode, etc).

### Winit

By now all of the Rust ecosystem has firmly converged on [`winit`](https://github.com/rust-windowing/winit) as the windowing platform-abstraction solution of choice.

(Well, not all! [One small project](https://github.com/makepad/makepad) with indomitable maintainers still holds out against the invaders.)

The consensus during this discussion seemed to be that Winit was there to stay.
[TAO](https://github.com/tauri-apps/tao) users present were generally interested in being able to use upstream Winit in the future, if its API adopts or supersedes the improvements made in TAO.
Most were happy to see Winit becoming more trait-oriented, and to see the `dpi` crate spun off from Winit.

Glazier is still shelved and we're planning to port its features to Winit until it reaches feature parity.

A discussion that came up was the idea of splitting more "vocabulary types" from Winit into standalone crates.
Vocabulary crates, e.g. crates that only define types and very little code, have a very low cost in memory and build times, while allowing projects to interop with e.g. Winit without actually adding a dependency on Winit.

This was connected with the discussions about text editing: having a vocabulary crate that would list Winit's events and also Parley's cursor movements would allow for a very low-dependency text editing widget.

### Accessibility

*Note: This section is also my personal opinion and doesn't represent the Linebender project either.*

The subject of Accessibility came up informally, but I'll claim it was underexplored.

While the subject was on the agenda, by the time we reached it the Unconference had been running long enough that everyone was tired and ready to leave or get to the catering.
This is an unfortunate pattern that I think is worth pointing out: if projects don't make a deliberate effort to prioritize discussions of accessibility, the subject will be left out.

Matt Campbell, the maintainer of AccessKit, could not attend the Unconference.
I'm concerned that, in his absence, other maintainers might have felt a dilution of responsibility: since the accessibility guy wasn't there, nobody felt empowered to bring up accessibility.

(This might sound like it contradicts my section about structurelessness above.
It doesn't.
I'm not saying organizers should have pushed harder for accessibility - they did try - I'm saying that we should strive for a culture where framework developers think about accessibility without waiting for someone to tell them to.)

One point did come up during the discussion which I'd like to hammer in: the Rust ecosystem should adopt a holistic view of accessibility.
Accessibility isn't just about screen readers, and making a framework accessible doesn't stop at adding AccessKit as a dependency, though doing so is a great first step.

Rik asked if there was a minimal set of ARIA attributes that people could start using in their UI, as the set of available attributes is gigantic.
The AccessKit framework is rich, and it's not obvious at first glance how much is needed versus nice-to-have.
The current reference in the Rust ecosystem is [the AccessKit implementation in egui](https://github.com/emilk/egui/pull/2294).

### Requests for Rust features

Jon Kelley, the creator of Dioxus, had [a laundry list of features](https://dioxus.notion.site/Dioxus-Labs-High-level-Rust-5fe1f1c9c8334815ad488410d948f05e) he wanted from the Rust language.
That list was later [filed in the Project Goals repository](https://github.com/rust-lang/rust-project-goals/pull/10).

The requests were well-documented, with motivating examples in existing Dioxus code that would clearly be improved by the features.
Most of them felt both self-contained and like they would tremendously both beginners and veteran Rust developers; though Jon made the point that the papercuts he wanted to remove were especially punishing for new developers trying to become familiar with Rust.

Overall people were very enthusiastic about them, and the lang team members in the room seemed pretty optimistic that those features could be added.

My favorite proposals were:

- `Capture` trait for auto-cloning reference-counted types in closures.
- Automatic partial borrows.
- Optional struct attributes.

Jon also wanted support for a remote cache of pre-built crates, so that users wouldn't suffer from having to rebuild dozens of crates every time you download your crate's dependencies.
The consensus from Rust maintainers was that such a cache would be essentially impossible with Rust's current architecture.

Niko Matsakis recently released [The borrow checker within](https://smallcultfollowing.com/babysteps/blog/2024/06/02/the-borrow-checker-within/), which feels like a follow-up to some of these discussions.


## Collaboration in the Rust GUI ecosystem

Over the last two years I've joked a few times that everybody in the Rust GUI ecosystem wants the ecosystem to standardize on common crates, as long as it's *their* crates.

After RustNL, I'm pleased to report this joke isn't accurate anymore.

A vibe that permeated all discussions was people wanting work to be pooled.
It felt like people didn't strongly care whose crate we adopted, as long as we all agreed the crate made sense:
we're slowly crawling out of the [xkcd 927](https://xkcd.com/927/) dilemma.

The main ecosystem crates that people are enthusiastic about sharing are:

- [winit](https://github.com/rust-windowing/winit) for creating windows.
- [AccessKit](https://github.com/AccessKit/accesskit/) for plugging into accessibility APIs.
- [wgpu](https://github.com/gfx-rs/wgpu/) for using modern GPU features.

Jon Kelley also said he would be interested in using Masonry as the native backend for Dioxus, though we haven't hashed out details yet.

### Corporate funding

For open-source projects, "Who funds this" is a difficult question: in any healthy project, there is a large scale of contributions, with individual non-corporate contributors at one end, and companies paying cash to the project's treasury at the other.
In-between are self-employed people like me contracted to work on an open-source project, and corporate employees who contribute to the project as part of their 9-to-5 job.

Some monetary contributions can also come from individual non-corporate donors: Servo has about fifty of them, for instance.

With that in mind, some notable sponsors for projects represented at the Unconference were:

- **Google Fonts:** Linebender projects.
- **Futurewei:** Dioxus, Makepad, Robius and Servo.
- **Embark:** Bevy, winit and rust-gpu.
- **Foresight Spatial Labs:** Bevy.
- **Rerun.io:** egui.

Not present at RustNL but relevant to the ecosystem are **System76** (funding COSMIC-Text and contributing to iced), **Kraken** (funding iced), and **Slint** who are self-funding as a startup targetting embedded UIs and couldn't attend due to time constraints.

Overall the number of different backers feels like a symptom of a healthy ecosystem: while some large corporate sponsors bring much more resources than others (Google and Futurewei especially), the ecosystem isn't in a state where any specific backer pulling out would completely collapse progress.


## Conclusion

Before anything else, I'd like to express a big thanks to the volunteers who helped organize RustNL.

Most of the attendees were absolutely exhausted after two days of talks and two more days of Unconference, but these folks had to *run* the darn thing.
I think everyone who attended was impressed with them.

Overall RustNL and the Unconference felt like getting a year of coordination done in a week.
It was a thoroughly productive time, and I couldn't be happier I attended.
Everyone I asked about it shared the same opinion.

And the discussions we had during the event showed there was a great appetite to push the ecosystem towards more cooperation, both between GUI projects and with Rust maintainers!

I'm feeling more hopeful about the future of Rust GUI than ever.

Now it's time to get back to work.
