use isahc::http;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IsahcError {0:?}")]
    IsahcError(#[from] isahc::Error),
    #[error("InvalidUri {0:?}")]
    InvalidUri(#[from] http::uri::InvalidUri),
    #[error("UrlParseError {0:?}")]
    UrlParseError(#[from] url::ParseError),
    #[error("IoError {0:?}")]
    IoError(#[from] std::io::Error),
    //
    #[error("ExecuteFailed {0:?}")]
    ExecuteFailed(http::StatusCode),
}
