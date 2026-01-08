+++
title = "Releasing in Linebender projects"
+++

Several Linebender packages are published to [crates.io](https://crates.io).
When we do so, we follow a process to ensure that:

- All release steps are reviewed.
- There is consensus that the project is ready for a release.
- Our users are made aware that a release has happened, and what has changed in the release.

For backports, see the [Backports in Linebender Projects](@/wiki/process/backports.md) post

## Before the release

### Updating the changelog

We should make sure that the changelog accurately reflects all important changes since the previous release.
It's especially important that we capture all breaking changes.
This happens as a normal PR, following normal processes.
We want to have any updates to the changelog content happen before the release PR itself (see [Updating version numbers](#updating-version-numbers)).
This is primarily because changelog content updates are more likely to run into merge conflicts, and require more thorough review.

### Dependencies

We make sure that all dependencies are at their latest version.
This is achieved by running:

```sh
# Updates dependencies in Cargo.toml files.
cargo upgrade --ignore-rust-version
# Updates Cargo.lock to match what new users who add our crate will use.
# This ensures our testing is realistic.
cargo update --ignore-rust-version
```

The `cargo upgrade` command is part of the [`cargo-edit`](https://github.com/killercup/cargo-edit/blob/master/README.md#cargo-upgrade) project.

You should also check that the `cargo upgrade` output doesn't list any breaking releases of our dependencies.
Note that we have a public dependency on some crates (especially wgpu), which we want to keep synchronised across our projects.

This will be submitted as a PR, following normal processes.

See [rfcs#5](https://github.com/linebender/rfcs/blob/main/rfcs/0005-version-matrix.md#cargolock-is-updated-regularly) for motivation of this strategy.

### Updating version numbers

Once the above steps are done, we will be ready to create the release PR.
This PR should only:

- update version numbers in `Cargo.toml` files (and therefore also `Cargo.lock`) files; AND
- rename the "unreleased" section of the changelog to the version number of the new release.

Anyone can prepare the release PR (although it should be an organisation member).
The PR which makes a release should *only* bump version numbers in Cargo.toml (and Cargo.lock) files, and update the changelog as described below.

#### Updating package versions

The version field in the Cargo.toml file of each package to be published (or the workspace version number) should be increased.
The version specification for each package to be released in the `[workspace.dependencies]` section of the workspace root `Cargo.toml` will also need to be updated (if present).

Once the Cargo.toml files are updated, to validate the release, you can run: `cargo publish -p {crate1} -p {crate2} --dry-run`.
At this stage, you should *only* be doing a dry run.
We recommend not having Cargo logged into crates.io at all times, to avoid accidentally publishing at this time, as well as for security in general.

#### Announcing release in the changelog

The updates required in the changelog are as follows:

- The release header for the new release should be created.
  You should duplicate the copy the MSRV of the unreleased version.
  This should create a new header for the release, between the two MSRV lines, which will look something like:

  ```md
  ## [v{x.x.x}][] - 202{x}-{xx}-{xx}
  ```

- That is, the final result should look something like:
  
  ```md
  ## [Unreleased][]

  This release has an [MSRV][] of 1.82.

  ## [0.3.2][] (2025-09-10)

  This release has an [MSRV][] of 1.82.

  ### Added
  ```

- The paragraph at the top should be updated.
  This involves updating all of: the latest release number, the link from that number to the release's header, the date of the release in the text, and the link for the text "documented below".
- The links at the bottom need to be updated.
  This involves adding a new link to the changes for the new release, and updating the unreleased link to be "from" the new tag.

It's recommended to run [markdownlint](https://github.com/DavidAnson/markdownlint) on the resultant changelog, to ensure that all links are up-to-date.
The author does this through the `DavidAnson.vscode-markdownlint` VSCode extension, but other ways of running it are available.

Note: Linebender uses the current date in the UTC time zone for release dates in changelogs.
In the evening in America this might be the next day than the one in your calendar, and for the morning in Australia, this might be the previous day.

## Releasing

The following steps are to be performed by the person who will perform the release (i.e. run `cargo publish`).
These all happen in sequence, once the release PR has been approved and everything else is in order.

### Checking permissions

The person who is making the release needs all of the following permissions:

- Ownership of the crate on crates.io.
  This can be checked by navigating to the crate's page on crates.io, and seeing if the publisher is listed as an owner.
- The "maintain" role on the corresponding repository.
  This allows them to make the release's tag and GitHub release.
  <!-- The author is an org admin, so there aren't any repos I don't have this maintain access to -->
  TODO: How can you check this beforehand?

### Checking the dates

The release date in the changelog should be updated to the correct current date just before merging.
This person performing the release should do this step, not the PR author.

Note: Linebender uses the current date in the UTC time zone.
In the evening in America this might be the next day than the one in your calendar, and for the morning in Australia, this might be the previous day.

### Merging the release PR

The release PR should be merged, after it has been approved.
This will use our standard merge queue.
The merge will be queued by the person performing the release.

Note that release pull requests are an exception to our "author merges" policy.
Instead release PRs can only be merged by the person who is performing the release.
This ensures that proper ownership of the release process is maintained.

### Validating correct commit

Once the release PR has been merged, you should check out the main branch, and pull from it.
You should also validate that the current checked out commit `git rev-parse --short HEAD` matches the newly created commit on main (e.g. through the GitHub web interface, using `gh repo view --web`).

### Publishing

You are now ready to perform the publish.
As a security best practice, you should only log `cargo` into crates.io whilst making releases.
The author uses 1Password to achieve this, but alternatively making a short-lived token on crates.io as you make a release is also an acceptable way to achieve this.
Note that we don't enforce this.

If you're publishing multiple crates at once, you should publish using `cargo publish -p {crate1} -p {crate2}`.
This ensures that each of the crates will compile successfully.

### Adding owners (new crates only)

For new crates, we make sure that at least three Linebender organisation members have publish permissions.
This should always include Raph, and at least one other organisation admin.
Note that this includes yourself, as the person who released the new crate.
Ask on Zulip if you're unsure about this step.

### GitHub release

We also create a GitHub release and git tag for each release.
The easiest way to do both is through GitHub's web interface.

- You should navigate to the repo's main page, and click "Tags", then navigate to the "releases" tab.
- From there, you should press "Draft a new release"
- In this page, you should enter the name of the new tag in the "Tag: Select tag" dropdown, pressing the "create new tag" option.
- Select the previous tag, then press "Generate release notes".
  We do this for the release title, new contributors and "Full Changelog" links.
  Remove the What's changed section.
- Add:
  - The crates.io/docs.rs links to the top of the page

    ```md
    **[Crates.io](https://crates.io/crates/{package_name}/{x.x.x}) | [Docs](https://docs.rs/{package_name}/{x.x.x})**
    ```

    You should be able to validate the crates.io link now, although the docs job will likely not have completed yet.

  - Beneath this, copy in the relevant section from the changelog (including the MSRV), with the following transformations:
    - The section headings should be kept at the same level (i.e. level 3)
    - All of the links to PRs (e.g. [`[#123][]`]) should be changed into just PR references (`#123`).
      This means that GitHub will show a card for the PR for people who view the release in the web interface.
    - All of the links to authors (e.g. [`[@raphlinus][]`]) should be changed into normal mentions (`@raphlinus`).
      This means that GitHub will show a card for the user for people who view the release in the web interface.
    - An MSRV link definition should be added at the bottom. This is the same in all our repositories:

      ```md
      [MSRV]: README.md#minimum-supported-rust-version-msrv
      ```

Once this is done, the GitHub release can be created.

### Zulip message

We also share releases on the [Linebender Zulip](https://xi.zulipchat.com/).
The format for this message is:

```md
{Crate Name} v{x.x.x} is out now:

- [GitHub Release](URL of the release on GitHub)
- [Crates.io](https://crates.io/crates/{package_name}/{x.x.x})
- [Docs](https://docs.rs/{package_name}/{x.x.x})
```

The format can be slightly different if the release involves multiple packages.
See the message sent for the previous release for that project in this case.

We also occasionally share releases on Mastodon.

## Checklist

<fieldset>

<legend>Before Release</legend>

<div>
<input type="checkbox" id="before-changelog"></input><label for="before-changelog">Changelog up-to-date.</label>
<a href="#updating-the-changelog" aria-label="Section on updating the changelog">↩︎</a>
</div>

<div>
<input type="checkbox" id="before-dependencies"></input><label for="before-dependencies">Dependencies up-to-date.</label>
<a href="#dependencies" aria-label="Section on updating dependencies">↩︎</a>
</div>

<div>
<input type="checkbox" id="before-version-number"></input><label for="before-version-number">Version numbers bumped in Cargo.toml. <b>Dry run</b> of the publish performed.</label>
<a href="#updating-package-versions" aria-label="Section on increasing Cargo version numbers">↩︎</a>
</div>

<div>
<input type="checkbox" id="before-changelog-links"></input><label for="before-changelog-links">All parts of the changelog reflect the new release, and markdownlint ran.</label>
<a href="#announcing-release-in-the-changelog" aria-label="Section on showing the release in the changelog">↩︎</a>
</div>

</fieldset>

<fieldset>

<legend>During Release</legend>

<div>
<input type="checkbox" id="during-permissions"></input><label for="during-permissions">Correct permissions validated.</label>
<a href="#checking-permissions" aria-label="Section on validating permissions">↩︎</a>
</div>

<div>
<input type="checkbox" id="during-date"></input><label for="during-date">Release date in changelog updated to today's date (in UTC). markdownlint executed.</label>
<a href="#checking-the-dates" aria-label="Section on updating the date">↩︎</a>
</div>

<div>
<input type="checkbox" id="during-merged"></input><label for="during-merged">Release PR merged by person who will do the release.</label>
<a href="#merging-the-release-pr" aria-label="Section on merging the release PR">↩︎</a>
</div>

<div>
<input type="checkbox" id="during-checkout"></input><label for="during-checkout">Checked out (and validated) the merged commit on main.</label>
<a href="#validating-correct-commit" aria-label="Section on checking the commit">↩︎</a>
</div>

<div>
<input type="checkbox" id="during-published"></input><label for="during-published">Published to crates.io.</label>
<a href="#publishing" aria-label="Section on publishing">↩︎</a>
</div>

<div>
<input type="checkbox" id="during-added-owners"></input><label for="during-added-owners">For new crates only: Additional owners added on crates.io.</label>
<a href="#adding-owners-new-crates-only" aria-label="Section on adding new owners">↩︎</a>
</div>

<div>
<input type="checkbox" id="during-github-release"></input><label for="during-github-release">Release on GitHub made.</label>
<a href="#github-release" aria-label="Section on making a GitHub release">↩︎</a>
</div>

<div>
<input type="checkbox" id="during-zulip"></input><label for="during-zulip">Announcement message sent to the Zulip for the release.</label>
<a href="#zulip-message" aria-label="Section on announcing on Zulip">↩︎</a>
</div>

</fieldset>
