use config::{Config, Environment, File};
use crate::models::{
    AppConfig,
    CLIArgs,
    constants::ENV_PREFIX,
};

/// Load configuration from file, environment, and CLI arguments.
pub fn load_config(args: &CLIArgs) -> Result<AppConfig, config::ConfigError> {
    let mut builder = Config::builder();

    // Lowest priority: configuration file
    if let Some(path) = &args.config {
        builder = builder.add_source(File::with_name(path).required(false));
    }

    // Medium priority: environment variables
    builder = builder.add_source(
        Environment::with_prefix(ENV_PREFIX)
    );

    // Highest priority: CLI overrides
    if let Some(v) = &args.iproyal_endpoint {
        builder = builder.set_override("iproyal.endpoint", v.clone())?;
    }
    if let Some(v) = &args.iproyal_token {
        builder = builder.set_override("iproyal.token", v.clone())?;
    }
    if let Some(v) = &args.iproyal_timeout {
        builder = builder.set_override("iproyal.timeout", v.clone())?;
    }

    // Build the final merged config and deserialize it
    let cfg = builder.build()?;
    cfg.try_deserialize::<AppConfig>()
}