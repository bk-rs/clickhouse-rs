use core::marker::PhantomData;
use std::{
    collections::HashMap,
    io::{BufRead as _, Error as IoError},
};

use serde::{Deserialize, de::DeserializeOwned};
use serde_aux::field_attributes::deserialize_option_number_from_string;
use serde_json::Value;

use crate::format_name::FormatName;

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
    IoError(#[from] IoError),
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

    fn format_name() -> FormatName {
        FormatName::JsonEachRowWithProgress
    }

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        let mut data: Vec<T> = vec![];
        let mut info = Option::<JsonEachRowProgress>::None;

        for line in slice.lines() {
            let line = line?;

            if info.is_some() {
                return Err(JsonEachRowWithProgressOutputError::ProgressInTheWrongPosition);
            }

            let line = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&line)?;

            if let Some(_meta) = line.get("meta") {
                continue;
            } else if let Some(row) = line.get("row") {
                let row: T = serde_json::from_value(row.clone())?;
                data.push(row);
                continue;
            } else if let Some(progress) = line.get("progress") {
                let progress: JsonEachRowProgress = serde_json::from_value(progress.clone())?;
                info = Some(progress);
                break;
            } else {
                continue;
            }
        }

        let info = info.ok_or(JsonEachRowWithProgressOutputError::ProgressMissing)?;

        Ok((data, info))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonEachRowProgress {
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub read_rows: Option<usize>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub read_bytes: Option<usize>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub total_rows_to_read: Option<usize>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub written_rows: Option<usize>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub written_bytes: Option<usize>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub elapsed_ns: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::PathBuf};

    use crate::test_helpers::{TEST_ROW_1, TestRow};

    #[test]
    fn simple() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = PathBuf::new().join("tests/files/JSONEachRowWithProgress.txt");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            GeneralJsonEachRowWithProgressOutput::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) =
            GeneralJsonEachRowWithProgressOutput::new().deserialize(content.as_bytes())?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info.read_rows.unwrap(), 2);

        let (rows, info) =
            JsonEachRowWithProgressOutput::<TestRow>::new().deserialize(content.as_bytes())?;
        assert_eq!(rows.first().unwrap(), &*TEST_ROW_1);
        assert_eq!(info.read_rows.unwrap(), 2);

        Ok(())
    }
}
