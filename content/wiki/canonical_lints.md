+++
title = "Canonical lint set for Linebender projects"
+++

All Linebender projects should include the following set of lints:

## `Cargo.toml`

```toml
[lints]
# This one may vary depending on the project.
rust.unsafe_code = "forbid"

# LINEBENDER LINT SET - Cargo.toml - v6
# See https://linebender.org/wiki/canonical-lints/
rust.keyword_idents_2024 = "forbid"
rust.non_ascii_idents = "forbid"
rust.non_local_definitions = "forbid"
rust.unsafe_op_in_unsafe_fn = "forbid"

rust.elided_lifetimes_in_paths = "warn"
rust.missing_debug_implementations = "warn"
rust.missing_docs = "warn"
rust.trivial_numeric_casts = "warn"
rust.unexpected_cfgs = "warn"
rust.unnameable_types = "warn"
rust.unreachable_pub = "warn"
rust.unused_import_braces = "warn"
rust.unused_lifetimes = "warn"
rust.unused_macro_rules = "warn"

clippy.too_many_arguments = "allow"

clippy.allow_attributes_without_reason = "warn"
clippy.cast_possible_truncation = "warn"
clippy.collection_is_never_read = "warn"
clippy.dbg_macro = "warn"
clippy.debug_assert_with_mut_call = "warn"
clippy.doc_markdown = "warn"
clippy.fn_to_numeric_cast_any = "warn"
clippy.infinite_loop = "warn"
clippy.large_stack_arrays = "warn"
clippy.mismatching_type_param_order = "warn"
clippy.missing_assert_message = "warn"
clippy.missing_fields_in_debug = "warn"
clippy.same_functions_in_if_condition = "warn"
clippy.semicolon_if_nothing_returned = "warn"
clippy.should_panic_without_expect = "warn"
clippy.todo = "warn"
clippy.unseparated_literal_suffix = "warn"
clippy.use_self = "warn"

clippy.cargo_common_metadata = "warn"
clippy.negative_feature_names = "warn"
clippy.redundant_feature_names = "warn"
clippy.wildcard_dependencies = "warn"
# END LINEBENDER LINT SET
```

## `lib.rs`

```rust
// LINEBENDER LINT SET - lib.rs - v3
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
```

## `.clippy.toml`

```toml
# LINEBENDER LINT SET - .clippy.toml - v1
# See https://linebender.org/wiki/canonical-lints/

# The default Clippy value is capped at 8 bytes, which was chosen to improve performance on 32-bit.
# Given that we are building for the future and even low-end mobile phones have 64-bit CPUs,
# it makes sense to optimize for 64-bit and accept the performance hits on 32-bit.
# 16 bytes is the number of bytes that fits into two 64-bit CPU registers.
trivial-copy-size-limit = 16

# END LINEBENDER LINT SET
```

This is a curated list: Clippy has a *lot* of lints, and most of them are not included above.

The list above should be considered canonical.
If you think a new lint should be added to Linebender projects, add it to this file in alphabetical order, then copy-paste the list across projects.

To keep this process simple, avoid modifying this list in individual projects.
If you want to add other per-project lints, add them above the list.
If you want to remove a lint, `#![allow]` or `#![expect]` it at the crate root.

## Periodic Passes

As a lot of our code is research code, there are lints which we believe improve code quality, but don't need to validate in CI.
These lints can be run occasionally, such as when releases are near, to improve code quality asynchronously:

```text
let_underscore_drop
single_use_lifetimes
unit_bindings
unused_qualifications
variant_size_differences

clippy::allow_attributes
clippy::large_include_file
clippy::match_same_arms
clippy::partial_pub_fields
clippy::default_trait_access
clippy::return_self_not_must_use
clippy::shadow_unrelated
```

As a runnable command, there are:

```sh
cargo clippy -- -W let_underscore_drop -W single_use_lifetimes -W unit_bindings -W unused_qualifications -W variant_size_differences -W clippy::large_include_file -W clippy::match_same_arms -W clippy::partial_pub_fields -W clippy::default_trait_access -W clippy::return_self_not_must_use -W clippy::shadow_unrelated
```

You may also wish to enable some of these lints specifically in your editor, to improve as you go.
The ones which are trivial to do this for are below (as the changes it will propose are always very small):

```sh
cargo clippy -- -W single_use_lifetimes -W unit_bindings -W unused_qualifications -W clippy::allow_attributes -W clippy::default_trait_access
```

If there are many failures of one of these lints across the project, a separate PR would be recommended.
Otherwise, resolving these in a drive-by manner is fine.

### Pedantic

You may occasionally want to run `cargo clippy` with `clippy::pedantic` on the codebase, which will cast a very wide net and catch a lot of very minor issues.
There are a few lints which are definitely not in scope, such as:

```sh
cargo clippy -- -W clippy::pedantic -A clippy::module_name_repetitions
```

A command line which allows certain restriction lints and checks the rest is provided below.
The vast majority of issues this raise should not be actioned, but for our smaller crates with a high quality bar (such as [Color](https://github.com/linebender/color)), this can be worthwhile.

```sh
cargo clippy -- -W clippy::restriction -A clippy::module_name_repetitions -A clippy::pattern_type_mismatch -A clippy::implicit_return -A clippy::missing_inline_in_public_items -A clippy::missing_docs_in_private_items -A clippy::float_arithmetic -A clippy::min_ident_chars -A clippy::impl_trait_in_params -A clippy::exhaustive_structs
```
