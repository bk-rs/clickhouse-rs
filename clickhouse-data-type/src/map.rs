use pest::iterators::{Pair, Pairs};

use crate::{
    array,
    decimal::{self, DecimalPrecision, DecimalScale},
    fixed_string::{self, FixedStringN},
    type_name::TypeName,
    type_name_parser::Rule,
    ParseError,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MapKey {
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
}

impl TryFrom<Pair<'_, Rule>> for MapKey {
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
            _ => Err(ParseError::Unknown),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MapValue {
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
    //
    //
    //
    Array(Box<TypeName>),
}

impl TryFrom<Pair<'_, Rule>> for MapValue {
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
            //
            //
            //
            Rule::Array => {
                let data_type = self::array::get_data_type(pair.into_inner())?;

                Ok(Self::Array(data_type.into()))
            }
            _ => Err(ParseError::Unknown),
        }
    }
}

pub(crate) fn get_map_key_and_map_value(
    mut map_pairs: Pairs<'_, Rule>,
) -> Result<(MapKey, MapValue), ParseError> {
    let key = MapKey::try_from(
        map_pairs
            .next()
            .ok_or(ParseError::Unknown)?
            .into_inner()
            .next()
            .ok_or(ParseError::Unknown)?,
    )?;
    let value = MapValue::try_from(
        map_pairs
            .next()
            .ok_or(ParseError::Unknown)?
            .into_inner()
            .next()
            .ok_or(ParseError::Unknown)?,
    )?;

    Ok((key, value))
}
