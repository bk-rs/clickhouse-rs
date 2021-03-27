use std::collections::HashMap;

use super::json_each_row::JSONEachRowOutput;

pub type JSONStringsEachRowOutput<T> = JSONEachRowOutput<T>;

pub type GeneralJSONStringsEachRowOutput = JSONStringsEachRowOutput<HashMap<String, String>>;

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::{output::Output as _, test_helpers::TestStringsRow};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content =
            fs::read_to_string(PathBuf::new().join("tests/files/JSONStringsEachRow.txt"))?;

        let (rows, info) =
            GeneralJSONStringsEachRowOutput::new().deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().get("tuple1").unwrap(), "(1,'a')");
        assert_eq!(info, ());

        let (rows, info) = JSONStringsEachRowOutput::<TestStringsRow>::new()
            .deserialize(&content.as_bytes()[..])?;
        assert_eq!(rows.first().unwrap().tuple1, "(1,'a')");
        assert_eq!(info, ());

        Ok(())
    }
}
