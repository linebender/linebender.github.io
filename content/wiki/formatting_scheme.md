+++
title = "Formatting scheme for Linebender projects"
+++

## Copyright

Code files should start with a copypright notice following this format:

```rust
// Copyright 2024 the XXXX Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT
```

The year should be the file's creation year.
"XXXX" should be the project's name.
The license depends on the project and the file type, but is usually "either Apache 2.0 or MIT License, whichever is least restrictive".

## Rust

Most Linebender projects files are written in Rust.
For those files, the default formatting produced by `rustfmt` is fine.

## Markdown

In Markdown files, every paragraph should have one line per sentence. This makes git diff easier to read.
