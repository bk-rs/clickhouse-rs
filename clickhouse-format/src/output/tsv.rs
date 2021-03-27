use std::{collections::HashMap, marker::PhantomData};

use csv::{ReaderBuilder, StringRecordsIntoIter};
use serde::de::DeserializeOwned;

use super::{tsv_raw::TSVRawOutput, Output};

pub struct TSVOutput<T> {
    names: Option<Vec<String>>,
    types: Option<Vec<String>>,
    phantom: PhantomData<T>,
}
impl<T> Default for TSVOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> TSVOutput<T> {
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
    pub(crate) fn from_raw_parts(names: Option<Vec<String>>, types: Option<Vec<String>>) -> Self {
        Self {
            names,
            types,
            phantom: PhantomData,
        }
    }
}

impl<T> Output for TSVOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = Option<HashMap<String, String>>;

    type Error = csv::Error;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error> {
        let rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(slice);

        self.deserialize_with_records(rdr.into_records())
    }
}
impl<T> TSVOutput<T>
where
    T: DeserializeOwned,
{
    pub(crate) fn deserialize_with_records(
        &self,
        records: StringRecordsIntoIter<&[u8]>,
    ) -> Result<(Vec<<Self as Output>::Row>, <Self as Output>::Info), <Self as Output>::Error> {
        // TODO, unescape
        TSVRawOutput::from_raw_parts(self.names.to_owned(), self.types.to_owned())
            .deserialize_with_records(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::output::test_helpers::TestStringsRow;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/TSV.tsv"))?;

        let (rows, info) = TSVOutput::<HashMap<String, String>>::with_names(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, None);

        let (rows, info) =
            TSVOutput::<TestStringsRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, "(1,'a')");
        assert_eq!(info, None);

        Ok(())
    }
}
