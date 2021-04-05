use std::convert::TryFrom;

use chrono_tz::Tz;
use pest::iterators::{Pair, Pairs};

use crate::{
    date_time,
    date_time64::{self, DateTime64Precision},
    decimal::{self, DecimalPrecision, DecimalScale},
    fixed_string::{self, FixedStringN},
    r#enum::{self, Enum16, Enum8},
    type_name_parser::Rule,
    ParseError,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NullableTypeName {
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
}

impl TryFrom<Pair<'_, Rule>> for NullableTypeName {
    type Error = ParseError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
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
            _ => Err(ParseError::Unknown),
        }
    }
}

pub(crate) fn get_type_name(
    mut nullable_pairs: Pairs<'_, Rule>,
) -> Result<NullableTypeName, ParseError> {
    let nullable_pair = nullable_pairs.next().ok_or(ParseError::Unknown)?;

    let mut type_name_pairs = nullable_pair.into_inner();
    let type_name_pair = type_name_pairs.next().ok_or(ParseError::Unknown)?;

    let data_type = NullableTypeName::try_from(type_name_pair)?;

    Ok(data_type)
}
