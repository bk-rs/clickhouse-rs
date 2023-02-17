use std::io::Error as IoError;

use isahc::http;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IsahcError {0:?}")]
    IsahcError(#[from] isahc::Error),
    #[error("HttpInvalidUri {0:?}")]
    HttpInvalidUri(#[from] http::uri::InvalidUri),
    #[error("UrlParseError {0:?}")]
    UrlParseError(#[from] url::ParseError),
    #[error("IoError {0:?}")]
    IoError(#[from] IoError),
    //
    #[error("ClientExecuteError {0:?}")]
    ClientExecuteError(#[from] ClientExecuteError),
    #[error("ClientInsertWithFormatError {0:?}")]
    ClientInsertWithFormatError(#[from] ClientInsertWithFormatError),
    #[error("ClientSelectWithFormatError {0:?}")]
    ClientSelectWithFormatError(#[from] ClientSelectWithFormatError),
}

#[derive(thiserror::Error, Debug)]
pub enum ClientExecuteError {
    #[error("StatusCodeMismatch {0:?}")]
    StatusCodeMismatch(http::StatusCode),
}

#[derive(thiserror::Error, Debug)]
pub enum ClientInsertWithFormatError {
    #[error("FormatSerError {0:?}")]
    FormatSerError(String),
    #[error("StatusCodeMismatch {0:?}")]
    StatusCodeMismatch(http::StatusCode),
}

#[derive(thiserror::Error, Debug)]
pub enum ClientSelectWithFormatError {
    #[error("StatusCodeMismatch {0:?}")]
    StatusCodeMismatch(http::StatusCode),
    #[error("FormatMismatch {0:?}")]
    FormatMismatch(String),
    #[error("FormatDeError {0:?}")]
    FormatDeError(String),
}
