// Copyright 2021 The kurbo Authors.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A general kurbo implementation of Euler spirals.
//! This file is lightly adapted from kurbo#169, which never got merged.

#![allow(unused)]

use xilem_web::svg::kurbo::{
    Affine, CubicBez, Line, ParamCurve, ParamCurveArclen, ParamCurveCurvature, ParamCurveDeriv,
    PathEl, Point, Vec2,
};

/// An Euler spiral segment.
///
/// This is only enabled when the `euler` feature is selected.
#[derive(Clone, Copy, Debug)]
pub struct EulerSeg {
    p0: Point,
    p1: Point,
    params: EulerParams,
}

/// The derivative of an Euler spiral segment.
#[derive(Clone, Copy)]
pub struct EulerSegDeriv {
    c0: f64,
    c1: f64,
    c2: f64,
    scale: f64,
}

/// The second derivative of an Euler spiral segment.
pub struct EulerSegDeriv2(EulerSegDeriv);

/// Parameters for an Euler spiral segment. Does not include endpoint geometry.
///
/// This is something of an internal detail for [`EulerSeg`] and might not make
/// it to the public interface. It's public here for experimentation.
///
/// It's entirely possible the disposition of this is to be inlined into `EulerSeg`.
/// I'm not sure it's useful by itself.
#[derive(Clone, Copy, Debug)]
pub struct EulerParams {
    k0: f64,
    k1: f64,
    chord: f64,
    chth: f64,
}

/// A path consisting of piecewise Euler spiral segments.
///
/// TODO: develop this further, including implementing the [`Shape`][crate::Shape] trait.
///
/// This is only enabled when the `euler` feature is selected.
pub struct EulerPath(Vec<EulerPathEl>);

/// An element of a piecewise Euler spiral path.
#[derive(Clone, Copy, Debug)]
pub enum EulerPathEl {
    /// Start a new subpath at the given point.
    MoveTo(Point),
    /// A line segment to the given point.
    LineTo(Point),
    /// An Euler spiral segment to the given point.
    EulerTo(EulerParams, Point),
    /// Close the subpath.
    ClosePath,
}

/// An iterator producing euler segments from a cubic bezier.
///
/// Discussion: should this be an anonymous (`from_fn`) type?
pub struct CubicToEulerIter {
    c: CubicBez,
    tolerance: f64,
    // [t0 * dt .. (t0 + 1) * dt] is the range we're
    // currently considering.
    t0: u64,
    dt: f64,
}

fn integ_euler_12(k0: f64, k1: f64) -> (f64, f64) {
    let t1_1 = k0;
    let t1_2 = 0.5 * k1;
    let t2_2 = t1_1 * t1_1;
    let t2_3 = 2. * (t1_1 * t1_2);
    let t2_4 = t1_2 * t1_2;
    let t3_4 = t2_2 * t1_2 + t2_3 * t1_1;
    let t3_6 = t2_4 * t1_2;
    let t4_4 = t2_2 * t2_2;
    let t4_5 = 2. * (t2_2 * t2_3);
    let t4_6 = 2. * (t2_2 * t2_4) + t2_3 * t2_3;
    let t4_7 = 2. * (t2_3 * t2_4);
    let t4_8 = t2_4 * t2_4;
    let t5_6 = t4_4 * t1_2 + t4_5 * t1_1;
    let t5_8 = t4_6 * t1_2 + t4_7 * t1_1;
    let t5_10 = t4_8 * t1_2;
    let t6_6 = t4_4 * t2_2;
    let t6_7 = t4_4 * t2_3 + t4_5 * t2_2;
    let t6_8 = t4_4 * t2_4 + t4_5 * t2_3 + t4_6 * t2_2;
    let t6_9 = t4_5 * t2_4 + t4_6 * t2_3 + t4_7 * t2_2;
    let t6_10 = t4_6 * t2_4 + t4_7 * t2_3 + t4_8 * t2_2;
    let t7_8 = t6_6 * t1_2 + t6_7 * t1_1;
    let t7_10 = t6_8 * t1_2 + t6_9 * t1_1;
    let t8_8 = t6_6 * t2_2;
    let t8_9 = t6_6 * t2_3 + t6_7 * t2_2;
    let t8_10 = t6_6 * t2_4 + t6_7 * t2_3 + t6_8 * t2_2;
    let t9_10 = t8_8 * t1_2 + t8_9 * t1_1;
    let t10_10 = t8_8 * t2_2;
    let mut u = 1.;
    u -= (1. / 24.) * t2_2 + (1. / 160.) * t2_4;
    u += (1. / 1920.) * t4_4 + (1. / 10752.) * t4_6 + (1. / 55296.) * t4_8;
    u -= (1. / 322560.) * t6_6 + (1. / 1658880.) * t6_8 + (1. / 8110080.) * t6_10;
    u += (1. / 92897280.) * t8_8 + (1. / 454164480.) * t8_10;
    u -= 2.4464949595157930e-11 * t10_10;
    let mut v = (1. / 12.) * t1_2;
    v -= (1. / 480.) * t3_4 + (1. / 2688.) * t3_6;
    v += (1. / 53760.) * t5_6 + (1. / 276480.) * t5_8 + (1. / 1351680.) * t5_10;
    v -= (1. / 11612160.) * t7_8 + (1. / 56770560.) * t7_10;
    v += 2.4464949595157932e-10 * t9_10;
    (u, v)
}

