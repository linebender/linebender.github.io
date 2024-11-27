+++
title = "Color resources"
+++
This page is an annotated list of resources about color.

## Libraries

* Rust crates
  + [Color](https://crates.io/crates/color) is the Linebender crate for color
  + [palette](https://crates.io/crates/palette) - does more color science stuff
  + [kolor](https://crates.io/crates/kolor) - color conversion crate with minimal API
  + [bevy_color](https://docs.rs/bevy_color/latest/bevy_color/) - color representations in [Bevy](https://bevyengine.org/)
  + [colstodion](https://github.com/fu5ha/colstodian) - practical, opinionated color management library
* [color.js](https://github.com/color-js/color.js) is co-developed with CSS Color, can be considered authoritative
* [Colour](https://www.colour-science.org/) for Python - quite comprehensive
* [OpenColorIO](https://opencolorio.org/) - widely used in motion pictures
* [Nanocolor](https://github.com/meshula/Nanocolor) - lightweight library (motion picture / XR focus)

## Standards

* [CSS Color Level 4](https://www.w3.org/TR/css-color-4/) 
  + Note that the [editor's draft](https://drafts.csswg.org/css-color-4/) may be more up to date
  + There's also [Color Level 5](https://www.w3.org/TR/css-color-5/) (cmyk & relative colors)
  + [Color Level 6](https://drafts.csswg.org/css-color-6/) - contrast-color and color-layers
  + [Color HDR](https://drafts.csswg.org/css-color-hdr/)

## Color spaces

* [Oklab](https://bottosson.github.io/posts/oklab/) by Björn Ottosson
  + [Raph's review](https://raphlinus.github.io/color/2021/01/18/oklab-critique.html)
  + [Okhsv and Okhsl](https://bottosson.github.io/posts/colorpicker/) - color spaces for color picking
* [HWB–A More Intuitive Hue-Based Color Model](http://alvyray.com/Papers/CG/HWB_JGTv208.pdf) (PDF)
* [ICtCp color space](https://en.wikipedia.org/wiki/ICtCp) (Wikipedia)
* [Jzazbz color space](https://observablehq.com/@jrus/jzazbz) - Observable notebook by Jacob Rus
* [sRGB](https://en.wikipedia.org/wiki/SRGB) (Wikipedia)
* [DeviceN Color Space and Color Model](https://www.setec.org/~izaac/postscriptbooks/partners.adobe.com/asn/developer/pdfs/tn/TN5604.DeviceN_Color.pdf) - (PDF, Adobe)

## ICC profiles

* [International Color Consortium](https://www.color.org/)
* [Argyll Color Management System](https://www.argyllcms.com/) - see also [qcms](https://github.com/FirefoxGraphics/qcms) which is a Rust port
* [about icc colour profiles](https://www.colourmanagement.net/advice/about-icc-colour-profiles) by Neil Barstow

## Tone mapping

* [ACES Central](https://acescentral.com/)
* [Tone Mapping Considerations for Physically-Based Rendering](https://modelviewer.dev/examples/tone-mapping)
* [Tone Mapping](https://64.github.io/tonemapping/) by Matt Taylor

## Pedagogical work

* [Color Spaces](https://ciechanow.ski/color-spaces/) from Bartosz Ciechanowski
* [handprint watercolors](https://www.handprint.com/HP/WCL/water.html) - art focus but deep [color theory](https://www.handprint.com/HP/WCL/wcolor.html)
  + Bruce McEvoy *was* working on a book called "[Color Experience](https://web.archive.org/web/20240319075344/http://handprint.com/CE/book.html)" but it seems to have been taken down
* [A Better Default Colormap for Matplotlib](https://www.youtube.com/watch?v=xAoljeRJ3lU) (video) - basis for viridis color scales
* [Poynton's Color FAQ](https://poynton.ca/ColorFAQ.html) - also see related [Gamma FAQ](https://poynton.ca/GammaFAQ.html)
  + [Guided Tour of Color Space](https://poynton.ca/PDFs/Guided_tour.pdf) by Poynton is also good

## Chromatic adaptation

* [Chromatic adaptation](http://www.brucelindbloom.com/index.html?Eqn_ChromAdapt.html) by Bruce Lindbloom
* [color.js documentation](https://colorjs.io/docs/adaptation)
* [Chromatic adaptation](https://cran.r-project.org/web/packages/spacesXYZ/vignettes/adaptation.html) by Glenn Davis (mathematical vignette)

## Color systems

* [Pantone](https://www.pantone.com/) is widely used in commercial applications
* [Freiefarbe](https://freiefarbe.de/en/) is a free alternative based on CIE HLC
* [Munsell color system](https://en.wikipedia.org/wiki/Munsell_color_system) (Wikipedia) - the great historical color system

## Color appearance models

* [Algorithmic improvements for the CIECAM02 and CAM16 color appearance models](https://arxiv.org/abs/1802.06067)
* [The non-Riemannian nature of perceptual color space](https://www.pnas.org/doi/10.1073/pnas.2119753119)

## Other color management resources

* [color-and-Color management and HDR documentation for FOSS graphics](https://gitlab.freedesktop.org/pq/color-and-hdr) - freedesktop
* [CG Cinematography book](https://chrisbrejon.com/cg-cinematography/) by Chris Brejon
  + [Chapter 1: Color management](https://chrisbrejon.com/cg-cinematography/chapter-1-color-management/)
  + [Chapter 1.5: Academy Color Encoding System (ACES)](https://chrisbrejon.com/cg-cinematography/chapter-1-5-academy-color-encoding-system-aces/)
  + [Chapter 2: Color theory](https://chrisbrejon.com/cg-cinematography/chapter-2-color-theory/)