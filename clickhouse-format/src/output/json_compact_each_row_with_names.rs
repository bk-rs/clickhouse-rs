use core::marker::PhantomData;
use std::{
    collections::HashMap,
    io::{BufRead as _, Error as IoError},
};

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::format_name::FormatName;

use super::{Output, OutputResult};

pub struct JsonCompactEachRowWithNamesOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for JsonCompactEachRowWithNamesOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> JsonCompactEachRowWithNamesOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

pub type GeneralJsonCompactEachRowWithNamesOutput =
    JsonCompactEachRowWithNamesOutput<HashMap<String, Value>>;

#[derive(thiserror::Error, Debug)]
pub enum JsonCompactEachRowWithNamesOutputError {
    #[error("IoError {0:?}")]
    IoError(#[from] IoError),
    #[error("SerdeJsonError {0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl<T> Output for JsonCompactEachRowWithNamesOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = Vec<String>;

    type Error = JsonCompactEachRowWithNamesOutputError;

    fn format_name() -> FormatName {
        FormatName::JsonCompactEachRowWithNames
    }

    fn deserialize(&self, mut slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        let mut data: Vec<T> = vec![];

        let mut buf = String::new();

        slice.read_line(&mut buf)?;
        let names: Vec<String> = serde_json::from_str(&buf)?;
        buf.clear();

        for line in slice.lines() {
            let line = line?;
            let values: Vec<Value> = serde_json::from_str(&line)?;

            let row: T =
                serde_json::from_value(Value::Object(names.iter().cloned().zip(values).collect()))?;

            data.push(row);
        }

        Ok((data, names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::PathBuf};

    use crate::test_helpers::{TEST_ROW_1, TestRow};

    #[test]
    fn simple() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = PathBuf::new().join("tests/files/JSONCompactEachRowWithNames.txt");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            GeneralJsonCompactEachRowWithNamesOutput::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) =
            GeneralJsonCompactEachRowWithNamesOutput::new().deserialize(content.as_bytes())?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info, vec!["array1", "array2", "tuple1", "tuple2", "map1"]);

        let (rows, info) =
            JsonCompactEachRowWithNamesOutput::<TestRow>::new().deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap(), &*TEST_ROW_1);
        assert_eq!(info, vec!["array1", "array2", "tuple1", "tuple2", "map1"]);

        Ok(())
    }
}
