//! # types.rs
//!
//! Shared internal data structures used by the `#[derive(ApplyOverrides)]`
//! procedural macro.
//!
//! This module defines two core enums that represent the **semantic
//! results of attribute parsing** before code generation occurs:
//!
//! - [`FieldOverrideMeta`] → raw parse result for `#[override_key(...)]` attributes
//! - [`KeyStrategy`] → normalized representation of how to compute the final key
//!
//! These types form the boundary between the *parsing* stage (in `field_parser.rs`)
//! and the *code generation* stage (in `builder_gen.rs`).
//!
//! They intentionally contain no procedural-macro or `syn` logic beyond minimal data storage,
//! allowing `builder_gen` to focus purely on emitting code rather than understanding
//! the original syntax tree.
//!
//! ## Example Parsing Flow
//!
//! ```text
//! #[override_key = "iproyal.token"]
//!         │
//!         ▼
//! FieldOverrideMeta::Explicit("iproyal.token")
//!         │
//!         ▼
//! KeyStrategy::Explicit("iproyal.token")
//! ```
//!
//! or
//!
//! ```text
//! #[apply_overrides(infer_keys, prefix = "iproyal")]
//! pub iproyal_timeout: Option<String>
//!         │
//!         ▼
//! FieldOverrideMeta::None  (no explicit override_key attribute)
//! struct-level infer_keys = true
//! struct-level prefix = "iproyal"
//!         │
//!         ▼
//! KeyStrategy::Inferred { prefix: Some("iproyal") }
//! ```
//!
//! ## Design Principles
//!
//! - Keep data immutable and trivially clonable (`LitStr` and `String` only).
//! - Do **not** depend on `syn::Attribute`, `Meta`, or parsing logic here.
//! - Treat this as an “intermediate representation” layer between parsing and codegen.
//!
//! ## Usage Summary
//!
//! - [`field_parser`] produces [`FieldOverrideMeta`]
//! - [`builder_gen`] consumes it, merges it with struct-level defaults,
//!   and emits a [`KeyStrategy`] to drive code emission.

use syn::LitStr;

/// Represents how a single field-level `#[override_key(...)]` attribute was parsed.
///
/// This enum reflects *exactly what was found* during parsing,
/// before any struct-level defaults or inference logic are applied.
///
/// ## Variants
///
/// - `Explicit(LitStr)` — The attribute provided a concrete key string,
///   e.g. `#[override_key = "iproyal.token"]`.
///
/// - `Infer { prefix }` — The attribute requested key inference, optionally with
///   a per-field prefix, e.g. `#[override_key(infer, prefix = "netnut")]`.
///
/// - `Invalid` — The attribute was present but malformed.
///   (The macro will emit a compile error but continue processing other fields.)
///
/// - `None` — No attribute was found on this field.
pub enum FieldOverrideMeta {
    /// Explicit key provided by the user.
    Explicit(LitStr),

    /// Key should be inferred automatically; may include custom prefix.
    Infer {
        /// Optional string prefix (e.g. `"netnut"`).
        prefix: Option<String>,
    },

    /// Parsing failed — invalid attribute form or syntax.
    Invalid,

    /// No override attribute was found.
    None,
}

/// Represents the *finalized strategy* for computing a key after
/// merging field-level and struct-level configuration.
///
/// This type drives the actual code generation in [`builder_gen`].
///
/// ## Variants
///
/// - `Explicit(LitStr)` — Use the given key string verbatim.
/// - `Inferred { prefix }` — Construct a key by replacing underscores
///   in the field name with dots (`_` → `.`), optionally prepending a prefix.
///
/// Example:
/// ```text
/// prefix = Some("iproyal")
/// field ident = "region_id"
/// → "iproyal.region.id"
/// ```
pub enum KeyStrategy {
    /// Use a literal key string directly.
    Explicit(LitStr),

    /// Infer key from field name and optional prefix.
    Inferred {
        /// Optional prefix (e.g. `"iproyal"`).
        prefix: Option<String>,
    },
}
