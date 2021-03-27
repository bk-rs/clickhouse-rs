use std::collections::HashMap;

use super::json_compact_each_row::JSONCompactEachRowOutput;

pub type JSONCompactStringsEachRowOutput<T> = JSONCompactEachRowOutput<T>;

pub type GeneralJSONCompactStringsEachRowOutput =
    JSONCompactStringsEachRowOutput<HashMap<String, String>>;

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::{output::Output as _, test_helpers::TestStringsRow};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content =
            fs::read_to_string(PathBuf::new().join("tests/files/JSONCompactStringsEachRow.txt"))?;

        let (rows, info) = GeneralJSONCompactStringsEachRowOutput::new(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, ());

        let (rows, info) = JSONCompactStringsEachRowOutput::<TestStringsRow>::new(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, "(1,'a')");
        assert_eq!(info, ());

        Ok(())
    }
}
