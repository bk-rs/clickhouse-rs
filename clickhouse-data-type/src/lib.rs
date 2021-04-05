pub mod date_time;
pub mod date_time64;
pub mod decimal;
pub mod r#enum;
pub mod fixed_string;
pub mod low_cardinality;

pub mod type_name;

// https://github.com/pest-parser/pest/issues/490#issuecomment-808942497
#[allow(clippy::upper_case_acronyms)]
pub(crate) mod type_name_parser;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("FormatMismatch {0}")]
    FormatMismatch(String),
    #[error("ValueInvalid {0}")]
    ValueInvalid(String),
    #[error("Unknown")]
    Unknown,
}
