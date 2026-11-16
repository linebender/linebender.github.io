// Copyright 2026 the Linebender Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Standalone entry point for `trunk serve`, for iterating without zola.
//!
//! It mounts into the same element id the wiki page uses, so this and the
//! embedded build run identical code.

fn main() {
    squircle::start("squircle-demo-root");
}
