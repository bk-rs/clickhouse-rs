use std::{collections::HashMap, marker::PhantomData};

use csv::{ReaderBuilder, StringRecord, StringRecordsIntoIter};
use serde::de::DeserializeOwned;

use super::Output;

pub struct TsvRawOutput<T> {
    names: Option<Vec<String>>,
    types: Option<Vec<String>>,
    phantom: PhantomData<T>,
}
impl<T> Default for TsvRawOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> TsvRawOutput<T> {
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

impl<T> Output for TsvRawOutput<T>
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
impl<T> TsvRawOutput<T>
where
    T: DeserializeOwned,
{
    pub(crate) fn deserialize_with_records(
        &self,
        records: StringRecordsIntoIter<&[u8]>,
    ) -> Result<(Vec<<Self as Output>::Row>, <Self as Output>::Info), <Self as Output>::Error> {
        let header = &self.names.to_owned().map(StringRecord::from);
        let mut data: Vec<T> = vec![];
        for record in records {
            let record = record?;
            let row: T = record.deserialize(header.as_ref())?;
            data.push(row);
        }

        let info = if let Some(types) = &self.types {
            self.names
                .to_owned()
                .map(|x| x.into_iter().zip(types.to_owned()).collect())
        } else {
            None
        };

        Ok((data, info))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::{TestStringsRow, TEST_STRINGS_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/TSVRaw.tsv"))?;

        let (rows, info) = TsvRawOutput::<HashMap<String, String>>::with_names(vec![
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
            TsvRawOutput::<TestStringsRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info, None);

        Ok(())
    }
}
