use std::collections::HashMap;

use serde::de::DeserializeOwned;

use crate::format_name::FormatName;

use super::{Output, OutputResult, json_each_row_with_progress::JsonEachRowWithProgressOutput};

type Inner<T> = JsonEachRowWithProgressOutput<T>;

#[derive(Default)]
pub struct JsonStringsEachRowWithProgressOutput<T> {
    inner: Inner<T>,
}
impl<T> JsonStringsEachRowWithProgressOutput<T> {
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
        }
    }
}

pub type GeneralJsonStringsEachRowWithProgressOutput =
    JsonStringsEachRowWithProgressOutput<HashMap<String, String>>;

impl<T> Output for JsonStringsEachRowWithProgressOutput<T>
where
    T: DeserializeOwned,
{
    type Row = <Inner<T> as Output>::Row;
    type Info = <Inner<T> as Output>::Info;

    type Error = <Inner<T> as Output>::Error;

    fn format_name() -> crate::format_name::FormatName {
        FormatName::JsonStringsEachRowWithProgress
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
        let file_path = PathBuf::new().join("tests/files/JSONStringsEachRowWithProgress.txt");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            GeneralJsonStringsEachRowWithProgressOutput::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) =
            GeneralJsonStringsEachRowWithProgressOutput::new().deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info.read_rows, 2);

        let (rows, info) = JsonStringsEachRowWithProgressOutput::<TestStringsRow>::new()
            .deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info.read_rows, 2);

        Ok(())
    }
}
