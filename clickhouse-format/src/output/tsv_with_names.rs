use std::{collections::HashMap, marker::PhantomData};

use csv::ReaderBuilder;
use serde::de::DeserializeOwned;

use super::{tsv::TSVOutput, Output};

pub struct TSVWithNamesOutput<T> {
    types: Option<Vec<String>>,
    phantom: PhantomData<T>,
}
impl<T> TSVWithNamesOutput<T> {
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

impl<T> Output for TSVWithNamesOutput<T>
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

        let records = rdr.into_records();

        if let Some(types) = &self.types {
            TSVOutput::with_names_and_types(names, types.to_owned())
                .deserialize_with_records(records)
        } else {
            TSVOutput::with_names(names).deserialize_with_records(records)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::output::test_helpers::TestStringsRow;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/TSVWithNames.tsv"))?;

        let (rows, info) = TSVWithNamesOutput::<HashMap<String, String>>::new()
            .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, None);

        let (rows, info) =
            TSVWithNamesOutput::<TestStringsRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, "(1,'a')");
        assert_eq!(info, None);

        Ok(())
    }
}
