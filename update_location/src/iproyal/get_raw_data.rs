use std::time::Duration;
use reqwest::{{Client}};
use url::ParseError;
use crate::iproyal::iproyal_data_models::Root;
use crate::models::IPRoyalConfig;

#[derive(Debug)]
pub enum IPRoyalGetCountryError {
    JoinURLError(ParseError),
    URLError(reqwest::Error),
}

const ENDPOINT: &str = "access/countries";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn get_raw_data(cfg: &IPRoyalConfig) -> Result<Root, IPRoyalGetCountryError> {
    let http_client = Client::new();

    let mut sanitized_url = cfg.get_endpoint().to_owned();
    if !sanitized_url.path().ends_with('/'){
        sanitized_url.path_segments_mut().unwrap().push("");
    }
    sanitized_url = sanitized_url.join(ENDPOINT).map_err(IPRoyalGetCountryError::JoinURLError)?;

    let token = cfg.get_token().to_owned();
    let timeout = cfg.get_timeout().unwrap_or_else(|| &DEFAULT_TIMEOUT).to_owned();

    Ok(
        http_client
            .get(sanitized_url)
            .bearer_auth(token)
            .timeout(timeout)
            .send()
            .await
            .map_err(IPRoyalGetCountryError::URLError)?
            .json::<Root>()
            .await
            .map_err(IPRoyalGetCountryError::URLError)?
    )
}