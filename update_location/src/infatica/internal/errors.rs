//! Error definitions used by Infatica HTTP modules.

use thiserror::Error;
use url::ParseError;

/// Generic HTTP-level error type shared by all Infatica calls.
#[derive(Debug, Error)]
pub enum HTTPError {
	/// URL composition or join failure (invalid base or endpoint).
	#[error("failed to join URL: {0}")]
	JoinURLError(#[from] ParseError),

	/// `reqwest` network, timeout, or deserialization error.
	#[error("request error: {0}")]
	URLError(#[from] reqwest::Error),
}