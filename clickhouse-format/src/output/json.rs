use std::{collections::HashMap, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

use super::Output;

pub struct JsonOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for JsonOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> JsonOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
pub type GeneralJsonOutput = JsonOutput<HashMap<String, Value>>;

impl<T> Output for JsonOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = JsonDataInfo;

    type Error = serde_json::Error;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error> {
        let json_data: JsonData<Self::Row> = serde_json::from_slice(slice)?;
        let JsonData {
            meta,
            data,
            rows,
            statistics,
        } = json_data;
        Ok((
            data,
            JsonDataInfo {
                meta,
                rows,
                statistics,
            },
        ))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct JsonData<T>
where
    T: Sized,
{
    pub meta: Vec<JsonDataMetaItem>,
    pub data: Vec<T>,
    pub rows: usize,
    pub statistics: JsonDataStatistics,
}
#[derive(Deserialize, Debug, Clone)]
pub struct JsonDataMetaItem {
    pub name: String,
    pub r#type: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct JsonDataStatistics {
    pub elapsed: f64,
    pub rows_read: usize,
    pub bytes_read: usize,
}
pub struct JsonDataInfo {
    pub meta: Vec<JsonDataMetaItem>,
    pub rows: usize,
    pub statistics: JsonDataStatistics,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::{TestRow, TEST_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/JSON.json"))?;

        let (rows, info) = GeneralJsonOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info.rows, 2);

        let (rows, info) = JsonOutput::<TestRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_ROW_1);
        assert_eq!(info.rows, 2);

        Ok(())
    }
}
