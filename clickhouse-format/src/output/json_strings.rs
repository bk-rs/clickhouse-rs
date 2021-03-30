use std::collections::HashMap;

use super::json::JsonOutput;

pub type JsonStringsOutput<T> = JsonOutput<T>;
pub type GeneralJsonStringsOutput = JsonStringsOutput<HashMap<String, String>>;

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
        let content = fs::read_to_string(PathBuf::new().join("tests/files/JSONStrings.json"))?;

        let (rows, info) = GeneralJsonStringsOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info.rows, 2);

        let (rows, info) =
            JsonStringsOutput::<TestStringsRow>::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap(), &*TEST_STRINGS_ROW_1);
        assert_eq!(info.rows, 2);

        Ok(())
    }
}
