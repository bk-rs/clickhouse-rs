use std::{
    collections::HashMap,
    io::{self, BufRead as _},
    marker::PhantomData,
};

use serde::de::DeserializeOwned;
use serde_json::Value;

use super::Output;

pub struct JSONCompactEachRowOutput<T> {
    names: Vec<String>,
    phantom: PhantomData<T>,
}
impl<T> JSONCompactEachRowOutput<T> {
    pub fn new(names: Vec<String>) -> Self {
        Self {
            names,
            phantom: PhantomData,
        }
    }
}

pub type GeneralJSONCompactEachRowOutput = JSONCompactEachRowOutput<HashMap<String, Value>>;

#[derive(thiserror::Error, Debug)]
pub enum JSONCompactEachRowOutputError {
    #[error("IoError {0:?}")]
    IoError(#[from] io::Error),
    #[error("SerdeJsonError {0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl<T> Output for JSONCompactEachRowOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = ();

    type Error = JSONCompactEachRowOutputError;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error> {
        let mut data: Vec<T> = vec![];

        for line in slice.lines() {
            let line = line?;
            let values: Vec<Value> = serde_json::from_str(&line)?;

            let row: T = serde_json::from_value(Value::Object(
                self.names.to_owned().into_iter().zip(values).collect(),
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

    use crate::test_helpers::TestRow;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content =
            fs::read_to_string(PathBuf::new().join("tests/files/JSONCompactEachRow.txt"))?;

        let (rows, info) = GeneralJSONCompactEachRowOutput::new(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(&content.as_bytes()[..])?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info, ());

        let (rows, info) = JSONCompactEachRowOutput::<TestRow>::new(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, (1_usize, "a".to_string()));
        assert_eq!(info, ());

        Ok(())
    }
}
