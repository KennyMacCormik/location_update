//! Queries Infatica’s ZIP/postal code dictionary (`zip-codes.php`).

use reqwest::Client;
use super::consts::{ZIP_CODES_ENDPOINT};
use super::helpers::extras_empty;
use super::errors::HTTPError;
use super::models::{InfaticaZipRecord, InfaticaZipRecords};
use super::query_infatica::query_infatica;
use crate::models::InfaticaConfig;

/// Fetches the ZIP/postal dictionary from Infatica.
pub async fn zip_codes(cfg: &InfaticaConfig) -> Result<Vec<InfaticaZipRecord>, HTTPError> {
	let http_client = Client::new();

	let resp = query_infatica::<InfaticaZipRecords>(
		&http_client,
		cfg.get_endpoint(),
		ZIP_CODES_ENDPOINT,
		cfg,
		extras_empty(),
	).await?;

	let parsed = resp.into_iter()
		.flatten()
		.collect::<Vec<InfaticaZipRecord>>();

	Ok(parsed)
}