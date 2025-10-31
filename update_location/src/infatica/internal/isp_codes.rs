//! Queries Infatica’s `isp_codes.php` endpoint — the ISP dictionary.

use reqwest::Client;
use super::consts::ISP_CODES_ENDPOINT;
use super::helpers::extras_empty;
use super::errors::HTTPError;
use super::models::{InfaticaIspRecord, InfaticaIspRecords};
use super::query_infatica::{query_infatica};
use crate::models::InfaticaConfig;

/// Fetches the ISP dictionary.
/// Each record maps an ISP name to its internal numeric code.
///
/// The legacy Infatica API wraps results in a `Vec<Vec<...>>`,
/// which this function flattens into a single vector.
pub async fn isp_codes(cfg: &InfaticaConfig) -> Result<Vec<InfaticaIspRecord>, HTTPError> {
    let http_client = Client::new();

    let resp = query_infatica::<InfaticaIspRecords>(
            &http_client,
            cfg.get_endpoint(),
            ISP_CODES_ENDPOINT,
            cfg,
            extras_empty(),
        ).await?;

	let parsed = resp.into_iter()
        .flatten()
        .collect::<Vec<InfaticaIspRecord>>();

	Ok(parsed)
}