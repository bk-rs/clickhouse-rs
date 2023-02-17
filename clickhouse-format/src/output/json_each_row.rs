use core::marker::PhantomData;
use std::{
    collections::HashMap,
    io::{BufRead as _, Error as IoError},
};

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::format_name::FormatName;

use super::{Output, OutputResult};

pub struct JsonEachRowOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for JsonEachRowOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> JsonEachRowOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

pub type GeneralJsonEachRowOutput = JsonEachRowOutput<HashMap<String, Value>>;

#[derive(thiserror::Error, Debug)]
pub enum JsonEachRowOutputError {
    #[error("IoError {0:?}")]
    IoError(#[from] IoError),
    #[error("SerdeJsonError {0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl<T> Output for JsonEachRowOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = ();

    type Error = JsonEachRowOutputError;

    fn format_name() -> FormatName {
        FormatName::JsonEachRow
    }

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        let mut data: Vec<T> = vec![];
        for line in slice.lines() {
            let line = line?;
            let row: T = serde_json::from_str(&line)?;
            data.push(row);
        }

        Ok((data, ()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::PathBuf};

    use crate::test_helpers::{TestRow, TEST_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = PathBuf::new().join("tests/files/JSONEachRow.txt");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            GeneralJsonEachRowOutput::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, _info): (_, ()) =
            GeneralJsonEachRowOutput::new().deserialize(content.as_bytes())?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );

        let (rows, _info): (_, ()) =
            JsonEachRowOutput::<TestRow>::new().deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap(), &*TEST_ROW_1);

        Ok(())
    }
}
