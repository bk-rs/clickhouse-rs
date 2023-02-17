use core::num::ParseIntError;

use pest::iterators::Pairs;

use crate::{type_name_parser::Rule, ParseError};

const PRECISION_MIN: usize = 1;
const PRECISION_MAX: usize = 76;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DecimalPrecision(pub usize);
impl TryFrom<&str> for DecimalPrecision {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let precision: usize = s
            .parse()
            .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

        if precision < PRECISION_MIN {
            return Err(ParseError::ValueInvalid(
                "invalid decimal precision".to_string(),
            ));
        }
        if precision > PRECISION_MAX {
            return Err(ParseError::ValueInvalid(
                "invalid decimal precision".to_string(),
            ));
        }

        Ok(Self(precision))
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DecimalScale(pub usize);
impl TryFrom<(&str, &DecimalPrecision)> for DecimalScale {
    type Error = ParseError;
    fn try_from(t: (&str, &DecimalPrecision)) -> Result<Self, Self::Error> {
        let (s, precision) = t;

        let scale: usize = s
            .parse()
            .map_err(|err: ParseIntError| ParseError::ValueInvalid(err.to_string()))?;

        if scale > precision.0 {
            return Err(ParseError::ValueInvalid(
                "invalid decimal scale".to_string(),
            ));
        }

        Ok(Self(scale))
    }
}

pub(crate) fn get_precision_and_scale(
    mut decimal_pairs: Pairs<'_, Rule>,
) -> Result<(DecimalPrecision, DecimalScale), ParseError> {
    let precision_pair = decimal_pairs.next().ok_or(ParseError::Unknown)?;
    let scale_pair = decimal_pairs.next().ok_or(ParseError::Unknown)?;

    let precision = DecimalPrecision::try_from(precision_pair.as_str())?;

    let scale = DecimalScale::try_from((scale_pair.as_str(), &precision))?;

    Ok((precision, scale))
}
