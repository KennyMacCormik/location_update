use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to override key {key} with {value}: {error}")]
    OverrideConfigFormat {
        key: String,
        value: String,
        #[source]
        error: toml::de::Error,
    },

    #[error("failed to build config: {0}")]
    BuildConfigError(#[from] config::ConfigError),

    #[error("failed to deserialize config: {source}")]
    DeserializeConfigError {
        #[source]
        source: config::ConfigError,
    },
}