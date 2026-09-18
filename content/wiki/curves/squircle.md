+++
title = "Squircles"
+++

A squircle is a rounded rectangle whose corners blend into the straight edges without a visible seam.
A conventional rounded rectangle joins a circular arc to a line, which leaves curvature jumping from $1/r$ to zero at the join.
The eye picks that discontinuity up as a crease, so a number of different constructions have been proposed to smooth it out.

This page compares four of them.
They are close enough in shape to be hard to tell apart directly, so the tester below also plots curvature against arc length, where the differences are obvious.

## The gauge parameter

All four constructions are controlled here by a single shape parameter, the *superellipse gauge*.
Working in a unit square, with the shape inscribed so that it touches $(1, 0)$ and $(0, 1)$, the gauge $g$ is the coordinate at which the shape crosses the diagonal: the curve passes through $(g, g)$.
A circle gives $g = 1/\sqrt{2} \approx 0.707$ and a square gives $g = 1$.

This is the same quantity as the "Superellipse" column in [Curve families](@/wiki/curve_families.md), which makes it a convenient common axis for comparing families that are otherwise parameterized quite differently.

<div id="squircle-demo-root">
  <p class="squircle-fallback">Loading the interactive tester…</p>
</div>

<link rel="stylesheet" href="/rust/squircle-demo.css">

<script type="module">
  const root = document.getElementById("squircle-demo-root");
  try {
    const { default: init, start } = await import("/rust/squircle.js");
    await init();
    root.replaceChildren();
    start("squircle-demo-root");
  } catch (err) {
    console.error(err);
    root.replaceChildren(
      Object.assign(document.createElement("p"), {
        className: "squircle-fallback",
        textContent:
          "The interactive tester could not be loaded. It needs JavaScript and WebAssembly.",
      }),
    );
  }
</script>

## Superellipse

TODO: the defining equation, the relation between the exponent and the gauge, and why the curvature goes to zero at the axes.

## Chromium approximation

TODO: the two-cubic fit from [Implementing corner-shape], and how closely it tracks the true superellipse.

## Clothoid

TODO: a clothoid in from the edge, a circular arc through the corner, a clothoid back out, with the split between them as the smoothness parameter.

## Figma

TODO: the corner-smoothing construction, and how it differs from both of the above.

[Implementing corner-shape]: https://developer.chrome.com/blog/implementing-corner-shape
