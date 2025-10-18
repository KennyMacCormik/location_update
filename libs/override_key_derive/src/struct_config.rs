//! # struct_config.rs
//!
//! This module handles **struct-level configuration parsing** for the
//! `#[derive(ApplyOverrides)]` procedural macro.
//!
//! It processes the `#[apply_overrides(...)]` attribute applied to a struct,
//! extracting global options that influence how field-level override keys are generated.
//!
//! ## Extracted Configuration
//!
//! 1. **`infer_keys`** — enables automatic key inference for fields that do not
//!    have an explicit `#[override_key(...)]` attribute.
//! 2. **`prefix`** — optional string that will be prepended to all inferred keys
//!    (e.g., `"iproyal"` → `"iproyal.timeout"`).
//!
//! ## Example
//!
//! ```ignore
//! #[apply_overrides(infer_keys, prefix = "iproyal")]
//! #[derive(ApplyOverrides)]
//! pub struct CLIArgs {
//!     pub iproyal_token: Option<String>,
//!     pub iproyal_timeout: Option<String>,
//! }
//! ```
//!
//! This configuration instructs the derive macro to:
//! - Automatically infer keys for all unannotated fields (`infer_keys`).
//! - Prepend `"iproyal."` to those inferred keys (`prefix = "iproyal"`).
//!
//! Resulting keys: `"iproyal.token"` and `"iproyal.timeout"`.
//!
//! ## Return Value
//!
//! ```ignore
//! (bool, Option<String>, Vec<Error>)
//! ```
//!
//! - **`bool`** → whether `infer_keys` was specified.
//! - **`Option<String>`** → the parsed prefix string, if present.
//! - **`Vec<syn::Error>`** → accumulated syntax or semantic errors to be surfaced
//!   as `compile_error!()`s later during code generation.
//!
//! ## Behavior Summary
//!
//! | Attribute Form | Effect |
//! |----------------|--------|
//! | `#[apply_overrides(infer_keys)]` | Enables inference for all fields |
//! | `#[apply_overrides(prefix = "foo")]` | Applies `"foo."` prefix to inferred keys |
//! | `#[apply_overrides(infer_keys, prefix = "foo")]` | Enables both behaviors |
//!
//! - If no `#[apply_overrides(...)]` attribute is present, defaults to `(false, None, vec![])`.
//! - Invalid tokens (e.g., `#[apply_overrides("bad")]`) produce `syn::Error` instances
//!   but do **not** cause an immediate panic; errors are accumulated and reported later.
//! - Compatible with **Rust 2024** and **syn v2+** (uses `ParseNestedMeta` API).

use syn::{DeriveInput, Error, LitStr};
use syn::meta::ParseNestedMeta;

/// Parses the `#[apply_overrides(...)]` struct-level attribute.
///
/// This function scans all attributes attached to the struct, looking for
/// `#[apply_overrides(...)]`, and extracts its parameters (`infer_keys` and `prefix`).
///
/// # Arguments
///
/// * `input` — The `syn::DeriveInput` representation of the struct under analysis.
///
/// # Returns
///
/// ```ignore
/// (infer_keys_enabled, optional_prefix, collected_errors)
/// ```
///
/// Example:
/// ```ignore
/// (true, Some("iproyal".to_string()), vec![])
/// ```
///
/// # Error Handling
///
/// - This function is **panic-free**.
/// - Any invalid arguments or unsupported syntax are reported via `syn::Error`
///   and collected in the returned `Vec<Error>`.
/// - It never returns `Err`; instead, errors are surfaced later as
///   `compile_error!` tokens in the generated output.
pub fn parse_struct_level_config(input: &DeriveInput) -> (bool, Option<String>, Vec<Error>) {
    // Accumulators for parsed options
    let mut infer_keys = false;       // default: disabled
    let mut prefix: Option<String> = None; // default: no prefix

    // Collector for any syntax/semantic errors we encounter while parsing.
    // We never panic; we return all errors for the caller to emit.
    let mut errors: Vec<Error> = Vec::new();

    // Walk all attributes attached to the struct
    for attr in &input.attrs {
        // Only care about: #[apply_overrides(...)]
        if !attr.path().is_ident("apply_overrides") {
            continue;
        }

        // Parse the nested meta inside #[apply_overrides(...)]
        // Example accepted forms:
        //   - infer_keys
        //   - prefix = "iproyal"
        //   - infer_keys, prefix = "iproyal"
        //
        // Any unrecognized token becomes a syn::Error we push into `errors`.
        if let Err(e) = attr.parse_nested_meta(|meta: ParseNestedMeta| {
            // Flag: infer unannotated field names into config keys
            if meta.path.is_ident("infer_keys") {
                infer_keys = true;
                return Ok(());
            }

            // Option: prefix = "some.namespace"
            if meta.path.is_ident("prefix") {
                // Move to the value side of `prefix = ...`, then parse a string literal
                let lit: LitStr = meta.value()?.parse()?;
                prefix = Some(lit.value());
                return Ok(());
            }

            // Anything else is considered invalid for this attribute
            Err(meta.error(r#"expected `infer_keys` or `prefix = "..."`"#))
        }) {
            // If parse_nested_meta returns Err, record it (don’t panic).
            errors.push(e);
        }
    }

    // Return parsed flags + any collected errors for the caller to surface
    (infer_keys, prefix, errors)
}