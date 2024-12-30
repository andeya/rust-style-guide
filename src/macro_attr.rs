use proc_macro::TokenStream;
use quote::{ToTokens, quote};

pub fn global_rustc_clippy_attributes(_args: TokenStream) -> TokenStream {
    quote! {
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
            }
    .into_token_stream()
    .into()
}
