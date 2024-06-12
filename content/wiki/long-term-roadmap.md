+++
title = "Post-July Linebender roadmap"
+++

Unstructured notes from our last roadmap brainstorming that were left out of the formal roadmap blog post.

## Vello

* Emoji
* Image filters / blur / shadows
    * Render graph (Raph to drive, see [thread](https://xi.zulipchat.com/#narrow/stream/197075-gpu/topic/Render.20Graph))
* Gpu embedding / wgpu engine abstraction (see [Servo thread](https://xi.zulipchat.com/#narrow/stream/147922-new-members/topic/Greg.20from.20Servo))
* Glyph caching
    * Render graph? 
    * or CPU side? 
    * (sparse strips probably)
* Frame pacing
* Damage regions
    * MVP: just render sub-rects
* CPU impl
    * Testing
    * Downlevel
* Fuzzing?

## Masonry

* ‘Compositing’
* Frame pacing (relevant to Vello and AccessKit)
* Damage regions
* Styling options
* Copy features from [Tomorrow Corporation Tech Demo](https://www.youtube.com/watch?v=72y2EC5fkcE)

## Parley

* IME support
    * Suggestion regions?
* Split out blob & other vocabulary types
    * Blob may be prioritized by other project needs

## Xilem

* Android
    * Material UI style widgets/views? Requires styling
* Styling
* Hot reloading
* Async behavior in Masonry widgets (image loading etc)
* Cleanup run for issues and PRs

## AccessKit

* Haven’t talked about it a lot
  * Improve documentation (see [#402](https://github.com/AccessKit/accesskit/issues/402))
  * Point to / create a repository of accessibility best practices.
  * Not just code, also social: Have people with disabilities test your stuff!
  * Point to resources, forums, etc? Need to talk to Matt about this. (What were the best practices at Microsoft?)
  * Accessibility isn’t just screen readers. Also color contrast, customizing size, IME support, motor disabilities (avoid stuff that requires simultaneous input).

* “Accessibility is more than adding a dependency to AccessKit”
    * (Olivier: Note to self: include GMTK video about Celeste Assist Mode)

## Android specific work
* Need to prioritize Android
  * Masonry:
    * Hot state for pointer pressed?
    * Gesture recognition (scrolling) P3
        * There is a dumb solution for scrolling without momentum
        * Potentially adapt code from egui (used in vello demo)
    * Pencil gestures
    * Multitouch
    * IME
        * Landing the Lucas Meurer winit PR P1
        * Extend winit IME PR for richer functionality
        * Review glazier IME API & come up with design proposal P3 (aaron)
        * Plumbing suggestion spans etc (winit)
    * Accessibility (matt, jul 1+) P-1
    * Study Tao
    * To what extent do we tap into Android View etc
  * winit future
    * short term plan: PR functionality into winit
    * look for natural extension points
        * these will be platform-specific
        * Mac: subclassing the NSView/NSApplication subclasses
        * wayland: messy / smithay has message dispatcher
        * Android: do we subclass View/Application/Activity? raph will investigate P1
            * Accessibility (matt, jul 1+)
            * Can we access accessibility from native-activity / game-activity?
            * Note: React Native requires writing Java-land activity
            * What minimum platform version?
  * Look at Makepad’s implementation (momentum scrolling etc)


