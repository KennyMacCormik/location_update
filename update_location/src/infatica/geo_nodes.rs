use reqwest::Client;
use crate::infatica::consts::{GEO_NODES_ENDPOINT};
use crate::infatica::helpers::extras_exclude_corporate;
use crate::infatica::InfaticaHTTPError;
use crate::infatica::models::{InfaticaGeoNodeRecord, InfaticaRecords};
use crate::infatica::query_infatica::{query_infatica};
use crate::models::InfaticaConfig;

pub async fn geo_nodes(cfg: &InfaticaConfig) -> Result<Vec<InfaticaGeoNodeRecord>, InfaticaHTTPError> {
    let http_client = Client::new();

    let resp = query_infatica::<InfaticaRecords>(
            &http_client,
            cfg.get_endpoint(),
            GEO_NODES_ENDPOINT,
            cfg,
            extras_exclude_corporate(),
        )
        .await?;

    let parsed = resp.into_iter()
        .flatten()
        .collect::<Vec<InfaticaGeoNodeRecord>>();

    Ok(parsed)
}