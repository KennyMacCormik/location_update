//! Queries Infatica’s `geo_nodes.php` endpoint.
//!
//! Returns a list of nodes grouped by (country, region, city, ISP).
//! Each record contains location identifiers, ISP metadata, ASN,
//! ZIP code, and node counts.

use reqwest::Client;
use super::consts::GEO_NODES_ENDPOINT;
use super::helpers::extras_exclude_corporate;
use super::errors::HTTPError;
use super::models::{InfaticaGeoNodeRecord, InfaticaRecords};
use super::query_infatica::{query_infatica};
use crate::models::InfaticaConfig;

/// Fetches geo-node dataset from Infatica.
/// Automatically adds `excludeCorporate=1` to filter out corporate nodes.
///
/// On success, flattens the double array format (`Vec<Vec<Record>>`)
/// into a single `Vec<InfaticaGeoNodeRecord>`.
pub async fn geo_nodes(cfg: &InfaticaConfig) -> Result<Vec<InfaticaGeoNodeRecord>, HTTPError> {
    let http_client = Client::new();

    let resp = query_infatica::<InfaticaRecords>(
            &http_client,
            cfg.get_endpoint(),
            GEO_NODES_ENDPOINT,
            cfg,
            extras_exclude_corporate(),
        ).await?;

    let parsed = resp.into_iter()
        .flatten()
        .collect::<Vec<InfaticaGeoNodeRecord>>();

    Ok(parsed)
}