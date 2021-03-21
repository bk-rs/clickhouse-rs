use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{Map, Value};

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

impl<T> Output for JSONOutput<T>
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
pub type GeneralData = Data<Map<String, Value>>;

#[derive(Deserialize, Debug, Clone)]
pub struct BaseData<T>
where
    T: Sized,
{
    pub meta: Vec<MetaItem>,
    pub data: Vec<T>,
    pub rows: usize,
    pub rows_before_limit_at_least: usize,
}
#[derive(Deserialize, Debug, Clone)]
pub struct MetaItem {
    pub name: String,
    pub r#type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/JSON.json"))?;

        let data: GeneralData = JSONOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(data.data.first().unwrap().get("'hello'").unwrap(), "hello");

        #[derive(Deserialize, Debug, Clone)]
        struct Foo {
            #[serde(rename = "'hello'")]
            hello: String,
            #[serde(rename = "multiply(42, number)")]
            multiply: String,
            #[serde(rename = "range(5)")]
            range: Vec<usize>,
        }
        let data: Data<Foo> = JSONOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(data.data.first().unwrap().hello, "hello");

        Ok(())
    }
}
