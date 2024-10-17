+++
title = "Canonical lint set for Linebender projects"
+++

All Linebender projects should include the following set of lints in their `Cargo.toml`:

```toml
[lints]
# This one may vary depending on the project.
rust.unsafe_code = "forbid"

# LINEBENDER LINT SET - v1
# See https://linebender.org/wiki/canonical-lints/
rust.keyword_idents_2024 = "forbid"
rust.non_ascii_idents = "forbid"
rust.non_local_definitions = "forbid"
rust.unsafe_op_in_unsafe_fn = "forbid"

rust.elided_lifetimes_in_paths = "warn"
rust.let_underscore_drop = "warn"
rust.missing_debug_implementations = "warn"
rust.missing_docs = "warn"
rust.single_use_lifetimes = "warn"
rust.trivial_numeric_casts = "warn"
rust.unexpected_cfgs = "warn"
rust.unit_bindings = "warn"
rust.unnameable_types = "warn"
rust.unreachable_pub = "warn"
rust.unused_import_braces = "warn"
rust.unused_lifetimes = "warn"
rust.unused_macro_rules = "warn"
rust.unused_qualifications = "warn"
rust.variant_size_differences = "warn"

clippy.allow_attributes = "warn"
clippy.allow_attributes_without_reason = "warn"
clippy.cast_possible_truncation = "warn"
clippy.collection_is_never_read = "warn"
clippy.dbg_macro = "warn"
clippy.debug_assert_with_mut_call = "warn"
clippy.doc_markdown = "warn"
clippy.exhaustive_enums = "warn"
clippy.fn_to_numeric_cast_any = "forbid"
clippy.infinite_loop = "warn"
clippy.large_include_file = "warn"
clippy.large_stack_arrays = "warn"
clippy.match_same_arms = "warn"
clippy.mismatching_type_param_order = "warn"
clippy.missing_assert_message = "warn"
clippy.missing_errors_doc = "warn"
clippy.missing_fields_in_debug = "warn"
clippy.missing_panics_doc = "warn"
clippy.partial_pub_fields = "warn"
clippy.return_self_not_must_use = "warn"
clippy.same_functions_in_if_condition = "warn"
clippy.semicolon_if_nothing_returned = "warn"
clippy.shadow_unrelated = "warn"
clippy.should_panic_without_expect = "warn"
clippy.todo = "warn"
clippy.unseparated_literal_suffix = "warn"
clippy.use_self = "warn"
clippy.wildcard_imports = "warn"

clippy.cargo_common_metadata = "warn"
clippy.negative_feature_names = "warn"
clippy.redundant_feature_names = "warn"
clippy.wildcard_dependencies = "warn"
# END LINEBENDER LINT SET
```

And in their `lib.rs`:

```rust
// LINEBENDER LINT SET - v1
// See https://linebender.org/wiki/canonical-lints/
// These lints aren't included in Cargo.toml because they
// shouldn't apply to examples and tests
#![warn(unused_crate_dependencies)]
#![warn(clippy::print_stdout, clippy::print_stderr)]
```

This is a curated list: Clippy has a *lot* of lints, and most of them are not included above.

You may occasionally want to run `cargo clippy` with `clippy::pedantic` on your codebase, which will cast a very wide nets and catch a lot of very minor issues.
The lint set above focuses on problems that either impact correctness or tend to snowball over time.

The list above should be considered canonical.
If you think a new lint should be added to Linebender projects, add it to this file in alphabetical order, then copy-paste the list across projects.

To keep this process simple, avoid modifying this list in individual projects.
If you want to add other per-project lints, add them above the list.
If you want to remove a lint, `#![allow]` or `#![expect]` it at the crate root.
