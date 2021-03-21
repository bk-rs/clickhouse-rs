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
            rows_before_limit_at_least,
        } = json_data;
        Ok((
            data,
            JSONDataInfo {
                meta,
                rows,
                rows_before_limit_at_least,
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
    pub rows_before_limit_at_least: usize,
}
#[derive(Deserialize, Debug, Clone)]
pub struct JSONDataMetaItem {
    pub name: String,
    pub r#type: String,
}
pub struct JSONDataInfo {
    pub meta: Vec<JSONDataMetaItem>,
    pub rows: usize,
    pub rows_before_limit_at_least: usize,
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
            rows.first().unwrap().get("range(5)").unwrap(),
            &Value::Array(vec![0.into(), 1.into(), 2.into(), 3.into(), 4.into()])
        );
        assert_eq!(info.rows, 3);

        #[derive(Deserialize, Debug, Clone)]
        struct Foo {
            #[serde(rename = "'hello'")]
            hello: String,
            #[serde(rename = "multiply(42, number)")]
            multiply: String,
            #[serde(rename = "range(5)")]
            range: Vec<usize>,
        }
        let (rows, info) = JSONOutput::<Foo>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().range, vec![0, 1, 2, 3, 4]);
        assert_eq!(info.rows, 3);

        Ok(())
    }
}
