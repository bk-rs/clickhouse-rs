use std::{
    fmt,
    num::{ParseFloatError, ParseIntError},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use chrono::NaiveDateTime as ChronoNaiveDateTime;
use pest::Parser as _;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

use crate::{
    date_and_time_parser::{DateAndTimeParser, Rule},
    MAX_DATETIME_UNIX_TIMESTAMP,
};

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
        let pair = DateAndTimeParser::parse(Rule::datetime64, s)
            .map_err(|err| ParseError::FormatMismatch(err.to_string()))?
            .next()
            .ok_or(ParseError::Unknown)?
            .into_inner()
            .next()
            .ok_or(ParseError::Unknown)?;

        match pair.as_rule() {
            Rule::datetime64_simple_milli => parse_simple_str(pair.as_str(), Precision::Milli),
            Rule::datetime64_simple_micro => parse_simple_str(pair.as_str(), Precision::Micro),
            Rule::datetime64_simple_nano => parse_simple_str(pair.as_str(), Precision::Nano),
            Rule::datetime64_iso_milli => parse_iso_str(pair.as_str(), Precision::Milli),
            Rule::datetime64_iso_micro => parse_iso_str(pair.as_str(), Precision::Micro),
            Rule::datetime64_iso_nano => parse_iso_str(pair.as_str(), Precision::Nano),
            Rule::datetime64_unix_timestamp_milli => {
                parse_unix_timestamp_str(pair.as_str(), Precision::Milli)
            }
            Rule::datetime64_unix_timestamp_micro => {
                parse_unix_timestamp_str(pair.as_str(), Precision::Micro)
            }
            Rule::datetime64_unix_timestamp_nano => {
                parse_unix_timestamp_str(pair.as_str(), Precision::Nano)
            }
            _ => Err(ParseError::Unknown),
        }
    }
}

#[derive(Debug)]
enum Precision {
    Milli,
    Micro,
    Nano,
}
fn parse_simple_str(s: &str, precision: Precision) -> Result<NaiveDateTime, ParseError> {
    let fmt = match precision {
        Precision::Milli => "%Y-%m-%d %H:%M:%S%.3f",
        Precision::Micro => "%Y-%m-%d %H:%M:%S%.6f",
        Precision::Nano => "%Y-%m-%d %H:%M:%S%.9f",
    };
    ChronoNaiveDateTime::parse_from_str(s, fmt)
        .map(Into::into)
        .map_err(|err| ParseError::ValueInvalid(err.to_string()))
}
fn parse_iso_str(s: &str, precision: Precision) -> Result<NaiveDateTime, ParseError> {
    let fmt = match precision {
        Precision::Milli => "%Y-%m-%dT%H:%M:%S%.3fZ",
        Precision::Micro => "%Y-%m-%dT%H:%M:%S%.6fZ",
        Precision::Nano => "%Y-%m-%dT%H:%M:%S%.9fZ",
    };
    ChronoNaiveDateTime::parse_from_str(s, fmt)
        .map(Into::into)
        .map_err(|err| ParseError::ValueInvalid(err.to_string()))
}
fn parse_unix_timestamp_str(s: &str, precision: Precision) -> Result<NaiveDateTime, ParseError> {
    let nsecs_str = match precision {
        Precision::Milli => &s[s.len() - 3..],
        Precision::Micro => &s[s.len() - 6..],
        Precision::Nano => &s[s.len() - 9..],
    };
    let nsecs: u32 = nsecs_str
        .parse()
        .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;
    let nsecs = match precision {
        Precision::Milli => nsecs * 1_000_000,
        Precision::Micro => nsecs * 1_000,
        Precision::Nano => nsecs,
    };

    let v: f64 = s
        .parse()
        .map_err(|err: ParseFloatError| ParseError::ValueInvalid(err.to_string()))?;

    if v as u64 > MAX_DATETIME_UNIX_TIMESTAMP {
        return Err(ParseError::ValueInvalid(
            "Override the max Unix Timestamp".to_string(),
        ));
    }

    Ok(ChronoNaiveDateTime::from_timestamp(v as i64, nsecs).into())
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

    use std::{error, fs, path::PathBuf};

    use chrono::NaiveDate;

    #[test]
    fn test_parse() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            "2021-03-01 01:02:03.123".parse::<NaiveDateTime>()?,
            NaiveDate::from_ymd(2021, 3, 1)
                .and_hms_milli(1, 2, 3, 123)
                .into()
        );

        assert_eq!(
            "2021-03-01 01:02:03.123456".parse::<NaiveDateTime>()?,
            NaiveDate::from_ymd(2021, 3, 1)
                .and_hms_micro(1, 2, 3, 123456)
                .into()
        );

        assert_eq!(
            "2021-03-01T01:02:03.123Z".parse::<NaiveDateTime>()?,
            NaiveDate::from_ymd(2021, 3, 1)
                .and_hms_milli(1, 2, 3, 123)
                .into()
        );

        assert_eq!(
            "1614560523.123".parse::<NaiveDateTime>()?,
            NaiveDate::from_ymd(2021, 3, 1)
                .and_hms_milli(1, 2, 3, 123)
                .into()
        );

        match format!("").parse::<NaiveDateTime>() {
            Ok(_) => assert!(false),
            Err(ParseError::FormatMismatch(err)) if err.ends_with("= expected datetime64") => {}
            Err(err) => assert!(false, "{:?}", err),
        }

        match format!(
            "{}.123",
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
    struct Row {
        #[serde(deserialize_with = "crate::datetime64::deserialize")]
        datetime64_milli_utc: chrono::NaiveDateTime,
        #[allow(dead_code)]
        datetime64_milli_shanghai: NaiveDateTime,
        #[serde(deserialize_with = "crate::datetime64::deserialize")]
        datetime64_micro_utc: chrono::NaiveDateTime,
        #[allow(dead_code)]
        datetime64_micro_shanghai: NaiveDateTime,
        #[serde(deserialize_with = "crate::datetime64::deserialize")]
        datetime64_nano_utc: chrono::NaiveDateTime,
        #[allow(dead_code)]
        datetime64_nano_shanghai: NaiveDateTime,
    }

    #[test]
    fn test_de() -> Result<(), Box<dyn error::Error>> {
        let deserializer = de::IntoDeserializer::<de::value::Error>::into_deserializer;
        assert_eq!(
            super::deserialize(deserializer("2021-03-01 01:02:03.123")).unwrap(),
            NaiveDate::from_ymd(2021, 3, 1).and_hms_milli(1, 2, 3, 123)
        );

        for format in ["simple", "iso", "unix_timestamp"].iter() {
            let content = fs::read_to_string(
                PathBuf::new().join(format!("tests/files/datetime64_{}.txt", format)),
            )?;
            let line = content.lines().next().unwrap();

            let Row {
                datetime64_milli_utc,
                datetime64_milli_shanghai: _,
                datetime64_micro_utc,
                datetime64_micro_shanghai: _,
                datetime64_nano_utc,
                datetime64_nano_shanghai: _,
            } = serde_json::from_str(line)?;
            assert_eq!(
                datetime64_milli_utc,
                NaiveDate::from_ymd(2021, 3, 1).and_hms_milli(1, 2, 3, 123)
            );
            assert_eq!(
                datetime64_micro_utc,
                NaiveDate::from_ymd(2021, 3, 1).and_hms_micro(1, 2, 3, 123456)
            );
            assert_eq!(
                datetime64_nano_utc,
                NaiveDate::from_ymd(2021, 3, 1).and_hms_nano(1, 2, 3, 123456789)
            );
        }

        Ok(())
    }
}
