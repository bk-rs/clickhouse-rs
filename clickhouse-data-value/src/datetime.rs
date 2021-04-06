use std::{
    fmt,
    num::ParseIntError,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use chrono::NaiveDateTime as ChronoNaiveDateTime;
use pest::{iterators::Pairs, Parser as _};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

use crate::date_and_time_parser::{DateAndTimeParser, Rule};

// 2105-12-31 23:59:59
pub(crate) const UNIX_TIMESTAMP_MAX: u64 = 4291718399;

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
        let pair = DateAndTimeParser::parse(Rule::datetime, s)
            .map_err(|err| ParseError::FormatMismatch(err.to_string()))?
            .next()
            .ok_or(ParseError::Unknown)?
            .into_inner()
            .next()
            .ok_or(ParseError::Unknown)?;

        match pair.as_rule() {
            Rule::datetime_simple => from_simple_pairs(pair.into_inner()),
            Rule::datetime_iso => from_iso_pairs(pair.into_inner()),
            Rule::datetime_unix_timestamp => from_unix_timestamp_pairs(pair.into_inner()),
            _ => Err(ParseError::Unknown),
        }
    }
}

fn from_simple_pairs(
    mut datetime_simple_pairs: Pairs<'_, Rule>,
) -> Result<NaiveDateTime, ParseError> {
    let date_pair = datetime_simple_pairs.next().ok_or(ParseError::Unknown)?;
    let time_pair = datetime_simple_pairs.next().ok_or(ParseError::Unknown)?;
    let precision_str = datetime_simple_pairs
        .next()
        .map(|time_nf_pair| time_nf_pair.as_str());

    let (str, fmt) = if let Some(precision_str) = precision_str {
        if precision_str.is_empty() {
            return Err(ParseError::Unknown);
        }
        if precision_str.len() > 9 {
            return Err(ParseError::Unknown);
        }

        (
            format!(
                "{} {}.{:0<width$}",
                date_pair.as_str(),
                time_pair.as_str(),
                precision_str,
                width = [3, 3, 3, 6, 6, 6, 9, 9, 9][precision_str.len() - 1]
            ),
            format!(
                "%Y-%m-%d %H:%M:%S%.{}f",
                [3, 3, 3, 6, 6, 6, 9, 9, 9][precision_str.len() - 1]
            ),
        )
    } else {
        (
            format!("{} {}", date_pair.as_str(), time_pair.as_str()),
            "%Y-%m-%d %H:%M:%S".to_string(),
        )
    };

    ChronoNaiveDateTime::parse_from_str(&str, &fmt)
        .map(Into::into)
        .map_err(|err| ParseError::ValueInvalid(err.to_string()))
}

fn from_iso_pairs(mut datetime_iso_pairs: Pairs<'_, Rule>) -> Result<NaiveDateTime, ParseError> {
    let date_pair = datetime_iso_pairs.next().ok_or(ParseError::Unknown)?;
    let time_pair = datetime_iso_pairs.next().ok_or(ParseError::Unknown)?;
    let precision_str = datetime_iso_pairs
        .next()
        .map(|time_nf_pair| time_nf_pair.as_str());

    let (str, fmt) = if let Some(precision_str) = precision_str {
        if precision_str.is_empty() {
            return Err(ParseError::Unknown);
        }
        if precision_str.len() > 9 {
            return Err(ParseError::Unknown);
        }

        (
            format!(
                "{}T{}.{:0<width$}Z",
                date_pair.as_str(),
                time_pair.as_str(),
                precision_str,
                width = [3, 3, 3, 6, 6, 6, 9, 9, 9][precision_str.len() - 1]
            ),
            format!(
                "%Y-%m-%dT%H:%M:%S%.{}fZ",
                [3, 3, 3, 6, 6, 6, 9, 9, 9][precision_str.len() - 1]
            ),
        )
    } else {
        (
            format!("{}T{}Z", date_pair.as_str(), time_pair.as_str()),
            "%Y-%m-%dT%H:%M:%SZ".to_string(),
        )
    };

    ChronoNaiveDateTime::parse_from_str(&str, &fmt)
        .map(Into::into)
        .map_err(|err| ParseError::ValueInvalid(err.to_string()))
}

