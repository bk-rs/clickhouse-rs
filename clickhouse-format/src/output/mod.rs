#[cfg(feature = "with-json")]
pub mod json;
#[cfg(feature = "with-json")]
pub mod json_compact;
#[cfg(feature = "with-json")]
pub mod json_compact_strings;
#[cfg(feature = "with-json")]
pub mod json_strings;
#[cfg(feature = "with-tsv")]
pub mod tsv;
#[cfg(feature = "with-tsv")]
pub mod tsv_with_names;
#[cfg(feature = "with-tsv")]
pub mod tsv_with_names_and_types;

#[cfg(feature = "with-json")]
pub use self::{
    json::{GeneralJSONOutput, JSONOutput},
    json_compact::{GeneralJSONCompactOutput, JSONCompactOutput},
    json_compact_strings::{GeneralJSONCompactStringsOutput, JSONCompactStringsOutput},
    json_strings::{GeneralJSONStringsOutput, JSONStringsOutput},
    tsv::TSVOutput,
    tsv_with_names::TSVWithNamesOutput,
    tsv_with_names_and_types::TSVWithNamesAndTypesOutput,
};
#[cfg(feature = "with-tsv")]
pub type TabSeparatedOutput<T> = self::tsv::TSVOutput<T>;
#[cfg(feature = "with-tsv")]
pub type TabSeparatedWithNamesOutput<T> = self::tsv_with_names::TSVWithNamesOutput<T>;
#[cfg(feature = "with-tsv")]
pub type TabSeparatedWithNamesAndTypesOutput<T> =
    self::tsv_with_names_and_types::TSVWithNamesAndTypesOutput<T>;

pub trait Output {
    type Row;
    type Info;
    type Error;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error>;
}

#[cfg(test)]
pub(crate) mod test_helpers {
    use std::collections::HashMap;

    use serde::Deserialize;

    #[cfg(test)]
    #[derive(Deserialize, Debug, Clone)]
    pub(crate) struct TestRow {
        pub(crate) array1: Vec<usize>,
        pub(crate) array2: Vec<String>,
        pub(crate) tuple1: (usize, String),
        pub(crate) tuple2: (usize, Option<String>),
        pub(crate) map1: HashMap<String, String>,
    }
    #[cfg(test)]
    #[derive(Deserialize, Debug, Clone)]
    pub(crate) struct TestStringsRow {
        pub(crate) array1: String,
        pub(crate) array2: String,
        pub(crate) tuple1: String,
        pub(crate) tuple2: String,
        pub(crate) map1: String,
    }
}
