use pest::iterators::Pairs;

use crate::{ParseError, type_name::TypeName, type_name_parser::Rule};

pub(crate) fn get_data_type(mut array_pairs: Pairs<'_, Rule>) -> Result<TypeName, ParseError> {
    let data_type_pair = array_pairs
        .next()
        .ok_or(ParseError::Unknown)?
        .into_inner()
        .next()
        .ok_or(ParseError::Unknown)?;

    let data_type = TypeName::from_pair(data_type_pair)?;

    Ok(data_type)
}
