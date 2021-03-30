use serde::Serialize;
use serde_json::{ser::CompactFormatter, Serializer};

use crate::input::Input;

pub struct JsonCompactEachRowInput<T> {
    rows: Vec<Vec<T>>,
}
impl<T> JsonCompactEachRowInput<T> {
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        Self { rows }
    }
}

impl<T> Input for JsonCompactEachRowInput<T>
where
    T: Serialize,
{
    type Error = serde_json::Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Error> {
        let mut buf = vec![];
        let mut ser_buf = Vec::with_capacity(128);

        for row in &self.rows {
            buf.push(b'[');
            for (i, item) in row.iter().enumerate() {
                ser_buf.clear();
                let mut ser = Serializer::with_formatter(&mut ser_buf, CompactFormatter);
                item.serialize(&mut ser)?;

                buf.extend_from_slice(&ser.into_inner());

                if i < (row.len() - 1) {
                    buf.extend_from_slice(b", ");
                }
            }
            buf.push(b']');

            buf.push(b'\n');
        }

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::test_helpers::{TEST_ROW_1, TEST_ROW_2};

    use serde_json::{Map, Value};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content =
            fs::read_to_string(PathBuf::new().join("tests/files/JSONCompactEachRow.txt"))?;

        let mut rows: Vec<Vec<Value>> = vec![];
        rows.push(vec![
            TEST_ROW_1.array1.to_owned().into(),
            TEST_ROW_1.array2.to_owned().into(),
            vec![
                Value::Number(TEST_ROW_1.tuple1.to_owned().0.into()),
                Value::String(TEST_ROW_1.tuple1.to_owned().1),
            ]
            .into(),
            vec![
                Value::Number(TEST_ROW_1.tuple2.to_owned().0.into()),
                Value::Null,
            ]
            .into(),
            Value::Object(TEST_ROW_1.map1.to_owned().into_iter().fold(
                Map::new(),
                |mut m, (k, v)| {
                    m.insert(k, Value::String(v));
                    m
                },
            )),
        ]);
        rows.push(vec![
            TEST_ROW_2.array1.to_owned().into(),
            TEST_ROW_2.array2.to_owned().into(),
            vec![
                Value::Number(TEST_ROW_2.tuple1.to_owned().0.into()),
                Value::String(TEST_ROW_2.tuple1.to_owned().1),
            ]
            .into(),
            vec![
                Value::Number(TEST_ROW_2.tuple2.to_owned().0.into()),
                Value::String(TEST_ROW_2.tuple2.to_owned().1.unwrap()),
            ]
            .into(),
            Value::Object(TEST_ROW_2.map1.to_owned().into_iter().fold(
                Map::new(),
                |mut m, (k, v)| {
                    m.insert(k, Value::String(v));
                    m
                },
            )),
        ]);

        let bytes = JsonCompactEachRowInput::new(rows).serialize()?;
        assert_eq!(bytes, content.as_bytes());

        Ok(())
    }
}
