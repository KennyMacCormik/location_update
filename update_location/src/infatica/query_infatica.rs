use std::collections::HashMap;
use reqwest::Client;
use crate::infatica::consts::{DEFAULT_TIMEOUT, EMAIL_FIELD, PASSWORD_FIELD};
use crate::infatica::InfaticaHTTPError;
use crate::infatica::models::InfaticaFormFields;

pub async fn query_infatica<T>(
    client: &Client,
    base: &url::Url,
    endpoint: &str,
    cfg: &crate::models::InfaticaConfig,
    extra_form_fields: InfaticaFormFields,
) -> Result<T, InfaticaHTTPError>
where
    T: serde::de::DeserializeOwned,
{
    let mut sanitized = base.clone();
    if !sanitized.path().ends_with('/') {
        sanitized.path_segments_mut().unwrap().push("");
    }
    let url = sanitized.join(endpoint)?;

    let timeout = cfg.get_timeout().unwrap_or(&DEFAULT_TIMEOUT).to_owned();

    let mut form: HashMap<String, String> = HashMap::new();
    form.insert(EMAIL_FIELD.to_string(), cfg.get_email().to_string());
    form.insert(PASSWORD_FIELD.to_string(), cfg.get_password().to_string());
    for (k, v) in extra_form_fields {
        form.insert(k, v);
    }

    let resp =client
        .post(url)
        .timeout(timeout)
        .form(&form)
        .send()
        .await?;

    let parsed = resp.json::<T>().await?;

    Ok(parsed)
}