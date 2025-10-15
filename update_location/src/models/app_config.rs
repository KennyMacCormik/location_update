use serde::Deserialize;
use crate::models::IPRoyalConfig;

#[derive(Deserialize)]
pub struct AppConfig {
    pub iproyal: IPRoyalConfig,
}