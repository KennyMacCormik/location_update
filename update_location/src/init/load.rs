use config::{Config, Environment, File};
use crate::models::{AppConfig, CLIArgs, constants::ENV_PREFIX, ConfigError, ApplyOverrides};

/// Load configuration from file, environment, and CLI arguments.
pub fn load_config(args: &CLIArgs) -> Result<AppConfig, ConfigError> {
    let mut builder = Config::builder();

    // Lowest priority: configuration file
    if let Some(path) = &args.config {
        builder = builder.add_source(File::with_name(path).required(false));
    }

    // Medium priority: environment variables
    builder = builder.add_source(
        Environment::with_prefix(ENV_PREFIX)
    );

    builder = args.apply_overrides(builder)?;

    // Build the final merged config and deserialize it
    let cfg = builder.build()?;

    let dump: serde_json::Value = cfg.clone().try_deserialize()?;
    println!("{:#}", dump);

    cfg.try_deserialize::<AppConfig>()
        .map_err(|source| ConfigError::DeserializeConfigError { source })
}