// Copyright 2026 the Linebender Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An interactive tester for squircle constructions, embedded in the wiki.
//!
//! The crate is compiled to Wasm and mounted into a host element by [`start`];
//! see `content/wiki/curves/squircle.md` for the embedding side, and
//! `../index.html` for the standalone `trunk serve` one.
//!
//! Colors and sizing are deliberately *not* decided here. Every drawn path
//! carries a CSS class, and the page stylesheet supplies the stroke color, so
//! the demo picks up the site's light and dark themes instead of hardcoding a
//! palette that only works on one of them.

mod apple_squircle;
mod clothoid_squircle;
mod euler;
mod figma_squircle;
mod squircle;
mod view;

use wasm_bindgen::prelude::wasm_bindgen;
use xilem_web::App;

use crate::squircle::Squircles;

/// The state driving the tester.
#[derive(Debug)]
pub(crate) struct AppState {
    /// Which construction is drawn against the superellipse reference.
    choice: Squircles,
    /// Shape parameter, in the range [0.707, 0.999].
    ///
    /// This is the "superellipse gauge": the distance from the centre to the
    /// shape along the 45 degree diagonal, as a fraction of the half-width. A
    /// circle is `FRAC_1_SQRT_2`, a square is 1.
    gauge: f64,
    /// Draw a single corner rather than the whole four-fold shape.
    zoom: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            choice: Squircles::default(),
            gauge: 0.841,
            zoom: false,
        }
    }
}

/// Mount the tester into the element with the given id.
#[wasm_bindgen]
pub fn start(id: &str) {
    console_error_panic_hook::set_once();

    let root = xilem_web::get_element_by_id(id);
    App::new(root, AppState::default(), view::app_logic).run();
}
