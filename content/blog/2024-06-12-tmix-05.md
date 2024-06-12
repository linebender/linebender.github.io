+++
title = "This Month in Xilem, May 2024"
authors = ["Daniel McNab"]
+++

Several members of Linebender attended [RustNL][] at the start of May.
Raph gave a talk titled *Xilem: Let's Build High Performance Rust UI* ([YouTube](https://youtu.be/OvfNipIcRiQ)).
The conference also provided a great opportunity to meet with developers from other Rust user interface projects;
Olivier has produced a report on the unconference, which will be published here shortly.
In addition, those in attendance met after the conference and worked on a roadmap for Linebender, which we are also planning to share here.

We have taken several important steps on Xilem.
At the start of the month, we released [Xilem 0.1.0][] and [Masonry 0.2.0][].
This is the first published release of Xilem, and includes text input ([xilem#241][]), AccessKit integration ([xilem#244][]) and a TodoMVC inspired example ([xilem#257][]).
Work through the rest of the month included explorations of a generic `View` trait ([xilem#310][]), and an Android port ([xilem#309][], [xilem#313]).
[rfcs#6][] is designed to reduce boilerplate when using Masonry, which should help those creating custom Masonry widgets.

Other Linebender projects have seen steady improvements.
[vello#575][] significantly reduced shader compilation time at startup for projects using Vello, and [vello#516][] enabled clearer error handling.
Nico Burns has started an exciting project for inline box layout ([parley#67][]), which will enable Parley to be used in projects which must implement Web layout.

[rustnl]: https://2024.rustnl.org/
[Xilem 0.1.0]: https://github.com/linebender/xilem/releases/tag/v0.1.0
[Masonry 0.2.0]: https://github.com/linebender/xilem/releases/tag/masonry%2Fv0.2.0
[xilem#241]: https://github.com/linebender/xilem/pull/241
[xilem#244]: https://github.com/linebender/xilem/pull/244
[xilem#257]: https://github.com/linebender/xilem/pull/257
[xilem#310]: https://github.com/linebender/xilem/pull/310
[xilem#309]: https://github.com/linebender/xilem/pull/309
[xilem#313]: https://github.com/linebender/xilem/pull/313
[rfcs#6]: https://github.com/linebender/rfcs/pull/6
[vello#575]: https://github.com/linebender/vello/pull/575
[vello#516]: https://github.com/linebender/vello/pull/516
[parley#67]: https://github.com/linebender/parley/pull/67
