use std::collections::HashMap;

use serde::de::DeserializeOwned;

use crate::format_name::FormatName;

use super::{json_each_row::JsonEachRowOutput, Output, OutputResult};

type Inner<T> = JsonEachRowOutput<T>;

#[derive(Default)]
pub struct JsonStringsEachRowOutput<T> {
    inner: Inner<T>,
}
impl<T> JsonStringsEachRowOutput<T> {
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
        }
    }
}
pub type GeneralJsonStringsEachRowOutput = JsonStringsEachRowOutput<HashMap<String, String>>;

impl<T> Output for JsonStringsEachRowOutput<T>
where
    T: DeserializeOwned,
{
    type Row = <Inner<T> as Output>::Row;
    type Info = <Inner<T> as Output>::Info;

    type Error = <Inner<T> as Output>::Error;

    fn format_name() -> crate::format_name::FormatName {
        FormatName::JsonStringsEachRow
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
        let file_path = PathBuf::new().join("tests/files/JSONStringsEachRow.txt");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            GeneralJsonStringsEachRowOutput::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) =
            GeneralJsonStringsEachRowOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, ());

        let (rows, info) = JsonStringsEachRowOutput::<TestStringsRow>::new()
            .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info, ());

        Ok(())
    }
}
