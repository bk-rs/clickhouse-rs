use std::{num::ParseIntError, str::FromStr};

use chrono::NaiveDateTime as InnerNaiveDateTime;
use pest::Parser;

use crate::MAX_DATETIME_UNIX_TIMESTAMP;

#[derive(Parser)]
#[grammar = "clickhouse.pest"]
struct ClickhouseParser;

#[derive(PartialEq, Debug, Clone)]
pub struct NaiveDateTime(pub InnerNaiveDateTime);
impl From<InnerNaiveDateTime> for NaiveDateTime {
    fn from(inner: InnerNaiveDateTime) -> Self {
        Self(inner)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("PestParseError {0}")]
    PestParseError(String),
    #[error("PestNonePair")]
    PestNonePair,
    #[error("PestRuleMismatch {0}")]
    PestRuleMismatch(String),
    #[error("ValueParseError {0}")]
    ValueParseError(String),
    #[error("ValueInvalid {0}")]
    ValueInvalid(String),
}

impl FromStr for NaiveDateTime {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match ClickhouseParser::parse(Rule::datetime_simple, s) {
            Ok(pair) => InnerNaiveDateTime::parse_from_str(pair.as_str(), "%Y-%m-%d %H:%M:%S")
                .map(Into::into)
                .map_err(|err| ParseError::ValueParseError(err.to_string())),
            Err(_) => match ClickhouseParser::parse(Rule::datetime_iso, s) {
                Ok(pair) => InnerNaiveDateTime::parse_from_str(pair.as_str(), "%Y-%m-%dT%H:%M:%SZ")
                    .map(Into::into)
                    .map_err(|err| ParseError::ValueParseError(err.to_string())),
                Err(_) => match ClickhouseParser::parse(Rule::uint, s) {
                    Ok(pair) => {
                        let v: u64 = pair.as_str().parse().map_err(|err: ParseIntError| {
                            ParseError::ValueParseError(err.to_string())
                        })?;

                        if v >= MAX_DATETIME_UNIX_TIMESTAMP {
                            return Err(ParseError::ValueInvalid(pair.as_str().to_owned()));
                        }

                        Ok(InnerNaiveDateTime::from_timestamp(v as i64, 0).into())
                    }
                    Err(_) => return Err(ParseError::PestRuleMismatch(s.to_owned())),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use chrono::NaiveDate;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            "2021-03-01 01:02:03".parse::<NaiveDateTime>()?,
            NaiveDate::from_ymd(2021, 3, 1).and_hms(1, 2, 3).into()
        );

        assert_eq!(
            "2021-03-01T01:02:03Z".parse::<NaiveDateTime>()?,
            NaiveDate::from_ymd(2021, 3, 1).and_hms(1, 2, 3).into()
        );

        assert_eq!(
            "1614560523".parse::<NaiveDateTime>()?,
            NaiveDate::from_ymd(2021, 3, 1).and_hms(1, 2, 3).into()
        );

        Ok(())
    }
}
