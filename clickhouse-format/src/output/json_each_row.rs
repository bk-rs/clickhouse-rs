use std::{
    collections::HashMap,
    io::{self, BufRead as _},
    marker::PhantomData,
};

use serde::de::DeserializeOwned;
use serde_json::Value;

use super::Output;

pub struct JSONEachRowOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for JSONEachRowOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> JSONEachRowOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

pub type GeneralJSONEachRowOutput = JSONEachRowOutput<HashMap<String, Value>>;

#[derive(thiserror::Error, Debug)]
pub enum JSONEachRowOutputError {
    #[error("IoError {0:?}")]
    IoError(#[from] io::Error),
    #[error("SerdeJsonError {0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl<T> Output for JSONEachRowOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = ();

    type Error = JSONEachRowOutputError;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error> {
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

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::{TestRow, TEST_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/JSONEachRow.txt"))?;

        let (rows, info) = GeneralJSONEachRowOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info, ());

        let (rows, info) =
            JSONEachRowOutput::<TestRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_ROW_1);
        assert_eq!(info, ());

        Ok(())
    }
}
