+++
title = "Text"
+++

Storing, editing and displaying text is hard. As a newcomer to the world of text, I faced almost unfathomable complexity. The aim of this page is to compartmentalize the different aspects of text, to make the complexity as manageable as possible.

# Overview

# History

Although not strictly necessary to understand text rendering, a brief understanding of how words have been recorded over the millennia helps to explain some of the confusing names and conventions of modern typesetting.

 - Written text/calligraphy
 - The movable-type printing press (Gutenberg 1440)
   - Brief description of the process of casting type in a foundry, compositing with composing sticks, fixing in a forme with furniture and printing. Sorts (em en quad) Perhaps justification.
   - fi, fl, ffi, ffl ligatures (nice introduction to the fact there isn't a 1-1 correspondence between characters and glyphs)
 - Distinction between font (specific set of type, with a weight, size, etc) and typeface (family of fonts)
   - [Non-latin typefaces and letterpress printing](https://www.printweek.in/features/a-journey-through-letterpress-typefaces-41275)
 - Hot metal typesetting
 - CRT displays - an alternative to paper
 - Raster image processors, dots, PostScript, DPI?

# Text data

The usual suspects:

 - ASCII
 - Unicode
   - Graphemes, grapheme clusters, normalization, segmentation: all that good stuff
   - 
   - in particular, things like sentence break, soft line break, hard line break, character classes
   - RTL, Bidi
 - UTF-{8, 16, 32}
   - in particular, UTF-16 surrogates, BOM

# Fonts

 - Fonts vs. typefaces
 - Classes of typefaces (monotype, italic, ...)
 - Computer representations of fonts. Bitmap/truetype/opentype (raster vs vector)
 - System default fonts, font fallback chains, locations of fonts on windows/mac/linux
 - Modern opentype tables for things like font color, variable fonts
 - Emojis 😊
 - em quadrat (width of 'M'), en quadrat

# Shaping

Shaping is essentially characters -> glyphs

 - Latin is almost 1-1 (apart from ligatures, other things?)
 - Devanagari/Arabic/Hebrew (also RTL)

# Layout

 - Line breaks
 - Page breaks & layout when you have pages (e.g. not on the web)
 - Often backwards-and-forwards between layout and shaping - try different shaping runs and see how much room they take up, ...
 - Text layout data structures
   - Layout is expensive, so if the text doesn't change you probably want to compute the layout and store it.

# Rendering

 - Bitmap vs 
 - Link to curves page

# Localization

Although not strictly a part of text rendering, it is also worth mentioning localization alongside the other sections, many of which also deal implicitly with localization themselves. Here we cover some topics in localization that are not explicitly mentioned elsewhere.

 - Dates
 - Numbers

 # Misc

TODO find a suitable section for these topics

 - IME
