//! # field_parser/explicit.rs
//!
//! This module implements parsing for **explicit field mappings** in the
//! `#[derive(ApplyOverrides)]` macro system.
//!
//! Specifically, it handles the canonical form:
//!
//! ```ignore
//! #[override_key = "iproyal.token"]
//! ```
//!
//! which directly maps a struct field to a fixed configuration key string.
//!
//! ## Responsibilities
//!
//! - Validate that the left-hand identifier is literally `override_key`.
//! - Extract and validate the right-hand side as a string literal (`LitStr`).
//! - Convert it into a [`FieldOverrideMeta::Explicit`] variant for the caller.
//!
//! ## Example
//!
//! ```ignore
//! #[override_key = "netnut.endpoint"]
//! pub netnut_endpoint: Option<String>;
//! ```
//!
//! ➜ becomes ➜
//!
//! ```ignore
//! FieldOverrideMeta::Explicit("netnut.endpoint")
//! ```
//!
//! ## Error Conditions
//!
//! | Condition | Example | Result |
//! |------------|----------|--------|
//! | Wrong identifier | `#[something_else = "foo"]` | Emits error: “expected `override_key` identifier before `=`” |
//! | Non-string literal | `#[override_key = 123]` | Emits error: “expected string literal, e.g. #[override_key = \"custom.path\"]” |
//! | Non-literal RHS | `#[override_key = SOME_CONST]` | Same as above |
//!
//! ## Safety and Design Notes
//!
//! - This parser **never panics** — all invalid forms result in
//!   `FieldOverrideMeta::Invalid` with accumulated `syn::Error`s.
//! - It intentionally does *not* interpret interpolated or concatenated strings
//!   (like `"foo".to_string()`), since those are not literal expressions and
//!   cannot be evaluated at compile time.

use syn::{Error, Expr, ExprLit, LitStr, MetaNameValue};

use crate::types::FieldOverrideMeta;
use super::utils::push_error;

/// Parses a field-level attribute of the form:
///
/// ```rust,ignore
/// #[override_key = "some.path"]
/// ```
///
/// # Behavior
/// - Confirms that the key on the left-hand side of `=` is exactly `override_key`.
/// - Extracts the right-hand side as a string literal (`LitStr`).
/// - Produces a [`FieldOverrideMeta::Explicit`] result if valid.
/// - Otherwise pushes a syntax error into the shared error accumulator.
///
/// # Arguments
/// * `nv` — A [`MetaNameValue`] node parsed from the `syn` AST (represents `key = value`).
/// * `errors` — A mutable list of accumulated [`syn::Error`]s for multi-error reporting.
///
/// # Returns
/// * [`FieldOverrideMeta::Explicit`] on success.
/// * [`FieldOverrideMeta::Invalid`] on malformed or non-literal input.
pub fn parse_field_explicit(nv: &MetaNameValue, errors: &mut Vec<Error>) -> FieldOverrideMeta {
    // Validate that we are parsing exactly `#[override_key = ...]`
    if !nv.path.is_ident("override_key") {
        push_error(errors, &nv.path, "expected `override_key` identifier before `=`");
        return FieldOverrideMeta::Invalid;
    }

    // Right-hand side must be a literal string expression
    match &nv.value {
        // Correct case: string literal
        Expr::Lit(ExprLit { lit: syn::Lit::Str(lit), .. }) => {
            FieldOverrideMeta::Explicit(LitStr::new(&lit.value(), lit.span()))
        }

        // Any other literal form (integer, bool, float, etc.)
        other => {
            push_error(
                errors,
                other,
                "expected string literal, e.g. #[override_key = \"custom.path\"]",
            );
            FieldOverrideMeta::Invalid
        }
    }
}
