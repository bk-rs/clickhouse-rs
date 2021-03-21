use std::{collections::HashMap, marker::PhantomData};

use serde::de::DeserializeOwned;

use super::{json::BaseData, Output};

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

impl<T> Output for JSONStringOutput<T>
where
    T: DeserializeOwned,
{
    type Value = Data<T>;

    type Error = serde_json::Error;

    fn deserialize(&self, slice: &[u8]) -> Result<Self::Value, Self::Error> {
        serde_json::from_slice(slice)
    }
}

pub type Data<T> = BaseData<T>;
pub type GeneralData = Data<HashMap<String, String>>;

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use serde::Deserialize;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/JSONString.json"))?;

        let data: GeneralData = JSONStringOutput::new().deserialize(&content.as_bytes()[..])?;
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
        let data: Data<Foo> = JSONStringOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(data.data.first().unwrap().hello, "hello");

        Ok(())
    }
}
