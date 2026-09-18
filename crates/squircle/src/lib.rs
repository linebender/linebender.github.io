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
    /// Overall gauge, between `squircle::GAUGE_MIN` and `squircle::GAUGE_MAX`.
    ///
    /// This is the "superellipse gauge": the distance from the centre to the
    /// shape along the 45 degree diagonal, as a fraction of the half-width. A
    /// circle is `FRAC_1_SQRT_2`, a square is 1.
    ///
    /// Ignored for constructions whose corner is fixed, which derive their
    /// gauge from [`AppState::flat`] instead; see
    /// [`Corner::resolve`](crate::squircle::Corner::resolve).
    gauge: f64,
    /// How much straight run to put along each edge, as a fraction of the most
    /// the current gauge allows, in the range [0, 1].
    ///
    /// It is a fraction rather than a length because the two are not
    /// independent; [`Corner::resolve`](crate::squircle::Corner::resolve)
    /// explains why.
    flat: f64,
    /// Draw a single corner rather than the whole four-fold shape.
    zoom: bool,
    /// Fill the shape rather than stroking its outline.
    fill: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            choice: Squircles::default(),
            gauge: 0.841,
            flat: 0.0,
            zoom: false,
            fill: false,
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
