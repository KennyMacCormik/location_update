use reqwest::Client;
use crate::infatica::consts::ISP_CODES_ENDPOINT;
use crate::infatica::helpers::extras_empty;
use crate::infatica::InfaticaHTTPError;
use crate::infatica::models::{InfaticaIspRecord, InfaticaIspRecords};
use crate::infatica::query_infatica::{query_infatica};
use crate::models::InfaticaConfig;

pub async fn isp_codes(cfg: &InfaticaConfig) -> Result<Vec<InfaticaIspRecord>, InfaticaHTTPError> {
    let http_client = Client::new();

    let resp = query_infatica::<InfaticaIspRecords>(
            &http_client,
            cfg.get_endpoint(),
            ISP_CODES_ENDPOINT,
            cfg,
            extras_empty(),
        )
        .await?;

	let parsed = resp.into_iter()
        .flatten()
        .collect::<Vec<InfaticaIspRecord>>();

	Ok(parsed)
}