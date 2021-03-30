use std::{collections::HashMap, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

use super::Output;

pub struct JSONOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for JSONOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> JSONOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
pub type GeneralJSONOutput = JSONOutput<HashMap<String, Value>>;

impl<T> Output for JSONOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = JSONDataInfo;

    type Error = serde_json::Error;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error> {
        let json_data: JSONData<Self::Row> = serde_json::from_slice(slice)?;
        let JSONData {
            meta,
            data,
            rows,
            statistics,
        } = json_data;
        Ok((
            data,
            JSONDataInfo {
                meta,
                rows,
                statistics,
            },
        ))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct JSONData<T>
where
    T: Sized,
{
    pub meta: Vec<JSONDataMetaItem>,
    pub data: Vec<T>,
    pub rows: usize,
    pub statistics: JSONDataStatistics,
}
#[derive(Deserialize, Debug, Clone)]
pub struct JSONDataMetaItem {
    pub name: String,
    pub r#type: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct JSONDataStatistics {
    pub elapsed: f64,
    pub rows_read: usize,
    pub bytes_read: usize,
}
pub struct JSONDataInfo {
    pub meta: Vec<JSONDataMetaItem>,
    pub rows: usize,
    pub statistics: JSONDataStatistics,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::{TestRow, TEST_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/JSON.json"))?;

        let (rows, info) = GeneralJSONOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info.rows, 2);

        let (rows, info) = JSONOutput::<TestRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_ROW_1);
        assert_eq!(info.rows, 2);

        Ok(())
    }
}
