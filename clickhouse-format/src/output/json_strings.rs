use std::{collections::HashMap, marker::PhantomData};

use serde::de::DeserializeOwned;

use super::{
    json::{JSONData, JSONDataInfo},
    Output,
};

pub struct JSONStringsOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> JSONStringsOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
pub type GeneralJSONStringsOutput = JSONStringsOutput<HashMap<String, String>>;

impl<T> Output for JSONStringsOutput<T>
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use super::super::json::TestStringsRow;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/JSONStrings.json"))?;

        let (rows, info) = GeneralJSONStringsOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info.rows, 1);

        let (rows, info) =
            JSONStringsOutput::<TestStringsRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, "(1,'a')");
        assert_eq!(info.rows, 1);

        Ok(())
    }
}
