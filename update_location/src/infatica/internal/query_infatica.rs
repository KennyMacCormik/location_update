//! Core query helper used by all Infatica API calls.
//!
//! Responsible for:
//! - Ensuring trailing `/` in base URL before joining endpoint
//! - Constructing POST form fields (email, password, extras)
//! - Executing HTTP request with timeout
//! - Deserializing JSON response into a generic `T`

use std::collections::HashMap;
use reqwest::Client;
use super::consts::{DEFAULT_TIMEOUT, EMAIL_FIELD, PASSWORD_FIELD};
use super::errors::HTTPError;
use super::models::InfaticaFormFields;

pub async fn query_infatica<T>(
    client: &Client,
    base: &url::Url,
    endpoint: &str,
    cfg: &crate::models::InfaticaConfig,
    extra_form_fields: InfaticaFormFields,
) -> Result<T, HTTPError>
where
    T: serde::de::DeserializeOwned,
{
	// Ensure base URL ends with a slash, otherwise `join()` drops last path segment.
    let mut sanitized = base.clone();
    if !sanitized.path().ends_with('/') {
        sanitized.path_segments_mut().unwrap().push("");
    }

    let url = sanitized.join(endpoint)?;
    let timeout = cfg.get_timeout().unwrap_or(&DEFAULT_TIMEOUT).to_owned();

	// Prepare POST form data
    let mut form: HashMap<String, String> = HashMap::new();
    form.insert(EMAIL_FIELD.to_string(), cfg.get_email().to_string());
    form.insert(PASSWORD_FIELD.to_string(), cfg.get_password().to_string());
    for (k, v) in extra_form_fields {
        form.insert(k, v);
    }

	// Execute and decode
    let resp =client
        .post(url)
        .timeout(timeout)
        .form(&form)
        .send()
        .await?;

    let parsed = resp.json::<T>().await?;

    Ok(parsed)
}