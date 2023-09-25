+++
title = "Curve families"
+++


| Curve                | Params | Subdiv | Affine | Inflections | Superellipse |
| -------------------- | ------ | ------ | ------ | ----------- | ------------ | 
| Line                 |      0 | yes    | yes    |           0 |          0.5 |
| Arc                  |      1 | yes    | no     |           0 |        0.785 |
| Parabola             |      2 | yes    | yes    |           0 |        0.833 |
| Euler spiral         |      2 | yes    | no     |           1 |        0.785 |
| Rectangular elastica |      2 | yes    | no     |           1 |       ~0.8   |
| Elastica             |      3 | yes    | no     |          1* |   0.5..~0.85 |
| Balanced Bézier      |      3 | no     | no     |           2 |    0.5..0.95 |
| Euler spiral PC      |      3 | yes    | no     |           1 |        0.785 |
| Conic                |      3 | yes    | yes    |           0 |       0.5..1 |
| Cubic Bézier         |      4 | yes    | yes    |           2 |    0.5..0.95 |
| Spiro                |      4 | yes    | no     |           3 |     TODO     |
| Hyperbezier          |      4 | yes    | no     |           1 |       0.5..1 |
| Quintic Bézier       |      8 | yes    | yes    |          4? |   0.8..0.996 |

## Relationship graph

![graph of relations](/curve_families_graph.svg)

This graph has an arrow when one curve family fully includes the parameter space of another. A dotted line represents an approximation which is better than can be expected from scaling based on the number of parameters.

## Properties

### Number of parameters

