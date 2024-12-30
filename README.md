# rust-style-guide

A tool for applying [Rust coding guidelines](https://rust-coding-guidelines.github.io/rust-coding-guidelines-zh/).

[![Crates.io](https://img.shields.io/crates/v/rust-style-guide)](https://crates.io/crates/rust-style-guide)
[![License](https://img.shields.io/crates/l/rust-style-guide)](https://github.com/andeya/rust-style-guide?tab=MIT-1-ov-file)

## Usage

Level of Enforcement:

> - REQUIRED: Practices that are mandatory to adhere to, reported as an Error.
> - RECOMMENDED: Recommended best practices, reported as a Warning.
> - OPTIONAL: Generally accepted best practices, but their implementation depends on the context, reported as Info. For example, when writing infrastructure code, strict measures are often used. However, for rapidly iterating business code, some requirements may be too stringent, and adherence depends on the specific practice context.

### Method 1

- Run the following Cargo command in your project directory:

```sh
cargo add rust-style-guide
```

- Add global attributes to the top of the file `main.rs` or `lib.rs`:

```rust
#![allow(internal_features, unused_imports)]
#![feature(custom_inner_attributes)]
#![feature(prelude_import)]
#![rust_style_guide::global_attributes]
```

### Method 2

Manually add configurations to the corresponding files according to the descriptions below.

## Format

Use the official `Rustfmt` tool provided by Rust as your code formatter. It helps teams adhere to a consistent code style by automatically adjusting code indentation, spacing, line breaks, and other formatting rules, thereby improving code readability. This tool is included by default with Rust. Official documentation: Rustfmt : https://rust-lang.github.io/rustfmt


We enforce the inclusion of a `rustfmt.toml` file in every project, with the following configuration:

```toml
# -------- https://rust-lang.github.io/rustfmt/?search={key} --------

# Run rustfmt with this config (it should be picked up automatically).
style_edition = "2024"
unstable_features = true

# Prevent carriage returns
newline_style = "Unix"

# Maximum line width of 120 characters
max_width = 120
# Fine-grained width values are calculated as a proportion of `max_width`.
use_small_heuristics = "Default"
# (default) P.FMT.02 Indent with 4 spaces
tab_spaces = 4
# (default) P.FMT.02 Disallow the use of hard tabs for indentation
hard_tabs = false

# (default) P.FMT.03 Minimum blank lines between code lines: 0
blank_lines_lower_bound = 0
# (default) P.FMT.03 Maximum blank lines between code lines: 1
blank_lines_upper_bound = 1

# (default) P.FMT.04 Use the same line brace style for most language constructs (functions, structs, etc.)
brace_style = "SameLineWhere"
# (default) P.FMT.04 Place the opening brace of `where` clauses on the next line
where_single_line = false
# (default) P.FMT.04 Use the same line brace style for control structures (e.g. `if`, `match`)
control_brace_style = "AlwaysSameLine"

# (default) P.FMT.05 Use block indentation style for multiple identifier definitions
indent_style = "Block"

# (default) P.FMT.06 Place operators at the start of new lines for multi-line expressions
binop_separator = "Front"

# (default) P.FMT.07 Do not align discriminants of enum variants
enum_discrim_align_threshold = 0
# (default) P.FMT.07 Do not align struct fields
struct_field_align_threshold = 0

# (default) P.FMT.08 For up to five function parameters, place them on a single line; use block style for more
fn_params_layout = "Tall"
# (default) P.FMT.08 Place import statements on new lines if more than four per line
imports_layout = "Mixed"

# (default) P.FMT.09 Do not add spaces before colons
space_before_colon = false
# (default) P.FMT.09 Add spaces after colons
space_after_colon = true
# (default) P.FMT.09 Do not add spaces around range operators (`..`, `..=`)
spaces_around_ranges = false
# (default) P.FMT.09 Add spaces around `+` and `=` operators (in type annotations)
type_punctuation_density = "Wide"

# (default) P.FMT.10 Use block style when match arms are too long to fit on the same line as `=>`
match_arm_blocks = true
# (default) P.FMT.10 Do not prepend match arms with extra leading pipes (`|`)
match_arm_leading_pipes = "Never"

# P.FMT.11 Put imports from the same module within the same group
imports_granularity = "Crate"
# P.FMT.11 Group module imports as follows:
#   * Imports from `std`, `core`, and `alloc` come first.
#   * Imports from third-party crates come next.
#   * Imports from local crates (with `self`, `super`, and `crate` prefixes) come last.
group_imports = "StdExternalCrate"
# (default) P.FMT.11 Order imports within groups in lexicographical order
reorder_imports = true

# P.FMT.12 Use compact format for macro matchers (left-hand side of `=>`) in macro definitions
format_macro_matchers = true
# (default) P.FMT.12 Use relaxed format for macro bodies (right-hand side of `=>`) in macro definitions
format_macro_bodies = true

# (default) P.FMT.13 Do not use field initialization shorthand
use_field_init_shorthand = false

# (default) P.FMT.14 Always explicitly specify the ABI for external functions
force_explicit_abi = true

# P.FMT.15 Allow the use of `..` to denote remaining elements when destructuring tuples
condense_wildcard_suffixes = true

# P.FMT.16 Do not merge unrelated traits into a single line in derive macros
merge_derives = false

# Replace `try!` macro with the `?` operator
use_try_shorthand = true
# (default) P.CMT.02 Set the maximum width for a single-line comment to 80 characters
comment_width = 80
# P.CMT.02 Automatically wrap multi-line comments at the maximum width
wrap_comments = true

# P.CMT.03 Convert block comments (`/* */`) to line comments (`//`) where possible
normalize_comments = true
# (default) P.CMT.03 Convert `#![doc]` and `#[doc]` attributes to `//!` and `///` respectively
normalize_doc_attributes = false


# By default, ignore everything in the repository
# Tidy only checks files which are not ignored; each entry follows gitignore style
ignore = []
```

## Lint

`Rustc` is Rust's code compiler, which also includes many lint checks. When compiling code, Rustc runs these lint checks and provides feedback at different levels based on the configuration.
Rustc lint documentation: Rustc Lints : https://doc.rust-lang.org/rustc/lints/index.html

`Clippy` is a linter tool for Rust that performs static analysis to identify potential issues, style problems, anti-patterns, and possible optimizations in Rust code. It acts like a code quality watchdog, assisting developers in writing more efficient, safer, and stylistically consistent Rust code.

- Clippy documentation: Clippy Docs : https://doc.rust-lang.org/stable/clippy/index.html
- Clippy lint categories: Clippy Lint Groups : https://github.com/rust-lang/rfcs/blob/master/text/2476-clippy-uno.md#reference-level-explanation

- Installing Clippy:

```sh
rustup component add clippy [--toolchain=<name>]
```

- Running Clippy to apply code fixes:

```sh
cargo clippy --fix
```

- Running Clippy to check code and deny warnings:

```sh
cargo clippy -- -D warnings
```

- Configuring Rustc and Clippy lints must be done in `main.rs` or `lib.rs`:

```rust
// -------- rust coding guidelines: https://rust-coding-guidelines.github.io/rust-coding-guidelines-zh/ --------
// -------- rustc lint doc: https://doc.rust-lang.org/rustc/lints/listing/index.html --------
// -------- rust-clippy doc: https://rust-lang.github.io/rust-clippy/master/index.html --------

// [REQUIRED] G.VAR.02 Do not use non-ASCII characters in identifiers
#![deny(non_ascii_idents)]
// [REQUIRED]
#![allow(clippy::disallowed_names)]
// [REQUIRED]
#![allow(clippy::blanket_clippy_restriction_lints)]
// [OPTIONAL] G.CMT.01 Add Error documentation in the docs of public functions that return Result
// #![warn(clippy::missing_errors_doc)]
// [REQUIRED] G.CMT.02 Add Panic documentation in the docs of public APIs that may panic under certain circumstances
#![warn(clippy::missing_panics_doc)]
// [RECOMMENDED] G.VAR.03 Variable shadowing should be used carefully
#![warn(clippy::shadow_reuse, clippy::shadow_same, clippy::shadow_unrelated)]
// [RECOMMENDED] G.CNS.05 Use const fn for functions or methods wherever applicable
#![warn(clippy::missing_const_for_fn)]
// [REQUIRED] G.TYP.01 Prefer safe conversion functions over `as` for type casting
#![warn(
    clippy::as_conversions,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::ptr_as_ptr
)]
// [RECOMMENDED] G.VAR.01 Avoid using too many meaningless variable names when destructuring tuples with more than four
// variables
#![warn(clippy::many_single_char_names)]
// [RECOMMENDED] G.TYP.02 Explicitly specify the type for numeric literals
#![warn(clippy::default_numeric_fallback)]
// [RECOMMENDED] G.TYP.03 Use `try_from` methods instead of relying on numeric boundaries for safe conversion
#![warn(clippy::checked_conversions)]
// [RECOMMENDED] G.TYP.BOL.02 Use `if` expressions instead of `match` for boolean conditions
#![warn(clippy::match_bool)]
// [RECOMMENDED] G.TYP.BOL.05 Use logical operators (&&/||) instead of bitwise operators (&/|) for boolean operations
// when not necessary
#![warn(clippy::needless_bitwise_bool)]
// [RECOMMENDED] G.TYP.INT.01 Consider the risks of integer overflow, wrapping, and truncation in integer arithmetic
#![warn(clippy::arithmetic_side_effects)]
// [REQUIRED] G.TYP.INT.02 Avoid `as` casting between signed and unsigned integers; use safe conversion functions
#![deny(clippy::cast_sign_loss)]
// [REQUIRED] G.TYP.INT.03 Avoid using `%` for modulo operations on negative numbers
#![warn(clippy::modulo_arithmetic)]
// [REQUIRED] G.TYP.FLT.02 Avoid precision loss when casting from any numeric type to floating-point; use safe
// conversion functions
#![warn(clippy::cast_precision_loss)]
// [REQUIRED] G.TYP.FLT.03 Be cautious of precision loss in floating-point arithmetic and comparisons
#![warn(clippy::float_arithmetic, clippy::float_cmp, clippy::float_cmp_const)]
// [REQUIRED] G.TYP.FLT.04 Use Rust's built-in methods for floating-point calculations
#![warn(clippy::imprecise_flops, clippy::suboptimal_flops)]
// [OPTIONAL] G.TYP.ARR.01 Use static variables instead of constants for large global arrays
// #![warn(clippy::large_stack_arrays)]
// [RECOMMENDED] G.TYP.SCT.01 Add `#[non_exhaustive]` attribute to publicly exported structs
#![warn(clippy::exhaustive_structs)]
// [RECOMMENDED] G.TYP.ENM.05 Add `#[non_exhaustive]` attribute to publicly exported enums
#![warn(clippy::exhaustive_enums)]
// [OPTIONAL] G.TYP.SCT.02 Consider refactoring when a struct contains more than three boolean fields
// #![warn(clippy::struct_excessive_bools)]
// [RECOMMENDED] G.FUD.03 Consider using a custom struct or enum instead of many boolean parameters in function
// signatures
#![warn(clippy::fn_params_excessive_bools)]
// [RECOMMENDED] G.TYP.ENM.04 Avoid using glob imports for enum variants in `use` statements
#![warn(clippy::enum_glob_use)]
// [RECOMMENDED] G.CTF.02 Ensure `else` branches are present whenever `else if` is used
#![warn(clippy::else_if_without_else)]
// [OPTIONAL] G.STR.02 Use `push_str` method for appending strings
// #![warn(clippy::string_add_assign, clippy::string_add)]
// [RECOMMENDED] G.STR.03 Convert string literals containing only ASCII characters to byte sequences using `b"str"`
// syntax instead of `as_bytes()`
#![warn(clippy::string_lit_as_bytes)]
// [RECOMMENDED] G.STR.05 Take care to avoid disrupting UTF-8 encoding when slicing strings at specific positions
#![warn(clippy::string_slice)]
// [RECOMMENDED] G.FUD.02 Prefer passing large values by reference if function parameters implement `Copy`
#![warn(clippy::large_types_passed_by_value)]
// [RECOMMENDED] G.FUD.04 Pass small `Copy` type values by value instead of by reference
#![warn(clippy::trivially_copy_pass_by_ref)]
// [OPTIONAL] G.FUD.05 Avoid using `inline(always)` for functions indiscriminately
// #![warn(clippy::inline_always)]
// [REQUIRED] G.GEN.02 Be cautious to avoid using generic default implementations of some methods from Rust's standard
// library; prefer specific type implementations
#![warn(clippy::inefficient_to_string)]
// [OPTIONAL] G.TRA.BLN.01 Prefer using the concrete type's `default()` method over calling `Default::default()`
// #![warn(clippy::default_trait_access)]
// [REQUIRED] G.TRA.BLN.02 Do not implement the `Copy` trait for iterators
#![warn(clippy::copy_iterator)]
// [RECOMMENDED] G.TRA.BLN.07 Use `copied` method instead of `cloned` for iterable `Copy` types
#![warn(clippy::cloned_instead_of_copied)]
// [RECOMMENDED] G.ERR.01 Avoid using `unwrap` indiscriminately when handling `Option<T>` and `Result<T, E>`
#![warn(clippy::unwrap_used)]
// [RECOMMENDED] G.MOD.03 Avoid using wildcard imports in module declarations
#![warn(clippy::wildcard_imports)]
// [REQUIRED] G.MOD.04 Avoid using different module layout styles within the same project
#![warn(clippy::self_named_module_files)]
// [RECOMMENDED] G.CAR.02 Ensure that necessary metadata is included in the `Cargo.toml` of the crate
#![warn(clippy::cargo_common_metadata)]
// [RECOMMENDED] G.CAR.03 Avoid negative or redundant prefixes and suffixes in feature names
#![warn(clippy::negative_feature_names, clippy::redundant_feature_names)]
// [REQUIRED] G.CAR.04 Avoid using wildcard dependencies in `Cargo.toml`
#![warn(clippy::wildcard_dependencies)]
// [RECOMMENDED] G.MAC.01 Only use the `dbg!()` macro for debugging code
#![warn(clippy::dbg_macro)]
// [REQUIRED] Ensure that locks are released before `await` is called in asynchronous code
#![warn(clippy::await_holding_lock)]
// [REQUIRED] Handle `RefCell` references across `await` points
#![warn(clippy::await_holding_refcell_ref)]
// [RECOMMENDED] G.ASY.04 Avoid defining unnecessary async functions
#![warn(clippy::unused_async)]
// [REQUIRED] G.UNS.SAS.02 Use `assert!` instead of `debug_assert!` to verify boundary conditions in unsafe functions
#![warn(clippy::debug_assert_with_mut_call)]
```


- Additional `Clippy lint` configurations should be placed in the project's `clippy.toml` file, with the following content:

```toml
# -------- rust-clippy doc: https://rust-lang.github.io/rust-clippy/master/index.html --------

# This lint will not trigger if the function is an exported API. This avoids suggesting changes that would break the API. Default is true
avoid-breaking-exported-api = true

# [RECOMMENDED] G.VAR.01 Avoid using too many meaningless variable names when destructuring tuples with more than four variables
single-char-binding-names-threshold = 4

# [RECOMMENDED] G.TYP.SCT.02 Consider refactoring when a struct contains more than three boolean fields
max-struct-bools = 3

# [RECOMMENDED] G.FUD.03 Configures the maximum number of boolean parameters allowed in a function, which is 3 by default.
max-fn-params-bools = 3

# [RECOMMENDED] G.TYP.ENM.06 Variants within an enum should not differ significantly in size. Default is 200 bytes
enum-variant-size-threshold = 200

# [REQUIRED] G.FUD.01 Limit the number of function parameters to no more than five
too-many-arguments-threshold = 5

# Determines if wildcard imports in prelude and super (in test modules) should be warned. Default is false
warn-on-all-wildcard-imports = false
```