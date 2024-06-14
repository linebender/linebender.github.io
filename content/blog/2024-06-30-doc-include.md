+++
title = "#![doc = include_str!()] with intra-doc links"
authors = ["Daniel McNab"]
+++

When using `#![doc = include_str!("../README.md")]` for your documentation, you will also want to link directly to mentioned items.
These links need to work in other places that the README is rendered, such as on your package's <https://crates.io> homepage.
Because of this, you should be using direct links to the online documentation of the items:

```md
To get started with foobar, use the [`frobnicate`][] function.

[`frobnicate`]: https://docs.rs/foobar/latest/foobar/fn.frobnicate.html
```

However, [including](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html#the-doc-attribute) this as your crate's root documentation means that any users of `cargo doc` will be redirected to your crate's online docs (rather than their local docs) when clicking on that link.
It is however possible to make this link be an intra-doc link, by adding a link definition *before* the `doc = include_str!()` line, inside the same documentation:

```rust
// https://linebender.org/blog/doc-include
//! [`frobnicate`]: frobnicate
#![doc = include_str("../README.md")]
```

This means that the link has the expected link target on both <https://crates.io> and in rustdoc (including on <https://docs.rs>).
This trick works because when there are duplicate markdown link reference definitions, ["the first one takes precedence"](https://spec.commonmark.org/0.31.2/#example-204).
When rendering using rustdoc, the intra-doc link appears before the link to the online docs, so that is used.
However, with standalone markdown rendering, only the link to the online docs is present, and so that fallback is used.

### Example

For an example of both of these techniques in action, see the `tracing_android_trace` crate:

- [GitHub Readme](https://github.com/linebender/android_trace/tree/main/tracing_android_trace)
- [crates.io page](https://crates.io/crates/tracing_android_trace)
- [docs.rs page](https://docs.rs/tracing_android_trace/latest/tracing_android_trace/)

And the corresponding source code:

- [README.md](https://github.com/linebender/android_trace/blob/main/tracing_android_trace/README.md?plain=1)
- [lib.rs](https://github.com/linebender/android_trace/blob/main/tracing_android_trace/src/lib.rs)

### Getting external documentation links

rust-analyzer has an "Open External Docs" command when you have an item selected.
This will open the online documentation for the selected item, generally on <https://docs.rs>.
This works even if the item is yet to be published, in which case it will open the URL where the item would be.
You may wish to replace the resolved version number in the URL with `latest`, e.g. `https://docs.rs/foobar/latest/foobar/`

### Limitations

`#![doc = include_str!("../README.md")]` has two main relevant limitations which you may need to work around.

The first of these is in code blocks.
Normal rustdoc examples can include hidden setup lines, starting with a `#` character.
These will not be rendered by rustdoc.

````md
```rust
# use foobar::frobnicate;
frobnicate();
```
````

However, other markdown renderers do not support this extension, so the example above will be rendered as something like:

```rust
# use foobar::frobnicate;
frobnicate();
```

Unfortunately, I am not aware of a workaround for this; my current best suggestion is to avoid using these hidden lines in your README.
These doctests also some diagnostics issues, see [rust-lang/rust#81070](https://github.com/rust-lang/rust/issues/81070).

The second limitation is with file links, such as a link to your license file of the form `[LICENSE-MIT](LICENSE-MIT)` (see [C-PERMISSIVE](https://rust-lang.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive)).
This is because rustdoc does not support relative file links in Markdown.
This can be worked around by using the "opposite" of this trick - you can use a web link in your `lib.rs`, and a file link in the README.

## A second trick

READMEs contain some content which are not expected to be present in a crate's documentation.
The primary example is a top-level header, which would duplicate the `crate foobar` header.
This can be worked around by using css in your documentation to hide these items; rustdoc allows CSS.
However, this should only be included in your `lib.rs`, so that the.

```rs
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
```

Any text in the README which should be excluded from your docs page can be surrounded by a `rustdoc-hidden` `div`, i.e.:

```md
<div class = "rustdoc-hidden">

# Tracing Android Trace

</div>
```

As far as I'm aware, this does not have any accessibility considerations, as browsers will not make `display: none` items available in their accessibility tree.

Tracing Android Trace (mentioned [above](#example)) also uses this trick to hide the license section from the crate docs.

## Conclusion

Using `#![doc = include_str!("../README.md")]` can integrate nicely with intra-doc links, if you use some tricks.
I believe that this pattern reduces the quality gap between bespoke documentation in `lib.rs` enough that using README includes is the right pattern for most crates.
This is because it reduces the maintenance burden which would otherwise be imposed by needing to ensure that text in both locations does not become out-of-sync.
