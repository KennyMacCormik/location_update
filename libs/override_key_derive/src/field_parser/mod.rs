//! # field_parser
//!
//! This module is responsible for **parsing and interpreting field-level attributes**
//! within structs annotated by `#[derive(ApplyOverrides)]`.
//!
//! Specifically, it handles:
//!
//! - `#[override_key = "some.path"]` — explicit override mapping
//! - `#[override_key(infer[, prefix = "..."])]` — inferred key mapping
//! - Unannotated fields — optionally inferred from struct-level defaults
//!
//! ## Role in the Pipeline
//!
//! The field parser sits between the raw `syn::DeriveInput` and the final code
//! emission stage (`builder_gen`). Its primary responsibility is to produce a
//! `proc_macro2::TokenStream` for each field — a snippet of Rust code that looks like:
//!
//! ```ignore
//! if let Some(v) = &self.field_name {
//!     builder = builder.set_override("iproyal.token", v.clone())?;
//! }
//! ```
//!
//! ## Module Layout
//!
//! - [`explicit`] — parses explicit attributes like `#[override_key = "iproyal.token"]`
//! - [`infer`] — parses inference attributes like `#[override_key(infer, prefix = "netnut")]`
//! - [`utils`] — provides shared helpers (error handling, key generation, etc.)
//!
//! ## Public API
//!
//! | Function | Description |
//! |-----------|-------------|
//! | [`parse_fields`] | Extracts named struct fields (enforces struct-only use). |
//! | [`process_field`] | Produces codegen for one field, merging struct-level config. |
//!
//! ## Error Handling
//!
//! - All invalid attributes generate `syn::Error` entries via `push_error()`.
//! - The macro **never panics**; errors are later rendered as `compile_error!()` tokens.
//!
//! ## Supported Syntax
//!
//! ```rust,ignore
//! #[override_key = "iproyal.token"]
//! #[override_key(infer)]
//! #[override_key(infer, prefix = "netnut")]
//! ```
//!
//! ## Example
//!
//! Given:
//!
//! ```ignore
//! #[apply_overrides(infer_keys, prefix = "iproyal")]
//! pub struct CLIArgs {
//!     #[override_key = "iproyal.endpoint"]
//!     pub iproyal_endpoint: Option<String>,
//!
//!     #[override_key(infer, prefix = "netnut")]
//!     pub netnut_token: Option<String>,
//!
//!     pub iproyal_timeout: Option<String>,
//! }
//! ```
//!
//! the resulting generated code snippets would be equivalent to:
//!
//! ```ignore
//! if let Some(v) = &self.iproyal_endpoint {
//!     builder = builder.set_override("iproyal.endpoint", v.clone())?;
//! }
//! if let Some(v) = &self.netnut_token {
//!     builder = builder.set_override("netnut.netnut_token", v.clone())?;
//! }
//! if let Some(v) = &self.iproyal_timeout {
//!     builder = builder.set_override("iproyal.iproyal.timeout", v.clone())?;
//! }
//! ```

use syn::{Attribute, Data, DeriveInput, Error, Field, Fields};

mod explicit;
mod infer;
mod utils;

use explicit::parse_field_explicit;
use infer::parse_field_infer_list;
use utils::*;

use crate::types::{FieldOverrideMeta, KeyStrategy};

/// Extracts named fields from a struct definition.
///
/// # Errors
/// Returns a `syn::Error` if:
/// - The input type is not a struct.
/// - The struct does not have named fields.
///
/// # Example
/// ```rust,ignore
/// let fields = parse_fields(&input)?;
/// for field in fields {
///     println!("Field: {}", field.ident.as_ref().unwrap());
/// }
/// ```
pub fn parse_fields(
    input: &DeriveInput,
) -> Result<&syn::punctuated::Punctuated<Field, syn::token::Comma>, Error> {
    match &input.data {
        // Struct with fields
        Data::Struct(data_struct) => match &data_struct.fields {
            // ✅ Named fields are required for ApplyOverrides
            Fields::Named(named) => Ok(&named.named),

            // Tuple or unit structs not supported
            _ => Err(Error::new_spanned(
                &data_struct.fields,
                "ApplyOverrides requires a struct with named fields",
            )),
        },

        // Non-structs (enums, unions)
        _ => Err(Error::new_spanned(
            &input.ident,
            "ApplyOverrides can only be used on structs",
        )),
    }
}

