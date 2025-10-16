//! Core runtime trait used by the `#[derive(ApplyOverrides)]` macro.

/// Trait that the derive macro will implement automatically.
pub trait ApplyOverrides {
    fn apply_overrides(
        &self,
        builder: config::ConfigBuilder<config::builder::DefaultState>,
    ) -> Result<config::ConfigBuilder<config::builder::DefaultState>, config::ConfigError>;
}