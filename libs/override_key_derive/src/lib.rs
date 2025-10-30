//! # override_key_derive
//!
//! Procedural macro crate implementing `#[derive(ApplyOverrides)]`.
//!
//! This crate is a **compile-time companion** to the runtime crate
//! [`override_key_core`](https://docs.rs/override_key_core), which defines
//! the actual [`ApplyOverrides`] trait.
//!
//! The macro generates an implementation of that trait for configuration-like
//! structs (such as CLI argument definitions) and automates calling
//! `builder.set_override(key, value)` for every field annotated with
//! `#[override_key(...)]`.
//!
//! ## Crate Relationship
//!
//! - **This crate (`override_key_derive`)** runs entirely at compile-time
//!   and outputs Rust code implementing the trait.
//! - **The [`override_key_core`] crate** provides the runtime definition of the
//!   `ApplyOverrides` trait itself and must be linked by downstream crates
//!   using this macro.
//!
//! In practice, downstream users import **both crates**:
//!
//! ```rust,ignore
//! use override_key_core::ApplyOverrides;   // brings the trait into scope
//! use override_key_derive::ApplyOverrides; // enables the derive macro
//!
//! #[derive(ApplyOverrides)]
//! #[apply_overrides(infer_keys, prefix = "iproyal")]
//! pub struct CLIArgs {
//!     #[override_key = "iproyal.endpoint"]
//!     pub iproyal_endpoint: Option<String>,
//! }
//! ```
//!
//! This ensures that the generated implementation correctly links to
//! `override_key_core::ApplyOverrides` and that `.apply_overrides()` is
//! available at runtime.
//!
//! Supported forms:
//! ```ignore
//! #[derive(ApplyOverrides)]
//! #[apply_overrides(infer_keys, prefix = "iproyal")]
//! pub struct CLIArgs {
//!     // Explicitly mapped to "iproyal.endpoint"
//!     #[override_key = "iproyal.endpoint"]
//!     pub iproyal_endpoint: Option<String>,
//!
//!     // Per-field inference with a custom prefix "netnut"
//!     // field `netnut_token` → key "netnut.netnut.token"
//!     #[override_key(infer, prefix = "netnut")]
//!     pub netnut_token: Option<String>,
//!
//!     // Struct-level inference + prefix
//!     // field `iproyal_timeout` → key "iproyal.iproyal.timeout"
//!     pub iproyal_timeout: Option<String>,
//!
//!     // Struct-level inference + prefix
//!     // field `region_id` → key "iproyal.region.id"
//!     pub region_id: Option<u32>,
//! }
//! ```
//!
//! The generated implementation looks roughly like this:
//!
//! ```ignore
//! impl override_key_core::ApplyOverrides for CLIArgs {
//!     fn apply_overrides(
//!         &self,
//!         mut builder: config::ConfigBuilder<config::builder::DefaultState>,
//!     ) -> Result<config::ConfigBuilder<config::builder::DefaultState>, config::ConfigError> {
//!         if let Some(v) = &self.iproyal_endpoint {
//!             builder = builder.set_override("iproyal.endpoint", v.clone())?;
//!         }
//!         if let Some(v) = &self.netnut_token {
//!             builder = builder.set_override("netnut.netnut_token", v.clone())?;
//!         }
//!         if let Some(v) = &self.iproyal_timeout {
//!             builder = builder.set_override("iproyal.timeout", v.clone())?;
//!         }
//!         Ok(builder)
//!     }
//! }
//! ```
//!
//! ## Design Overview
//!
//! - `lib.rs` serves as the **entry point** only — it parses the input syntax tree
//!   and delegates to specialized submodules for actual work.
//!
//! - Logic is split across four internal modules:
//!   - [`builder_gen`] → orchestrates generation of the `impl ApplyOverrides` block.
//!   - [`struct_config`] → parses `#[apply_overrides(...)]` struct-level attributes.
//!   - [`field_parser`] → handles `#[override_key(...)]` attributes on individual fields.
//!   - [`types`] → defines shared enums used across the pipeline.
//!
//! This separation ensures **each module has a single responsibility** and keeps
//! `syn` parsing complexity isolated.
//!
//! ## Safety and Compilation Notes
//!
//! - The crate is declared with `proc-macro = true`, so it cannot export
//!   runtime symbols or be linked directly — it only provides compiler plugins.
//!
//! - It relies on `syn v2+` API (`attr.meta` and `parse_nested_meta`),
//!   compatible with the Rust 2024 edition.
//!
//! - All parsing errors are collected as `syn::Error` instances and emitted
//!   via `compile_error!()` rather than panicking, ensuring clean compiler diagnostics.
//!
//! ## Runtime Dependencies
//!
//! This macro assumes that the target crate links against `override_key_core`
//! and that the generated `impl` refers to the canonical path
//! `::override_key_core::ApplyOverrides`.
//!
//! If you rename the core crate in `Cargo.toml`, you must adjust the import path
//! via `extern crate` or a re-export to maintain consistency.
//!
//! ## Future Extensibility
//!
//! Future extensions should *not* be added directly here. Instead:
//! - Add new parsing logic to `field_parser/`.
//! - Add new configuration hints to `struct_config.rs`.
//! - Update codegen in `builder_gen.rs` if the emitted code shape changes.
//!
//! ```

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

mod builder_gen;
mod struct_config;
mod types;
mod field_parser;

use builder_gen::generate_impl;

/// Derive macro entry point for `#[derive(ApplyOverrides)]`.
///
/// This macro inspects the annotated struct, reads its field and struct-level
/// attributes (`#[override_key]` and `#[apply_overrides]`), and generates
/// an `impl ApplyOverrides` block that programmatically calls
/// `builder.set_override(key, value)` for all eligible fields.
///
/// ### Error Handling
/// All syntax and semantic issues are captured as `syn::Error`s, collected,
/// and emitted using `compile_error!()` to ensure graceful compilation.
///
/// ### Panics
/// This function **must never panic**; if an unrecoverable condition occurs,
/// prefer returning a `syn::Error` so the compiler can render a human-friendly
/// diagnostic.
#[proc_macro_derive(ApplyOverrides, attributes(override_key, apply_overrides))]
pub fn derive_apply_overrides(input: TokenStream) -> TokenStream {
    // Step 1: Parse compiler-provided token stream into a syn-compatible AST.
    let input = parse_macro_input!(input as DeriveInput);

    // Step 2: Delegate to builder_gen::generate_impl for structured code generation.
    match generate_impl(&input) {
        // Step 3a: Return generated token stream.
        Ok(tokens) => tokens.into(),
        // Step 3b: On syntax error, convert to compile_error! token.
        Err(e) => e.to_compile_error().into(),
    }
}
