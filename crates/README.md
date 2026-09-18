# Crates

Two kinds of crate live here.

**Web demos** are interactive figures embedded in a page, written with
[xilem_web](https://github.com/linebender/xilem/tree/main/xilem_web) and
compiled to Wasm. A crate is treated as one exactly when it declares a
`cdylib` target, which is the only thing `build.sh` keys off; nothing has
to be listed anywhere.

**Native tools** are everything else: programs run by hand to generate a
figure whose output is committed, usually as an SVG under `static/`. They
are never compiled for Wasm, and the site build ignores them.

| Crate      | Kind     | Used by                            |
| ---------- | -------- | ---------------------------------- |
| `squircle` | web demo | `content/wiki/curves/squircle.md`  |

## Two builds from one crate

A crate has a `cdylib` lib target and a `<name>-dev` bin target, sharing all
their code through the `rlib`:

* The **lib** is what ships. `wasm-bindgen` turns it into a
  `<crate>.js` / `<crate>_bg.wasm` pair in `static/rust/`, which zola copies to
  the site root, and the page imports `/rust/<crate>.js` and calls the crate's
  `#[wasm_bindgen] pub fn start(id: &str)` to mount it.
* The **bin** exists only for `trunk serve`. It calls the same `start` with the
  same element id as the page, so what you see under trunk is what the page
  gets.

Site builds pass `--lib`, so the dev bin is never compiled in CI, and select
crates by `cdylib`, so native tools in this workspace are never built for Wasm.

## Iterating

For one demo, with no zola in the way:

```sh
cd crates/squircle && trunk serve
```

For the real page, run `zola serve` and rebuild the Wasm beside it:

```sh
crates/build.sh                            # once
cargo watch -w crates -s crates/build.sh   # or on every change
```

`static/rust/` is generated and git-ignored, so `build.sh` has to run at least
once before `zola serve` will show a demo.

Stylesheets live next to the crate that uses them (`squircle-demo.css`) and are
copied into `static/rust/` by the same step, so trunk and the page share one
file. The page supplies the site's theme variables; the trunk `index.html`
supplies stand-ins for them.

## Adding a crate

Add it to `members` in `Cargo.toml`. If it declares a `cdylib` it is built and
packaged as a web demo; if it does not, it is left to be run by hand. Either
way there are no workflow changes to make.

## Versions

The `wasm-bindgen` CLI version must match the `wasm-bindgen` in `Cargo.lock`;
both are pinned in `.github/workflows/main.yml`.

```sh
cargo install -f wasm-bindgen-cli --version 0.2.128
```
