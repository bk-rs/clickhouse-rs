use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use chrono_tz::Tz;
use pest::Parser as _;

// https://github.com/pest-parser/pest/issues/490#issuecomment-808942497
#[allow(clippy::upper_case_acronyms)]
mod type_name_parser {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "grammars/type_name.pest"]
    pub(super) struct TypeNameParser;
}
use type_name_parser::{Rule, TypeNameParser};

const DECIMAL_PRECISION_MIN: u8 = 1;
const DECIMAL_PRECISION_MAX: u8 = 76;

const FIXEDSTRING_N_MIN: usize = 1;

const DATETIME64_PRECISION_MAX: u8 = 9;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeName {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt256,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Int256,
    Float32,
    Float64,
    Decimal { precision: u8, scale: u8 },
    String,
    FixedString { n: usize },
    Uuid,
    Date,
    DateTime { timezone: Option<Tz> },
    DateTime64 { precision: u8, timezone: Option<Tz> },
    Enum8(HashMap<String, i8>),
    Enum16(HashMap<String, i16>),
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
impl FromStr for TypeName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = TypeNameParser::parse(Rule::type_name, s)
            .map_err(|err| ParseError::FormatMismatch(err.to_string()))?
            .next()
            .ok_or(ParseError::Unknown)?
            .into_inner()
            .next()
            .ok_or(ParseError::Unknown)?;

        match pair.as_rule() {
            Rule::UInt8 => Ok(Self::UInt8),
            Rule::UInt16 => Ok(Self::UInt16),
            Rule::UInt32 => Ok(Self::UInt32),
            Rule::UInt64 => Ok(Self::UInt64),
            Rule::UInt256 => Ok(Self::UInt256),
            Rule::Int8 => Ok(Self::Int8),
            Rule::Int16 => Ok(Self::Int16),
            Rule::Int32 => Ok(Self::Int32),
            Rule::Int64 => Ok(Self::Int64),
            Rule::Int128 => Ok(Self::Int128),
            Rule::Int256 => Ok(Self::Int256),
            Rule::Float32 => Ok(Self::Float32),
            Rule::Float64 => Ok(Self::Float64),
            Rule::Decimal => {
                let mut pair_inner = pair.into_inner();
                let precision: u8 = pair_inner
                    .next()
                    .ok_or(ParseError::Unknown)?
                    .as_str()
                    .parse()
                    .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

                if precision < DECIMAL_PRECISION_MIN {
                    return Err(ParseError::ValueInvalid(
                        "invalid decimal precision".to_string(),
                    ));
                }
                if precision > DECIMAL_PRECISION_MAX {
                    return Err(ParseError::ValueInvalid(
                        "invalid decimal precision".to_string(),
                    ));
                }

                let scale: u8 = pair_inner
                    .next()
                    .ok_or(ParseError::Unknown)?
                    .as_str()
                    .parse()
                    .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

                if scale > precision {
                    return Err(ParseError::ValueInvalid(
                        "invalid decimal scale".to_string(),
                    ));
                }

                Ok(Self::Decimal { precision, scale })
            }
            Rule::String => Ok(Self::String),
            Rule::FixedString => {
                let mut pair_inner = pair.into_inner();
                let n: usize = pair_inner
                    .next()
                    .ok_or(ParseError::Unknown)?
                    .as_str()
                    .parse()
                    .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

                if n < FIXEDSTRING_N_MIN {
                    return Err(ParseError::ValueInvalid(
                        "invalid fixedstring n".to_string(),
                    ));
                }

                Ok(Self::FixedString { n })
            }
            Rule::UUID => Ok(Self::Uuid),
            Rule::Date => Ok(Self::Date),
            Rule::DateTime => {
                let mut pair_inner = pair.into_inner();
                let timezone = if let Some(pair_timezone) = pair_inner.next() {
                    Some(
                        pair_timezone
                            .as_str()
                            .parse::<Tz>()
                            .map_err(|err: &str| ParseError::ValueInvalid(err.to_string()))?,
                    )
                } else {
                    None
                };

                Ok(Self::DateTime { timezone })
            }
            Rule::DateTime64 => {
                let mut pair_inner = pair.into_inner();

                let precision: u8 = pair_inner
                    .next()
                    .ok_or(ParseError::Unknown)?
                    .as_str()
                    .parse()
                    .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

                if precision > DATETIME64_PRECISION_MAX {
                    return Err(ParseError::ValueInvalid(
                        "invalid datetime64 precision".to_string(),
                    ));
                }

                let timezone = if let Some(pair_timezone) = pair_inner.next() {
                    Some(
                        pair_timezone
                            .as_str()
                            .parse::<Tz>()
                            .map_err(|err: &str| ParseError::ValueInvalid(err.to_string()))?,
                    )
                } else {
                    None
                };

                Ok(Self::DateTime64 {
                    precision,
                    timezone,
                })
            }
            Rule::Enum8 => {
                let pair_inner = pair.into_inner();

                let mut map = HashMap::new();
                for pair in pair_inner {
                    let mut pair_inner = pair.into_inner();
                    let key = pair_inner
                        .next()
                        .ok_or(ParseError::Unknown)?
                        .as_str()
                        .to_string();
                    let value: i8 = pair_inner
                        .next()
                        .ok_or(ParseError::Unknown)?
                        .as_str()
                        .parse()
                        .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

                    map.insert(key, value);
                }

                Ok(Self::Enum8(map))
            }
            Rule::Enum16 => {
                let pair_inner = pair.into_inner();

                let mut map = HashMap::new();
                for pair in pair_inner {
                    let mut pair_inner = pair.into_inner();
                    let key = pair_inner
                        .next()
                        .ok_or(ParseError::Unknown)?
                        .as_str()
                        .to_string();
                    let value: i16 = pair_inner
                        .next()
                        .ok_or(ParseError::Unknown)?
                        .as_str()
                        .parse()
                        .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

                    map.insert(key, value);
                }

                Ok(Self::Enum16(map))
            }
            _ => Err(ParseError::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    #[test]
    fn test_parse_int_uint() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/int_uint.txt"))?;
        let line = content.lines().skip(2).next().unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::UInt8, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::UInt16, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::UInt32, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::UInt64, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::UInt256, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::Int8, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::Int16, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::Int32, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::Int64, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::Int128, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::Int256, iter.next().unwrap().parse()?);

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_float() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/float.txt"))?;
        let line = content.lines().skip(2).next().unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::Float32, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::Float64, iter.next().unwrap().parse()?);

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_decimal() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/decimal.txt"))?;
        let line = content.lines().skip(2).next().unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        for (precision, scale) in vec![
            (9, 9),
            (9, 1),
            (18, 18),
            (18, 2),
            (38, 38),
            (38, 3),
            (76, 76),
            (76, 4),
        ]
        .into_iter()
        {
            assert_eq!(
                TypeName::Decimal {
                    precision: precision,
                    scale: scale
                },
                iter.next().unwrap().parse()?
            );
        }

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_string() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/string.txt"))?;
        let line = content.lines().skip(2).next().unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::String, iter.next().unwrap().parse()?);

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_fixedstring() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/fixedstring.txt"))?;
        let line = content.lines().skip(2).next().unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(
            TypeName::FixedString { n: 8 },
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_uuid() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/uuid.txt"))?;
        let line = content.lines().skip(2).next().unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::Uuid, iter.next().unwrap().parse()?);

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_date() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/date.txt"))?;
        let line = content.lines().skip(2).next().unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::Date, iter.next().unwrap().parse()?);

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_datetime() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/datetime.txt"))?;
        let line = content.lines().skip(2).next().unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(
            TypeName::DateTime { timezone: None },
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::DateTime {
                timezone: Some(Tz::UTC)
            },
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::DateTime {
                timezone: Some(Tz::Asia__Shanghai)
            },
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_datetime64() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/datetime64.txt"))?;
        let line = content.lines().skip(2).next().unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(
            TypeName::DateTime64 {
                precision: 0,
                timezone: None
            },
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::DateTime64 {
                precision: 3,
                timezone: Some(Tz::UTC)
            },
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::DateTime64 {
                precision: 9,
                timezone: Some(Tz::Asia__Shanghai)
            },
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_enum() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/enum.txt"))?;
        let line = content.lines().skip(2).next().unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(
            TypeName::Enum8(
                vec![("a".to_owned(), -128), ("b".to_owned(), 127)]
                    .into_iter()
                    .collect()
            ),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Enum16(
                vec![("a".to_owned(), -32768), ("b".to_owned(), 32767)]
                    .into_iter()
                    .collect()
            ),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Enum8(
                vec![("0".to_owned(), 0), ("1".to_owned(), 1)]
                    .into_iter()
                    .collect()
            ),
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }
}
