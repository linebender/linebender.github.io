// Copyright 2026 the Linebender Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Generic trait for squircles

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, LN_2, SQRT_2};

use xilem_web::svg::kurbo::{
    Affine, BezPath, Line, ParamCurve, ParamCurveArclen, ParamCurveCurvature, PathSeg, Point,
    common::solve_itp,
};

use crate::{
    apple_squircle::AppleSquircle, clothoid_squircle::ClothoidSquircle,
    figma_squircle::FigmaSquircle,
};

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

/// Smallest overall gauge the tester offers.
pub const GAUGE_MIN: f64 = 0.707;
/// Largest overall gauge the tester offers.
pub const GAUGE_MAX: f64 = 0.999;

/// A circular corner, the roundest any of these reach.
const CORNER_MIN: f64 = FRAC_1_SQRT_2;

/// A straight run along each half edge plus a corner profile scaled into the
/// square left over, so `gauge = c + h * (1 - c)`.
#[derive(Clone, Copy, Debug)]
pub struct Corner {
    /// Where the corner profile crosses its own diagonal.
    pub c: f64,
    /// Length of the straight run along each half edge, as a fraction of the
    /// half width.
    pub h: f64,
    /// What to hand the construction to land on `c`.
    param: f64,
}

impl Corner {
    /// Turns the two slider positions into a corner.
    ///
    /// The gauge always holds. The flat picks the corner within what the gauge
    /// allows, a flat of 1 being exactly a circular corner; where the construction's
    /// family has no such corner it makes the nearest one it can and the flat
    /// takes up the difference.
    pub fn resolve(choice: Squircles, gauge: f64, flat: f64) -> Self {
        let h_max = ((gauge - CORNER_MIN) / (1.0 - CORNER_MIN)).max(0.0);
        let wanted = flat.clamp(0.0, 1.0) * h_max;
        let (c, param) = choice.solve_corner((gauge - wanted) / (1.0 - wanted));
        Self {
            c,
            h: ((gauge - c) / (1.0 - c)).max(0.0),
            param,
        }
    }
}

/// The exponent `n` of the superellipse `|x|^n + |y|^n = 1` whose quadrant
/// crosses its diagonal at `c`.
///
/// A circular corner is 2; the usual squircle is 4, at a gauge near 0.841.
pub fn superellipse_exponent(c: f64) -> f64 {
    -LN_2 / c.ln()
}

/// Where a quadrant crosses its own diagonal, which is the gauge.
pub fn diagonal_crossing(path: &BezPath) -> f64 {
    let diag = Line::new(Point::ZERO, Point::new(1.0, 1.0));
    path.segments()
        .flat_map(|seg| {
            seg.intersect_line(diag)
                .into_iter()
                .map(move |hit| seg.eval(hit.segment_t).x)
        })
        .fold(f64::NAN, f64::max)
}

/// Assembles a full quadrant: a straight run, the corner profile scaled into
/// the corner square, and another straight run.
pub fn quadrant(choice: Squircles, corner: Corner) -> BezPath {
    let Corner { h, param, .. } = corner;
    let profile = choice.render(&[param]);
    if h <= 0.0 {
        return profile;
    }
    let scaled = Affine::translate((h, h)) * Affine::scale(1.0 - h) * profile;
    let mut result = BezPath::new();
    result.move_to((1.0, 0.0));
    result.line_to((1.0, h));
    // The scaled profile starts on the point just added, so drop its move_to.
    result.extend(scaled.elements().iter().skip(1).copied());
    result.line_to((0.0, 1.0));
    result
}

