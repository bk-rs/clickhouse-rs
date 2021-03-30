use std::{
    collections::HashMap,
    io::{self, BufRead as _},
    marker::PhantomData,
};

use serde::{de::DeserializeOwned, Deserialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_json::Value;

use super::{Output, OutputResult};

pub struct JsonEachRowWithProgressOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for JsonEachRowWithProgressOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> JsonEachRowWithProgressOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

pub type GeneralJsonEachRowWithProgressOutput =
    JsonEachRowWithProgressOutput<HashMap<String, Value>>;

#[derive(thiserror::Error, Debug)]
pub enum JsonEachRowWithProgressOutputError {
    #[error("IoError {0:?}")]
    IoError(#[from] io::Error),
    #[error("SerdeJsonError {0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("ProgressInTheWrongPosition")]
    ProgressInTheWrongPosition,
    #[error("ProgressMissing")]
    ProgressMissing,
}

impl<T> Output for JsonEachRowWithProgressOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = JsonEachRowProgress;

    type Error = JsonEachRowWithProgressOutputError;

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        let mut data: Vec<T> = vec![];
        let mut info = Option::<JsonEachRowProgress>::None;

        for line in slice.lines() {
            let line = line?;

            if info.is_some() {
                return Err(JsonEachRowWithProgressOutputError::ProgressInTheWrongPosition);
            }

            match serde_json::from_str::<JsonEachRowLine<T>>(&line)? {
                JsonEachRowLine::Row { row } => {
                    data.push(row);
                    continue;
                }
                JsonEachRowLine::Progress { progress } => {
                    info = Some(progress);
                    break;
                }
            }
        }

        let info = info.ok_or(JsonEachRowWithProgressOutputError::ProgressMissing)?;

        Ok((data, info))
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum JsonEachRowLine<T>
where
    T: Sized,
{
    Row { row: T },
    Progress { progress: JsonEachRowProgress },
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonEachRowProgress {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub read_rows: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub read_bytes: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub written_rows: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub written_bytes: usize,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_rows_to_read: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::{TestRow, TEST_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content =
            fs::read_to_string(PathBuf::new().join("tests/files/JSONEachRowWithProgress.txt"))?;

        let (rows, info) =
            GeneralJsonEachRowWithProgressOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info.read_rows, 2);

        let (rows, info) =
            JsonEachRowWithProgressOutput::<TestRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_ROW_1);
        assert_eq!(info.read_rows, 2);

        Ok(())
    }
}
