+++
title = "Roadmap for the Xilem backend in 2024"
authors = ["Olivier Faure"]
date = "2024-01-19 13:00:00"
+++

As you may have heard by now, Google Fonts is funding me this year to work on Xilem.

I'm not alone in that: Aaron Muir Hamilton, Daniel McNab and Matt Campbell were funded as well to work on various parts of the ecosystem. I believe this is Matt's third year getting funding from Google Fonts.

Now, *what* I was hired to do might be unclear to you. The scope we agreed on was fairly broad and was stated as "contributing to Xilem's view tree and developer experience", but that could mean a lot of things.

The first thing I want to do is communicate a clearer picture. I have a a lot of plans for Xilem, and they involve major changes from the current architectures, changes that might be worrying to the community at large or even to Raph Levien. I certainly don't want to give the impression that I'm wresting the project away from the Linebender community for Google's interest, and that means being transparent early and often about the things I want to rework.


## What I want for Xilem

Xilem is a research-and-development project, the latest in a series (Xi-Core, Druid, Crochet, Lasagna, Idiopath), that aims to explore how UI development can be made compatible with Rust.

This might feel a little banal now, because Rust GUI frameworks are popping up everywhere and starting to settle on similar patterns, but those patterns weren't obvious in 2019! Xilem is very different from Druid, and getting to its current architecture has been a process.

Xilem has been perma-experimental since May 2022. Once it got started, the main Druid maintainers stopped contributing almost entirely with the expectation that Xilem would replace Druid once it was ready. Unfortunately, it still hasn't made enough progress to be on parity with Druid, yet Druid still lies abandoned, which isn't great when people had started relying on it.

