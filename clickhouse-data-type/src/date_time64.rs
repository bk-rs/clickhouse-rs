use core::num::ParseIntError;

use chrono_tz::Tz;
use pest::iterators::Pairs;

use crate::{ParseError, type_name_parser::Rule};

const PRECISION_MAX: usize = 9;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DateTime64Precision(pub usize);
impl TryFrom<&str> for DateTime64Precision {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let n: usize = s
            .parse()
            .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

        if n > PRECISION_MAX {
            return Err(ParseError::ValueInvalid(
                "invalid datetime64 precision".to_string(),
            ));
        }

        Ok(Self(n))
    }
}

pub(crate) fn get_precision_and_timezone(
    mut date_time64_pairs: Pairs<'_, Rule>,
) -> Result<(DateTime64Precision, Option<Tz>), ParseError> {
    let precision_pair = date_time64_pairs.next().ok_or(ParseError::Unknown)?;
    let precision = DateTime64Precision::try_from(precision_pair.as_str())?;

    let timezone = if let Some(pair_timezone) = date_time64_pairs.next() {
        Some(
            pair_timezone
                .as_str()
                .parse::<Tz>()
                .map_err(|err: &str| ParseError::ValueInvalid(err.to_string()))?,
        )
    } else {
        None
    };

    Ok((precision, timezone))
}
