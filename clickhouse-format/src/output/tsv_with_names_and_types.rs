use std::{collections::HashMap, io, marker::PhantomData};

use csv::ReaderBuilder;
use serde::de::DeserializeOwned;

use super::{tsv::TSVOutput, Output};

pub struct TSVWithNamesAndTypesOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for TSVWithNamesAndTypesOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> TSVWithNamesAndTypesOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T> Output for TSVWithNamesAndTypesOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = Option<HashMap<String, String>>;

    type Error = csv::Error;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error> {
        let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_reader(slice);

        let header = rdr.headers()?;
        let names: Vec<String> = header.iter().map(ToOwned::to_owned).collect();

        let mut records = rdr.into_records();

        let record = records
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, ""))??;
        let types: Vec<String> = record.iter().map(ToOwned::to_owned).collect();

        TSVOutput::with_names_and_types(names, types).deserialize_with_records(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::TestStringsRow;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content =
            fs::read_to_string(PathBuf::new().join("tests/files/TSVWithNamesAndTypes.tsv"))?;

        let info_expected: HashMap<String, String> = vec![
            ("array1".into(), "Array(UInt8)".into()),
            ("array2".into(), "Array(String)".into()),
            ("tuple1".into(), "Tuple(UInt8, String)".into()),
            ("tuple2".into(), "Tuple(UInt8, Nullable(Nothing))".into()),
            ("map1".into(), "Map(String,String)".into()),
        ]
        .into_iter()
        .collect();

        let (rows, info) = TSVWithNamesAndTypesOutput::<HashMap<String, String>>::new()
            .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, Some(info_expected.clone()));

        let (rows, info) = TSVWithNamesAndTypesOutput::<TestStringsRow>::new()
            .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, "(1,'a')");
        assert_eq!(info, Some(info_expected));

        Ok(())
    }
}
