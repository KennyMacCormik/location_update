use serde::Deserialize;
use crate::models::infatica_config::InfaticaConfig;
use crate::models::IPRoyalConfig;

#[derive(Deserialize)]
pub struct AppConfig {
    pub iproyal: IPRoyalConfig,
    pub infatica: InfaticaConfig,
}