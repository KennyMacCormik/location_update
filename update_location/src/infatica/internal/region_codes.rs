//! Queries Infatica’s region/subdivision dictionary (`subdivision_codes.php`).

use reqwest::Client;
use super::consts::{REGION_CODES_ENDPOINT};
use super::helpers::extras_empty;
use super::errors::HTTPError;
use super::models::{InfaticaRegionRecord, InfaticaRegionRecords};
use super::query_infatica::query_infatica;
use crate::models::InfaticaConfig;

/// Fetches the region/subdivision dictionary from Infatica.
pub async fn region_codes(cfg: &InfaticaConfig) -> Result<Vec<InfaticaRegionRecord>, HTTPError> {
	let http_client = Client::new();

	let resp = query_infatica::<InfaticaRegionRecords>(
		&http_client,
		cfg.get_endpoint(),
		REGION_CODES_ENDPOINT,
		cfg,
		extras_empty(),
	).await?;

	let parsed = resp.into_iter()
		.flatten()
		.collect::<Vec<InfaticaRegionRecord>>();

	Ok(parsed)
}