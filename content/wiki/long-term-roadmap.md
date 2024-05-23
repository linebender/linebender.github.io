# Post-July Linebender roadmap

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
