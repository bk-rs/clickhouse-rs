use std::convert::TryFrom;

use chrono_tz::Tz;
use pest::iterators::Pair;

use crate::{
    type_name_parser::{parse_date_time, parse_fixed_string, Rule},
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
    FixedString(usize),
    Date,
    DateTime { timezone: Option<Tz> },
}

impl TryFrom<Pair<'_, Rule>> for LowCardinalityDataType {
    type Error = ParseError;

    fn try_from(data_type_pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match data_type_pair.as_rule() {
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
                let n = parse_fixed_string(data_type_pair)?;

                Ok(Self::FixedString(n))
            }
            Rule::Date => Ok(Self::Date),
            Rule::DateTime => {
                let timezone = parse_date_time(data_type_pair)?;

                Ok(Self::DateTime { timezone })
            }
            _ => Err(ParseError::Unknown),
        }
    }
}