#[doc(hidden)]
/// Computation of the Euler spiral integral using subdivision.
pub fn integ_euler_12n(mut k0: f64, mut k1: f64, n: usize) -> (f64, f64) {
    let th1 = k0;
    let th2 = 0.5 * k1;
    let ds = (n as f64).recip();

    k0 *= ds;
    k1 *= ds;

    let mut x = 0.0;
    let mut y = 0.0;
    let s0 = 0.5 * ds - 0.5;

    for i in 0..n {
        let s = s0 + ds * (i as f64);
        let km0 = k1 * s + k0;
        let km1 = k1 * ds;

        let (u, v) = integ_euler_12(km0, km1);

        let th = (th2 * s + th1) * s;
        let cth = th.cos();
        let sth = th.sin();

        x += cth * u - sth * v;
        y += cth * v + sth * u;
    }
    (x * ds, y * ds)
}

/// Evaluate the Euler spiral integral.
///
/// Compute the following integral to the desired accuracy.
///
/// $$
/// \int_{-0.5}^{0.5} \exp(i(k_0 s + 1/2 k_1 s^2)) ds
/// $$
///
/// This is discussed in section 8.1 of [Raph's thesis], and the error bounds
/// are validated in the notebook attached to the parallel curve blog post.
///
/// [Raph's thesis]: https://www.levien.com/phd/thesis.pdf
pub fn integ_euler(k0: f64, k1: f64, accuracy: f64) -> (f64, f64) {
    let c1 = k1.abs();
    let c0 = k0.abs() + 0.5 * c1;
    let est_err_raw = 0.006 * c0 * c0 + 0.029 * c1;
    // Fun performance note: if the accuracy were always known at compile time,
    // it would be theoretically cheaper to compare against accuracy^(1/6), which
    // is computed anyway in the subdivision case. But the cost of the powi(6) is
    // basically not measurable, and the cost of the ^(1/6) is ballpark double
    // the integration itself.
    if est_err_raw.powi(6) < accuracy {
        integ_euler_12(k0, k1)
    } else {
        let n = (est_err_raw / accuracy.powf(1.0 / 6.0)).ceil() as usize;
        integ_euler_12n(k0, k1, n)
    }
}

impl EulerParams {
    /// Find the Euler spiral parameters for the given deflection.
    ///
    /// TODO: use research for direct solution.
    ///
    /// Discussion question: should this take an accuracy parameter?
    /// This version basically hardcodes 1e-9.
    pub fn fit_euler(th0: f64, th1: f64) -> EulerParams {
        // Note: we could skip the solving for very small deflection
        let mut k1_old = 0.0;
        let dth = th1 - th0;
        let k0 = th0 + th1;
        let mut k1 = (6.0 - (1. / 70.) * dth * dth - 0.1 * k0 * k0) * dth;
        let mut error_old = dth;
        for _ in 0..10 {
            let (u, v) = integ_euler(k0, k1, 1e-12);
            let chth = v.atan2(u);
            let error = dth - (0.25 * k1 - 2.0 * chth);
            if error.abs() < 1e-9 {
                let chord = u.hypot(v);
                return EulerParams {
                    k0,
                    k1,
                    chord,
                    chth,
                };
            }
            let new_k1 = k1 + (k1_old - k1) * error / (error - error_old);
            k1_old = k1;
            error_old = error;
            k1 = new_k1;
        }
        panic!("fit_euler diverged on {}, {}", th0, th1);
    }

    /// Create `EulerParams` from k0 and k1 parameters.
    pub fn from_k0_k1(k0: f64, k1: f64) -> EulerParams {
        let (u, v) = integ_euler(k0, k1, 1e-12);
        let chth = v.atan2(u);
        let chord = u.hypot(v);
        EulerParams {
            k0,
            k1,
            chord,
            chth,
        }
    }

