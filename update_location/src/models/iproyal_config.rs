use url::Url;
use std::time::Duration;
use serde::Deserialize;

#[derive(Deserialize)]
/// Represents configuration for interacting with the IPRoyal API.
pub struct IPRoyalConfig {
    endpoint: Url,
    token: String,

    #[serde(with = "humantime_serde")]
    timeout: Duration,
}

impl IPRoyalConfig {
    /// Get the configured endpoint
    pub fn get_endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Get the configured token
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// Get the configured timeout
    pub fn get_timeout(&self) -> &Duration {
        &self.timeout
    }
}