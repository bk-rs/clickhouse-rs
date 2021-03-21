use std::{collections::HashMap, marker::PhantomData};

use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use super::{
    json::{JSONData, JSONDataInfo},
    Output,
};

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
    type Row = T;
    type Info = JSONDataInfo;

    type Error = serde_json::Error;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error> {
        let json_data_tmp: JSONData<Vec<String>> = serde_json::from_slice(slice)?;

        let keys: Vec<_> = json_data_tmp
            .meta
            .iter()
            .map(|x| x.name.to_owned())
            .collect();
        let mut data: Vec<T> = vec![];
        for values in json_data_tmp.data.into_iter() {
            let map: Map<_, _> = keys
                .iter()
                .zip(values)
                .map(|(k, v)| (k.to_owned(), Value::String(v)))
                .collect();
            data.push(serde_json::from_value(Value::Object(map))?);
        }

        Ok((
            data,
            JSONDataInfo {
                meta: json_data_tmp.meta,
                rows: json_data_tmp.rows,
                rows_before_limit_at_least: json_data_tmp.rows_before_limit_at_least,
            },
        ))
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

        let (rows, info) =
            GeneralJSONCompactStringOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(
            rows.first().unwrap().get("range(5)").unwrap(),
            "[0,1,2,3,4]"
        );
        assert_eq!(info.rows, 3);

        #[derive(Deserialize, Debug, Clone)]
        struct Foo {
            #[serde(rename = "'hello'")]
            hello: String,
            #[serde(rename = "multiply(42, number)")]
            multiply: String,
            #[serde(rename = "range(5)")]
            range: String,
        }
        let (rows, info) =
            JSONCompactStringOutput::<Foo>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().range, "[0,1,2,3,4]");
        assert_eq!(info.rows, 3);

        Ok(())
    }
}