We use the same parameter counting policy as [Raph's thesis] (section 2.8). The endpoints are considered to be fixed, so to use in an actual application, there is an additional transform consisting of translation, rotation, and scaling. This additional transformation is the same for all curves.

The number of parameters is a tradeoff. To represent an arbitrary curve, with a small number of parameters more subdivisions are needed. Increasing the number of parameters has a cost, for representing the parameter values, presenting them in a UI for humans to manipulate, and for doing computations. Generally, higher degrees of continuity requires more parameters.

In general, the accuracy of curve fitting scales with an exponent equal to the number of parameters plus 2. For example, fitting with a quadratic Bézier or Euler spiral has O(n^4) scaling, meaning that subdivision in half improves accuracy by a factor of 16. That said, actually achieving optimal curve fitting may be challenging; I'm 

### Closure under subdivision

If a curve is subdivided at an arbitrary point, will the two subdivisions also be members of the curve family?

Two-parameter curve families closed under subdivision are all derived by taking a segment of a fixed generating curve (section 4.1 of [Raph's thesis]). Thus, such families can be characterized entirely by the generating curve (parabola, Euler spiral, rectangular elastica). More or less any generating curve can work, and the properties of the generating curve (monotone curvature, periodicity, presence of an inflection point) map to the properties of the curve family. Section 4.8 of Raph's thesis explores this space somewhat.

### Closure under affine transformation

When an arbitrary affine transformation is applied to a curve, is that a member of the curve family?

As a general pattern, Bézier curves are closed under affine, including the rational variants, but spirals and curves defined variationally are not. However, it is certainly possible to mix the two. The affine closure of Euler spirals is a potentially interesting curve family.

### Inflections

An S-shaped curve has an inflection point. Some curve families only contain convex curves, while others can represent inflections. Arguably, having too many inflection points is a drawback, as it is not intuitive to control the curve.

### Superellipses

[Superellipse]-like shapes are very important in graphic design. The ability of a curve family to approximate a superellipse varies widely. As a general principle, curves defined variationally do not easily form superellipses, those with a rational polynomial at their core can represent the entire range, and Béziers are in the middle (they're reasonably good at representing arbitrary curves, but have a limit, beyond which the curve develops unwanted additional inflections).

To quantify this more precisely, we define the *superellipticity range* of a curve family as follows. Set both endpoint tangents to π/4 (45°), so the curve covers a quarter-circle of arc. Use the curve to define one quadrant of a superellipse shape, with an extent of 1 on both x and y axes. Compute the area of the resulting quadrant. For a circle, it is π/4 (≈0.785), and for a square it is 1. As the remaining parameters are varied, what is the range of areas that can result? The range is further constrained by the curve having no inflection points (though curvature is allowed to reach zero at the endpoints) and being symmetrical, as these are visually important features of a real superellipse.

Note that a two parameter curve family has a single value, as there are no additional parameters to vary after the endpoint tangents are set.

## The curves

Here we mostly point to a reference for the curve family. Each section could be expanded greatly, perhaps by giving the curve family its own page.

### Line segment

A [line segment] has zero curvature. A polyline has G0 continuity only.

### Circular arc

A circle has constant curvature. A [circular arc] is a segment of a circle.

In general, circular arcs can only achieve G0 continuity, though there are techniques to achieve G1 by applying additional constraints, notably [biarcs][biarc].

They are popular in CAD, partly because they are the only widely implemented curve family in [G-code].

### Quadratic Bézier (parabola)

A quadratic Bézier is a segment of a parabola.

Quadratic Béziers are in widespread use as the curve family of [TrueType] fonts (and thus TrueType-flavored OpenType).

### Euler spiral

An [Euler spiral] has curvature linear with arc length. It is extensively discussed in section 4.6 of [Raph's thesis]. It is an especially good choice for making a G2 continuous interpolating spline (for reasons discussed in the thesis), but a weakness is representing curves with significant curvature variation.

Another benefit of the Euler spiral is that its parallel curve is mathematically tractable to compute. See [Cleaner parallel curves with Euler spirals].

### Rectangular elastica

The minimum energy curve (MEC) is the curve minimizing the total bending energy when constrained to pass through the given control points, so is a mathematical idealization of a physical spline made out of a thin strip of flexible material. Each segment is a segment of the "rectangular elastica", which is a particular solution of the general elastica. It is extensively discussed in section 4.2 of [Raph's thesis].

### Elastica

The general elastica has an additional parameter, which corresponds to *tension,* resulting in a three-parameter curve. It has a narrow superelliptical range. In addition, it is symmetrical around the inflection point, which means it cannot model an asymmetrical S-shape. More precisely, when the endpoint tangents are anti-symmetrical, the area is zero for all values of the tension parameter. It is rarely used directly, but because of its physical importance, it is worth evaluating other curve families in their ability to accurately approximate an elastica.

Probably worth noting here, the SI-MEC is a two parameter curve family which is *not* closed under subdivision. It is analyzed in section 3.11 of [Raph's thesis], with the conclusion being that the Euler spiral is superior in most cases.

A definitive treatment of the elastica is in chapter 3 of [Raph's thesis].

### Balanced Bézier

A "balanced Bézier" is work in progress to restrict the parameter space of cubic Béziers to those with a smooth parametrization. See the [Sweet Béziers](https://xi.zulipchat.com/#narrow/stream/260979-kurbo/topic/Sweet.20B.C3.A9ziers) Zulip thread for more information.

It approximates circular arcs with O(n^6) accuracy.

### Rational quadratic Bézier

A rational quadratic Bézier is a segment of a [conic section]. It is fairly widely used in CAD, less so in graphic design applications. It cannot represent an inflection point, but does have the full range of superellipticity.

An additional superpower is that it is closed under perspective transformation.

### Cubic Bézier

A cubic Bézier is by far the most common curve representation. It can accurately represent an extremely wide range of curves, and also has a control scheme suitable for direct manipulation in a UI. See [A Primer on Bézier Curves] for more information.

Curve-fitting is challenging; no good general technique was known until Raph's [Bézier curve fitting] blog posts. In general, it requires solution of a quartic equation, and the process is discontinuous; there exist input curves extremely close to each other with very different best fits.

### Spiro

The Spiro curve family was an attempt to create a tool more suitable for interactive curve editing. It is defined as the curvature being a cubic function of arc length. It is the basis of a practical G4-continuous interpolating spline, and is an excellent choice when high degrees of continuity are required. One weakness is the limited superellipticity range. It is discussed in section 7.3 of [Raph's thesis], and has had some limited implementation success.

### Hyperbezier

The hyperbezier is defined as $$\kappa(s) = (as+b)/(cs^2+ds+e)^2$$. There are four parameters as multiplying all five coefficients by a constant does not change the result.

Development of this curve family is work in progress. Two Zulip threads are [Hyperbezier progress](https://xi.zulipchat.com/#narrow/stream/260979-kurbo/topic/Hyperbezier.20progress) and [Hyperbezier parameter mapping](https://xi.zulipchat.com/#narrow/stream/260979-kurbo/topic/Hyperbezier.20parameter.20mapping).

An earlier draft is presented in [The hyperbezier pen tool] and is implemented in the [spline] library. That draft had some weaknesses (not closed under subdivision, limited superellipticity range), so will be superseded by the new version.

### Quintic Bézier

The quintic Bézier pushes the number of parameters into unmanageable territory. It is not clear that there is any practical way to achieve the theoretical O(n^10) curve fitting accuracy; from experience with cubics it is clear that it is a highly nonlinear optimization problem with many local minima. That said, it does have some uses.

[Raph's thesis]: https://levien.com/phd/phd.html
[Superellipse]: https://en.wikipedia.org/wiki/Superellipse
[G-code]: https://en.wikipedia.org/wiki/G-code
[Circular arc]: https://en.wikipedia.org/wiki/Circular_arc
[Line segment]: https://en.wikipedia.org/wiki/Line_segment
[biarc]: https://en.wikipedia.org/wiki/Biarc
[TrueType]: https://en.wikipedia.org/wiki/TrueType
[Euler spiral]: https://en.wikipedia.org/wiki/Euler_spiral
[Cleaner parallel curves with Euler spirals]: https://raphlinus.github.io/curves/2021/02/19/parallel-curves.html
[conic section]: https://en.wikipedia.org/wiki/Conic_section
[A Primer on Bézier Curves]: https://pomax.github.io/bezierinfo/
[Fitting cubic Bézier curves]: https://raphlinus.github.io/curves/2021/03/11/bezier-fitting.html
[The hyperbezier pen tool]: https://www.cmyr.net/blog/hyperbezier.html
[spline]: https://github.com/linebender/spline
