use chrono_tz::Tz;
use pest::iterators::Pairs;

use crate::{ParseError, type_name_parser::Rule};

pub(crate) fn get_timezone(mut date_time_pairs: Pairs<'_, Rule>) -> Result<Option<Tz>, ParseError> {
    let timezone = if let Some(pair_timezone) = date_time_pairs.next() {
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
