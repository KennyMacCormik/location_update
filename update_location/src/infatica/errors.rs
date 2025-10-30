use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum HTTPRequestError {
    #[error("failed to join URL: {0}")]
    JoinURLError(#[from] ParseError),

    #[error("request error: {0}")]
    URLError(#[from] reqwest::Error),
}