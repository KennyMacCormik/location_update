use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("could not read config file {path}: {error}")]
    IoError {
        path: String,
        #[source]
        error: std::io::Error,
    },

    #[error("invalid TOML format in {path}: {error}")]
    InvalidConfigFormat {
        path: String,
        #[source]
        error: toml::de::Error,
    },
}