use std::convert::TryFrom;

use chrono_tz::Tz;
use pest::iterators::{Pair, Pairs};

use crate::{
    date_time,
    fixed_string::{self, FixedStringN},
    nullable::{self, NullableTypeName},
    type_name_parser::Rule,
    ParseError,
};

// https://clickhouse.tech/docs/en/sql-reference/data-types/lowcardinality/
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum LowCardinalityDataType {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    String,
    FixedString(FixedStringN),
    Date,
    DateTime(Option<Tz>),
    Nullable(NullableTypeName),
}

impl TryFrom<Pair<'_, Rule>> for LowCardinalityDataType {
    type Error = ParseError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::UInt8 => Ok(Self::UInt8),
            Rule::UInt16 => Ok(Self::UInt16),
            Rule::UInt32 => Ok(Self::UInt32),
            Rule::UInt64 => Ok(Self::UInt64),
            Rule::Int8 => Ok(Self::Int8),
            Rule::Int16 => Ok(Self::Int16),
            Rule::Int32 => Ok(Self::Int32),
            Rule::Int64 => Ok(Self::Int64),
            Rule::Float32 => Ok(Self::Float32),
            Rule::Float64 => Ok(Self::Float64),
            Rule::String => Ok(Self::String),
            Rule::FixedString => {
                let n = fixed_string::get_n(pair.into_inner())?;

                Ok(Self::FixedString(n))
            }
            Rule::Date => Ok(Self::Date),
            Rule::DateTime => {
                let timezone = date_time::get_timezone(pair.into_inner())?;

                Ok(Self::DateTime(timezone))
            }
            Rule::Nullable => {
                let data_type = nullable::get_type_name(pair.into_inner())?;

                Ok(Self::Nullable(data_type))
            }
            _ => Err(ParseError::Unknown),
        }
    }
}

pub(crate) fn get_data_type(
    mut low_cardinality_pairs: Pairs<'_, Rule>,
) -> Result<LowCardinalityDataType, ParseError> {
    let low_cardinality_pair = low_cardinality_pairs.next().ok_or(ParseError::Unknown)?;

    let mut data_type_pairs = low_cardinality_pair.into_inner();
    let data_type_pair = data_type_pairs.next().ok_or(ParseError::Unknown)?;

    let data_type = LowCardinalityDataType::try_from(data_type_pair)?;

    Ok(data_type)
}
