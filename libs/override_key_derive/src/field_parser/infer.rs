//! # field_parser/infer.rs
//!
//! This module implements parsing for **inferred key** forms of
//! `#[override_key(...)]` attributes within `#[derive(ApplyOverrides)]`.
//!
//! ## Supported Forms
//!
//! ```ignore
//! #[override_key(infer)]
//! #[override_key(infer, prefix = "netnut")]
//! ```
//!
//! These tell the macro to derive the configuration key automatically from
//! the field name, optionally adding a prefix (e.g. `"netnut"`).
//!
//! ## Example
//!
//! ```ignore
//! #[override_key(infer, prefix = "netnut")]
//! pub token: Option<String>;
//! ```
//!
//! ➜ becomes ➜
//!
//! ```ignore
//! FieldOverrideMeta::Infer { prefix: Some("netnut") }
//! ```
//!
//! ## Error Conditions
//!
//! | Condition | Example | Result |
//! |------------|----------|--------|
//! | Missing `infer` keyword | `#[override_key(prefix = "foo")]` | Emits error: “missing `infer` keyword” |
//! | Unexpected token | `#[override_key(foo)]` | Emits error: “unexpected token … expected `infer` or `prefix = ...`” |
//! | Invalid prefix literal | `#[override_key(infer, prefix = 123)]` | Emits error from `syn` parse |
//!
//! ## Implementation Notes
//!
//! - Uses `syn::Attribute::parse_nested_meta` (v2 API) to walk nested meta items.
//! - Returns a [`FieldOverrideMeta::Infer`] variant on success.
//! - Always validates that the keyword `infer` is explicitly present.
//!
//! ## Design Rationale
//!
//! The explicit requirement of the `infer` token avoids ambiguity between:
//! ```ignore
//! #[override_key(infer)]
//! #[override_key(prefix = "foo")] // ← invalid without `infer`
//! ```
//!
//! This design makes the syntax more readable and future-proof against
//! additional parameters being introduced later.

use syn::{Attribute, Error, LitStr};
use syn::meta::ParseNestedMeta;

use crate::types::FieldOverrideMeta;
use super::utils::push_error;

/// Parses `#[override_key(infer[, prefix = "..."])]` attributes.
///
/// # Behavior
/// - Extracts the presence of the `infer` flag.
/// - Optionally captures a string `prefix` literal.
/// - Returns [`FieldOverrideMeta::Infer`] if valid.
/// - Accumulates syntax errors otherwise.
///
/// # Example
/// ```rust,ignore
/// #[override_key(infer, prefix = "iproyal")]
/// pub api_token: Option<String>;
/// ```
///
/// ➜
/// ```ignore
/// FieldOverrideMeta::Infer { prefix: Some("iproyal") }
/// ```
pub fn parse_field_infer_list(attr: &Attribute, errors: &mut Vec<Error>) -> FieldOverrideMeta {
    let mut prefix = None;
    let mut infer = false;

    // Walk each token inside the parentheses (...)
    let res = attr.parse_nested_meta(|meta: ParseNestedMeta| {
        if meta.path.is_ident("infer") {
            // Mark presence of `infer`
            infer = true;
            Ok(())
        } else if meta.path.is_ident("prefix") {
            // Parse prefix literal: prefix = "some.value"
            let lit: LitStr = meta.value()?.parse()?;
            prefix = Some(lit.value());
            Ok(())
        } else {
            // Unexpected argument → human-readable diagnostic
            Err(meta.error(
                r#"unexpected token in #[override_key(...)] — expected `infer` or `prefix = "..."`"#,
            ))
        }
    });

    // Register parsing error from syn if meta traversal failed
    if let Err(e) = res {
        push_error(errors, attr, &format!("invalid #[override_key(...)] syntax: {}", e));
    }

    // Ensure that `infer` was explicitly present
    if !infer {
        push_error(
            errors,
            attr,
            "missing `infer` keyword — expected #[override_key(infer[, prefix = \"...\"])]",
        );
        FieldOverrideMeta::Invalid
    } else {
        FieldOverrideMeta::Infer { prefix }
    }
}