    /// Determine tangent angle at the given parameter.
    ///
    /// The sign may be confusing, but it matches the spiro code. When `t = 0`,
    /// the result is `-th0`, and when `t = 1`, the result is `th1`.
    pub fn th(&self, t: f64) -> f64 {
        let u = t - 0.5;
        (0.5 * self.k1 * u + self.k0) * u - self.chth
    }

    /// Evaluate the curve at the given parameter.
    ///
    /// The parameter is in the range 0..1, and the result goes from (0, 0) to (1, 0).
    pub fn eval(&self, t: f64, accuracy: f64) -> Point {
        let th_m = self.th(t * 0.5);
        let k0 = self.k0;
        let k1 = self.k1;
        let (u, v) = integ_euler((k0 + k1 * 0.5 * (t - 1.0)) * t, k1 * t * t, accuracy);
        let s = t / self.chord * th_m.sin();
        let c = t / self.chord * th_m.cos();
        let x = u * c - v * s;
        let y = -v * c - u * s;
        Point::new(x, y)
    }
}

impl EulerSeg {
    /// Create a new Euler segment.
    ///
    /// TODO: document the conventions. An SVG would be especially nice.
    pub fn new(p0: Point, p1: Point, th0: f64, th1: f64) -> EulerSeg {
        let params = EulerParams::fit_euler(th0, th1);
        EulerSeg { p0, p1, params }
    }

    /// Create an Euler segment from a cubic Bézier.
    ///
    /// The curve is fit according to G1 geometric Hermite interpolation, in
    /// other words the endpoints and tangents match the given curve.
    pub fn from_cubic(c: CubicBez) -> EulerSeg {
        let d01 = c.p1 - c.p0;
        let d23 = c.p3 - c.p2;
        let d03 = c.p3 - c.p0;
        let th0 = d03.cross(d01).atan2(d03.dot(d01));
        let th1 = d23.cross(d03).atan2(d23.dot(d03));
        let params = EulerParams::fit_euler(th0, th1);
        EulerSeg {
            p0: c.p0,
            p1: c.p3,
            params,
        }
    }

    /// Create a segment from params and endpoints.
    ///
    /// Mostly used for experimentation.
    #[doc(hidden)]
    pub fn from_params(p0: Point, p1: Point, params: EulerParams) -> EulerSeg {
        EulerSeg { p0, p1, params }
    }

    /// Report whether the segment is a straight line.
    pub fn is_line(&self) -> bool {
        self.params.k0 == 0.0 && self.params.k1 == 0.0
    }

    /// Convert to cubic beziers.
    pub fn to_cubics(&self, accuracy: f64) -> impl Iterator<Item = PathEl> {
        let this = *self;
        let mut t0_int = 0usize;
        let mut dt = 1.0;
        let mut p0 = self.p0;
        let chord_atan = (self.p1 - self.p0).atan2();
        let thresh = accuracy * self.params.chord / (self.p1 - self.p0).hypot();
        std::iter::from_fn(move || {
            let t0 = (t0_int as f64) * dt;
            if t0 == 1.0 {
                return None;
            }
            loop {
                let t1 = t0 + dt;
                let k0 = dt * (this.params.k0 + 0.5 * (t0 + t1 - 1.0) * this.params.k1);
                let k1 = dt * dt * this.params.k1;
                let a0 = k0.abs();
                let a1 = k1.abs();
                // Error metric empirically determined, using `fit_cubic_plot` in example.
                let err = 1.5e-5 * a0.powi(5)
                    + 6e-4 * a0 * a0 * a1
                    + 1e-4 * a0 * a1 * a1
                    + 3e-6 * a1.powi(3);
                // TODO: scale error by arc length
                if err * dt <= thresh {
                    let p1 = if t1 == 1.0 { this.p1 } else { this.eval(t1) };

                    let dp = p1 - p0;
                    // Transform to take (0, 0) - (1, 0) chord to p0 - p1.
                    let a = Affine::new([dp.x, dp.y, -dp.y, dp.x, p0.x, p0.y]);

                    // Note: it's possible to this with rotation and normalization,
                    // avoiding the trig.
                    let d_atan = chord_atan - dp.atan2();
                    let th0 = d_atan - this.params.th(t0);
                    let th1 = -d_atan + this.params.th(t1);
                    let v0 = Vec2::from_angle(th0);
                    let c0 = Point::new(0., 0.);
                    let c1 = c0 + 2. / 3. / (1. + v0.x) * v0;
                    let c3 = Point::new(1., 0.);
                    let v1 = Vec2::from_angle(-th1);
                    let c2 = c3 - 2. / 3. / (1. + v1.x) * v1;

                    // Advance subdivision parameters
                    t0_int += 1;
                    let shift = t0_int.trailing_zeros();
                    t0_int >>= shift;
                    dt *= (1 << shift) as f64;
                    p0 = p1;

                    return Some(PathEl::CurveTo(a * c1, a * c2, p1));
                }
                t0_int *= 2;
                dt *= 0.5;
            }
        })
    }

}


