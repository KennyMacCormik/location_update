use url::Url;
use std::time::Duration;
use serde::Deserialize;

#[derive(Deserialize)]
/// Represents configuration for interacting with the IPRoyal API.
pub struct InfaticaConfig {
    endpoint: Url,
    email: String,
    password: String,
    #[serde(default, with = "humantime_serde::option")]
    timeout: Option<Duration>,
}

impl InfaticaConfig {
    /// Get the configured endpoint
    pub fn get_endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Get the configured email
    pub fn get_email(&self) -> &str {
        &self.email
    }

    /// Get the configured password
    pub fn get_password(&self) -> &str {
        &self.password
    }

    ///Get the configured timeout
    pub fn get_timeout(&self) -> Option<&Duration> {
        self.timeout.as_ref()
    }
}