+++
title = "Formatting scheme for Linebender projects"
+++

## Copyright

Code files should start with a copypright notice composed of two comments following this format:

```rust
// Copyright 2024 the XXXX Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT
```

The year should be the file's creation year.
"XXXX" should be the project's name.
The license depends on the project and the file type, but is usually "either Apache 2.0 or MIT License, whichever is least restrictive".
Shaders are usually licensed under the former two licenses OR the Unlicense.

## Rust

Most Linebender projects files are written in Rust.
For those files, the formatting produced by `rustfmt` is required.

The following rustfmt config is recommended:

```toml
# LINEBENDER RUSTFMT CONFIG - v1
# Ensure lines end with \n even if the git configuration core.autocrlf is not set to true
newline_style = "Unix"

# `Foobar { foo, bar }` is more readable than `Foo { foo: foo, bar: bar }`
use_field_init_shorthand = true

# Commented out because it is still unstable, but works fine in practice.
# imports_granularity = "Module"

# END LINEBENDER RUSTFMT CONFIG
```

You may also want to occasionally use this unstable config:

```toml
# Groups imports in a predictable way: first core/alloc/std, then other crates, then the current crate.
group_imports = "StdExternalCrate"
```

We don't recommend it as a permanent config value even for nightly projects, because the way it reorders items [isn't always ideal](https://github.com/linebender/linebender.github.io/issues/87).

## Markdown

In Markdown files, every paragraph should have one line per sentence.
This makes git diff easier to read.
[[1]](https://nick.groenen.me/notes/one-sentence-per-line/) [[2]](https://sive.rs/1s)

## Cargo Metadata

We order metadata in our `Cargo.toml` for packages consistently, in the order:

* Ordering of sections: `package`, `features`, `dependencies`, `lints`, `profile`, `example`.
* Ordering of `package` fields: `name`, `version`, `description`, `keywords`, `categories`, `edition`, `rust-version`, `license`, `repository`, `homepage`, `exclude`.

This reduces the number of choices we need to make over time.
This isn't worth failing a review over, but can be cleaned up in a later audit.
Source: [xilem#972](https://github.com/linebender/xilem/pull/972).
`package` includes the docs.rs metadata:

```toml
[package.metadata.docs.rs]
all-features = true
```

We used to implement <https://blog.rust-lang.org/2026/04/04/docsrs-only-default-targets/> manually, but no longer need to, so you can remove that.
We must also follow the [linting setup](@/wiki/canonical_lints.md).

The baseline `.gitignore` should contain only `/target`. We choose not to ignore IDE specific files, but should ignore `.DS_Store`.

### Taplo

We currently use Taplo for TOML formatting - config file looks something like:

```toml
# See https://taplo.tamasfe.dev/configuration/file.html
# and https://taplo.tamasfe.dev/configuration/formatter-options.html

[formatting]
# Aligning comments with the largest line creates
# diff noise when neighboring lines are changed.
align_comments = false

# Matches how rustfmt formats Rust code
column_width = 100
indent_string = "    "
```

## Setting up no_std properly

As many of our crates as possible are made `no_std`.
Crates which are `no_std` should have the `"no-std"` crates.io category.
Even for crates which don't use `std`, they still get a default `std` feature, which gates a line such as:

```rust
// Ensure that we don't compile if you're using the std feature on a platform without `std`
#[cfg(feature = "std")]
extern crate std as _;
```

By default, we do not promise to make our crates no-alloc. Crates which would otherwise be `no-alloc` should have a line such as:

```rust
// Avoid adding alloc in the future being a breaking change.
extern crate alloc as _;
```

This means that if we get a user who wishes to use the crate in a `no-alloc` environment then we won't accidentally break them later.
If we do get a request, we should set up dedicated CI for this use case, but we're not aware of this having happened yet, so haven't done so.
There will be a note about this in the `# Features` section of the crate level docs.
See [#blogging > no_std survey @ 💬](https://xi.zulipchat.com/#narrow/channel/181284-blogging/topic/no_std.20survey/near/618972372).

## Documentation

All top-level headings in Rust documentation should use `#`, only after a `#` can we use `##`.
Our crate level readme has a `Features` section, which should look something like the following:

```rust
//! # Features
//!
//! The following crate [feature flags](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features) are available:
//!
//! - `std` (enabled by default): This is currently unused and is provided for forward compatibility.
//! 
//! Note that [Crate Name] does require that an allocator is available (i.e. it uses [alloc][]).
```

For the crates which do the `libm`/`std` dance, those features should be documented as:

```rust
//! - `std` (enabled by default): Get floating point functions from the standard library
//!   (likely using your target's libc).
//! - `libm`: Use floating point implementations from [libm][].
//!   This is useful for `no_std` environments.
//!   However, note that the `libm` crate is not as efficient as the standard library.
//!
//! At least one of `std` and `libm` is required; `std` overrides `libm`.
//! Note that [Crate Name] does require that an allocator is available (i.e. it uses [alloc][]).
#![cfg_attr(feature = "libm", doc = "[libm]: libm")]
#![cfg_attr(not(feature = "libm"), doc = "[libm]: https://crates.io/crates/libm")]

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("[Crate Name] requires either the `std` or `libm` feature to be enabled");
```

The docs for each feature should also be copied to their `Cargo.toml` feature description.
"Bare" intra doc links (such as `[Item]`) should be avoided in the top level docs - you must use something like `[Item][]`.
This allows adding an equivalent link to docs.rs in the README to be linted for.

### Cargo rdme

We use [cargo-rdme](https://crates.io/crates/cargo-rdme/1.5.1) v1.5.1 for our readmes.
(The 2.0 series is much harder to use due to its handling of intra-doc links)
Each crate README should have the following block:

```md
<!-- We use cargo-rdme to update the README with the contents of lib.rs.
To edit the following section, update it in lib.rs, then run:
cargo rdme --workspace-project=[crate_id]
Full documentation at https://github.com/orium/cargo-rdme -->

<!-- Intra-doc links used in lib.rs should be evaluated here.
See https://linebender.org/blog/doc-include/ for related discussion. -->
[`Affine`]: https://docs.rs/kurbo/latest/kurbo/struct.Affine.html
[`alloc`]: https://doc.rust-lang.org/stable/alloc/
<!-- cargo-rdme start -->
```

Intra-doc links to this or other crates must be to `latest` (or `stable` for crates in the Rust distribution).

We also choose to document the command at the top of `lib.rs` (after the copyright header but before the doc):

```rust
// After you edit the crate's doc comment, run this command, then check README.md for any missing links
// cargo rdme --workspace-project=gazeto
```

### Readme Setup

The readme order for individual packages is as follows. We don't yet have the format fully worked out for workspace (i.e. repository) level readmes.

```md
<div align="center">

# Name (Title Case)

**Tagline**

[![Latest published version.](https://img.shields.io/crates/v/vello.svg)](https://crates.io/crates/vello)
[![Documentation build status.](https://img.shields.io/docsrs/vello.svg)](https://docs.rs/vello)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](#license)
\
[![Linebender Zulip chat.](https://img.shields.io/badge/Linebender-%23gpu-blue?logo=Zulip)](https://xi.zulipchat.com/#narrow/stream/197075-gpu)
[![GitHub Actions CI status.](https://img.shields.io/github/actions/workflow/status/linebender/vello/ci.yml?logo=github&label=CI)](https://github.com/linebender/vello/actions)
[![Dependency staleness status.](https://deps.rs/crate/vello/latest/status.svg)](https://deps.rs/crate/vello)

</div>

<!-- We use cargo-rdme ... -->
<!-- cargo-rdme start -->
...
<!-- cargo-rdme end -->


## Minimum supported Rust Version (MSRV)

This version of [Crate Name] has been verified to compile with **Rust 1.96** and later.

Future versions of [Crate Name] might increase the Rust version requirement.
It will not be treated as a breaking change and as such can even happen with small patch releases.

## Community

<!-- Replace channel as appropriate -->
Discussion of [Crate Name] development happens in the [Linebender Zulip](https://xi.zulipchat.com/), specifically the [#general channel](https://xi.zulipchat.com/#narrow/channel/147921-general).
All public content can be read without logging in.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Contributions are welcome by pull request. The [Rust code of conduct] applies.
Please feel free to add your name to the [AUTHORS] file in any substantive pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.

[Rust Code of Conduct]: https://www.rust-lang.org/policies/code-of-conduct
[AUTHORS]: ../AUTHORS
```

We haven't yet decided on the badges for top-level readmes.
See [#linebender > Bikeshedding badges @ 💬](https://xi.zulipchat.com/#narrow/channel/419691-linebender/topic/Bikeshedding.20badges/near/600855247).

Each crate must have their license files copied into the crate, as well as at the top level.
