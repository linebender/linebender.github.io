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
        svg::{g, svg},
    },
    interfaces::{Element, HtmlInputElement, SvgGeometryElement, SvgPathElement},
    svg::{
        kurbo::{Affine, BezPath, Shape, Stroke},
        peniko::color::palette::css,
    },
};

use crate::{
    AppState,
    squircle::{Squircle, Squircles, Superellipse, render_profile},
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

/// Rounds `value` up to the next 1, 2 or 5 times a power of ten.
///
/// Used for the arc length axis, which varies only between about 1.6 and 2 and
/// so settles on one value in practice.
fn nice_ceil(value: f64) -> f64 {
    if !(value > 0.0) || !value.is_finite() {
        return 1.0;
    }
    let magnitude = 10f64.powf(value.log10().floor());
    let normalized = value / magnitude;
    let step = if normalized <= 1.0 {
        1.0
    } else if normalized <= 2.0 {
        2.0
    } else if normalized <= 5.0 {
        5.0
    } else {
        10.0
    };
    step * magnitude
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
            .attr("name", "squircle-choice")
            .checked(state.choice == choice)
            .on_input(move |state: &mut AppState, _| state.choice = choice),
        choice.name(),
    ))
    .class("squircle-choice")
}

/// The radio group, zoom toggle and gauge slider.
fn controls(state: &AppState) -> impl DomView<AppState> + use<> {
    let gauge = state.gauge;

    let choices = div((
        span("Construction").class("squircle-control-label"),
        choice_radio(state, Squircles::Superellipse),
        choice_radio(state, Squircles::ChromiumApprox),
        choice_radio(state, Squircles::Clothoid),
        choice_radio(state, Squircles::Figma),
    ))
    .class("squircle-choices");

    let zoom = label((
        input(())
            .type_("checkbox")
            .checked(state.zoom)
            .on_input(|state: &mut AppState, _| state.zoom = !state.zoom),
        "Zoom to a single corner",
    ))
    .class("squircle-choice");

    let slider = div((
        label("Gauge")
            .attr("for", "squircle-gauge")
            .class("squircle-control-label"),
        input(())
            .attr("id", "squircle-gauge")
            .type_("range")
            .attr("min", "707")
            .attr("max", "999")
            .attr("step", "1")
            .attr("value", (gauge * 1000.0).round() as i32)
            .on_input(|state: &mut AppState, event| {
                if let Some(value) = input_event_target_value(&event) {
                    if let Ok(parsed) = value.parse::<f64>() {
                        state.gauge = parsed * 1e-3;
                    }
                }
            }),
        span(format!("{gauge:.3}")).class("squircle-readout"),
    ))
    .class("squircle-slider");

    div((choices, zoom, slider)).class("squircle-controls")
}

/// The shape panel: the selected construction over a superellipse reference.
fn shape_panel(state: &AppState) -> impl DomView<AppState> + use<> {
    let params = [state.gauge];
    let mut shape = state.choice.render(&params);
    let mut reference = Superellipse.render(&params);
    if !state.zoom {
        shape = quadruple_up(&shape);
        reference = quadruple_up(&reference);
    }

    let transform = shape_transform();
    let drawing = svg(g((
        (transform * reference)
            .stroke(css::DARK_ORANGE, Stroke::new(1.5))
            .fill(css::TRANSPARENT)
            .class(["squircle-path", "squircle-path--reference"]),
        (transform * shape)
            .stroke(css::STEEL_BLUE, Stroke::new(2.0))
            .fill(css::TRANSPARENT)
            .class(["squircle-path", "squircle-path--shape"]),
    )))
    .attr("viewBox", format!("0 0 {SHAPE_VIEW} {SHAPE_VIEW}"))
    .class("squircle-figure")
    .attr("role", "img")
    .attr(
        "aria-label",
        "The selected squircle construction drawn over a superellipse of the same gauge",
    );

    let key = div((
        span("Selected construction")
            .class(["squircle-key", "squircle-key--shape"]),
        span("Superellipse reference")
            .class(["squircle-key", "squircle-key--reference"]),
    ))
    .class("squircle-legend");

    div((drawing, key)).class("squircle-panel")
}

/// The curvature panel: curvature against arc length along one quadrant.
fn curvature_panel(state: &AppState) -> impl DomView<AppState> + use<> {
    let params = [state.gauge];
    let profile = render_profile(&state.choice.curvature_profile(&params));

    // The profile is a polyline in (arc length, curvature) space, so its
    // bounding box is exactly the data range the axes have to cover.
    let bounds = profile.bounding_box();
    let s_max = nice_ceil(bounds.x1);
    let k_max = curvature_axis_max(bounds.y1);
    let overflows = bounds.y1 > k_max;
    let transform = plot_transform(s_max, k_max);

    let drawing = svg(g((
        (transform * grid_path(s_max, k_max))
            .stroke(css::GAINSBORO, Stroke::new(1.0))
            .fill(css::TRANSPARENT)
            .class("squircle-grid"),
        (transform * axes_path(s_max, k_max))
            .stroke(css::GRAY, Stroke::new(1.5))
            .fill(css::TRANSPARENT)
            .class("squircle-axes"),
        (transform * profile)
            .stroke(css::STEEL_BLUE, Stroke::new(2.0))
            .fill(css::TRANSPARENT)
            .class(["squircle-path", "squircle-path--curvature"]),
    )))
    .attr("viewBox", format!("0 0 {PLOT_W} {PLOT_H}"))
    .class("squircle-figure")
    .attr("role", "img")
    .attr(
        "aria-label",
        "Curvature plotted against arc length along one quadrant",
    );

    let overflow_note = if overflows {
        format!(" The peak of {:.0} runs off the top.", bounds.y1)
    } else {
        String::new()
    };
    let caption = div(format!(
        "Curvature, 0 to {k_max}, against arc length along one quadrant, 0 to {s_max}. \
         Gridlines divide each axis into {PLOT_DIVISIONS} equal parts.{overflow_note}"
    ))
    .class("squircle-caption");

    div((drawing, caption)).class("squircle-panel")
}

/// Top-level view.
pub(crate) fn app_logic(state: &mut AppState) -> impl DomView<AppState> + use<> {
    div((
        controls(state),
        div((shape_panel(state), curvature_panel(state)))
            .class("squircle-panels"),
    ))
    .class("squircle-demo")
}
