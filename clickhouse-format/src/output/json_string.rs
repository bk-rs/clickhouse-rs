use std::{collections::HashMap, marker::PhantomData};

use serde::de::DeserializeOwned;

use super::{
    json::{JSONData, JSONDataInfo},
    Output,
};

pub struct JSONStringOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> JSONStringOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
pub type GeneralJSONStringOutput = JSONStringOutput<HashMap<String, String>>;

impl<T> Output for JSONStringOutput<T>
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use serde::Deserialize;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/JSONString.json"))?;

        let (rows, info) = GeneralJSONStringOutput::new().deserialize(&content.as_bytes()[..])?;
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
        let (rows, info) = JSONStringOutput::<Foo>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().range, "[0,1,2,3,4]");
        assert_eq!(info.rows, 3);

        Ok(())
    }
}
