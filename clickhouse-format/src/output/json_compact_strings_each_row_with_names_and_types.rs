use std::collections::HashMap;

use serde::de::DeserializeOwned;

use crate::format_name::FormatName;

use super::{
    Output, OutputResult,
    json_compact_each_row_with_names_and_types::JsonCompactEachRowWithNamesAndTypesOutput,
};

type Inner<T> = JsonCompactEachRowWithNamesAndTypesOutput<T>;

#[derive(Default)]
pub struct JsonCompactStringsEachRowWithNamesAndTypesOutput<T> {
    inner: Inner<T>,
}
impl<T> JsonCompactStringsEachRowWithNamesAndTypesOutput<T> {
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
        }
    }
}
pub type GeneralJsonCompactStringsEachRowWithNamesAndTypesOutput =
    JsonCompactStringsEachRowWithNamesAndTypesOutput<HashMap<String, String>>;

impl<T> Output for JsonCompactStringsEachRowWithNamesAndTypesOutput<T>
where
    T: DeserializeOwned,
{
    type Row = <Inner<T> as Output>::Row;
    type Info = <Inner<T> as Output>::Info;

    type Error = <Inner<T> as Output>::Error;

    fn format_name() -> crate::format_name::FormatName {
        FormatName::JsonCompactStringsEachRowWithNamesAndTypes
    }

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        self.inner.deserialize(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::PathBuf};

    use crate::test_helpers::{TEST_STRINGS_ROW_1, TestStringsRow};

    #[test]
    fn simple() -> Result<(), Box<dyn std::error::Error>> {
        let file_path =
            PathBuf::new().join("tests/files/JSONCompactStringsEachRowWithNamesAndTypes.txt");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            GeneralJsonCompactStringsEachRowWithNamesAndTypesOutput::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) = GeneralJsonCompactStringsEachRowWithNamesAndTypesOutput::new()
            .deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info.get("array1"), Some(&"Array(UInt8)".to_owned()));

        let (rows, info) =
            JsonCompactStringsEachRowWithNamesAndTypesOutput::<TestStringsRow>::new()
                .deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info.get("array1"), Some(&"Array(UInt8)".to_owned()));

        Ok(())
    }
}
