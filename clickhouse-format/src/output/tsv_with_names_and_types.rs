use core::marker::PhantomData;
use std::{collections::HashMap, io::Error as IoError};

use csv::ReaderBuilder;
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
    type Info = Option<HashMap<String, String>>;

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

        TsvOutput::with_names_and_types(names, types).deserialize_with_records(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::PathBuf};

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

        let info_expected: HashMap<String, String> = vec![
            ("array1".into(), "Array(UInt8)".into()),
            ("array2".into(), "Array(String)".into()),
            ("tuple1".into(), "Tuple(UInt8, String)".into()),
            ("tuple2".into(), "Tuple(UInt8, Nullable(String))".into()),
            ("map1".into(), "Map(String,String)".into()),
        ]
        .into_iter()
        .collect();

        let (rows, info) = TsvWithNamesAndTypesOutput::<HashMap<String, String>>::new()
            .deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, Some(info_expected.clone()));

        let (rows, info) =
            TsvWithNamesAndTypesOutput::<TestStringsRow>::new().deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info, Some(info_expected));

        Ok(())
    }
}