/// The curvature profile of the assembled quadrant.
///
/// Scaling by `s` multiplies arc length by `s` and divides curvature by `s`, so
/// the corner's profile transforms directly rather than being resampled.
pub fn quadrant_profile(choice: Squircles, corner: Corner) -> Vec<ProfileSample> {
    let Corner { h, param, .. } = corner;
    let inner = choice.curvature_profile(&[param]);
    if h <= 0.0 {
        return inner;
    }
    let scale = 1.0 - h;
    // `render_profile` drops non-finite samples, so the corner's length comes
    // from the last one with a finite `s`; the flats still have to be placed.
    let corner_len = inner
        .iter()
        .rev()
        .find(|sample| sample.s.is_finite())
        .map_or(0.0, |sample| sample.s);
    let end = h + corner_len * scale;
    let mut result = Vec::with_capacity(inner.len() + 4);
    result.push(ProfileSample::new(0.0, 0.0));
    result.push(ProfileSample::new(h, 0.0));
    for sample in inner {
        result.push(ProfileSample {
            s: h + sample.s * scale,
            k: sample.k / scale,
        });
    }
    result.push(ProfileSample::new(end, 0.0));
    result.push(ProfileSample::new(end + h, 0.0));
    result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Squircles {
    #[default]
    Superellipse,
    ChromiumApprox,
    Clothoid,
    Figma,
    Apple,
}

impl Squircles {
    /// The name shown on this construction's radio button.
    pub fn name(self) -> &'static str {
        match self {
            Self::Superellipse => "Superellipse",
            Self::ChromiumApprox => "Chromium approximation",
            Self::Clothoid => "Clothoid",
            Self::Figma => "Figma",
            Self::Apple => "Apple",
        }
    }

    /// Whether the superellipse exponent describes this construction's corner.
    pub fn has_exponent(self) -> bool {
        matches!(self, Self::Superellipse | Self::ChromiumApprox)
    }

    /// The corner nearest `c` this construction can make, and the parameter for it.
    ///
    /// The superellipse and Chromium take the gauge as is, and Apple has one
    /// corner. The clothoid and Figma take a smoothness, so those are solved
    /// for; the clothoid's family stops near 0.79, well short of the slider.
    fn solve_corner(self, c: f64) -> (f64, f64) {
        match self {
            Self::Superellipse | Self::ChromiumApprox => (c, c),
            Self::Apple => (crate::apple_squircle::CORNER_GAUGE, 0.0),
            Self::Clothoid | Self::Figma => {
                let crossing = |p: f64| diagonal_crossing(&self.render(&[p]));
                let (a, b) = (GAUGE_MIN, GAUGE_MAX);
                let (ya, yb) = (crossing(a) - c, crossing(b) - c);
                // Negated so a NaN measurement takes this branch too.
                if !(ya * yb <= 0.0) {
                    let p = if ya.abs() < yb.abs() { a } else { b };
                    return (crossing(p), p);
                }
                // solve_itp wants f(a) < 0 < f(b); flip if the mapping descends.
                let flip = if ya > 0.0 { -1.0 } else { 1.0 };
                let p = solve_itp(
                    |p| flip * (crossing(p) - c),
                    a,
                    b,
                    1e-9,
                    1,
                    0.2 / (b - a),
                    flip * ya,
                    flip * yb,
                );
                (c, p)
            }
        }
    }

    /// The corner this construction is pinned to, if it has no shape parameter.
    pub fn fixed_corner(self) -> Option<f64> {
        match self {
            Self::Apple => Some(crate::apple_squircle::CORNER_GAUGE),
            _ => None,
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
            Self::Apple => AppleSquircle.render(params),
        }
    }

    fn curvature_profile(&self, params: &[f64]) -> Vec<ProfileSample> {
        match self {
            Self::Superellipse => Superellipse.curvature_profile(params),
            Self::ChromiumApprox => ChromiumApprox.curvature_profile(params),
            Self::Clothoid => ClothoidSquircle.curvature_profile(params),
            Self::Figma => FigmaSquircle.curvature_profile(params),
            Self::Apple => AppleSquircle.curvature_profile(params),
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
            // Pinned as `render` pins it: the last `th` lands a ulp past
            // `FRAC_PI_2`, so `cos` goes negative, `powf` returns NaN, and that
            // NaN reaches `s` through the chord below and stays there.
            let (v, u) = if i == N { (1.0, 0.0) } else { th.sin_cos() };
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
