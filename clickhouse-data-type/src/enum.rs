use core::num::ParseIntError;
use std::collections::HashMap;

use pest::iterators::Pairs;

use crate::{ParseError, type_name_parser::Rule};

pub type Enum8 = HashMap<String, i8>;
pub type Enum16 = HashMap<String, i16>;

pub(crate) fn get_enum8(enum_pairs: Pairs<'_, Rule>) -> Result<Enum8, ParseError> {
    let mut map = HashMap::new();
    for pair in enum_pairs {
        let mut pair_inner = pair.into_inner();
        let key = pair_inner
            .next()
            .ok_or(ParseError::Unknown)?
            .as_str()
            .to_string();
        let value = pair_inner
            .next()
            .ok_or(ParseError::Unknown)?
            .as_str()
            .parse()
            .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

        map.insert(key, value);
    }
    Ok(map)
}

pub(crate) fn get_enum16(enum_pairs: Pairs<'_, Rule>) -> Result<Enum16, ParseError> {
    let mut map = HashMap::new();
    for pair in enum_pairs {
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
    Ok(map)
}
