// Copyright 2026 the Linebender Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Generic trait for squircles

use std::f64::consts::{FRAC_PI_2, SQRT_2};

use xilem_web::svg::kurbo::{
    BezPath, ParamCurve, ParamCurveArclen, ParamCurveCurvature, PathSeg, Point,
};

use crate::{clothoid_squircle::ClothoidSquircle, figma_squircle::FigmaSquircle};

pub struct ProfileSample {
    s: f64,
    k: f64,
}

pub trait Squircle {
    // Render one quadrant from (1, 0) to (0, 1)
    fn render(&self, params: &[f64]) -> BezPath;

    fn curvature_profile(&self, params: &[f64]) -> Vec<ProfileSample> {
        let path = self.render(params);
        let mut result = vec![];
        let mut s0 = 0.0;
        for seg in path.segments() {
            match seg {
                PathSeg::Line(l) => {
                    result.push(ProfileSample::new(s0, 0.0));
                    result.push(ProfileSample::new(s0 + l.length(), 0.0));
                }
                PathSeg::Cubic(c) => {
                    const N: usize = 50;
                    for i in 0..=N {
                        let t = i as f64 / N as f64;
                        let s = s0 + c.subsegment(0.0..t).arclen(1e-9);
                        let k = -c.curvature(t);
                        result.push(ProfileSample { s, k });
                    }
                }
                _ => todo!(),
            }
            s0 += seg.arclen(1e-9);
        }
        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Squircles {
    #[default]
    Superellipse,
    ChromiumApprox,
    Clothoid,
    Figma,
}

impl Squircles {
    /// The name shown on this construction's radio button.
    pub fn name(self) -> &'static str {
        match self {
            Self::Superellipse => "Superellipse",
            Self::ChromiumApprox => "Chromium approximation",
            Self::Clothoid => "Clothoid",
            Self::Figma => "Figma",
        }
    }
}

impl Squircle for Squircles {
    fn render(&self, params: &[f64]) -> BezPath {
        match self {
            Self::Superellipse => Superellipse.render(params),
            Self::ChromiumApprox => ChromiumApprox.render(params),
            Self::Clothoid => ClothoidSquircle.render(params),
            Self::Figma => FigmaSquircle.render(params),
        }
    }

    fn curvature_profile(&self, params: &[f64]) -> Vec<ProfileSample> {
        match self {
            Self::Superellipse => Superellipse.curvature_profile(params),
            Self::ChromiumApprox => ChromiumApprox.curvature_profile(params),
            Self::Clothoid => ClothoidSquircle.curvature_profile(params),
            Self::Figma => FigmaSquircle.curvature_profile(params),
        }
    }
}

pub struct Superellipse;

impl Squircle for Superellipse {
    fn render(&self, params: &[f64]) -> BezPath {
        let gauge = params[0];
        let exp_adjust = gauge.ln() * (-1.0 / SQRT_2.ln());
        const N: usize = 50;
        let mut result = BezPath::new();
        for i in 0..=N {
            let th = i as f64 * (FRAC_PI_2 / N as f64);
            let (v, u) = if i == N {
                (1.0, 0.0)
            } else {
                th.sin_cos()
            };
            let x = u.powf(exp_adjust);
            let y = v.powf(exp_adjust);
            let p = Point::new(x, y);
            if i == 0 {
                result.move_to(p);
            } else {
                result.line_to(p);
            }
        }
        result
    }

    fn curvature_profile(&self, params: &[f64]) -> Vec<ProfileSample> {
        let gauge = params[0];
        let exp_adjust = gauge.ln() * (-1.0 / SQRT_2.ln());
        let exp = 2.0 / exp_adjust;
        const N: usize = 200;
        let mut last_pt = Point::new(1.0, 0.0);
        let mut result = vec![];
        let mut s = 0.0;
        // The profile is symmetric about the diagonal, so sweeping a half
        // quadrant would carry the same information. It sweeps the full
        // quadrant anyway, to put this on the same arc length axis as the
        // constructions that use the default `curvature_profile`.
        for i in 0..=N {
            let th = i as f64 * (FRAC_PI_2 / N as f64);
            let (v, u) = th.sin_cos();
            let x = u.powf(exp_adjust);
            let y = v.powf(exp_adjust);
            let p = Point::new(x, y);
            // This is an underestimate but good enough for visualization.
            s += p.distance(last_pt);
            let g = 2.0 - 2.0 * exp_adjust;
            let mut k = (exp - 1.0) * (u * v).powf(g) * (u.powf(2.0 + g) + (v.powf(2.0 + g))).powf(-1.5);
            if !k.is_finite() {
                k = 1.0;
            }
            result.push(ProfileSample { s, k });
            last_pt = p;
        }
        result
    }
}

pub struct ChromiumApprox;

// Adapted closely from https://developer.chrome.com/blog/implementing-corner-shape
fn calc_superellipse(k: f64) -> (f64, f64) {
    const P0: f64 = 1.2430920942724248;
    const P1: f64 = 2.010479023614843;
    const P2: f64 = 0.32922901179443753;
    const P3: f64 = 0.2823023142212073;
    const P4: f64 = 1.3473704261055421;
    const P5: f64 = 2.9149468637949814;
    const P6: f64 = 0.9106507102917086;

    let s = k.log2();
    let slope = P0 + (P6 - P0) * 0.5 * (1.0 + (P5 * (s - P1)).tanh());
    let base = 1.0 / (1.0 + (slope * P1).exp());
    let logistic = 1.0 / (1.0 + (slope * (P1 - s)).exp());

    let a = (logistic - base) / (1.0 - base);
    let b =  P2 * (-P3 * s.powf(P4)).exp();
    (a, b)
}

impl Squircle for ChromiumApprox {
    fn render(&self, params: &[f64]) -> BezPath {
        let gauge = params[0];
        let k = (0.5f64).ln() / gauge.ln();
        let (a, b) = calc_superellipse(k);
        let mut result = BezPath::new();
        result.move_to((1.0, 0.0));
        result.curve_to((1.0, a), (gauge + b, gauge - b), (gauge, gauge));
        result.curve_to((gauge - b, gauge + b), (a, 1.0), (0.0, 1.0));
        result
    }
}

/// Builds a polyline of a curvature profile in (arc length, curvature) space.
///
/// The caller is responsible for mapping this into screen coordinates, so that
/// the plot's axes and its data stay in step.
///
/// Samples that are not finite are dropped, and a gap starts a new subpath so
/// the line is never drawn straight through the discontinuity. Curvature really
/// is undefined at points these constructions can reach: at the bottom of the
/// gauge range the Figma corner collapses to a cubic whose first three control
/// points coincide, so its derivative is zero there and its curvature is 0/0.
/// Emitting one NaN would cost the whole curve rather than one point, because a
/// browser rejects an entire SVG `d` attribute that contains one.
pub fn render_profile(profile: &[ProfileSample]) -> BezPath {
    let mut result = BezPath::new();
    let mut pen_down = false;
    for sample in profile {
        if !sample.s.is_finite() || !sample.k.is_finite() {
            pen_down = false;
            continue;
        }
        let p = Point::new(sample.s, sample.k);
        if pen_down {
            result.line_to(p);
        } else {
            result.move_to(p);
            pen_down = true;
        }
    }
    result
}

impl ProfileSample {
    pub fn new(s: f64, k: f64) -> Self {
        Self { s, k }
    }
}
