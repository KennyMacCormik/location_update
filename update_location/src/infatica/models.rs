use serde::{Deserialize, Serialize};

/// Root-level type: Infatica returns an array of records.
pub type InfaticaRecords = Vec<Vec<InfaticaRecord>>;

/// One record in the Infatica dataset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfaticaRecord {
    /// ISO 3166-1 alpha-2 country code (e.g. "US", "DE")
    pub country: String,

    /// Subdivision / region / state (may be empty string)
    pub subdivision: String,

    /// City name (may be "XX" when missing)
    pub city: String,

    /// Internet Service Provider name
    pub isp: String,

    /// Autonomous System Number
    pub asn: u32,

    /// Postal / ZIP code (may contain non-numeric text)
    pub zip: String,

    /// Number of nodes available in this region/city/ISP
    pub nodes: u32,
}