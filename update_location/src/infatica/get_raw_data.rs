use std::time::Duration;
use reqwest::{{Client}};
use url::ParseError;
use thiserror::Error;
use std::collections::HashMap;
use crate::infatica::models::{InfaticaRecord, InfaticaRecords};
use crate::models::{InfaticaConfig};

#[derive(Debug, Error)]
pub enum InfaticaGetTreeError {
    #[error("failed to join URL: {0}")]
    JoinURLError(ParseError),
    #[error("request error: {0}")]
    URLError(reqwest::Error),
}

const ENDPOINT: &str = "includes/api/client/geo_nodes.php";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn get_raw_data(cfg: &InfaticaConfig) -> Result<Vec<InfaticaRecord>, InfaticaGetTreeError> {
    let http_client = Client::new();

    let mut sanitized_url = cfg.get_endpoint().to_owned();
    if !sanitized_url.path().ends_with('/'){
        sanitized_url.path_segments_mut().unwrap().push("");
    }
    sanitized_url = sanitized_url.join(ENDPOINT).map_err(InfaticaGetTreeError::JoinURLError)?;

    let timeout = cfg.get_timeout().unwrap_or_else(|| &DEFAULT_TIMEOUT).to_owned();

    let mut form = HashMap::new();
    form.insert("email", cfg.get_email());
    form.insert("password", cfg.get_password());

    Ok(
        http_client
            .post(sanitized_url)
            .timeout(timeout)
            .form(&form)
            .send()
            .await
            .map_err(InfaticaGetTreeError::URLError)?
            .json::<InfaticaRecords>()
            .await
            .map_err(InfaticaGetTreeError::URLError)?
            .into_iter()
            .flatten()
            .collect::<Vec<InfaticaRecord>>()
    )
}