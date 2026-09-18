// Copyright 2026 the Linebender Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The view layer for the squircle tester.
//!
//! Two panels are drawn side by side: the shape itself, and the curvature
//! profile of one quadrant. Both are plain SVG with a `viewBox` and no
//! intrinsic size, so the page stylesheet controls how large they are and they
//! reflow on narrow screens.
//!
//! Every stroked path carries a CSS class. Stroke colors set here are only
//! fallbacks for when the stylesheet is missing; `squircle-demo.css` overrides
//! them per theme.

use xilem_web::{
    DomView, input_event_target_value,
    elements::{
        html::{div, input, label, span},
        svg::{g, svg, text},
    },
    interfaces::{Element, HtmlInputElement, SvgGeometryElement, SvgPathElement},
    svg::{
        kurbo::{Affine, BezPath, Point, Shape, Stroke},
        peniko::color::palette::css,
    },
};

use crate::{
    AppState,
    squircle::{
        Corner, GAUGE_MAX, GAUGE_MIN, Squircles, quadrant, quadrant_profile, render_profile,
        superellipse_exponent,
    },
};

/// Side of the square `viewBox` the shape is drawn into.
const SHAPE_VIEW: f64 = 560.0;
/// Margin between the shape and the edge of its `viewBox`.
const SHAPE_MARGIN: f64 = 40.0;

/// Width of the curvature plot's `viewBox`.
const PLOT_W: f64 = 560.0;
/// Height of the curvature plot's `viewBox`.
const PLOT_H: f64 = 300.0;
/// Number of gridline divisions along each axis of the curvature plot.
const PLOT_DIVISIONS: usize = 4;

/// Gap between a curvature label and the axis, in `viewBox` units.
const LABEL_GAP: f64 = 8.0;

/// Labels for the curvature axis, one per gridline.
fn curvature_labels(transform: Affine, k_max: f64) -> impl DomView<AppState> + use<> {
    let labels: Vec<_> = (1..=PLOT_DIVISIONS)
        .map(|i| {
            let k = k_max * i as f64 / PLOT_DIVISIONS as f64;
            let at = transform * Point::new(0.0, k);
            let shown = if k.fract() == 0.0 {
                format!("{k:.0}")
            } else {
                format!("{k:.1}")
            };
            text(shown)
                .attr("x", format!("{:.1}", at.x - LABEL_GAP))
                .attr("y", format!("{:.1}", at.y + 4.0))
                .class("squircle-axis-label")
        })
        .collect();
    g(labels)
}

/// A tick up from the arc length axis at `s_end`, where the curve stops.
///
/// In the axis' style, since it reports where the quadrant ends rather than
/// anything about the profile, and the curve's trailing flat runs along the
/// axis. In `viewBox` units so it keeps one size as the curvature axis
/// rescales; up only, since curvature is never negative here.
fn end_tick(transform: Affine, s_end: f64) -> BezPath {
    let foot = transform * Point::new(s_end, 0.0);
    let mut path = BezPath::new();
    path.move_to((foot.x, foot.y));
    path.line_to((foot.x, foot.y - END_TICK));
    path
}

/// Maximum of the curvature plot's arc length axis.
///
/// A quadrant is at most 2 long, reached only by the degenerate square corner.
/// Absolute rather than fitted to the profile, whose length varies with both
/// sliders: the axis would otherwise rescale mid-drag, and the flats would read
/// as a share of a moving total rather than at their true length.
const PLOT_S_MAX: f64 = 2.0;

/// Length of the tick marking where the curve ends, in `viewBox` units.
const END_TICK: f64 = 6.0;

/// Steps the curvature axis can take.
///
/// Curvature runs from about 1 at the circular end of the gauge range to
/// several hundred as the shape approaches a square, so no single fixed axis
/// serves the whole slider. Three steps cover it while rescaling only twice, so
/// the plot is still for most of a drag rather than resizing under the cursor.
/// Past the top step the curve is left to run off the plot: stretching the axis
/// to a peak in the hundreds would flatten everything below it onto the
/// baseline, which hides the shape of the profile that the plot exists to show.
const CURVATURE_STEPS: [f64; 3] = [2.0, 20.0, 200.0];

/// The smallest [`CURVATURE_STEPS`] entry that contains `peak`, else the largest.
fn curvature_axis_max(peak: f64) -> f64 {
    if !peak.is_finite() {
        return CURVATURE_STEPS[0];
    }
    for step in CURVATURE_STEPS {
        if peak <= step {
            return step;
        }
    }
    CURVATURE_STEPS[CURVATURE_STEPS.len() - 1]
}

/// Maps the unit square the shapes are defined in onto the shape `viewBox`.
fn shape_transform() -> Affine {
    let scale = SHAPE_VIEW - 2.0 * SHAPE_MARGIN;
    Affine::translate((SHAPE_MARGIN, SHAPE_MARGIN)) * Affine::scale(scale)
}