/// Processes a single struct field and produces its generated code snippet.
///
/// # Parameters
/// - `field`: The AST node representing the struct field.
/// - `struct_infer`: Whether struct-level `infer_keys` is enabled.
/// - `struct_prefix`: Optional prefix from the struct-level attribute.
/// - `errors`: Mutable vector for collecting parsing errors.
///
/// # Returns
/// - `Some(TokenStream)` containing builder override code if successful.
/// - `None` if the field is not relevant or has no attribute.
///
/// # Example Output
/// ```rust,ignore
/// if let Some(v) = &self.iproyal_endpoint {
///     builder = builder.set_override("iproyal.endpoint", v.clone())?;
/// }
/// ```
pub fn process_field(
    field: &Field,
    struct_infer: bool,
    struct_prefix: Option<&str>,
    errors: &mut Vec<Error>,
) -> Option<proc_macro2::TokenStream> {
    // Field identifier (e.g., iproyal_token)
    let ident = field.ident.as_ref()?;
    let ty = &field.ty;

    // Find `#[override_key(...)]` attribute if present
    let attr = field.attrs.iter().find(|a| a.path().is_ident("override_key"));

    // Parse field attribute → FieldOverrideMeta
    let field_meta = parse_field_override_meta(attr, errors);

    // Combine field meta + struct-level config into final strategy
    let strategy = merge_with_struct_defaults(field_meta, struct_infer, struct_prefix)?;

    // Compute key literal string ("iproyal.token" or inferred variant)
    let key = make_key_literal(ident, &strategy);

    // Emit final builder code for this field
    Some(build_override_snippet(ident, ty, &key))
}

// ------------------------------------------------------------------------------------------------
// Dispatcher: Selects appropriate parsing strategy for `#[override_key(...)]`
// ------------------------------------------------------------------------------------------------

/// Parses a field’s `#[override_key(...)]` attribute into a [`FieldOverrideMeta`].
///
/// This acts as a **dispatcher**, deciding whether to call:
/// - [`parse_field_explicit`] for `#[override_key = "..."]`
/// - [`parse_field_infer_list`] for `#[override_key(infer[, prefix = "..."])]`
///
/// # Error Handling
/// Invalid forms (e.g. `#[override_key("...")]`) are recognized and
/// emit descriptive `compile_error!` diagnostics via [`push_error`].
fn parse_field_override_meta(attr: Option<&Attribute>, errors: &mut Vec<Error>) -> FieldOverrideMeta {
    // No attribute — handled later by struct-level inference
    let Some(attr) = attr else {
        return FieldOverrideMeta::None;
    };

    match &attr.meta {
        // Explicit form: #[override_key = "iproyal.token"]
        syn::Meta::NameValue(nv) => parse_field_explicit(nv, errors),

        // Inferred form: #[override_key(infer[, prefix = "..."])]
        syn::Meta::List(list) => {
            // Handle common mistake #[override_key("...")] gracefully
            if list.tokens.to_string().starts_with('"') {
                push_error(
                    errors,
                    attr,
                    "invalid #[override_key(\"...\")] form — use #[override_key = \"...\"] instead",
                );
                FieldOverrideMeta::Invalid
            } else {
                parse_field_infer_list(attr, errors)
            }
        }

        // Missing argument form: #[override_key]
        syn::Meta::Path(_) => {
            push_error(
                errors,
                attr,
                "missing form — expected #[override_key = \"...\"] \
                 or #[override_key(infer[, prefix = \"...\"])]",
            );
            FieldOverrideMeta::Invalid
        }
    }
}