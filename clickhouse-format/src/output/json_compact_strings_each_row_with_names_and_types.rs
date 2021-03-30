use std::collections::HashMap;

use super::json_compact_each_row_with_names_and_types::JSONCompactEachRowWithNamesAndTypesOutput;

pub type JSONCompactStringsEachRowWithNamesAndTypesOutput<T> =
    JSONCompactEachRowWithNamesAndTypesOutput<T>;

pub type GeneralJSONCompactStringsEachRowWithNamesAndTypesOutput =
    JSONCompactStringsEachRowWithNamesAndTypesOutput<HashMap<String, String>>;

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::{
        output::Output as _,
        test_helpers::{TestStringsRow, TEST_STRINGS_ROW_1},
    };

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(
            PathBuf::new().join("tests/files/JSONCompactStringsEachRowWithNamesAndTypes.txt"),
        )?;

        let (rows, info) = GeneralJSONCompactStringsEachRowWithNamesAndTypesOutput::new()
            .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info.get("array1"), Some(&"Array(UInt8)".to_owned()));

        let (rows, info) =
            JSONCompactStringsEachRowWithNamesAndTypesOutput::<TestStringsRow>::new()
                .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info.get("array1"), Some(&"Array(UInt8)".to_owned()));

        Ok(())
    }
}
