// Copyright 2026 the Linebender Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The rounded rectangle Apple introduced in iOS 7.
//!
//! Unlike the other constructions here this one is not derived from a formula.
//! It is Apple's own control points, recovered by PaintCode by walking the live
//! `CGPath` with `CGPathApply` and published at
//! <https://www.paintcodeapp.com/blogpost/code-for-ios-7-rounded-rectangles>.
//! The numbers are the system's, not a fit to a picture of it.
//!
//! The published path takes a corner radius, which it clamps to a limit of
//! `min(width, height) / 2 / 1.52866483`. This renders the square aspect ratio
//! at exactly that limit, which is where the flat sides vanish and the corner
//! curve runs from one edge midpoint to the next, so the shape is comparable
//! with the others. Shorter radii leave a straight run along each side; that
//! flat length is the parameter this construction really has, and it is not
//! wired up yet, so [`Squircle::render`] ignores the gauge.
//!
//! A quadrant is four segments: a cubic, a short straight run, and two more
//! cubics. It is not symmetric about the corner's diagonal -- the straight run
//! sits on one side of it -- and it is not curvature continuous: curvature
//! drops to zero across the straight run, and steps from 1.638 to 1.095 at the
//! join between the last two cubics. The tangent turns by about 4.5 and 6.2
//! degrees at the straight run's ends and 2.5 degrees at that last join.
//!
//! Those are properties of Apple's shape, not of the transcription, which is
//! what makes this one worth having beside constructions that are smooth by
//! design. Reproduce the path as published; do not tidy it.

use xilem_web::svg::kurbo::{BezPath, Point};

use crate::squircle::Squircle;

pub struct AppleSquircle;

/// Half the side length, as a multiple of the corner radius.
///
/// The published code uses this constant to clamp the radius, and also as the
/// coordinate of the point where each corner's curve begins. It writes that
/// second use as 1.52866471 in places, which looks like rounding in whatever
/// produced the listing; using the one value throughout puts the quadrant's
/// ends exactly on the edge midpoints rather than a hundred-millionth off.
const HALF_SIDE_IN_RADII: f64 = 1.52866483;

/// A control point, in the units the published code uses.
///
/// Its coordinates are multiples of the corner radius, measured inwards from
/// the corner. This takes the bottom right corner, which is the one whose
/// published order already runs from one edge midpoint to the next in the
/// direction the other constructions use, and scales it so the half side
/// length is 1 and the shape's centre is the origin.
fn pt(x: f64, y: f64) -> Point {
    Point::new(
        1.0 - x / HALF_SIDE_IN_RADII,
        1.0 - y / HALF_SIDE_IN_RADII,
    )
}

impl Squircle for AppleSquircle {
    fn render(&self, _params: &[f64]) -> BezPath {
        const K: f64 = HALF_SIDE_IN_RADII;
        let mut result = BezPath::new();
        result.move_to(pt(0.0, K));
        result.curve_to(
            pt(0.0, 1.08849323),
            pt(0.0, 0.86840689),
            pt(0.06549569, 0.66993493),
        );
        result.line_to(pt(0.07491111, 0.63149399));
        result.curve_to(
            pt(0.16905883, 0.37282392),
            pt(0.37282392, 0.16905883),
            pt(0.63149399, 0.07491111),
        );
        result.curve_to(pt(0.86840689, 0.0), pt(1.08849323, 0.0), pt(K, 0.0));
        result
    }
}
