use std::{collections::HashMap, marker::PhantomData};

use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::format_name::FormatName;

use super::{
    json::{JsonData, JsonDataInfo},
    Output, OutputResult,
};

pub struct JsonCompactOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> Default for JsonCompactOutput<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> JsonCompactOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
pub type GeneralJsonCompactOutput = JsonCompactOutput<HashMap<String, Value>>;

impl<T> Output for JsonCompactOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = JsonDataInfo;

    type Error = serde_json::Error;

    fn format_name() -> FormatName {
        FormatName::JsonCompact
    }

    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error> {
        self.deserialize_with::<Value>(slice)
    }
}

impl<T> JsonCompactOutput<T>
where
    T: DeserializeOwned,
{
    pub(crate) fn deserialize_with<V>(
        &self,
        slice: &[u8],
    ) -> OutputResult<<Self as Output>::Row, <Self as Output>::Info, <Self as Output>::Error>
    where
        V: DeserializeOwned + Into<Value>,
    {
        let json_data_tmp: JsonData<Vec<V>> = serde_json::from_slice(slice)?;

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
                .map(|(k, v)| (k.to_owned(), v.into()))
                .collect();
            data.push(serde_json::from_value(Value::Object(map))?);
        }

        Ok((
            data,
            JsonDataInfo {
                meta: json_data_tmp.meta,
                rows: json_data_tmp.rows,
                statistics: json_data_tmp.statistics,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::{TestRow, TEST_ROW_1};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let file_path = PathBuf::new().join("tests/files/JSONCompact.json");
        let content = fs::read_to_string(&file_path)?;

        assert_eq!(
            GeneralJsonCompactOutput::format_name(),
            file_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .parse()
                .unwrap()
        );

        let (rows, info) = GeneralJsonCompactOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info.rows, 2);

        let (rows, info) =
            JsonCompactOutput::<TestRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_ROW_1);
        assert_eq!(info.rows, 2);

        Ok(())
    }
}
