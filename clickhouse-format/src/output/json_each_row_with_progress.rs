use std::{
    collections::HashMap,
    io::{self, BufRead as _},
    marker::PhantomData,
};

use serde::{de::DeserializeOwned, Deserialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_json::Value;

use super::Output;

pub struct JSONEachRowWithProgressOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for JSONEachRowWithProgressOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> JSONEachRowWithProgressOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

pub type GeneralJSONEachRowWithProgressOutput =
    JSONEachRowWithProgressOutput<HashMap<String, Value>>;

#[derive(thiserror::Error, Debug)]
pub enum JSONEachRowWithProgressOutputError {
    #[error("IoError {0:?}")]
    IoError(#[from] io::Error),
    #[error("SerdeJsonError {0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("ProgressInTheWrongPosition")]
    ProgressInTheWrongPosition,
    #[error("ProgressMissing")]
    ProgressMissing,
}

impl<T> Output for JSONEachRowWithProgressOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = JSONEachRowProgress;

    type Error = JSONEachRowWithProgressOutputError;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error> {
        let mut data: Vec<T> = vec![];
        let mut info = Option::<JSONEachRowProgress>::None;

        for line in slice.lines() {
            let line = line?;

            if info.is_some() {
                return Err(JSONEachRowWithProgressOutputError::ProgressInTheWrongPosition);
            }

            match serde_json::from_str::<JSONEachRowLine<T>>(&line)? {
                JSONEachRowLine::Row { row } => {
                    data.push(row);
                    continue;
                }
                JSONEachRowLine::Progress { progress } => {
                    info = Some(progress);
                    break;
                }
            }
        }

        let info = info.ok_or(JSONEachRowWithProgressOutputError::ProgressMissing)?;

        Ok((data, info))
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum JSONEachRowLine<T>
where
    T: Sized,
{
    Row { row: T },
    Progress { progress: JSONEachRowProgress },
}

#[derive(Deserialize, Debug, Clone)]
pub struct JSONEachRowProgress {
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

    use crate::output::test_helpers::TestRow;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content =
            fs::read_to_string(PathBuf::new().join("tests/files/JSONEachRowWithProgress.txt"))?;

        let (rows, info) =
            GeneralJSONEachRowWithProgressOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info.read_rows, 1);

        let (rows, info) =
            JSONEachRowWithProgressOutput::<TestRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, (1_usize, "a".to_string()));
        assert_eq!(info.read_rows, 1);

        Ok(())
    }
}
