//! # field_parser/utils.rs
//!
//! This module provides **shared helper utilities** used across all
//! field-level parsing and code-generation modules in the
//! `#[derive(ApplyOverrides)]` procedural macro.
//!
//! ## Responsibilities
//!
//! 1. **Error accumulation** — via [`push_error`], ensuring that all parse issues
//!    are collected and reported as `compile_error!()`s instead of panics.
//! 2. **Key strategy merging** — via [`merge_with_struct_defaults`], which merges
//!    field-specific and struct-level inference rules.
//! 3. **Key generation** — via [`make_key_literal`], which computes the final
//!    configuration key string (replacing `_` with `.` and applying optional prefixes).
//! 4. **Type inspection** — via [`is_option_type`] to detect optional fields for safe codegen.
//! 5. **Code snippet generation** — via [`build_override_snippet`], which emits the final
//!    `builder.set_override()` calls for each field.
//!
//! ## Module Role
//!
//! This module is **pure utility** — it contains no syntax parsing or AST traversal logic.
//! Instead, it focuses on deterministic transformations and helper functionality used by
//! higher-level components (`field_parser`, `builder_gen`).
//!
//! ## Error Handling
//!
//! - All functions are panic-free and return `Option` or emit errors through the shared
//!   accumulator (`Vec<syn::Error>`).
//! - All errors are generated with [`syn::Error::new_spanned`] so they attach to the
//!   offending syntax node, producing precise compiler diagnostics.
//!
//! ## Example (Generated Code)
//!
//! For a field:
//! ```ignore
//! #[override_key = "iproyal.endpoint"]
//! pub iproyal_endpoint: Option<String>,
//! ```
//!
//! The resulting code snippet emitted by [`build_override_snippet`] is equivalent to:
//!
//! ```ignore
//! if let Some(v) = &self.iproyal_endpoint {
//!     builder = builder.set_override("iproyal.endpoint", v.clone())?;
//! }
//! ```

use quote::quote;
use syn::{Error, Type, PathArguments, LitStr};

use crate::types::{FieldOverrideMeta, KeyStrategy};

/// Pushes a new [`syn::Error`] into the shared error accumulator.
///
/// # Parameters
/// * `errors` — A mutable list of collected errors for multi-diagnostic reporting.
/// * `span_src` — The syntax node to which the diagnostic should be attached.
/// * `msg` — A human-readable error message.
///
/// # Design
/// - Always uses [`syn::Error::new_spanned`] to preserve source span precision.
/// - Never panics; can be called freely from any parsing or validation stage.
pub fn push_error<T: quote::ToTokens>(errors: &mut Vec<Error>, span_src: &T, msg: &str) {
    errors.push(Error::new_spanned(span_src, msg));
}

/// Merges field-level override metadata with struct-level inference configuration.
///
/// This is the **decision point** that determines how a field’s configuration key
/// will be computed:
///
/// - Explicit field attribute → [`KeyStrategy::Explicit`]
/// - Field-level inference → [`KeyStrategy::Inferred`]
/// - Struct-level inference (no field attr) → [`KeyStrategy::Inferred`]
///
/// # Parameters
/// * `field_meta` — Result of parsing the field’s `#[override_key(...)]` attribute.
/// * `struct_infer` — Whether `#[apply_overrides(infer_keys)]` was set.
/// * `struct_prefix` — Optional struct-level prefix (e.g. `"iproyal"`).
///
/// # Returns
/// `Some(KeyStrategy)` if the field should generate code, or `None` if the field
/// should be ignored (no applicable rule).
pub fn merge_with_struct_defaults(
    field_meta: FieldOverrideMeta,
    struct_infer: bool,
    struct_prefix: Option<&str>,
) -> Option<KeyStrategy> {
    match field_meta {
        // Explicit attribute — always wins
        FieldOverrideMeta::Explicit(lit) => Some(KeyStrategy::Explicit(lit)),

        // Field-level infer with optional prefix
        FieldOverrideMeta::Infer { prefix } => Some(KeyStrategy::Inferred {
            prefix: prefix.or(struct_prefix.map(str::to_owned)),
        }),

        // No attribute but struct-level inference enabled
        FieldOverrideMeta::None if struct_infer => Some(KeyStrategy::Inferred {
            prefix: struct_prefix.map(str::to_owned),
        }),

        // No attribute and no struct-level inference
        _ => None,
    }
}

/// Constructs a [`LitStr`] key literal for a field.
///
/// - Replaces underscores (`_`) in the field name with dots (`.`).
/// - Applies prefix if present.
/// - Returns a string literal suitable for use in generated code.
///
/// # Example
/// ```ignore
/// make_key_literal("iproyal_timeout", &Inferred { prefix: Some("iproyal") })
/// → "iproyal.iproyal.timeout"
/// ```
pub fn make_key_literal(ident: &syn::Ident, strategy: &KeyStrategy) -> LitStr {
    match strategy {
        // Explicit: use provided literal as-is
        KeyStrategy::Explicit(lit) => lit.clone(),

        // Inferred: construct from field name + optional prefix
        KeyStrategy::Inferred { prefix } => {
            let mut key = ident.to_string().replace('_', ".");
            if let Some(pre) = prefix.as_deref() {
                // only prepend prefix if non-empty
                if !pre.is_empty() {
                    key = format!("{}.{}", pre, key);
                }
            }
            LitStr::new(&key, ident.span())
        }
    }
}

/// Determines whether a field type is an [`Option<T>`].
///
/// Used to decide whether code generation should wrap the `builder.set_override`
/// call inside an `if let Some(...)` guard.
///
/// # Implementation Detail
/// Checks the last segment of the type path for:
/// ```ignore
/// Option<...>
/// ```
/// and ensures it has angle-bracketed type arguments.
pub fn is_option_type(ty: &Type) -> bool {
    matches!(ty, Type::Path(tp)
        if tp.path.segments.last().map_or(false, |seg| {
            seg.ident == "Option" && matches!(seg.arguments, PathArguments::AngleBracketed(_))
        }))
}

/// Builds the final code snippet for overriding a single field.
///
/// This emits actual code that will appear inside the generated `apply_overrides()`
/// implementation for the user’s struct.
///
/// # Example Output
///
/// ```ignore
/// if let Some(v) = &self.iproyal_token {
///     builder = builder.set_override("iproyal.token", v.clone())?;
/// }
/// ```
///
/// # Behavior
/// - Wraps value access in `if let Some` if the field type is `Option`.
/// - Otherwise generates an unconditional call.
pub fn build_override_snippet(
    ident: &syn::Ident,
    ty: &Type,
    key: &LitStr,
) -> proc_macro2::TokenStream {
    if is_option_type(ty) {
        // Optional field → only override if value is present
        quote! {
            if let Some(v) = &self.#ident {
                builder = builder.set_override(#key, v.clone())?;
            }
        }
    } else {
        // Non-optional field → always override
        quote! {
            builder = builder.set_override(#key, self.#ident.clone())?;
        }
    }
}