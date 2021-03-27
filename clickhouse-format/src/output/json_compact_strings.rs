use std::{collections::HashMap, marker::PhantomData};

use serde::de::DeserializeOwned;

use super::{json::JSONDataInfo, json_compact::JSONCompactOutput, Output};

pub struct JSONCompactStringsOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for JSONCompactStringsOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> JSONCompactStringsOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
pub type GeneralJSONCompactStringsOutput = JSONCompactStringsOutput<HashMap<String, String>>;

impl<T> Output for JSONCompactStringsOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = JSONDataInfo;

    type Error = serde_json::Error;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error> {
        JSONCompactOutput::new().deserialize_with::<String>(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::output::test_helpers::TestStringsRow;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content =
            fs::read_to_string(PathBuf::new().join("tests/files/JSONCompactStrings.json"))?;

        let (rows, info) =
            GeneralJSONCompactStringsOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info.rows, 1);

        let (rows, info) = JSONCompactStringsOutput::<TestStringsRow>::new()
            .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, "(1,'a')");
        assert_eq!(info.rows, 1);

        Ok(())
    }
}
