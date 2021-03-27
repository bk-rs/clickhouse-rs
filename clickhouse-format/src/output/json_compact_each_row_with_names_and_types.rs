use std::{
    collections::HashMap,
    io::{self, BufRead as _},
    marker::PhantomData,
};

use serde::de::DeserializeOwned;
use serde_json::Value;

use super::Output;

pub struct JSONCompactEachRowWithNamesAndTypesOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for JSONCompactEachRowWithNamesAndTypesOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> JSONCompactEachRowWithNamesAndTypesOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

pub type GeneralJSONCompactEachRowWithNamesAndTypesOutput =
    JSONCompactEachRowWithNamesAndTypesOutput<HashMap<String, Value>>;

#[derive(thiserror::Error, Debug)]
pub enum JSONCompactEachRowWithNamesAndTypesOutputError {
    #[error("IoError {0:?}")]
    IoError(#[from] io::Error),
    #[error("SerdeJsonError {0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl<T> Output for JSONCompactEachRowWithNamesAndTypesOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = HashMap<String, String>;

    type Error = JSONCompactEachRowWithNamesAndTypesOutputError;

    fn deserialize(&self, mut slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error> {
        let mut data: Vec<T> = vec![];

        let mut buf = String::new();

        slice.read_line(&mut buf)?;
        let names: Vec<String> = serde_json::from_str(&buf)?;
        buf.clear();

        slice.read_line(&mut buf)?;
        let types: Vec<String> = serde_json::from_str(&buf)?;
        buf.clear();

        for line in slice.lines() {
            let line = line?;
            let values: Vec<Value> = serde_json::from_str(&line)?;

            let row: T = serde_json::from_value(Value::Object(
                names.to_owned().into_iter().zip(values).collect(),
            ))?;

            data.push(row);
        }

        Ok((data, names.into_iter().zip(types).collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::output::test_helpers::TestRow;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(
            PathBuf::new().join("tests/files/JSONCompactEachRowWithNamesAndTypes.txt"),
        )?;

        let (rows, info) = GeneralJSONCompactEachRowWithNamesAndTypesOutput::new()
            .deserialize(&content.as_bytes()[..])?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info.get("array1"), Some(&"Array(UInt8)".to_owned()));

        let (rows, info) = JSONCompactEachRowWithNamesAndTypesOutput::<TestRow>::new()
            .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, (1_usize, "a".to_string()));
        assert_eq!(info.get("array1"), Some(&"Array(UInt8)".to_owned()));

        Ok(())
    }
}
