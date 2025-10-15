mod app_config;
mod iproyal_config;
mod errors;
mod cli_args;
pub mod constants;

pub use crate::models::errors::ConfigError;
pub use app_config::AppConfig;
pub use iproyal_config::IPRoyalConfig;
pub use cli_args::CLIArgs;

pub trait ApplyOverrides {
    fn apply_overrides(
        &self,
        builder: config::ConfigBuilder<config::builder::DefaultState>,
    ) -> Result<config::ConfigBuilder<config::builder::DefaultState>, config::ConfigError>;
}