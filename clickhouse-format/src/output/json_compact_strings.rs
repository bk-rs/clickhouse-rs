use std::{collections::HashMap, marker::PhantomData};

use serde::de::DeserializeOwned;

use crate::format_name::FormatName;

use super::{json::JsonDataInfo, json_compact::JsonCompactOutput, Output, OutputResult};

pub struct JsonCompactStringsOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for JsonCompactStringsOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> JsonCompactStringsOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
pub type GeneralJsonCompactStringsOutput = JsonCompactStringsOutput<HashMap<String, String>>;

impl<T> Output for JsonCompactStringsOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = JsonDataInfo;

    type Error = serde_json::Error;

    fn format_name() -> FormatName {
        FormatName::JsonCompactStrings
    }

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        JsonCompactOutput::new().deserialize_with::<String>(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::{TestStringsRow, TEST_STRINGS_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let file_path = PathBuf::new().join("tests/files/JSONCompactStrings.json");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            GeneralJsonCompactStringsOutput::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) =
            GeneralJsonCompactStringsOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info.rows, 2);

        let (rows, info) = JsonCompactStringsOutput::<TestStringsRow>::new()
            .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info.rows, 2);

        Ok(())
    }
}