impl ParamCurve for EulerSeg {
    fn eval(&self, t: f64) -> Point {
        // The accuracy here is somewhat arbitrary, but should be adequate
        // for most work, and not entail loss of efficiency.
        let Point { x, y } = self.params.eval(t, 1e-9);
        let chord = self.p1 - self.p0;
        Point::new(
            self.p0.x + chord.x * x - chord.y * y,
            self.p0.y + chord.x * y + chord.y * x,
        )
    }

    fn subsegment(&self, range: std::ops::Range<f64>) -> Self {
        let p0 = self.eval(range.start);
        let p1 = self.eval(range.end);
        let dt = range.end - range.start;
        let k0 = dt * (self.params.k0 + 0.5 * (range.start + range.end - 1.0) * self.params.k1);
        let k1 = dt * dt * self.params.k1;
        let params = EulerParams::from_k0_k1(k0, k1);
        EulerSeg { p0, p1, params }
    }

    fn start(&self) -> Point {
        self.p0
    }

    fn end(&self) -> Point {
        self.p1
    }
}

impl ParamCurveArclen for EulerSeg {
    /// The arc length of the curve.
    ///
    /// Note that this implementation is fast and accurate.
    fn arclen(&self, _accuracy: f64) -> f64 {
        (self.p1 - self.p0).hypot() / self.params.chord
    }

    /// The parameter that results in the given arc length.
    ///
    /// This implementation is also fast and accurate.
    fn inv_arclen(&self, arclen: f64, _accuracy: f64) -> f64 {
        arclen * self.params.chord / (self.p1 - self.p0).hypot()
    }
}

impl ParamCurveDeriv for EulerSeg {
    type DerivResult = EulerSegDeriv;

    fn deriv(&self) -> Self::DerivResult {
        let EulerParams { k0, k1, chth, .. } = self.params;
        EulerSegDeriv {
            c0: 0.5 * k0 - 0.125 * k1 + chth + (self.p1 - self.p0).atan2(),
            c1: -k0 + 0.5 * k1,
            c2: -0.5 * k1,
            scale: self.arclen(0.0),
        }
    }
}

impl ParamCurveCurvature for EulerSeg {
    fn curvature(&self, t: f64) -> f64 {
        (self.params.k0 + (t - 0.5) * self.params.k1) * self.params.chord
            / (self.p1 - self.p0).hypot()
    }
}

impl ParamCurve for EulerSegDeriv {
    fn eval(&self, t: f64) -> Point {
        let theta = self.c0 + t * self.c1 + t * t * self.c2;
        (self.scale * Vec2::from_angle(theta)).to_point()
    }

    fn subsegment(&self, range: std::ops::Range<f64>) -> Self {
        let t0 = range.start;
        let t1 = range.end;
        let dt = t1 - t0;
        EulerSegDeriv {
            c0: self.c0 + t0 * self.c1 + t0 * t0 * self.c2,
            c1: dt * (self.c1 + t0 * self.c2),
            c2: dt * dt * self.c2,
            scale: dt * self.scale,
        }
    }
}

impl ParamCurveDeriv for EulerSegDeriv {
    type DerivResult = EulerSegDeriv2;

    fn deriv(&self) -> Self::DerivResult {
        EulerSegDeriv2(*self)
    }
}

impl ParamCurve for EulerSegDeriv2 {
    fn eval(&self, t: f64) -> Point {
        let p = self.0.eval(t);
        let scale = self.0.c1 + 2.0 * t * self.0.c2;
        Point::new(-p.y * scale, p.x * scale)
    }

    fn subsegment(&self, range: std::ops::Range<f64>) -> Self {
        EulerSegDeriv2(self.0.subsegment(range))
    }
}

// TODO: other ParamCurve traits.

impl From<Line> for EulerSeg {
    fn from(l: Line) -> EulerSeg {
        EulerSeg {
            p0: l.p0,
            p1: l.p1,
            params: EulerParams {
                k0: 0.,
                k1: 0.,
                chord: 1.,
                chth: 0.,
            },
        }
    }
}