fn from_unix_timestamp_pairs(
    mut datetime_unix_timestamp_pairs: Pairs<'_, Rule>,
) -> Result<NaiveDateTime, ParseError> {
    let unix_timestamp_pair = datetime_unix_timestamp_pairs
        .next()
        .ok_or(ParseError::Unknown)?;
    let precision_str = datetime_unix_timestamp_pairs
        .next()
        .map(|time_nf_pair| time_nf_pair.as_str());

    let secs: u64 = unix_timestamp_pair
        .as_str()
        .parse()
        .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

    if secs > UNIX_TIMESTAMP_MAX {
        return Err(ParseError::ValueInvalid(
            "Override the max Unix Timestamp".to_string(),
        ));
    }

    if let Some(precision_str) = precision_str {
        let nsecs_str = format!("{:0<9}", precision_str);

        let nsecs: u32 = nsecs_str
            .parse()
            .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

        Ok(ChronoNaiveDateTime::from_timestamp(secs as i64, nsecs).into())
    } else {
        Ok(ChronoNaiveDateTime::from_timestamp(secs as i64, 0).into())
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

    use std::{error, fs, path::PathBuf};

    use chrono::NaiveDate;

    #[test]
    fn test_parse() -> Result<(), Box<dyn error::Error>> {
        let dt_vec = vec![
            NaiveDate::from_ymd(2021, 3, 1).and_hms(1, 2, 3),
            NaiveDate::from_ymd(2021, 3, 1).and_hms_milli(1, 2, 3, 100),
            NaiveDate::from_ymd(2021, 3, 1).and_hms_milli(1, 2, 3, 120),
            NaiveDate::from_ymd(2021, 3, 1).and_hms_milli(1, 2, 3, 123),
            NaiveDate::from_ymd(2021, 3, 1).and_hms_micro(1, 2, 3, 123400),
            NaiveDate::from_ymd(2021, 3, 1).and_hms_micro(1, 2, 3, 123450),
            NaiveDate::from_ymd(2021, 3, 1).and_hms_micro(1, 2, 3, 123456),
            NaiveDate::from_ymd(2021, 3, 1).and_hms_nano(1, 2, 3, 123456700),
            NaiveDate::from_ymd(2021, 3, 1).and_hms_nano(1, 2, 3, 123456780),
            NaiveDate::from_ymd(2021, 3, 1).and_hms_nano(1, 2, 3, 123456789),
        ];

        for (s, dt) in vec![
            "2021-03-01 01:02:03",
            "2021-03-01 01:02:03.1",
            "2021-03-01 01:02:03.12",
            "2021-03-01 01:02:03.123",
            "2021-03-01 01:02:03.1234",
            "2021-03-01 01:02:03.12345",
            "2021-03-01 01:02:03.123456",
            "2021-03-01 01:02:03.1234567",
            "2021-03-01 01:02:03.12345678",
            "2021-03-01 01:02:03.123456789",
        ]
        .into_iter()
        .zip(dt_vec.clone())
        {
            assert_eq!(s.parse::<NaiveDateTime>()?, dt.into());
        }

        for (s, dt) in vec![
            "2021-03-01T01:02:03Z",
            "2021-03-01T01:02:03.1Z",
            "2021-03-01T01:02:03.12Z",
            "2021-03-01T01:02:03.123Z",
            "2021-03-01T01:02:03.1234Z",
            "2021-03-01T01:02:03.12345Z",
            "2021-03-01T01:02:03.123456Z",
            "2021-03-01T01:02:03.1234567Z",
            "2021-03-01T01:02:03.12345678Z",
            "2021-03-01T01:02:03.123456789Z",
        ]
        .into_iter()
        .zip(dt_vec.clone())
        {
            assert_eq!(s.parse::<NaiveDateTime>()?, dt.into());
        }

        for (s, dt) in vec![
            "1614560523",
            "1614560523.1",
            "1614560523.12",
            "1614560523.123",
            "1614560523.1234",
            "1614560523.12345",
            "1614560523.123456",
            "1614560523.1234567",
            "1614560523.12345678",
            "1614560523.123456789",
        ]
        .into_iter()
        .zip(dt_vec)
        {
            assert_eq!(s.parse::<NaiveDateTime>()?, dt.into());
        }

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
    struct Row {
        #[serde(deserialize_with = "crate::datetime::deserialize")]
        datetime_utc: chrono::NaiveDateTime,
        #[allow(dead_code)]
        datetime_shanghai: NaiveDateTime,
    }
    #[derive(Deserialize)]
    struct Row64 {
        #[serde(deserialize_with = "crate::datetime::deserialize")]
        datetime64_precision0_utc: chrono::NaiveDateTime,
        #[serde(deserialize_with = "crate::datetime::deserialize")]
        datetime64_precision1_utc: chrono::NaiveDateTime,
        //
        #[serde(deserialize_with = "crate::datetime::deserialize")]
        datetime64_milli_utc: chrono::NaiveDateTime,
        #[allow(dead_code)]
        datetime64_milli_shanghai: NaiveDateTime,
        #[serde(deserialize_with = "crate::datetime::deserialize")]
        datetime64_micro_utc: chrono::NaiveDateTime,
        #[allow(dead_code)]
        datetime64_micro_shanghai: NaiveDateTime,
        #[serde(deserialize_with = "crate::datetime::deserialize")]
        datetime64_nano_utc: chrono::NaiveDateTime,
        #[allow(dead_code)]
        datetime64_nano_shanghai: NaiveDateTime,
    }

    #[test]
    fn test_de() -> Result<(), Box<dyn error::Error>> {
        let deserializer = de::IntoDeserializer::<de::value::Error>::into_deserializer;
        assert_eq!(
            super::deserialize(deserializer("2021-03-01 01:02:03")).unwrap(),
            NaiveDate::from_ymd(2021, 3, 1).and_hms(1, 2, 3)
        );

        for format in ["simple", "iso", "unix_timestamp"].iter() {
            let content = fs::read_to_string(
                PathBuf::new().join(format!("tests/files/datetime_{}.txt", format)),
            )?;
            let line = content.lines().next().unwrap();

            let Row {
                datetime_utc,
                datetime_shanghai: _,
            } = serde_json::from_str(line)?;
            assert_eq!(
                datetime_utc,
                NaiveDate::from_ymd(2021, 3, 1).and_hms(1, 2, 3)
            );

            //
            let content = fs::read_to_string(
                PathBuf::new().join(format!("tests/files/datetime64_{}.txt", format)),
            )?;
            let line = content.lines().next().unwrap();

            let Row64 {
                datetime64_precision0_utc,
                datetime64_precision1_utc,
                datetime64_milli_utc,
                datetime64_milli_shanghai: _,
                datetime64_micro_utc,
                datetime64_micro_shanghai: _,
                datetime64_nano_utc,
                datetime64_nano_shanghai: _,
            } = serde_json::from_str(line)?;
            assert_eq!(
                datetime64_precision0_utc,
                NaiveDate::from_ymd(2021, 3, 1).and_hms(1, 2, 3)
            );
            assert_eq!(
                datetime64_precision1_utc,
                NaiveDate::from_ymd(2021, 3, 1).and_hms_milli(1, 2, 3, 100)
            );
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
