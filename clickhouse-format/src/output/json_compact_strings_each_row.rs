use std::collections::HashMap;

use super::json_compact_each_row::JsonCompactEachRowOutput;

pub type JsonCompactStringsEachRowOutput<T> = JsonCompactEachRowOutput<T>;

pub type GeneralJsonCompactStringsEachRowOutput =
    JsonCompactStringsEachRowOutput<HashMap<String, String>>;

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::{
        output::Output as _,
        test_helpers::{TestStringsRow, TEST_STRINGS_ROW_1},
    };

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let file_path = PathBuf::new().join("tests/files/JSONCompactStringsEachRow.txt");
        let content = fs::read_to_string(&file_path)?;

        // assert_eq!(
        //     GeneralJsonCompactStringsEachRowOutput::format_name(),
        //     file_path
        //         .file_stem()
        //         .unwrap()
        //         .to_string_lossy()
        //         .parse()
        //         .unwrap()
        // );

        let (rows, info) = GeneralJsonCompactStringsEachRowOutput::new(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, ());

        let (rows, info) = JsonCompactStringsEachRowOutput::<TestStringsRow>::new(vec![
            "array1".into(),
            "array2".into(),
            "tuple1".into(),
            "tuple2".into(),
            "map1".into(),
        ])
        .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info, ());

        Ok(())
    }
}
