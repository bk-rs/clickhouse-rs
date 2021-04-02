use std::{
    fmt,
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use chrono::NaiveDateTime as ChronoNaiveDateTime;
use pest::Parser;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

use crate::MAX_DATETIME_UNIX_TIMESTAMP;

#[derive(Parser)]
#[grammar = "grammars/datetime.pest"]
struct DatetimeParser;

#[derive(PartialEq, Debug, Clone)]
pub struct NaiveDateTime(pub ChronoNaiveDateTime);
impl From<ChronoNaiveDateTime> for NaiveDateTime {
    fn from(inner: ChronoNaiveDateTime) -> Self {
        Self(inner)
    }
}
impl Deref for NaiveDateTime {
    type Target = ChronoNaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for NaiveDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
                ChronoNaiveDateTime::parse_from_str(pair.as_str(), "%Y-%m-%d %H:%M:%S")
                    .map(Into::into)
                    .map_err(|err| ParseError::ValueInvalid(err.to_string()))
            }
            Rule::datetime_iso => {
                ChronoNaiveDateTime::parse_from_str(pair.as_str(), "%Y-%m-%dT%H:%M:%SZ")
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

                Ok(ChronoNaiveDateTime::from_timestamp(v as i64, 0).into())
            }
            _ => return Err(ParseError::Unknown),
        }
    }
}

struct NaiveDateTimeVisitor;
impl<'de> Visitor<'de> for NaiveDateTimeVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("format simple or iso or unix_timestamp")
    }

    fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        string
            .parse()
            .map_err(|err: ParseError| de::Error::custom(err.to_string()))
    }
}
impl<'de> Deserialize<'de> for NaiveDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NaiveDateTimeVisitor)
    }
}
pub fn deserialize<'de, D>(d: D) -> Result<ChronoNaiveDateTime, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_str(NaiveDateTimeVisitor).map(|x| x.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    use chrono::NaiveDate;

    #[test]
    fn test_parse() -> Result<(), Box<dyn error::Error>> {
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

    #[derive(Deserialize)]
    struct Foo {
        #[serde(deserialize_with = "crate::datetime::deserialize")]
        dt1: chrono::NaiveDateTime,
        dt2: NaiveDateTime,
    }

    #[test]
    fn test_de() -> Result<(), Box<dyn error::Error>> {
        let deserializer = de::IntoDeserializer::<de::value::Error>::into_deserializer;
        assert_eq!(
            super::deserialize(deserializer("2021-03-01 01:02:03")).unwrap(),
            NaiveDate::from_ymd(2021, 3, 1).and_hms(1, 2, 3)
        );

        let Foo { dt1, dt2 } = serde_json::from_str(
            r#"{"dt1": "2021-03-01 01:02:03", "dt2": "2021-03-01 01:02:03"}"#,
        )?;
        assert_eq!(dt1, NaiveDate::from_ymd(2021, 3, 1).and_hms(1, 2, 3));
        assert_eq!(dt2, NaiveDate::from_ymd(2021, 3, 1).and_hms(1, 2, 3).into());

        Ok(())
    }
}
