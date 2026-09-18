// Copyright 2026 the Linebender Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

use xilem_web::svg::kurbo::{Affine, BezPath, Point, Vec2};

use crate::{euler::{EulerParams, EulerSeg}, squircle::{ProfileSample, Squircle}};

pub struct ClothoidSquircle;

// The `a` parameter controls smoothness and varies from 0 (circular arc)
// to 1 (two clothoids back to back, no arc segment).
//
// Unscaled arc length is pi/2 (1 + a), but this scales the radius,
// so it must be scaled back to a radius of 1.

struct ClothoidStuff {
    th: f64,
    params: EulerParams,
    xy: Vec2,
}

impl ClothoidStuff {
    fn new(a: f64) -> Self {
        let th = FRAC_PI_4 * a;
        let params = EulerParams::from_k0_k1(th, 2.0 * th);
        let xy_raw = Vec2::from_angle(-params.th(0.0)).rotate_scale(params.eval(1.0, 1e-12).to_vec2());
        // xy coordinate of Euler spiral segment, starting at 0, 0.
        let xy = (2.0 * th) * xy_raw;
        Self { th, params, xy }
    }

    /// Reciprocal of cale factor to bring unscaled shape to radius of 1
    fn inv_scale(&self) -> f64 {
        let xy = Vec2::from_angle(-self.params.th(0.0)).rotate_scale(self.params.eval(1.0, 1e-12).to_vec2());
        // xy coordinate of Euler spiral segment, starting at 0, 0.
        let xy_euler = (2.0 * self.th) * xy;
        let (s_arc, c_arc) = self.th.sin_cos();
        let x_center = xy_euler.x + FRAC_1_SQRT_2 - s_arc;
        let y_center = xy_euler.y + c_arc - FRAC_1_SQRT_2;
        x_center + y_center
    }
}

impl Squircle for ClothoidSquircle {
    fn render(&self, params: &[f64]) -> xilem_web::svg::kurbo::BezPath {
        const ACCURACY: f64 = 1e-4;
        // hacky thing, probably better to do this upstream
        let a = params[0];
        let a = (a - 0.707) / (1.0 - 0.707);
        let stuff = ClothoidStuff::new(a);
        let scale = 1.0 / stuff.inv_scale();
        let p0 = Point::new(1.0, 0.0);
        let p1 = Point::new(1.0 - scale * stuff.xy.y, scale * stuff.xy.x);
        let params = EulerParams::from_k0_k1(-stuff.th, -2.0 * stuff.th);
        let seg = EulerSeg::from_params(p0, p1, params);
        let mut result = BezPath::new();
        result.move_to(p0);
        result.extend(seg.to_cubics(ACCURACY));
        let arc_params = EulerParams::from_k0_k1(-(FRAC_PI_2 - 2.0 * stuff.th), 0.0);
        let p2 = Point::new(p1.y, p1.x);
        let arc_seg2 = EulerSeg::from_params(p1, p2, arc_params);
        result.extend(arc_seg2.to_cubics(ACCURACY));
        let params2 = EulerParams::from_k0_k1(-stuff.th, 2.0 * stuff.th);
        let p4 = Point::new(0.0, 1.0);
        let seg2 = EulerSeg::from_params(p2, p4, params2);
        result.extend(seg2.to_cubics(ACCURACY));
        // It would be better to fix the above so it's the right direction, but I'm lazy
        result
    }

    fn curvature_profile(&self, params: &[f64]) -> Vec<crate::squircle::ProfileSample> {
        let a = params[0];
        // hacky thing, probably better to do this upstream
        let a = (a - 0.707) / (1.0 - 0.707);
        let stuff = ClothoidStuff::new(a);
        let inv_scale = stuff.inv_scale();
        let scale = 1.0 / inv_scale;
        let mut result = vec![];
        let es_s = scale * FRAC_PI_2 * a;
        let arc_s = scale * FRAC_PI_2 * (1.0 - a);
        result.push(ProfileSample::new(0.0, 0.0));
        result.push(ProfileSample::new(es_s, inv_scale));
        result.push(ProfileSample::new(es_s + arc_s, inv_scale));
        result.push(ProfileSample::new(2.0 * es_s + arc_s, 0.0));
        result
    }
}