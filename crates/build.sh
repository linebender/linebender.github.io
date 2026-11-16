#!/usr/bin/env bash
# Build the web demos into static/rust/, where zola picks them up.
#
#   crates/build.sh              # debug, fast to compile
#   crates/build.sh --release    # what CI ships
#   crates/build.sh --list       # print the -p flags naming the web demos
#
# Any other arguments are passed through to cargo, which is how CI adds
# --locked.
#
# Alongside `zola serve`, for live reload:
#   cargo watch -w crates -s crates/build.sh
#
# To iterate on a single demo without zola, use trunk instead:
#   cd crates/squircle && trunk serve
set -euo pipefail
# So the globs below expand to nothing rather than to themselves.
shopt -s nullglob

cd "$(dirname "$0")"

# A crate is a web demo if it declares a cdylib target. Other crates in this
# workspace are native tools -- figure generators run by hand, whose output is
# committed as SVG -- and have no business being compiled for wasm.
demos=()
for manifest in */Cargo.toml; do
    grep -qE '^[[:space:]]*crate-type[[:space:]]*=.*"cdylib"' "$manifest" || continue
    demos+=(-p "$(sed -n 's/^name[[:space:]]*=[[:space:]]*"\(.*\)"/\1/p' "$manifest" | head -1)")
done

if [[ ${#demos[@]} -eq 0 ]]; then
    echo "no crate here declares a cdylib target; nothing to build" >&2
    exit 1
fi

if [[ "${1:-}" == "--list" ]]; then
    echo "${demos[@]}"
    exit 0
fi

profile_dir=debug
for arg in "$@"; do
    [[ "$arg" == "--release" ]] && profile_dir=release
done

if ! command -v wasm-bindgen >/dev/null; then
    echo "wasm-bindgen not found. Install the version pinned in ../.github/workflows/main.yml:" >&2
    echo "    cargo install -f wasm-bindgen-cli --version 0.2.128" >&2
    exit 1
fi

out=../static/rust
mkdir -p "$out"

# Stylesheets first, so a CSS-only edit lands before the (no-op) cargo build.
for css in ./*/*.css; do cp "$css" "$out"; done

# --lib skips each demo's trunk-only dev bin.
cargo build "${demos[@]}" --lib --target wasm32-unknown-unknown "$@"

for wasm in "target/wasm32-unknown-unknown/$profile_dir"/*.wasm; do
    # A stale dev bin from a previous `trunk serve` can linger here.
    case "$(basename "$wasm")" in *-dev.wasm) continue ;; esac
    echo "packaging $(basename "$wasm")"
    wasm-bindgen --target web --out-dir "$out" --no-typescript "$wasm"
done
