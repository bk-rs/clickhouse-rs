use std::{collections::HashMap, marker::PhantomData};

use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use super::{
    json::{JSONData, JSONDataInfo},
    Output,
};

pub struct JSONCompactOutput<T> {
    phantom: PhantomData<T>,
}
impl<T> JSONCompactOutput<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}
pub type GeneralJSONCompactOutput = JSONCompactOutput<HashMap<String, Value>>;

impl<T> Output for JSONCompactOutput<T>
where
    T: DeserializeOwned,
{
    type Row = T;
    type Info = JSONDataInfo;

    type Error = serde_json::Error;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error> {
        self.deserialize_with::<Value>(slice)
    }
}

impl<T> JSONCompactOutput<T>
where
    T: DeserializeOwned,
{
    pub(crate) fn deserialize_with<V>(
        &self,
        slice: &[u8],
    ) -> Result<(Vec<<Self as Output>::Row>, <Self as Output>::Info), <Self as Output>::Error>
    where
        V: DeserializeOwned + Into<Value>,
    {
        let json_data_tmp: JSONData<Vec<V>> = serde_json::from_slice(slice)?;

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
            JSONDataInfo {
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

    use crate::output::test_helpers::TestRow;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/JSONCompact.json"))?;

        let (rows, info) = GeneralJSONCompactOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(
            rows.first().unwrap().get("tuple1").unwrap(),
            &Value::Array(vec![1.into(), "a".into()])
        );
        assert_eq!(info.rows, 1);

        let (rows, info) =
            JSONCompactOutput::<TestRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, (1_usize, "a".to_string()));
        assert_eq!(info.rows, 1);

        Ok(())
    }
}
