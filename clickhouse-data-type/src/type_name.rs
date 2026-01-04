use core::str::FromStr;

use chrono_tz::Tz;
use pest::{Parser as _, iterators::Pair};

use crate::{
    ParseError, array, date_time,
    date_time64::{self, DateTime64Precision},
    decimal::{self, DecimalPrecision, DecimalScale},
    r#enum::{self, Enum8, Enum16},
    fixed_string::{self, FixedStringN},
    low_cardinality::{self, LowCardinalityDataType},
    map::{self, MapKey, MapValue},
    nullable::{self, NullableTypeName},
    type_name_parser::{Rule, TypeNameParser},
};

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
    Decimal(DecimalPrecision, DecimalScale),
    String,
    FixedString(FixedStringN),
    Uuid,
    Date,
    DateTime(Option<Tz>),
    DateTime64(DateTime64Precision, Option<Tz>),
    Enum8(Enum8),
    Enum16(Enum16),
    Ipv4,
    Ipv6,
    //
    //
    //
    LowCardinality(LowCardinalityDataType),
    Nullable(NullableTypeName),
    Point,
    Ring,
    Polygon,
    MultiPolygon,
    //
    //
    //
    Array(Box<Self>),
    Tuple(Vec<Self>),
    Map(MapKey, MapValue),
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

        Self::from_pair(pair)
    }
}