/// Maps (arc length, curvature) onto the curvature plot's `viewBox`.
///
/// The vertical scale is negated because SVG's y axis points down.
fn plot_transform(s_max: f64, k_max: f64) -> Affine {
    let left = 56.0;
    let right = PLOT_W - 24.0;
    let top = 28.0;
    let bottom = PLOT_H - 48.0;
    Affine::new([
        (right - left) / s_max,
        0.0,
        0.0,
        -(bottom - top) / k_max,
        left,
        bottom,
    ])
}

/// The two axis lines of the curvature plot, in (arc length, curvature) space.
fn axes_path(s_max: f64, k_max: f64) -> BezPath {
    let mut path = BezPath::new();
    path.move_to((0.0, k_max));
    path.line_to((0.0, 0.0));
    path.line_to((s_max, 0.0));
    path
}

/// Gridlines dividing each axis into [`PLOT_DIVISIONS`] equal parts.
fn grid_path(s_max: f64, k_max: f64) -> BezPath {
    let mut path = BezPath::new();
    for i in 1..=PLOT_DIVISIONS {
        let fraction = i as f64 / PLOT_DIVISIONS as f64;
        path.move_to((s_max * fraction, 0.0));
        path.line_to((s_max * fraction, k_max));
        path.move_to((0.0, k_max * fraction));
        path.line_to((s_max, k_max * fraction));
    }
    path
}

/// Replicates one quadrant into the full four-fold shape.
///
/// The quadrant occupies the unit square, so each copy is scaled by a half and
/// rotated into place; the result again occupies the unit square. The first
/// element of each copy but the first is skipped, because it repeats the point
/// the previous copy ended on.
fn quadruple_up(path: &BezPath) -> BezPath {
    const AFFS: [Affine; 4] = [
        Affine::new([0.5, 0., 0., 0.5, 0.5, 0.5]),
        Affine::new([0.0, 0.5, -0.5, 0.0, 0.5, 0.5]),
        Affine::new([-0.5, 0., 0., -0.5, 0.5, 0.5]),
        Affine::new([0.0, -0.5, 0.5, 0.0, 0.5, 0.5]),
    ];
    let mut result = BezPath::new();
    for (i, aff) in AFFS.iter().enumerate() {
        result.extend(
            path.elements()
                .iter()
                .skip((i > 0) as usize)
                .map(|el| *aff * *el),
        );
    }
    result.close_path();
    result
}

/// One labelled radio button selecting a construction.
fn choice_radio(state: &AppState, choice: Squircles) -> impl DomView<AppState> + use<> {
    label((
        input(())
            .type_("radio")
            .name("squircle-choice")
            .checked(state.choice == choice)
            .on_input(move |state: &mut AppState, _| state.choice = choice),
        choice.name(),
    ))
    .class("squircle-choice")
}

/// One labelled range slider.
fn slider<F: Fn(&mut AppState, f64) + 'static>(
    id: &'static str,
    caption: &'static str,
    value: f64,
    readout: String,
    disabled: bool,
    on_change: F,
) -> impl DomView<AppState> + use<F> {
    div((
        label(caption)
            .attr("for", id)
            .class("squircle-control-label"),
        input(())
            .attr("id", id)
            .type_("range")
            .attr("min", "0")
            .attr("max", "1000")
            .attr("step", "1")
            .attr("value", (value * 1000.0).round() as i32)
            .disabled(disabled)
            .on_input(move |state: &mut AppState, event| {
                if let Some(raw) = input_event_target_value(&event) {
                    if let Ok(parsed) = raw.parse::<f64>() {
                        on_change(state, parsed * 1e-3);
                    }
                }
            }),
        span(readout).class("squircle-readout"),
    ))
    .class("squircle-slider")
}

/// One label-and-value row, for a number the tester reports but cannot set.
fn readout_row(
    caption: &'static str,
    value: String,
    muted: bool,
) -> impl DomView<AppState> + use<> {
    let tone = if muted { "squircle-muted" } else { "squircle-live" };
    div((
        span(caption).class(["squircle-control-label", tone]),
        span(value).class(["squircle-readout", tone]),
    ))
    .class("squircle-slider")
}

