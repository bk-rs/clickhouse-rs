use core::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use chrono::NaiveDate as ChronoNaiveDate;
use serde::{
    Deserialize, Deserializer,
    de::{self, Visitor},
};

#[derive(PartialEq, Debug, Clone)]
pub struct NaiveDate(pub ChronoNaiveDate);
impl From<ChronoNaiveDate> for NaiveDate {
    fn from(inner: ChronoNaiveDate) -> Self {
        Self(inner)
    }
}
impl Deref for NaiveDate {
    type Target = ChronoNaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for NaiveDate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub type ParseError = chrono::ParseError;
impl FromStr for NaiveDate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ChronoNaiveDate::parse_from_str(s, "%Y-%m-%d").map(Into::into)
    }
}

struct NaiveDateVisitor;
impl<'de> Visitor<'de> for NaiveDateVisitor {
    type Value = NaiveDate;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("format %Y-%m-%d")
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
impl<'de> Deserialize<'de> for NaiveDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NaiveDateVisitor)
    }
}
pub fn deserialize<'de, D>(d: D) -> Result<ChronoNaiveDate, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_str(NaiveDateVisitor).map(|x| x.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::NaiveDate;

    #[test]
    fn test_parse() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            "2021-03-01"
                .parse::<NaiveDate>()
                .map_err(|err| err.to_string())?,
            NaiveDate::from_ymd_opt(2021, 3, 1).expect("")
        );

        Ok(())
    }

    #[derive(Deserialize)]
    struct Row {
        #[serde(deserialize_with = "crate::date::deserialize")]
        date: chrono::NaiveDate,
    }

    #[test]
    fn test_de() -> Result<(), Box<dyn std::error::Error>> {
        let deserializer = de::IntoDeserializer::<de::value::Error>::into_deserializer;
        assert_eq!(
            super::deserialize(deserializer("2021-03-01")).unwrap(),
            NaiveDate::from_ymd_opt(2021, 3, 1).expect("")
        );

        let content = include_str!("../tests/files/date.txt");
        let line = content.lines().next().unwrap();

        let Row { date } = serde_json::from_str(line)?;
        assert_eq!(date, NaiveDate::from_ymd_opt(2021, 3, 1).expect(""));

        Ok(())
    }
}
