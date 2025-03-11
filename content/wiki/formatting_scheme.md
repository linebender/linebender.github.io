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

## Markdown

In Markdown files, every paragraph should have one line per sentence.
This makes git diff easier to read.
[[1]](https://nick.groenen.me/notes/one-sentence-per-line/) [[2]](https://sive.rs/1s)

