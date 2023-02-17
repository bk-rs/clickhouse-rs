use core::marker::PhantomData;
use std::collections::HashMap;

use csv::ReaderBuilder;
use serde::de::DeserializeOwned;

use crate::format_name::FormatName;

use super::{tsv::TsvOutput, Output, OutputResult};

pub struct TsvWithNamesOutput<T> {
    types: Option<Vec<String>>,
    phantom: PhantomData<T>,
}
impl<T> Default for TsvWithNamesOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> TsvWithNamesOutput<T> {
    pub fn new() -> Self {
        Self {
            types: None,
            phantom: PhantomData,
        }
    }
    pub fn with_types(types: Vec<String>) -> Self {
        Self {
            types: Some(types),
            phantom: PhantomData,
        }
    }
}

impl<T> Output for TsvWithNamesOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = Option<HashMap<String, String>>;

    type Error = csv::Error;

    fn format_name() -> FormatName {
        FormatName::TsvWithNames
    }

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_reader(slice);

        let header = rdr.headers()?;
        let names: Vec<String> = header.iter().map(ToOwned::to_owned).collect();

        let records = rdr.into_records();

        TsvOutput::from_raw_parts(Some(names), self.types.to_owned())
            .deserialize_with_records(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::PathBuf};

    use crate::test_helpers::{TestStringsRow, TEST_STRINGS_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = PathBuf::new().join("tests/files/TSVWithNames.tsv");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            TsvWithNamesOutput::<HashMap<String, String>>::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) =
            TsvWithNamesOutput::<HashMap<String, String>>::new().deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, None);

        let (rows, info) =
            TsvWithNamesOutput::<TestStringsRow>::new().deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info, None);

        Ok(())
    }
}