It's debatable how much this could have been avoided. As I've pointed out before, the Rust GUI ecosystem is subject to massive [yak-shaving](https://seths.blog/2005/03/dont_shave_that/): many of us came here because we wanted to build a text editor, and now we're all learning about [text rendering](https://faultlore.com/blah/text-hates-you/), [text editing](https://lord.io/text-editing-hates-you-too/), [compositing](https://raphlinus.github.io/ui/graphics/2020/09/13/compositor-is-evil.html)[^1], [accessibility trees](https://accesskit.dev/accesskit-integration-makes-bevy-the-first-general-purpose-game-engine-with-built-in-accessibility-support/), [using monoids to implement stuff on the GPU](https://raphlinus.github.io/gpu/2021/05/13/stack-monoid-revisited.html), [ECS](https://www.leafwing-studios.com/blog/ecs-gui-framework/), and some concepts that I'm absolutely certain Raph made up like [Bézier paths](https://raphlinus.github.io/curves/2023/04/18/bezpath-simplify.html) and [C++](https://raphlinus.github.io/rust/2023/04/01/rust-to-cpp.html).

And to a certain extent, yak-shaving is good! "I'm going to do better than X without learning how X works" is a child's attitude. It's the attitude we all start with, and it fills us with courage, but that courage is naive. True improvement comes from keeping that courage after spending years learning how immense the problem space is, and keeping faith that it's possible to do better. The Rust community is one that values not just doing things from scratch, but also learning from the past to get them right.

This is good, but the "get them right" part takes a lot of time, and in the meantime, I'd argue that we've neglected end users a bit. We've abandoned Druid before Xilem was ready as an alternative, and then we worked on foundational and experimental projects that would make Xilem stronger and sounder in the long term, but left the project in an awkward state in the meantime (except for the web backend, which made a lot of progress).

In the [ECS-backed GUI framework](https://www.leafwing-studios.com/blog/ecs-gui-framework/) article I linked, Bevy maintainers have this to say:

> Not to be too harsh, but a lot of the existing Rust GUI solutions... just aren't very good. There's a lot of passable options, but they all have non-trivial drawbacks. No one has really risen to the top as a clear winner. [...] Deep down, we all know that we can do better, and we should.

My ambition is to makes this paragraph obsolete before the end of the year. I want Xilem to get back to being suggested to newbies in the same breath as Iced and SlintUI. In the next few years, I want the entire ecosystem to get to a point where people talk about Rust GUI like they talk about ripgrep or rustls.

To give one specific example: my personal goal is to implement seamless hot reloading for Xilem before the end of 2024. I believe it's feasible, and I believe doing it will radically change the way people think about Rust GUI, along with [other improvements I hope to unlock over time](https://poignardazur.github.io/2023/02/02/masonry-01-and-my-vision-for-rust-ui/).

In the shorter term, my plan is to focus on basic features, widgets and styling options so people can experiment with Xilem, building on Raph's work on Vello over the past year. [John Skottis's Xilem UX Notes](https://gist.github.com/giannissc/172c6c591997ee14f6120134a7990697) give a good picture of the kinds of things I'll be working on.

The other big short-term priority is going to be documentation. We've seen a lot of people complain that the Linebender project were poorly documented and hard to get into; it was the most popular reaction to our [2024 announcement](https://linebender.org/blog/xilem-2024/), but it echoed sentiments we'd seen before. We're aware of the problem, and we intend to work on it.

In the medium term, I also plan to work on the performance aspect of Xilem. People from Google Fonts have expressed interest in seeing how much performance we can get from Rust UI. While my mission isn't *limited* to performance work, it will be an ongoing focus, especially since it's an area where the Rust language brings unique opportunities. I'll go into more details on Rust performance and power efficiency in a later article.


## Major changes I want to implement soon

These are the big changes I want to work on soon. Realistically, "soon" is going to be relative, because some of these changes are going to be controversial, and part of the work is going to be convincing people of their value.

### Switching to Masonry as a backend

Okay, this one makes me blush a little.

I'm the only maintainer of Masonry, a GUI crate with barely over 300 stars on GitHub and little outside interest. Can I really justify taking the money Google pays me to improve Xilem and spending my time on Masonry instead?

That choice isn't random, mind you: Masonry was forked from Druid, Xilem's ancestor, and in fact Xilem and Masonry share a lot of code right now.

In fact, I'd argue that Masonry's codebase is of better quality than Xilem's current native backend. This isn't meant to be a put-down of the work of Xilem maintainers; rather, it's the outcome of a difference in priorities. Nobody was very interested in Xilem's native backend (though Raph was aware of its importance, hence me getting paid to write this), and lots of people were very interested in the frontend and the web backends, therefore the frontend and web backends saw the most work.

From our [RFC#0001](https://github.com/linebender/rfcs/pull/1):

> As a result, Xilem's native backend is in a poor state:
>
> - There is [code commented out](https://github.com/linebender/xilem/blob/ea45b9f8c14e3708f0fcbe0a0e1c760f59146323/src/widget/widget.rs#L113-L120).
> - There are [*entire modules* commented out](https://github.com/linebender/xilem/blob/ea45b9f8c14e3708f0fcbe0a0e1c760f59146323/src/widget/mod.rs#L19-L20).
> - There is [documentation referring to items from Druid that no longer exist](https://github.com/linebender/xilem/blob/ea45b9f8c14e3708f0fcbe0a0e1c760f59146323/src/widget/widget.rs#L51-L71).
> - There are [TODOs without an associated issue](https://github.com/linebender/xilem/blob/ea45b9f8c14e3708f0fcbe0a0e1c760f59146323/src/widget/core.rs#L569C5-L569C66).

Masonry's backend codebase is a healthier starting point.  Masonry also comes with some built-in perks, like powerful unit tests and a structured widget graph.

It's not clear whether we want to import the Masonry codebase, the crate, or the entire repository, and it's not clear what the ultimate branding should be. The RFC lays out the different possibilities, and discussion is still ongoing, so the community can reach a consensus before doing more work (including possibly not using Masonry at all).

If we do move forward with this, then Masonry will need some porting work: 

- Using Vello and Parley instead of Piet.
- Using Glazier instead of druid-shell.
- Integrating AccessKit.
- Removing the dependency on GTK.

AccessKit support in particular is table stakes, by now. The Rust ecosystem is converging on the idea that accessibility (or at least, screen reader support) should be integrated early on in the design of GUI frameworks, not tacked on at the end.

There are other things Xilem added over Druid (notably virtual lists and async support), but I believe Masonry doesn't need to implement them right away for parity. Both will be very important for eventual performance work.


### Avoiding custom Widgets

I'm told by Raph this is a "molotov cocktail" which, uh, fair.

We should write Xilem's backend under the assumption that end users of the library (including the Xilem frontend) will very rarely create their own widgets. Instead, they will usually compose the primitives given to them the same way they compose DOM elements in the browser. There will be escape hatches, from a fixed-size canvas surface where end-users can use arbitrary paint primitives to full-on custom widget code, but we should design the framework under the assumption people will almost never *need* those hatches.

Clearly, this is a radical move. **There will be a prototyping phase before we choose whether we commit to it**, and RFCs to lay out the details. I'm confident it will pay off.

Developers of hobby GUI frameworks tend to provide a severely anemic set of primitives, under the assumption that if anyone wants other features in their widgets, they can implement them themselves. Both Druid and Xilem suffer from this.

On the other end of the spectrum, the web has shown that you can implement complex custom UIs given a rigid but rich set of primitives.

More importantly, the web is moving towards a powerful paradigm: declarative UI. As time goes on, people are realizing you can get better performance and composition by describing what things should look like and composing them than by imperatively making them do things.

Part of this is that native code is more performant than JS, but part of it just that declarative code is easier to work around than imperative code: its bounds are better defined, it lets you [commit to limitations](https://developer.mozilla.org/en-US/docs/Web/CSS/contain) that make the engine's job easier, and it's easier to analyze.

A set of proposals called [CSS Houdini](https://developer.mozilla.org/en-US/docs/Web/API/Houdini) came out that went in the exact opposite direction: pushing the layout engine towards more customization and JavaScript code. The proposal hasn't seen much movement since 2021, and I don't think it's a coincidence that the part that would have given the most power to imperative code, the Layout API, hasn't been stabilized in any browser.

The bottom line is that, while implementing custom widgets *will always be possible*, I want to encourage developers to go as far as they can composing declarative elements instead, and see how far it's possible to go using those building blocks.

#### More opinionated layout

One major customization point I want to restrict in Xilem is layout.

Today all Rust GUI frameworks ship their own layout algorithm, but none of them are *competing* on their layout algorithm. Often the layout algorithm is just a means to an end; developers that do want to work on a state-of-the-art layout engine will provide it in a modular crate like [Morphorm](https://github.com/vizia/morphorm) and [Taffy](https://github.com/DioxusLabs/taffy).

And here's the thing: realistically, most layout engines aren't that creative. Every Rust framework and every major GUI framework I'm aware of uses layout that boils down to "FlexBox with small variations". The number of meaningfully different ways to do GUI layout can be counted on, generously, two hands.

I'd argue we only need CSS Flow, FlexBox, CSS Grid, and maybe Morphorm to cover most people's needs. Conveniently, this is what Taffy aims to provide. There's already an experimental Taffy integration in Xilem, and I want to make it deeper.

That being said, there won't be a hard-coded set of layout algorithms. Rather, layout will rely on a web-inspired protocol between widgets (in other words, trait methods), and that protocol will be strongly opinionated to make implementation of these layout algorithms easier.

More on this in a latter blog post.

#### No boxing

Having a semi-fixed set of widgets means you can avoid using `Box<dyn Widget>`, which has some interesting performance implications.

It means you can add a lot of methods to the Widget trait without worrying about bloating the vtable. And you can write methods under the assumption that they'll be inlined, which opens up interesting DX improvements.


### Using ECS, maybe, sort of

In current Xilem code, if you want your FlexWidget to have children, it must have a `Vec<Pod<ChildWidget>>` member. Each Pod directly owns a Widget.

I think this is an interesting quirk of Rust GUI, tying directly to Rust's ownership semantics. Non-Rust frameworks often use pointers to child widgets, true, but they're often shared with the framework, with an understanding that the framework is charged with keeping track of how many widgets there are and which is a child of which. In Qt, for instance, it's preferred (but not mandatory) for parents to call `delete_later()` on their children instead of `delete`, because it plays better with event handling.

Xilem's approach of "you own your children" is a little bespoke. It means the framework has a hard time iterating over the entire widget tree (say, as part of a DOM inspector) unless each Widget specifically implements the iterating method. And it means if you want to target an event at a specific widget (say, the one with keyboard focus), you need to go through the entire ownership chain, using some heuristics to avoid visiting too many widgets.

I believe Widgets should be owned by the library. If your container has children, then the only thing the container will actually own is keys into a structure (probably a slotmap) where the widget is stored. This makes a lot of things easier, like serialization and debugging, but it has an impact on the entire backend. It's an infrastructure investment.

Lately, I've seen more and more discussion of implementing GUI through an ECS. A lot of that discussion comes from Bevy, which is natural, since the bevy community ~~is made up of ruthless cultists striving to feed ever more sacrifices to the ECS god until it consumes the Earth~~ is intimately familiar with the ECS pattern and has reached a phase where UI work is getting a lot of attention[^2]. But I've seen discussions about it in the Linebender community too.

Whether we actually want to use ECS is something we still need to research.

The way I understand it, the big draw of ECS is the "S" part, the Systems: it's about handling lots of similar data at once in arbitrary order. Which is why eg archetypes in bevy are useful: you're grouping together entities with the same components, so that you can say "execute this code for all entities with component X and component Y" and have this code be executed on arrays of values with minimal indirection or branching. And the "arbitrary order" part makes them work really well with slotmaps: since you can just read from the beginning of the slotmap, you don't pay the double-indirection penalty.

In other words, the ideal use case for ECS is code of the type `for (entity in entities) entity.position += entity.speed`.

GUI is pretty far from that ideal use-case: updates are sparse and should only run on a small number of changed widgets. Order often matters. And I'm not sure archetypes would work in a framework with a large variety of widgets and different attributes.

I think there are two things you really want from a Rust ECS library for GUI: slotmaps, and efficient ways to add and remove components from an entity.

Implementing those is going to be a major undertaking, which we'll have to divide into small experiments, but one I expect to pay many times over.


## Community involvement and more to come

I want to really put emphasis on something: none of the above is set in stone.

Part of what Raph wants to achieve this year is to make Xilem more of a community project, and that means running more decisions by the community.

I've created [an RFC repository](https://github.com/linebender/rfcs/) for this, and each of the sections above will get its own RFC, where I'll expand on my rationale in more detail.

This isn't a rubber-stamping process either: I want to implement those ideas, but if the community[^3] decides that they are bad, then we'll go another direction.

There's other ideas I haven't discussed yet that I want to come back to:

- **Declarative styling:** Giving Widgets attributes similar to CSS properties, with modifiers for animations, pseudo-classes, media queries and container queries. While inspired by CSS, it wouldn't involve any CSS parsing, selector resolution, specificity resolution, etc.
- **Compositing:** Xilem is heading towards a "do everything in the GPU" approach. This is good for performance, but Firefox engineers who have gone down this road before us have warned us of the many perils they faced, especially regarding battery life. In general, after layout and painting, you really want a `composite()` phase to deal with scrolling and some simple animations without touching the GPU at all if possible. [This is a complex subject](https://raphlinus.github.io/ui/graphics/2020/09/13/compositor-is-evil.html), and it would be nice if we could hide most of that complexity from app developers while still giving them good performance.
- **Resource loading:** This is another area where we might want to crib from game engines. At the very least, we want to be able to ask "Load me this texture/shader/font/etc" without having to write disk I/O in user code. Then we can start thinking about progressive loading, fallbacks, sharing, etc.

These ideas are likely to be the subject of further articles and/or RFCs in the near future.

### Relations with other projects

Presenting the current state of the Rust GUI ecosystem would probably take its own article.

Things are moving quickly, and anything I write now might not be up to date in two weeks. The projects we're most interested in right now are Winit and Bevy, and there's some potential for cooperation with Servo. The COSMIC ecosystem looks pretty cool, but so far it looks like we have less to share with them.

People from parallel projects are communicating and exchanging much, *much* more than they were in 2022. Between Vello, Parley, wgpu, Taffy, COSMIC-Text and others, projects are converging and maintainers are making more effort to reach out and integrate other people's code.

Stay tuned for more!


## Hey, this is really cool!

Something that came up in discussions I've had with Raph is that it's easy to miss the forest for the trees. In the middle of all the paperwork, blog-writing, RFC-writing, we can end up focusing on the process so much that we forget how excited we are.

So, I'm taking the moment to remind myself: this is really freaking exciting!

For all my criticisms and caveats and realistic assessments of the problems we need to overcome, I'm still feeling absolutely great about this!

I'm super thankful to Raph for getting us this far, to Google Fonts for sponsoring us, and to everyone else who contributed to the Linebender ecosystem. I think we can do something amazing and I'm honestly a little startled to be in this position at all.

To reiterate what I said: in the next few years, I want the entire ecosystem to get to a point where people talk about Rust GUI like they talk about ripgrep or rustls.

I legitimately think Rust has the potential to get best-in-class GUI frameworks, and now the time has come to prove it.

[^1]: By the way, I love this style of writing where we start humanizing a software process and complaining about how much it wants to kill us. The articles I cited are very high-quality too.

[^2]: I'm told the Bevy community found that joke funny, but I was too afraid for my sanity to check on their Discord server.

[^3]: "The community" is a nebulous concept here. For obvious reason, we'll give more weight to the opinions of people who have already put work into the Linebender ecosystem, but the general hope is to get a consensus from everyone involved. And you're welcome to weigh in even if you haven't contributed before.
