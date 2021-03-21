use std::{collections::HashMap, marker::PhantomData};

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;

use super::Output;

pub struct JSONOutput<T> {
    phantom: PhantomData<T>,
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
#[derive(Deserialize, Debug, Clone)]
pub(super) struct TestRow {
    pub(super) array1: Vec<usize>,
    pub(super) array2: Vec<String>,
    pub(super) tuple1: (usize, String),
    pub(super) tuple2: (usize, Option<String>),
    pub(super) map1: HashMap<String, String>,
}
#[cfg(test)]
#[derive(Deserialize, Debug, Clone)]
pub(super) struct TestStringsRow {
    pub(super) array1: String,
    pub(super) array2: String,
    pub(super) tuple1: String,
    pub(super) tuple2: String,
    pub(super) map1: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/JSON.json"))?;

        let (rows, info) = GeneralJSONOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info.rows, 1);

        let (rows, info) = JSONOutput::<TestRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, (1_usize, "a".to_string()));
        assert_eq!(info.rows, 1);

        Ok(())
    }
}
