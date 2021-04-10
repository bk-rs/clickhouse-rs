use std::collections::HashMap;

use super::json_each_row::JsonEachRowOutput;

pub type JsonStringsEachRowOutput<T> = JsonEachRowOutput<T>;

pub type GeneralJsonStringsEachRowOutput = JsonStringsEachRowOutput<HashMap<String, String>>;

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
        let file_path = PathBuf::new().join("tests/files/JSONStringsEachRow.txt");
        let content = fs::read_to_string(&file_path)?;

        // assert_eq!(
        //     GeneralJsonStringsEachRowOutput::format_name(),
        //     file_path
        //         .file_stem()
        //         .unwrap()
        //         .to_string_lossy()
        //         .parse()
        //         .unwrap()
        // );

        let (rows, info) =
            GeneralJsonStringsEachRowOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, ());

        let (rows, info) = JsonStringsEachRowOutput::<TestStringsRow>::new()
            .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info, ());

        Ok(())
    }
}
