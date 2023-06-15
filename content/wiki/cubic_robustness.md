+++
title = "Cubic Bézier robustness"
+++
Cubic Béziers have a clean and simple definition as a parametric curve (the coordinates are a cubic polynomial in the parameter), but they can also exhibit cusps and other degenerate behavior, which makes numerically robust algorithms challenging in some cases, especially stroking. Cusps and near-cusps also reduce the accuracy of numerical integration techniques for computing arc lengths.

At heart, the robustness problems are caused by the derivative of the curve becoming or nearing zero. In the latter case, the curve is not [regular], which among other things means it may not be G1 continuous, even though the curve segment itself is C∞. Freya Holmér's [Continuity of Splines][The Continuity of Splines] video does an excellent job explaining that distinction.

This page will discuss techniques for detecting and mitigating these robustness issues.

## Three cases

It's useful to consider three different cases, as they have different consequences for mitigation. Roughly in order from easiest to hardest, they are coincident control points, simple cusp, and colinear.

### Coincident control points

This case has a control point coincident (or nearly so) with an endpoint. It is extremely easy to detect.

The tangent vector is well defined at the endpoint, thanks to L'Hopital's rule; it is equal to the vector from the control point to the other control point.

A good mitigation is to displace the control point slightly in the direction of the other control point. This has a minimal impact on the shape, 

![Illustration of cubic Bézier with coincident endpoint](/cubic_robust_coincident.svg)


### Simple cusp

The simple cusp is in some ways a generalization of the coincident control point case, but with the cusp in the interior of the curve rather than at the endpoint.

The classic simple cusp is a [semicubical parabola], which can be represented as y = x^1.5, as well as a parametric cubic curve. All cusp cases in a cubic Bézier can be represented as as affine transformations of this curve. There is always a single cusp.

Detection is in two parts. First, this case can only occur when the control polygon is self-intersecting, which can be readily computed as alternating signs of cross-products. This is rare in non-adversarial inputs. Then, either a curve classification or hodograph technique can be used to detect the near-cusp, see below.

A reasonable mitigation is to displace the control points along the tangent lines, either making the loop bigger or the inflection points farther apart, in either cases reducing the curvature.

![Illustration of cubic Bézier with simple cusp](/cubic_robust_simple.svg)


### Colinear

Like the previous two cases, the colinear case is characterized by a (nearly) vanishing derivative, but is different in a number of important respects. Detection is reasonably straightforward, as the control points are close to colinear (which can be evaluated numerically as very small cross products relative to the magnitude of dot products).

From a classification perspective, the colinear case can either an S-shaped or loop curve, subject to an extreme nonuniform scale, squashing the gentle curve into a nearly flattened form, with high curvature. Unlike the simple cusp case, there are generally two cusps.

A few more observations. The line is not necessarily (nearly) parallel with the chord, as the chord may be very short.

Mitigating this case may require a different approach than for the other cusp cases. Moving the control points along the tangent (preserving G1 continuity) may not resolve the cusp. For stroking, it is possibly best to divide the cubic at the (near) cusps and stroke each as a line, using round end caps.

![Vector image](/cubic_robust_colinear.svg)

## Bézier classification

One approach to detecting the simple cusp is Bézier classification. There's a classic paper on this, Wang 1981, which is cited by a number of later references (see relevant [Stack Overflow thread](https://stackoverflow.com/questions/72107111/classifying-cubic-bezier-curves-according-to-loop-blinn-2005)).

The core element of the classification is x''×x', which is a quadratic polynomial. This is the numerator of the equation for curvature. A "loop" shape has no inflection points, and no real solutions of this polynomial. Thus, the discriminant (b² - 4ac) is a valid classifier. That said, choosing an epsilon value for near-cusp instances is tricky. This discriminant scales as the square of area under affine transformation, which is not particularly helpful in choosing an epsilon. For one, as the line case shows, extreme affine transformation can strongly affect the sharpness of the cusp.

Even so, the sign is useful for determining the mitigation; if it's a loop, then the control points should be moved farther from the endpoints to make the loop bigger (and lower in curvature), and conversely closer if it's two closely spaced inflection points.

## Hodograph

Another approach is the hodograph, or analysis of the derivative of the cubic curve, which is a quadratic Bézier.

Determining the minimum absolute value of the derivative is equivalent to the "nearest" method, finding the point on the hodograph nearest the origin. This is a cubic solve, and exists in kurbo. That may not be the exact curvature maximum (given that the numerator also varies), but is close enough for robustness work, and calibrated in more useful units.

The loop/double inflection classification as stated above can also be geometrically interpreted in terms of the hodograph: whether the origin is inside or outside the concave part of the parabola, respectively. The math turns out equivalent.

![Illustration of three near-cusp cubics with hodograph](/cubic_robust_hodograph.svg)

## Arc lengths

*Note: this section could move to another page on arc length, with more detail. Here are just observations on cusp behavior.*

It's been noted that Gauss-Legendre quadrature has excellent accuracy for smooth curves, scaling impressively with the degree of quadrature, but that accuracy suffers in (near) cusp conditions, or that it provokes numerical stability issues in analytical techniques. [Henkel 2014] contains some experimental results, and it was also noted in the [How long is that Bézier?] blog post and [followup]. The current implementation in kurbo employs an error metric and subdivides; note from the illustrations that the near-cusp conditions (near the bottom edge of the graph) require many more subdivisions. It might be worth investigating whether explicit cusp detection, splitting at the point of maximum curvature, might require fewer subdivisions for the same accuracy, and thus be faster.

## References

* [Converting stroked primitives to filled primitives][Nehab 2020], Diego Nehab, 2020
* Shape classification of the parametric cubic curve and parametric B-spline cubic curve, C. Y. Wang, 1981
* [Calculating the Cubic Bézier Arc Length by Elliptic Integrals][Henkel 2014], Hartmut Henkel, 2014

[The Continuity of Splines]: https://youtu.be/jvPPXbo87ds
[Nehab 2020]: https://dl.acm.org/doi/10.1145/3386569.3392392
[Henkel 2014]: http://hhenkel.de/metapost/arclength.pdf
[semicubical parabola]: https://en.wikipedia.org/wiki/Semicubical_parabola
[How long is that Bézier?]: https://raphlinus.github.io/curves/2018/12/28/bezier-arclength.html
[followup]: https://raphlinus.github.io/curves/graphics/2019/01/04/followups.html#arclength
[regular]: https://en.wikipedia.org/wiki/Differentiable_curve
