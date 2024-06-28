+++
title = "#![doc = include_str!()] with intra-doc links"
authors = ["Daniel McNab"]
+++

When using `#![doc = include_str!("../README.md")]` for your documentation, you will also want to link directly to mentioned [items][].
These links need to work in other places that the README is rendered, such as on your package's <https://crates.io> homepage.
Because of this, you should be using direct links to the online documentation of the items:

```md
To get started with foobar, use the [`frobnicate`][] function.

[`frobnicate`]: https://docs.rs/foobar/latest/foobar/fn.frobnicate.html
```

However, [including](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html#the-doc-attribute) this as your crate's root documentation means that any users of `cargo doc` will be redirected to your crate's online docs (rather than their local docs) when clicking on that link.
It is however possible to make this link be an intra-doc link, by adding a second link reference definition in the same doc comment.
This must be placed *before* the `doc = include_str!()` line, in `lib.rs`:

```rust
//! [`frobnicate`]: frobnicate
#![doc = include_str("../README.md")]
```

This means that the link has the expected link target on <https://crates.io> *and* in rustdoc (including on <https://docs.rs>).
This trick works because when there are duplicate markdown link reference definitions, ["the first one takes precedence"](https://spec.commonmark.org/0.31.2/#example-204).
When rendering using rustdoc, the intra-doc link appears before the link to the online docs, and so effectively overwrites that link.
However, when the README is rendered standalone, only the link reference definition for the online docs is present, and so that target is used.

### Example

For an example of both of these techniques in action, see the Android Trace crate (`android_trace`).
In particular, the text of interest is: "the main entry point to the library is **AndroidTrace**".
You can observe that this link goes to version 0.1.1 on the docs page, but version 0.1.0 elsewhere.

- [GitHub rendered readme](https://github.com/linebender/android_trace/blob/v0.1.1/android_trace/README.md)
- [crates.io page](https://crates.io/crates/android_trace/0.1.1)
- [docs.rs page](https://docs.rs/android_trace/0.1.1/android_trace/)

And the corresponding source code:

- [README.md](https://github.com/linebender/android_trace/blob/v0.1.1/android_trace/README.md?plain=1)
- [lib.rs](https://github.com/linebender/android_trace/blob/v0.1.1/android_trace/src/lib.rs)

The links have since been updated to use `latest`, but the direct link being to version 0.1.0 shows how this pattern works.

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
Doctests in included markdown files do also have some diagnostics issues, as reported in [rust-lang/rust#81070](https://github.com/rust-lang/rust/issues/81070).

The second limitation is with file links, such as a link to your license file of the form `[LICENSE-MIT](LICENSE-MIT)` (as recommended in [C-PERMISSIVE][]).
This is because rustdoc does not support relative file links in Markdown.
This can be solved by using the "opposite" of this trick - you can use a web link in your `lib.rs`, and a file link in the README.

## A second trick

READMEs contain some content which are not expected to be present in a crate's documentation.
For example, a top-level title, which would duplicate the `crate foobar` header added by rustdoc.
This can be resolved by using css in your documentation to hide these items; rustdoc allows embedding CSS in your documentation.
However, this should only be included in your `lib.rs`, so that the header is shown in other contexts.

```rs
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
```

Any text in the README which should be excluded from your docs page can be surrounded by a `rustdoc-hidden` `div`, for example:

```md
<div class = "rustdoc-hidden">

# Foobar

</div>
```

As far as I'm aware, this does not have any accessibility considerations, as browsers will not make `display: none` items available in their accessibility tree.

Tracing Android Trace (see [above](#example)) also uses this trick to hide the license section from the crate docs.
The license information is already present in the crate's info box.

## Conclusion

Using `#![doc = include_str!("../README.md")]` can integrate nicely with intra-doc links, if you use some tricks.
I believe that this pattern reduces the quality gap between bespoke documentation in `lib.rs` enough that using README includes becomes the right pattern for most crates.
This avoids the maintenance burden of needing to ensure that text in the crate level docs and the README do not become out-of-sync.
I intend to evaluate this technique by applying it across Linebender crates.

These patterns can also be applied when including non-`README` markdown documents.
I suggest linking to this post when using these patterns, to allow future readers of your code to understand the technique being used.

Discuss on [the Linebender Zulip](https://xi.zulipchat.com/#narrow/stream/181284-blogging/topic/.23!.5Bdoc.20.3D.20include_str!.28.29.5D.20with.20intra-doc.20links).

[items]: https://doc.rust-lang.org/reference/items.html
[C-PERMISSIVE]: https://rust-lang.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive
