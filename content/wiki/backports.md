+++
title = "Backporting in Linebender projects"
+++

## When to backport

Generally we publish the latest state of the `main` branch.
However, if `main` has breaking changes or we want an older release series to receive some non-breaking changes, then we can't just publish `main`.
That's when we need to create a backporting branch to achieve that goal.

The example commands below are based on Color and its `main` already being at `v0.3.1` when we also wanted to release a `v0.2.4`, with `v0.2.3` being the latest in that series.
Make sure you modify the commands based on your actual versions.

## Create the backport branch

The backport branch needs to be based on the most recent release tag in the series.
It must be named so it matches `v*.x`, e.g. `v0.2.x` or `v1.x.x`.
This branch will be used for all future releases in this series.

```sh
git checkout -b v0.2.x v0.2.3
```

Then push it to the Linebender GitHub repository.

```sh
git push --set-upstream origin v0.2.x
```

## Committing changes to the backport branch

All changes need to go through our usual review process via GitHub pull requests.

Create a working branch based on the backport branch.

```sh
git checkout -b my-changes v0.2.x
```

Then make your actual changes and open a pull request which targets the backport branch instead of `main`.

## Cherry picking vs. new changes

If the desired changes already exist in `main` or another backport branch, then those commits should be cherry picked.
The changelog entries must go into the *Unreleased* section of the backport branch's `CHANGELOG.md`.
Open a pull request targeting the backport branch with one or more cherry picked commits, their changelog entries, and no other changes.
Once the pull request has been approved, it must be merged using the **Rebase and merge** option on GitHub.

All other changes, including any final release preparation commits, need to be separate from any cherry picked pull requests.
Once these pull requests have been approved, they must be merged using the **Squash and merge** option on GitHub.

## Changelog modifications

In addition to the usual changes, the changelog header section must be updated as follows:

```diff
- The latest published Color release is [0.2.3](#023-2025-01-20) which was released on 2025-01-20.
- You can find its changes [documented below](#023-2025-01-20).
+ This is the backport branch for Color 0.2.x.
+ For the latest releases, check the [changelog on `main`](https://github.com/linebender/color/blob/main/CHANGELOG.md).
+ The latest published Color release in the 0.2.x series is [0.2.4](#024-2025-05-19) which was released on 2025-05-19.
+ You can find its changes [documented below](#024-2025-05-19).
```

Plus the *Unreleased* link in the footer must compare this specific backport branch:

```diff
- [Unreleased]: https://github.com/linebender/color/compare/v0.2.4...HEAD
+ [Unreleased]: https://github.com/linebender/color/compare/v0.2.4...v0.2.x
```

## Forward-porting the changelog

After a backport release has been published, its changelog section must be added to the `main` changelog.
Insert it into the correct middle spot, ordered based on the version number.
