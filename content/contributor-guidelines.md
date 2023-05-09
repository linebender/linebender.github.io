+++
title = "Linebender contributor guidelines"
+++

## About the projects

The Linebender organization is an umbrella for a diverse and ambitious set of project, with the goal of creating high performance, accessible, high quality UI in Rust. An explicit goal is to push the boundary and fill in gaps in our understanding of UI infrastructure, as opposed to implementing existing cookbook techniques.

## Discussions and community

We use a [Zulip instance](https://xi.zulipchat.com/) as the primary place for discussions. All are welcome, and everyone with a GitHub account can sign up. Additional discussion happens in issues in the individual repos, and we also have tracking issues. We don’t use GitHub discussions, preferring the Zulip.

We also hold weekly [office hours]. This is a chance to give status updates, make decisions, and discuss architectural directions. It is open to all, and there are public notes.

We have recently re-started a [wiki], and eagerly encourage gardening. While the Zulip does have archives, they are not indexed by search engines (though search is available within Zulip), so the wiki represents the definitive place for the results of discussions and resources.

To propose a nontrivial change, it is better to file an issue first rather than sending a PR. Depending on the scope and disruption of the change, it might make sense to bring it up on Zulip. Especially if a clear consensus doesn’t emerge in a reasonable time, discuss it in office hours.

While the main focus of the Linebender community infrastructure is to develop Xilem, Vello, and related modules, we also want to help move the entire ecosystem forward. People from other Rust UI efforts are welcome, and discussions about how to join forces and share infrastructure especially so.

We use the Rust Code of Conduct and hold people to a much higher standard of conduct than is usual on the Internet. Don’t worry if you’re interacting in good faith, though, the moderators will work with you to try to resolve issues.

## Code reviews

Here are our policies around code review:

* Every PR must be approved by someone with write access before committing.
* The [wiki] is an exception; there we optimistically merge without review
* Usually we wait for the original author of the PR to commit if they have write access.
* Both merge and squash commits are fine, it’s up to the committer.
* We give write access to everyone with a good faith intent to improve the code. Usually this is shortly after the first commit.
* There are currently five owners: @raphlinus, @dfrg, @DJMcNab, @xStrom, and @cmyr. They approve new committers and can make changes to the organization.

We struggle with getting PRs approved and landed quickly. To try to improve that, here are some suggestions.

First, we encourage everybody to review PRs. It’s a good way for newcomers to get up to speed on the project, and contribute in a meaningful way. There are two goals of the code review process: to improve the quality of the code that lands, and for people to learn something; it’s worthwhile if some knowledge flows in either direction or ideally both.

If a PR goes unreviewed too long, it’s absolutely a good idea to ping it on Zulip. It’s also fine to ask for reviewers during office hours, especially if there’s a deeper issue to be discussed.

## Responsibility

Linebender is a collection of open source projects, with a research flavor. We make no guarantees of any of them being suitable for production. We welcome contributions from anybody. Participating should be done out of joy and should not feel like a burden.

Sometimes PRs stall out, or people have an intent to work on an issue and don’t get to it. In those cases, we encourage others to take them over. We try to discourage “cookie licking” as much as possible.

Filing an issue does not come with a guarantee it will be worked on. We really value high quality issue reports, but they can be closed if there’s no clear action to be taken, and we encourage everybody in the project to feel empowered to do so, or otherwise garden such as asking for more details.

Over time, we hope that some of the projects will mature so that they *are* useful in production (this is already the case for `pulldown-cmark`, which is not officially part of the Linebender org). We very much welcome collaboration toward that goal, and use of the crates in all kinds of contexts including commercial projects. For such commercial use, we encourage a collaborative approach, where the organization dedicates sufficient staff time and energy to take responsibility for issues and feature requests, working collaboratively with the open source organization.

[cookie licking]: https://devblogs.microsoft.com/oldnewthing/20091201-00/?p=15843
[office hours]: /wiki/office-hours
[wiki]: /wiki
