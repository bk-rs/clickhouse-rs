use core::marker::PhantomData;
use std::io::Error as IoError;

use csv::ReaderBuilder;
use indexmap::IndexMap;
use serde::de::DeserializeOwned;

use crate::format_name::FormatName;

use super::{Output, OutputResult, tsv::TsvOutput};

pub struct TsvWithNamesAndTypesOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for TsvWithNamesAndTypesOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> TsvWithNamesAndTypesOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T> Output for TsvWithNamesAndTypesOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = IndexMap<String, String>;

    type Error = csv::Error;

    fn format_name() -> FormatName {
        FormatName::TsvWithNamesAndTypes
    }

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_reader(slice);

        let header = rdr.headers()?;
        let names: Vec<String> = header.iter().map(ToOwned::to_owned).collect();

        let mut records = rdr.into_records();

        let record = records.next().ok_or_else(|| IoError::other(""))??;
        let types: Vec<String> = record.iter().map(ToOwned::to_owned).collect();

        let info = names
            .iter()
            .zip(types.iter())
            .map(|(name, type_)| (name.to_owned(), type_.to_owned()))
            .collect();

        TsvOutput::with_names_and_types(names, types)
            .deserialize_with_records(records)
            .map(|(rows, _)| (rows, info))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{collections::HashMap, fs, path::PathBuf};

    use crate::test_helpers::{TEST_STRINGS_ROW_1, TestStringsRow};

    #[test]
    fn simple() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = PathBuf::new().join("tests/files/TSVWithNamesAndTypes.tsv");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            TsvWithNamesAndTypesOutput::<HashMap<String, String>>::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) = TsvWithNamesAndTypesOutput::<HashMap<String, String>>::new()
            .deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info.get("array1"), Some(&"Array(UInt8)".to_owned()));
        assert_eq!(info.get("array2"), Some(&"Array(String)".to_owned()));
        assert_eq!(info.get("tuple1"), Some(&"Tuple(UInt8, String)".to_owned()));
        assert_eq!(
            info.get("tuple2"),
            Some(&"Tuple(UInt8, Nullable(String))".to_owned())
        );
        assert_eq!(info.get("map1"), Some(&"Map(String, String)".to_owned()));

        let (rows, info) =
            TsvWithNamesAndTypesOutput::<TestStringsRow>::new().deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info.get("array1"), Some(&"Array(UInt8)".to_owned()));
        assert_eq!(info.get("array2"), Some(&"Array(String)".to_owned()));
        assert_eq!(info.get("tuple1"), Some(&"Tuple(UInt8, String)".to_owned()));
        assert_eq!(
            info.get("tuple2"),
            Some(&"Tuple(UInt8, Nullable(String))".to_owned())
        );
        assert_eq!(info.get("map1"), Some(&"Map(String, String)".to_owned()));

        Ok(())
    }
}
