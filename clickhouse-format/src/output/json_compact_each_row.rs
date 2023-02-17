use std::{
    collections::HashMap,
    io::{self, BufRead as _},
    marker::PhantomData,
};

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::format_name::FormatName;

use super::{Output, OutputResult};

pub struct JsonCompactEachRowOutput<T> {
    names: Vec<String>,
    phantom: PhantomData<T>,
}
impl<T> JsonCompactEachRowOutput<T> {
    pub fn new(names: Vec<String>) -> Self {
        Self {
            names,
            phantom: PhantomData,
        }
    }
}

pub type GeneralJsonCompactEachRowOutput = JsonCompactEachRowOutput<HashMap<String, Value>>;

#[derive(thiserror::Error, Debug)]
pub enum JsonCompactEachRowOutputError {
    #[error("IoError {0:?}")]
    IoError(#[from] io::Error),
    #[error("SerdeJsonError {0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl<T> Output for JsonCompactEachRowOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = ();

    type Error = JsonCompactEachRowOutputError;

    fn format_name() -> FormatName {
        FormatName::JsonCompactEachRow
    }

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        let mut data: Vec<T> = vec![];

        for line in slice.lines() {
            let line = line?;
            let values: Vec<Value> = serde_json::from_str(&line)?;

            let row: T = serde_json::from_value(Value::Object(
                self.names.iter().cloned().zip(values).collect(),
            ))?;

            data.push(row);
        }

        Ok((data, ()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::{TestRow, TEST_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let file_path = PathBuf::new().join("tests/files/JSONCompactEachRow.txt");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            GeneralJsonCompactEachRowOutput::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, _info): (_, ()) = GeneralJsonCompactEachRowOutput::new(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(content.as_bytes())?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );

        let (rows, _info): (_, ()) = JsonCompactEachRowOutput::<TestRow>::new(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap(), &*TEST_ROW_1);

        Ok(())
    }
}
