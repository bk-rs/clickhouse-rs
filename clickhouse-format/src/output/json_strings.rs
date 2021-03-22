use std::collections::HashMap;

use super::json::JSONOutput;

pub type JSONStringsOutput<T> = JSONOutput<T>;
pub type GeneralJSONStringsOutput = JSONStringsOutput<HashMap<String, String>>;

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    use crate::output::{test_helpers::TestStringsRow, Output as _};

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
