use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to build config: {0}")]
    BuildConfigError(#[from] config::ConfigError),

    #[error("failed to deserialize config: {source}")]
    DeserializeConfigError {
        #[source]
        source: config::ConfigError,
    },
}