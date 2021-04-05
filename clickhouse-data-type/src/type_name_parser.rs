use std::num::ParseIntError;

use chrono_tz::Tz;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::ParseError;

const FIXEDSTRING_N_MIN: usize = 1;

#[derive(Parser)]
#[grammar = "grammars/type_name.pest"]
pub(crate) struct TypeNameParser;

pub(crate) type FixedStringN = usize;
pub(crate) fn parse_fixed_string(pair: Pair<Rule>) -> Result<FixedStringN, ParseError> {
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

    Ok(n)
}

pub(crate) fn parse_date_time(pair: Pair<Rule>) -> Result<Option<Tz>, ParseError> {
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

    Ok(timezone)
}