impl TypeName {
    pub(crate) fn from_pair(pair: Pair<'_, Rule>) -> Result<TypeName, ParseError> {
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
                let (precision, scale) = decimal::get_precision_and_scale(pair.into_inner())?;

                Ok(Self::Decimal(precision, scale))
            }
            Rule::String => Ok(Self::String),
            Rule::FixedString => {
                let n = fixed_string::get_n(pair.into_inner())?;

                Ok(Self::FixedString(n))
            }
            Rule::UUID => Ok(Self::Uuid),
            Rule::Date => Ok(Self::Date),
            Rule::DateTime => {
                let timezone = date_time::get_timezone(pair.into_inner())?;

                Ok(Self::DateTime(timezone))
            }
            Rule::DateTime64 => {
                let (precision, timezone) =
                    date_time64::get_precision_and_timezone(pair.into_inner())?;

                Ok(Self::DateTime64(precision, timezone))
            }
            Rule::Enum8 => {
                let inner = r#enum::get_enum8(pair.into_inner())?;

                Ok(Self::Enum8(inner))
            }
            Rule::Enum16 => {
                let inner = r#enum::get_enum16(pair.into_inner())?;

                Ok(Self::Enum16(inner))
            }
            Rule::IPv4 => Ok(Self::Ipv4),
            Rule::IPv6 => Ok(Self::Ipv6),
            //
            //
            //
            Rule::LowCardinality => {
                let data_type = low_cardinality::get_data_type(pair.into_inner())?;

                Ok(Self::LowCardinality(data_type))
            }
            Rule::Nullable => {
                let type_name = nullable::get_type_name(pair.into_inner())?;

                Ok(Self::Nullable(type_name))
            }
            Rule::Point => Ok(Self::Point),
            Rule::Ring => Ok(Self::Ring),
            Rule::Polygon => Ok(Self::Polygon),
            Rule::MultiPolygon => Ok(Self::MultiPolygon),
            //
            //
            //
            Rule::Array => {
                let data_type = array::get_data_type(pair.into_inner())?;

                Ok(Self::Array(data_type.into()))
            }
            Rule::Tuple => {
                let pairs: Vec<_> = pair.into_inner().collect();

                if pairs.is_empty() {
                    return Err(ParseError::Unknown);
                }

                let mut type_names = vec![];
                for pair in pairs {
                    let this =
                        Self::from_pair(pair.into_inner().next().ok_or(ParseError::Unknown)?)?;
                    type_names.push(this);
                }
                Ok(Self::Tuple(type_names))
            }
            Rule::Map => {
                let (map_key, map_value) = map::get_map_key_and_map_value(pair.into_inner())?;

                Ok(Self::Map(map_key, map_value))
            }
            _ => Err(ParseError::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::map::{MapKey, MapValue};

    #[test]
    fn test_parse_int_uint() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/int_uint.txt");
        let line = content.lines().nth(2).unwrap();

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
    fn test_parse_float() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/float.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::Float32, iter.next().unwrap().parse()?);
        assert_eq!(TypeName::Float64, iter.next().unwrap().parse()?);

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_decimal() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/decimal.txt");
        let line = content.lines().nth(2).unwrap();

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
                TypeName::Decimal(DecimalPrecision(precision), DecimalScale(scale)),
                iter.next().unwrap().parse()?
            );
        }

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_string() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/string.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::String, iter.next().unwrap().parse()?);

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_fixedstring() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/fixedstring.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(
            TypeName::FixedString(FixedStringN(8)),
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_uuid() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/uuid.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::Uuid, iter.next().unwrap().parse()?);

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_date() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/date.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::Date, iter.next().unwrap().parse()?);

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_datetime() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/datetime.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::DateTime(None), iter.next().unwrap().parse()?);
        assert_eq!(
            TypeName::DateTime(Some(Tz::UTC)),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::DateTime(Some(Tz::Asia__Shanghai)),
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_datetime64() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/datetime64.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(
            TypeName::DateTime64(DateTime64Precision(0), None),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::DateTime64(DateTime64Precision(3), Some(Tz::UTC)),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::DateTime64(DateTime64Precision(9), Some(Tz::Asia__Shanghai)),
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_enum() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/enum.txt");
        let line = content.lines().nth(2).unwrap();

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

    #[test]
    fn test_parse_lowcardinality() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/lowcardinality.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::String),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::FixedString(FixedStringN(1))),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::Date),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::DateTime(None)),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::UInt8),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::UInt16),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::UInt32),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::UInt64),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::Int8),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::Int16),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::Int32),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::Int64),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::Float32),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::Float64),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::Nullable(NullableTypeName::String)),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::Nullable(NullableTypeName::String)),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::Ipv4),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::LowCardinality(LowCardinalityDataType::Ipv6),
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_nullable() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/nullable.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(
            TypeName::Nullable(NullableTypeName::UInt8),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::UInt256),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::Int8),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::Int256),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::Float64),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::Decimal(
                DecimalPrecision(76),
                DecimalScale(4)
            )),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::String),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::FixedString(FixedStringN(1))),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::Uuid),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::Date),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::DateTime(None)),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::DateTime64(DateTime64Precision(0), None)),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::Enum8(
                vec![("a".to_owned(), -128), ("b".to_owned(), 127)]
                    .into_iter()
                    .collect()
            )),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::Nothing),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::Ipv4),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Nullable(NullableTypeName::Ipv6),
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_array() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/array.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(
            TypeName::Array(TypeName::UInt8.into()),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Array(TypeName::UInt8.into()),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Array(TypeName::Nullable(NullableTypeName::UInt8).into()),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Array(TypeName::Array(TypeName::UInt8.into()).into()),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Array(
                TypeName::Tuple(vec![
                    TypeName::UInt8,
                    TypeName::Nullable(NullableTypeName::Nothing)
                ])
                .into()
            ),
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_tuple() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/tuple.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(
            TypeName::Tuple(vec![TypeName::String, TypeName::UInt8]),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Tuple(vec![TypeName::String, TypeName::UInt8]),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Tuple(vec![
                TypeName::String,
                TypeName::Nullable(NullableTypeName::UInt8)
            ]),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Tuple(vec![
                TypeName::String,
                TypeName::Array(TypeName::UInt8.into()),
            ]),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Tuple(vec![
                TypeName::String,
                TypeName::Tuple(vec![
                    TypeName::UInt8,
                    TypeName::Nullable(NullableTypeName::Nothing)
                ]),
            ]),
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_ipv4() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/ipv4.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::Ipv4, iter.next().unwrap().parse()?);

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_ipv6() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/ipv6.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(TypeName::Ipv6, iter.next().unwrap().parse()?);

        assert_eq!(iter.next(), None);

        Ok(())
    }

    #[test]
    fn test_parse_geo() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(TypeName::Point, "Point".parse()?);
        assert_eq!(TypeName::Ring, "Ring".parse()?);
        assert_eq!(TypeName::Polygon, "Polygon".parse()?);
        assert_eq!(TypeName::MultiPolygon, "MultiPolygon".parse()?);

        Ok(())
    }

    #[test]
    fn test_parse_map() -> Result<(), Box<dyn std::error::Error>> {
        let content = include_str!("../tests/files/map.txt");
        let line = content.lines().nth(2).unwrap();

        let mut iter = serde_json::from_str::<Vec<String>>(line)?.into_iter();

        assert_eq!(
            TypeName::Map(MapKey::String, MapValue::String),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Map(MapKey::FixedString(FixedStringN(2)), MapValue::String),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Map(MapKey::UInt256, MapValue::String),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Map(MapKey::Int256, MapValue::String),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Map(MapKey::Float64, MapValue::String),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Map(
                MapKey::Decimal(DecimalPrecision(9), DecimalScale(9)),
                MapValue::String
            ),
            iter.next().unwrap().parse()?
        );
        assert_eq!(
            TypeName::Map(MapKey::String, MapValue::Array(TypeName::String.into())),
            iter.next().unwrap().parse()?
        );

        assert_eq!(iter.next(), None);

        Ok(())
    }
}
