use std::collections::HashMap;

use serde::de::DeserializeOwned;

use crate::format_name::FormatName;

use super::{json_compact_each_row::JsonCompactEachRowOutput, Output, OutputResult};

type Inner<T> = JsonCompactEachRowOutput<T>;

pub struct JsonCompactStringsEachRowOutput<T> {
    inner: Inner<T>,
}
impl<T> JsonCompactStringsEachRowOutput<T> {
    pub fn new(names: Vec<String>) -> Self {
        Self {
            inner: Inner::new(names),
        }
    }
}

pub type GeneralJsonCompactStringsEachRowOutput =
    JsonCompactStringsEachRowOutput<HashMap<String, String>>;

impl<T> Output for JsonCompactStringsEachRowOutput<T>
where
    T: DeserializeOwned,
{
    type Row = <Inner<T> as Output>::Row;
    type Info = <Inner<T> as Output>::Info;

    type Error = <Inner<T> as Output>::Error;

    fn format_name() -> crate::format_name::FormatName {
        FormatName::JsonCompactStringsEachRow
    }

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        self.inner.deserialize(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::{TestStringsRow, TEST_STRINGS_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let file_path = PathBuf::new().join("tests/files/JSONCompactStringsEachRow.txt");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            GeneralJsonCompactStringsEachRowOutput::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) = GeneralJsonCompactStringsEachRowOutput::new(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, ());

        let (rows, info) = JsonCompactStringsEachRowOutput::<TestStringsRow>::new(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info, ());

        Ok(())
    }
}
