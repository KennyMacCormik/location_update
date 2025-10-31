//! Data model definitions for Infatica API responses.
use serde::{Deserialize, Serialize};

/// Extra form fields passed to Infatica HTTP queries.
pub type InfaticaFormFields = Vec<(String, String)>;

/// Root-level type: Infatica returns an array of records.
pub type InfaticaRecords = Vec<Vec<InfaticaGeoNodeRecord>>;

/// Geo-node record combining country, region, city, ISP, and node stats.
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

/// ISP dictionary — Infatica wraps in `Vec<Vec<_>>`.
pub type InfaticaIspRecords = Vec<Vec<InfaticaIspRecord>>;

/// ISP record mapping name to numeric code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfaticaIspRecord {
    /// The ISP’s name or descriptive label.
    /// May contain quotes, punctuation, or Unicode characters.
    pub isp: String,

    /// Internal Infatica numeric code for that ISP.
    pub code: u32,
}

/// Region dictionary — Infatica wraps in `Vec<Vec<_>>`.
pub type InfaticaRegionRecords = Vec<Vec<InfaticaRegionRecord>>;

/// Region/subdivision record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfaticaRegionRecord {
	/// Internal Infatica region code.
	pub code: u32,

	/// Human-readable region/subdivision name.
	#[serde(rename = "subdivision")]
	pub name: String,
}

/// ZIP dictionary — Infatica wraps in `Vec<Vec<_>>`.
pub type InfaticaZipRecords = Vec<Vec<InfaticaZipRecord>>;

/// Postal code record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfaticaZipRecord {
	/// ISO 3166-1 alpha-2 country code (e.g. "US", "JP").
	pub country: String,

	/// Subdivision / region / state (may be empty).
	pub subdivision: String,

	/// City name (may include Unicode, spaces, or punctuation).
	pub city: String,

	/// Postal / ZIP code (may include letters, hyphens, etc.).
	pub zip: String,
}