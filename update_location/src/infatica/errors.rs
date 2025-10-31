use thiserror::Error;
use crate::infatica::internal::errors::HTTPError;

/// Aggregated error type for top-level Infatica queries.
///
/// Each variant corresponds to a specific internal Infatica endpoint.
/// When using [`get_all`], multiple variants can appear simultaneously in the returned `Vec`.
#[derive(Debug, Error)]
pub enum InfaticaQueryError {
	/// Failure during `geo_nodes.php` query (network, timeout, or parse).
	#[error("geo_nodes request failed: {0}")]
	GeoNodes(HTTPError),

	/// Failure during `subdivision_codes.php` query.
	#[error("region_codes request failed: {0}")]
	RegionCodes(HTTPError),

	/// Failure during `zip-codes.php` query.
	#[error("zip_codes request failed: {0}")]
	ZipCodes(HTTPError),

	/// Failure during `isp_codes.php` query.
	#[error("isp_codes request failed: {0}")]
	IspCodes(HTTPError),
}