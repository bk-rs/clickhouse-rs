use serde::Serialize;
use serde_json::{ser::CompactFormatter, Serializer};

use crate::input::Input;

pub struct JSONCompactEachRowInput<T> {
    rows: Vec<Vec<T>>,
}
impl<T> JSONCompactEachRowInput<T> {
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        Self { rows }
    }
}

impl<T> Input for JSONCompactEachRowInput<T>
where
    T: Serialize,
{
    type Error = serde_json::Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Error> {
        let mut buf = vec![];

        for row in &self.rows {
            for (i, item) in row.iter().enumerate() {
                let ser_buf = Vec::with_capacity(128);
                let mut ser = Serializer::with_formatter(ser_buf, CompactFormatter);
                item.serialize(&mut ser)?;

                buf.extend_from_slice(&ser.into_inner());

                if i < (row.len() - 1) {
                    buf.extend_from_slice(b", ");
                }
            }

            buf.push('\n' as u8);
        }

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::TEST_ROW;

    use serde_json::{Map, Value};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content =
            fs::read_to_string(PathBuf::new().join("tests/files/JSONCompactEachRow.txt"))?;

        let mut rows: Vec<Vec<Value>> = vec![];
        rows.push(vec![
            TEST_ROW.array1.to_owned().into(),
            TEST_ROW.array2.to_owned().into(),
            vec![
                Value::Number(TEST_ROW.tuple1.to_owned().0.into()),
                Value::String(TEST_ROW.tuple1.to_owned().1),
            ]
            .into(),
            vec![
                Value::Number(TEST_ROW.tuple2.to_owned().0.into()),
                Value::Null,
            ]
            .into(),
            Value::Object(TEST_ROW.map1.to_owned().into_iter().fold(
                Map::new(),
                |mut m, (k, v)| {
                    m.insert(k, Value::String(v));
                    m
                },
            )),
        ]);

        let bytes = JSONCompactEachRowInput::new(rows).serialize()?;
        assert_eq!(bytes, content.as_bytes());

        Ok(())
    }
}
