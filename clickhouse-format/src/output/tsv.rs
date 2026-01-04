use core::marker::PhantomData;

use csv::{ReaderBuilder, StringRecordsIntoIter};
use indexmap::IndexMap;
use serde::de::DeserializeOwned;

use crate::format_name::FormatName;

use super::{Output, OutputResult, tsv_raw::TsvRawOutput};

pub struct TsvOutput<T> {
    names: Option<Vec<String>>,
    types: Option<Vec<String>>,
    phantom: PhantomData<T>,
}
impl<T> Default for TsvOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> TsvOutput<T> {
    pub fn new() -> Self {
        Self {
            names: None,
            types: None,
            phantom: PhantomData,
        }
    }
    pub fn with_names(names: Vec<String>) -> Self {
        Self {
            names: Some(names),
            types: None,
            phantom: PhantomData,
        }
    }
    pub fn with_names_and_types(names: Vec<String>, types: Vec<String>) -> Self {
        Self {
            names: Some(names),
            types: Some(types),
            phantom: PhantomData,
        }
    }
}

impl<T> Output for TsvOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = Option<IndexMap<String, String>>;

    type Error = csv::Error;

    fn format_name() -> FormatName {
        FormatName::Tsv
    }

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        let rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(slice);

        self.deserialize_with_records(rdr.into_records())
    }
}
impl<T> TsvOutput<T>
where
    T: DeserializeOwned,
{
    pub(crate) fn deserialize_with_records(
        &self,
        records: StringRecordsIntoIter<&[u8]>,
    ) -> OutputResult<<Self as Output>::Row, <Self as Output>::Info, <Self as Output>::Error> {
        // TODO, unescape
        TsvRawOutput::inner_new(self.names.to_owned(), self.types.to_owned())
            .deserialize_with_records(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{collections::HashMap, fs, path::PathBuf};

    use crate::test_helpers::{TEST_STRINGS_ROW_1, TestStringsRow};

    #[test]
    fn simple() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = PathBuf::new().join("tests/files/TSV.tsv");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            TsvOutput::<HashMap<String, String>>::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) = TsvOutput::<HashMap<String, String>>::with_names(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, None);

        let (rows, info) = TsvOutput::<TestStringsRow>::new().deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info, None);

        Ok(())
    }
}
