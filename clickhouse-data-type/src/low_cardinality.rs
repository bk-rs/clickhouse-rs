use chrono_tz::Tz;
use pest::iterators::Pairs;

use crate::{
    date_time,
    fixed_string::{self, FixedStringN},
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
}

pub(crate) fn get_data_type(
    mut low_cardinality_pairs: Pairs<'_, Rule>,
) -> Result<LowCardinalityDataType, ParseError> {
    let low_cardinality_pair = low_cardinality_pairs.next().ok_or(ParseError::Unknown)?;

    let mut data_type_pairs = low_cardinality_pair.into_inner();
    let data_type_pair = data_type_pairs.next().ok_or(ParseError::Unknown)?;

    let data_type = match data_type_pair.as_rule() {
        Rule::UInt8 => LowCardinalityDataType::UInt8,
        Rule::UInt16 => LowCardinalityDataType::UInt16,
        Rule::UInt32 => LowCardinalityDataType::UInt32,
        Rule::UInt64 => LowCardinalityDataType::UInt64,
        Rule::Int8 => LowCardinalityDataType::Int8,
        Rule::Int16 => LowCardinalityDataType::Int16,
        Rule::Int32 => LowCardinalityDataType::Int32,
        Rule::Int64 => LowCardinalityDataType::Int64,
        Rule::Float32 => LowCardinalityDataType::Float32,
        Rule::Float64 => LowCardinalityDataType::Float64,
        Rule::String => LowCardinalityDataType::String,
        Rule::FixedString => {
            let n = fixed_string::get_n(data_type_pair.into_inner())?;

            LowCardinalityDataType::FixedString(n)
        }
        Rule::Date => LowCardinalityDataType::Date,
        Rule::DateTime => {
            let timezone = date_time::get_timezone(data_type_pair.into_inner())?;

            LowCardinalityDataType::DateTime(timezone)
        }
        _ => return Err(ParseError::Unknown),
    };

    Ok(data_type)
}
