use serde::{Deserialize, Serialize};

/// Extra form fields passed to Infatica HTTP queries.
pub type InfaticaFormFields = Vec<(String, String)>;

/// Root-level type: Infatica returns an array of records.
pub type InfaticaRecords = Vec<Vec<InfaticaGeoNodeRecord>>;

/// One record in the Infatica dataset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfaticaGeoNodeRecord {
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

/// Root-level type: Infatica returns an array of arrays of ISP records.
pub type InfaticaIspRecords = Vec<Vec<InfaticaIspRecord>>;

/// A single ISP record in Infatica's dictionary dump.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfaticaIspRecord {
    /// The ISP’s name or descriptive label.
    /// May contain quotes, punctuation, or Unicode characters.
    pub isp: String,

    /// Internal Infatica numeric code for that ISP.
    pub code: u32,
}