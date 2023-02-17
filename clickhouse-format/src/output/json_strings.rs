use std::collections::HashMap;

use serde::de::DeserializeOwned;

use crate::format_name::FormatName;

use super::{json::JsonOutput, Output, OutputResult};

type Inner<T> = JsonOutput<T>;

#[derive(Default)]
pub struct JsonStringsOutput<T> {
    inner: Inner<T>,
}
impl<T> JsonStringsOutput<T> {
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
        }
    }
}
pub type GeneralJsonStringsOutput = JsonStringsOutput<HashMap<String, String>>;

impl<T> Output for JsonStringsOutput<T>
where
    T: DeserializeOwned,
{
    type Row = <Inner<T> as Output>::Row;
    type Info = <Inner<T> as Output>::Info;

    type Error = <Inner<T> as Output>::Error;

    fn format_name() -> crate::format_name::FormatName {
        FormatName::JsonStrings
    }

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        self.inner.deserialize(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::PathBuf};

    use crate::test_helpers::{TestStringsRow, TEST_STRINGS_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = PathBuf::new().join("tests/files/JSONStrings.json");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            GeneralJsonStringsOutput::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) = GeneralJsonStringsOutput::new().deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info.rows, 2);

        let (rows, info) =
            JsonStringsOutput::<TestStringsRow>::new().deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info.rows, 2);

        Ok(())
    }
}