/// The control row above the panels.
fn controls(state: &AppState, corner: Corner) -> impl DomView<AppState> + use<> {
    // Apple's corner is fixed, so its flat follows from the gauge.
    let flat_fixed = state.choice.fixed_corner().is_some();

    let choices = div((
        choice_radio(state, Squircles::Superellipse),
        choice_radio(state, Squircles::ChromiumApprox),
        choice_radio(state, Squircles::Clothoid),
        choice_radio(state, Squircles::Figma),
        choice_radio(state, Squircles::Apple),
    ))
    .class("squircle-choices")
    .attr("role", "radiogroup")
    .attr("aria-label", "Variant");

    let zoom = label((
        input(())
            .type_("checkbox")
            .checked(state.zoom)
            .on_input(|state: &mut AppState, _| state.zoom = !state.zoom),
        "Zoom to a single corner",
    ))
    .class("squircle-choice");

    let fill = label((
        input(())
            .type_("checkbox")
            .checked(state.fill)
            .on_input(|state: &mut AppState, _| state.fill = !state.fill),
        "Fill",
    ))
    .class("squircle-choice");

    // The gauge slider spans [GAUGE_MIN, GAUGE_MAX]; the helper takes 0..1.
    let gauge_span = GAUGE_MAX - GAUGE_MIN;
    let gauge_slider = slider(
        "squircle-gauge",
        "Gauge",
        (state.gauge - GAUGE_MIN) / gauge_span,
        format!("{:.3}", state.gauge),
        false,
        move |state, t| state.gauge = GAUGE_MIN + t * gauge_span,
    );

    // h runs 0..h_max as the gauge runs c..GAUGE_MAX, when the corner is fixed.
    let flat_position = if flat_fixed {
        ((state.gauge - corner.c) / (GAUGE_MAX - corner.c)).clamp(0.0, 1.0)
    } else {
        state.flat
    };
    let flat_slider = slider(
        "squircle-flat",
        "Flat",
        flat_position,
        format!("{:.3}", corner.h),
        flat_fixed,
        |state, t| state.flat = t,
    );

    let has_exponent = state.choice.has_exponent();
    let exponent = readout_row(
        "Exponent",
        if has_exponent {
            format!("{:.3}", superellipse_exponent(corner.c))
        } else {
            "\u{2014}".to_string()
        },
        !has_exponent,
    );

    div((choices, zoom, fill, gauge_slider, flat_slider, exponent)).class("squircle-controls")
}

/// The shape panel.
fn shape_panel(state: &AppState, corner: Corner) -> impl DomView<AppState> + use<> {
    let mut shape = quadrant(state.choice, corner);
    if state.zoom {
        // A lone quadrant is open, so filling it needs the two radii.
        if state.fill {
            shape.line_to((0.0, 0.0));
            shape.close_path();
        }
    } else {
        shape = quadruple_up(&shape);
    }

    let (mode, stroke, fill) = if state.fill {
        ("squircle-path--fill", css::TRANSPARENT, css::STEEL_BLUE)
    } else {
        ("squircle-path--shape", css::STEEL_BLUE, css::TRANSPARENT)
    };
    let drawing = svg(g((shape_transform() * shape)
        .stroke(stroke, Stroke::new(2.0))
        .fill(fill)
        .class(["squircle-path", mode])))
    .attr("viewBox", format!("0 0 {SHAPE_VIEW} {SHAPE_VIEW}"))
    .class("squircle-figure")
    .attr("role", "img")
    .attr("aria-label", "The selected squircle construction");

    div(drawing).class("squircle-panel")
}

/// The curvature panel: curvature against arc length along one quadrant.
fn curvature_panel(state: &AppState, corner: Corner) -> impl DomView<AppState> + use<> {
    let profile = render_profile(&quadrant_profile(state.choice, corner));

    // Only the curvature axis is fitted; arc length stays absolute.
    let bounds = profile.bounding_box();
    let k_max = curvature_axis_max(bounds.y1);
    // Clamped so that a degenerate profile cannot put the tick outside the plot.
    let s_end = bounds.x1.clamp(0.0, PLOT_S_MAX);
    let transform = plot_transform(PLOT_S_MAX, k_max);

    let drawing = svg(g((
        (transform * grid_path(PLOT_S_MAX, k_max))
            .stroke(css::GAINSBORO, Stroke::new(1.0))
            .fill(css::TRANSPARENT)
            .class("squircle-grid"),
        (transform * axes_path(PLOT_S_MAX, k_max))
            .stroke(css::GRAY, Stroke::new(1.5))
            .fill(css::TRANSPARENT)
            .class("squircle-axes"),
        (transform * profile)
            .stroke(css::STEEL_BLUE, Stroke::new(2.0))
            .fill(css::TRANSPARENT)
            .class(["squircle-path", "squircle-path--curvature"]),
        // Last, so it reads over the curve where the two meet.
        end_tick(transform, s_end)
            .stroke(css::GRAY, Stroke::new(1.5))
            .fill(css::TRANSPARENT)
            .class(["squircle-axes", "squircle-tick"]),
        curvature_labels(transform, k_max),
    )))
    .attr("viewBox", format!("0 0 {PLOT_W} {PLOT_H}"))
    .class("squircle-figure")
    .attr("role", "img")
    .attr(
        "aria-label",
        "Curvature plotted against arc length along one quadrant",
    );

    div(drawing).class("squircle-panel")
}

/// Top-level view.
pub(crate) fn app_logic(state: &mut AppState) -> impl DomView<AppState> + use<> {
    // Resolved once: inverting the clothoid and Figma parameters costs renders.
    let corner = Corner::resolve(state.choice, state.gauge, state.flat);
    div((
        controls(state, corner),
        div((shape_panel(state, corner), curvature_panel(state, corner)))
            .class("squircle-panels"),
    ))
    .class("squircle-demo")
}
