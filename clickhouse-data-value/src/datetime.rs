use std::{num::ParseIntError, str::FromStr};

use chrono::NaiveDateTime as InnerNaiveDateTime;
use pest::Parser;

use crate::MAX_DATETIME_UNIX_TIMESTAMP;

#[derive(Parser)]
#[grammar = "grammars/datetime.pest"]
struct DatetimeParser;

#[derive(PartialEq, Debug, Clone)]
pub struct NaiveDateTime(pub InnerNaiveDateTime);
impl From<InnerNaiveDateTime> for NaiveDateTime {
    fn from(inner: InnerNaiveDateTime) -> Self {
        Self(inner)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("FormatMismatch {0}")]
    FormatMismatch(String),
    #[error("ValueInvalid {0}")]
    ValueInvalid(String),
    #[error("Unknown")]
    Unknown,
}

impl FromStr for NaiveDateTime {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = DatetimeParser::parse(Rule::datetime, s)
            .map_err(|err| ParseError::FormatMismatch(err.to_string()))?
            .next()
            .ok_or(ParseError::Unknown)?
            .into_inner()
            .next()
            .ok_or(ParseError::Unknown)?;

        match pair.as_rule() {
            Rule::datetime_simple => {
                InnerNaiveDateTime::parse_from_str(pair.as_str(), "%Y-%m-%d %H:%M:%S")
                    .map(Into::into)
                    .map_err(|err| ParseError::ValueInvalid(err.to_string()))
            }
            Rule::datetime_iso => {
                InnerNaiveDateTime::parse_from_str(pair.as_str(), "%Y-%m-%dT%H:%M:%SZ")
                    .map(Into::into)
                    .map_err(|err| ParseError::ValueInvalid(err.to_string()))
            }
            Rule::datetime_unix_timestamp => {
                let v: u64 = pair
                    .as_str()
                    .parse()
                    .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

                if v > MAX_DATETIME_UNIX_TIMESTAMP {
                    return Err(ParseError::ValueInvalid(
                        "Override the max Unix Timestamp".to_string(),
                    ));
                }

                Ok(InnerNaiveDateTime::from_timestamp(v as i64, 0).into())
            }
            _ => return Err(ParseError::Unknown),
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

        match format!("").parse::<NaiveDateTime>() {
            Ok(_) => assert!(false),
            Err(ParseError::FormatMismatch(err)) if err.ends_with("= expected datetime") => {}
            Err(err) => assert!(false, "{:?}", err),
        }

        match format!(
            "{}",
            NaiveDate::from_ymd(2106, 1, 1).and_hms(0, 0, 0).timestamp()
        )
        .parse::<NaiveDateTime>()
        {
            Ok(_) => assert!(false),
            Err(ParseError::ValueInvalid(err)) if err == "Override the max Unix Timestamp" => {}
            Err(err) => assert!(false, "{:?}", err),
        }

        Ok(())
    }
}
