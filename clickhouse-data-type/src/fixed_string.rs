use core::num::ParseIntError;

use pest::iterators::Pairs;

use crate::{ParseError, type_name_parser::Rule};

const N_MIN: usize = 1;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FixedStringN(pub usize);
impl TryFrom<&str> for FixedStringN {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let n: usize = s
            .parse()
            .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

        if n < N_MIN {
            return Err(ParseError::ValueInvalid(
                "invalid fixedstring n".to_string(),
            ));
        }

        Ok(Self(n))
    }
}

pub(crate) fn get_n(mut fixed_string_pairs: Pairs<'_, Rule>) -> Result<FixedStringN, ParseError> {
    let n_pair = fixed_string_pairs.next().ok_or(ParseError::Unknown)?;

    let n = FixedStringN::try_from(n_pair.as_str())?;

    Ok(n)
}
