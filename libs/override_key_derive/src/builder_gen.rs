//! # builder_gen.rs
//!
//! This module serves as the **code generation orchestrator** for the
//! `#[derive(ApplyOverrides)]` procedural macro.
//!
//! Its purpose is to:
//! 1. Collect struct-level configuration (from `#[apply_overrides(...)]`).
//! 2. Traverse all named fields, delegating parsing of field attributes
//!    (such as `#[override_key(...)]`) to the [`field_parser`] module.
//! 3. Accumulate generated code snippets for each field.
//! 4. Emit a complete `impl ApplyOverrides for StructName` block.
//!
//! ## Key Responsibilities
//!
//! - All `syn::Error` values encountered during parsing are collected and
//!   converted into `compile_error!` invocations for graceful compiler output.
//! - The generated `impl` body always contains valid Rust code, even if
//!   one or more fields fail to parse correctly (errors are emitted inline).
//!
//! ## Output Contract
//!
//! The emitted code has this structure (simplified):
//!
//! ```ignore
//! impl ::override_key_core::ApplyOverrides for MyStruct {
//!     fn apply_overrides(
//!         &self,
//!         mut builder: config::ConfigBuilder<config::builder::DefaultState>,
//!     ) -> Result<config::ConfigBuilder<config::builder::DefaultState>, config::ConfigError> {
//!         if let Some(v) = &self.some_field {
//!             builder = builder.set_override("my.prefix.some_field", v.clone())?;
//!         }
//!         Ok(builder)
//!     }
//! }
//! ```
//!
//! ## Module Dependencies
//!
//! - [`struct_config::parse_struct_level_config`] — handles parsing of struct-level
//!   attributes such as `#[apply_overrides(infer_keys, prefix = "...")]`.
//! - [`field_parser::process_field`] — parses individual field annotations and
//!   returns `TokenStream` fragments representing builder override logic.
//!
//! ## Design Notes
//!
//! This module intentionally performs *no syntax analysis* itself.
//! All AST traversal and attribute handling are delegated to
//! specialized modules to minimize coupling and simplify future extension.
//!
//! - This module only assembles and emits the final `TokenStream`.
//! - It guarantees deterministic ordering of generated code (field order preserved).
//! - It never panics; all errors are represented as `syn::Error`.
//!
//! ## Implementation Flow
//!
//! 1. Extract struct-level inference and prefix configuration.
//! 2. Parse named fields using [`field_parser::parse_fields`].
//! 3. Generate per-field code fragments with [`process_field`].
//! 4. Collect and merge any compile-time errors.
//! 5. Emit a single `impl` block containing all generated snippets.
//!
//! The final result is returned as a `proc_macro2::TokenStream`
//! ready to be consumed by the derive entry point in `lib.rs`.

use quote::quote;
use syn::{DeriveInput, Error};

use crate::{
    field_parser::process_field,
    struct_config::parse_struct_level_config,
};

/// Main entry point for generating the `impl ApplyOverrides` block.
///
/// This function orchestrates parsing of the input struct and delegates
/// actual field-level code generation to the [`field_parser`] module.
///
/// # Arguments
/// * `input` - Parsed AST of the struct annotated with `#[derive(ApplyOverrides)]`.
///
/// # Returns
/// A `TokenStream` representing either:
/// - a complete `impl` block for `ApplyOverrides`, or
/// - `compile_error!` invocations if parsing failed.
///
/// # Errors
/// Returns a `syn::Error` if the struct shape is invalid (e.g., not named fields).
pub fn generate_impl(input: &DeriveInput) -> Result<proc_macro2::TokenStream, Error> {
    // The name of the struct being processed (e.g., `CLIArgs`)
    let name = &input.ident;

    // Parse and collect struct-level configuration:
    //
    // This reads the `#[apply_overrides(...)]` attribute attached to the struct and extracts:
    //   • `infer_keys` — whether to automatically infer override keys for unannotated fields.
    //   • `prefix`     — an optional key prefix applied to all inferred field names.
    //
    // Additionally, `parse_struct_level_config()` may return one or more `syn::Error`s if
    // the attribute contains invalid syntax or unsupported options. These errors are collected
    // into `struct_errors` and merged into the shared accumulator below.
    let (struct_infer, struct_prefix, struct_errors) = parse_struct_level_config(input);
    let mut errors = struct_errors;

    // Extract all named fields from the struct (enforces named field constraint)
    let fields = match super::field_parser::parse_fields(input) {
        Ok(f) => f,
        Err(e) => return Err(e), // bubble up early if the struct itself is malformed
    };

    // Collect compile-time parsing errors and generated per-field snippets
    let mut generated = Vec::new();

    // Process each field in order — this preserves the declaration order,
    //     which improves debug readability in generated code.
    for field in fields {
        if let Some(code) =
            process_field(field, struct_infer, struct_prefix.as_deref(), &mut errors)
        {
            generated.push(code);
        }
    }

    // If any struct-level or field-level errors occurred, emit all as compile errors.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return Ok(quote! { #(#compile_errors)* });
    }

    // Assemble the final code block.
    //
    // Note: We intentionally use a fully-qualified trait path (`::override_key_core::ApplyOverrides`)
    // to remove the requirement for downstream crates to import the trait explicitly.
    // This guarantees stable linkage across crates and simplifies usage.
    Ok(quote! {
        impl ::override_key_core::ApplyOverrides for #name {
            fn apply_overrides(
                &self,
                mut builder: config::ConfigBuilder<config::builder::DefaultState>,
            ) -> Result<config::ConfigBuilder<config::builder::DefaultState>, config::ConfigError> {
                // auto-generated per-field override logic
                #(#generated)*
                Ok(builder)
            }
        }
    })
}