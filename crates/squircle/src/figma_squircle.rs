// Copyright 2026 the Linebender Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An implementation of the Figma squircle
//! This is adapted from squircle-path-kit.

use std::f64::consts::PI;

use xilem_web::svg::kurbo::{Affine, BezPath, CubicBez, Point, Vec2};

use crate::{euler::{EulerParams, EulerSeg}, squircle::Squircle};

pub struct FigmaSquircle;

// Only implements the "squircle" type
struct Corner {
    start_point: Point,
    end_point: Point,
    in_bezier: CubicBez,
    // original has arc segments, but we'll use Euler
    reduced_sweep: f64,
    out_bezier: CubicBez,
}

fn compute_corner(prev: Point, curr: Point, next: Point, radius: f64, smoothness: f64, budget: f64) -> Corner {
    let dir_in = (prev - curr).normalize();
    let dir_out = (next - curr).normalize();
    let d = dir_in.dot(dir_out).min(1.0).max(-1.0);
    let phi = d.acos();
    let half_phi = 0.5 * phi;

    // logic elided to result in sharp corner
    let sin_half = half_phi.sin();
    let tan_half = half_phi.tan();

    let mut q = radius / tan_half;
    let mut xi = smoothness.min(1.0).max(0.0);

    if q > budget {
        q = budget;
        xi = 0.0;
    } else {
        let p = (1.0 + xi) * q;
        if p > budget {
            xi = budget / (q - 1.0);
        }
    }
    let p = (1.0 + xi) * q;
    let effective_radius = q * tan_half;
    let bisector = (dir_in + dir_out).normalize();
    let center = curr + (effective_radius / sin_half) * bisector;

    let tangent_in = curr + q * dir_in;
    let tangent_out =  curr + q * dir_out;

    let radial_in = (tangent_in - center).normalize();
    // We don't use this because we only do one direction, and transform later.
    let _is_ccw = radial_in.cross(dir_in) > 0.0;

    let start_angle = radial_in.atan2();
    let radial_out = (tangent_out - center).normalize();
    let end_angle = radial_out.atan2();

    let sweep = end_angle - start_angle;
    // TODO: modulo 2pi, respecting is_ccw

    let turn = PI - phi;
    let beta = (turn * 0.5) * xi;
    let t = effective_radius * (beta * 0.5).tan();

    let a_plus_b = p - (q - t);
    let b = a_plus_b / 3.0;
    let a = 2.0 * b;

    let reduced_sweep = sweep * (1.0 - xi);
    let mid_angle = start_angle + sweep * 0.5;
    let r_start = mid_angle - reduced_sweep * 0.5;
    let r_end = r_start + reduced_sweep;

    let arc_start_pt = center + effective_radius * Vec2::from_angle(r_start);
    let arc_end_pt = center + effective_radius * Vec2::from_angle(r_end);

    let start_point = curr + p * dir_in;
    let end_point = curr + p * dir_out;
    let in_bezier = CubicBez::new(start_point, curr + (p - a) * dir_in, curr + (q - t) * dir_in, arc_start_pt);
    let out_bezier = CubicBez::new(arc_end_pt, curr + (q - t) * dir_out, curr + (p - a) * dir_out, end_point);
    Corner {
        start_point,
        end_point,
        in_bezier,
        reduced_sweep,
        out_bezier,
    }
}

impl Corner {
    fn to_bez_path(&self) -> BezPath {
        let mut result = BezPath::new();
        result.move_to(self.start_point);
        result.curve_to(self.in_bezier.p1, self.in_bezier.p2, self.in_bezier.p3);
        // TODO: arc segments
        let arc_params = EulerParams::from_k0_k1(-self.reduced_sweep, 0.0);
        let arc_seg = EulerSeg::from_params(self.in_bezier.p3, self.out_bezier.p0, arc_params);
        const ACCURACY: f64 = 0.1;
        result.extend(arc_seg.to_cubics(ACCURACY));
        result.curve_to(self.out_bezier.p1, self.out_bezier.p2, self.out_bezier.p3);
        result
    }
}

/// Reproduces a corner that has been spot-verified against squircle-path-kit.
///
/// Kept as a reference for anyone re-checking the port; not used by the demo.
#[allow(dead_code)]
pub fn test_corner() -> BezPath {
    let prev = Point::new(160., 0.);
    let curr = Point::new(320., 0.);
    let next = Point::new(320., 90.);
    let budget = 90.0; // Not sure how to set this.
    let corner = compute_corner(prev, curr, next, 48., 0.68, budget);
    corner.to_bez_path()
}

impl Squircle for FigmaSquircle {
    // The strategy here is to render a verifiable path, then convert
    // into the form required.
    fn render(&self, params: &[f64]) -> BezPath {
        // Same hack as clothoid; probably should fix this for real
        let smooth = (params[0] - 0.707) / (1.0 - 0.707);
        let prev = Point::new(-10., 0.);
        let curr = Point::ORIGIN;
        let next = Point::new(0., 10.);
        let budget = 10.0;
        let corner = compute_corner(prev, curr, next, 1.0, smooth, budget);
        let scale = 1.0 / corner.end_point.y;
        let aff = Affine::new([0., scale, -scale, 0., 1., 1.]);
        aff * corner.to_bez_path()
    }
}
