//! # override_key_core
//!
//! Core runtime support crate for the `#[derive(ApplyOverrides)]` procedural macro.
//!
//! This crate defines the **`ApplyOverrides` trait**, which is automatically implemented
//! by the `override_key_derive` macro. It provides the runtime interface for applying
//! configuration key/value overrides on top of a [`config::ConfigBuilder`].
//!
//! ---
//!
//! ## Overview
//!
//! The trait enables structs (typically representing CLI arguments or user-specified
//! configuration overrides) to programmatically modify a [`config::ConfigBuilder`]
//! instance.
//!
//! The derive macro automatically generates an implementation that:
//!
//! - Iterates over all fields of the struct.
//! - Detects those annotated with `#[override_key(...)]`.
//! - Invokes `builder.set_override(key, value)` for each one.
//!
//! The `builder` comes from the [`config`] crate, which is the canonical Rust
//! configuration loader (`config-rs`) — supporting layered configuration sources.
//!
//! ---
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use override_key_core::ApplyOverrides;
//!
//! #[derive(ApplyOverrides)]
//! #[apply_overrides(infer_keys, prefix = "iproyal")]
//! pub struct CLIArgs {
//!     #[override_key = "iproyal.endpoint"]
//!     pub iproyal_endpoint: Option<String>,
//!
//!     #[override_key(infer, prefix = "netnut")]
//!     pub netnut_token: Option<String>,
//!
//!     pub region_id: Option<u32>,
//! }
//!
//! fn main() -> Result<(), config::ConfigError> {
//!     let args = CLIArgs {
//!         iproyal_endpoint: Some("https://api.iproyal.com".into()),
//!         netnut_token: None,
//!         region_id: Some(123),
//!     };
//!
//!     let builder = config::Config::builder();
//!     let merged = args.apply_overrides(builder)?;
//!
//!     let cfg = merged.build()?;
//!     println!("final config: {:?}", cfg);
//!     Ok(())
//! }
//! ```
//!
//! ---
//!
//! ## Trait Contract
//!
//! ```ignore
//! pub trait ApplyOverrides {
//!     fn apply_overrides(
//!         &self,
//!         builder: config::ConfigBuilder<config::builder::DefaultState>,
//!     ) -> Result<config::ConfigBuilder<config::builder::DefaultState>, config::ConfigError>;
//! }
//! ```
//!
//! - `builder`: A [`config::ConfigBuilder`] representing the base configuration.
//! - Returns a modified builder with all applicable overrides applied.
//!
//! The derive macro ensures that this method **never panics**, and that
//! all builder calls are chained in a fallible manner (`?`).
//!
//! ---
//!
//! ## Relation to `override_key_derive`
//!
//! - This crate must be imported at runtime:
//!   ```toml
//!   [dependencies]
//!   override_key_core = { path = "../override_key_core" }
//!   ```
//!
//! - The procedural macro crate depends on this one only at **compile time**:
//!   ```toml
//!   [dependencies]
//!   syn = { version = "2", features = ["full"] }
//!   quote = "1"
//!
//!   [dev-dependencies]
//!   override_key_core = { path = "../override_key_core" }
//!   ```
//!
//! ---
//!
//! ## Safety Notes
//!
//! - This trait is pure Rust — no `unsafe` code is involved.
//! - It has **no external dependencies** other than [`config`].
//! - It provides a stable ABI for the `override_key_derive` macro to target.
//!
//! ---
//!
//! [`config`]: https://docs.rs/config/latest/config/
//! [`config::ConfigBuilder`]: https://docs.rs/config/latest/config/struct.ConfigBuilder.html

/// Core runtime trait used by the `#[derive(ApplyOverrides)]` macro.
///
/// Implemented automatically by the `override_key_derive` procedural macro.
/// This trait defines how a struct applies configuration key/value overrides
/// to an existing [`config::ConfigBuilder`].
pub trait ApplyOverrides {
    /// Applies all active field overrides onto the provided configuration builder.
    ///
    /// # Parameters
    /// * `builder` — A [`config::ConfigBuilder`] representing the base configuration state.
    ///
    /// # Returns
    /// A modified `ConfigBuilder` with overrides applied, or a `config::ConfigError`
    /// if any override operation fails (e.g., invalid type conversion).
    ///
    /// # Example
    /// ```ignore
    /// let builder = config::Config::builder();
    /// let merged = args.apply_overrides(builder)?;
    /// ```
    ///
    /// # Notes
    /// - The generated implementations are **deterministic** and **idempotent**.
    /// - All calls are chained with `?`, preserving the builder’s fallible API.
    fn apply_overrides(
        &self,
        builder: config::ConfigBuilder<config::builder::DefaultState>,
    ) -> Result<config::ConfigBuilder<config::builder::DefaultState>, config::ConfigError>;
}