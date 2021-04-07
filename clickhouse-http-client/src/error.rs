#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IsahcError {0:?}")]
    IsahcError(#[from] isahc::Error),
    #[error("InvalidHeaderValue {0:?}")]
    InvalidHeaderValue(#[from] isahc::http::header::InvalidHeaderValue),
    #[error("InvalidUri {0:?}")]
    InvalidUri(#[from] isahc::http::uri::InvalidUri),
    #[error("UrlParseError {0:?}")]
    UrlParseError(#[from] url::ParseError),
}
