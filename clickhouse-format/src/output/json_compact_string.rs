use std::{collections::HashMap, marker::PhantomData};

use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use super::{json::BaseData, Output};

pub struct JSONCompactStringOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> JSONCompactStringOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
pub type GeneralJSONCompactStringOutput = JSONCompactStringOutput<HashMap<String, String>>;

impl<T> Output for JSONCompactStringOutput<T>
where
    T: DeserializeOwned,
{
    type Value = BaseData<T>;

    type Error = serde_json::Error;

    fn deserialize(&self, slice: &[u8]) -> Result<Self::Value, Self::Error> {
        let data_tmp: BaseData<Vec<String>> = serde_json::from_slice(slice)?;

        let keys: Vec<_> = data_tmp.meta.iter().map(|x| x.name.to_owned()).collect();
        let mut data: Vec<T> = vec![];
        for values in data_tmp.data.into_iter() {
            let map: Map<_, _> = keys
                .iter()
                .zip(values)
                .map(|(k, v)| (k.to_owned(), Value::String(v)))
                .collect();
            data.push(serde_json::from_value(Value::Object(map))?);
        }

        Ok(BaseData {
            meta: data_tmp.meta,
            data: data,
            rows: data_tmp.rows,
            rows_before_limit_at_least: data_tmp.rows_before_limit_at_least,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use serde::Deserialize;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content =
            fs::read_to_string(PathBuf::new().join("tests/files/JSONCompactString.json"))?;

        let data = GeneralJSONCompactStringOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(data.data.first().unwrap().get("'hello'").unwrap(), "hello");

        #[derive(Deserialize, Debug, Clone)]
        struct Foo {
            #[serde(rename = "'hello'")]
            hello: String,
            #[serde(rename = "multiply(42, number)")]
            multiply: String,
            #[serde(rename = "range(5)")]
            range: String,
        }
        let data = JSONCompactStringOutput::<Foo>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(data.data.first().unwrap().hello, "hello");

        Ok(())
    }
}
