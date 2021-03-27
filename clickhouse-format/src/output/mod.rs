//
#[cfg(feature = "with-tsv")]
pub mod tsv;
#[cfg(feature = "with-tsv")]
pub mod tsv_raw;
#[cfg(feature = "with-tsv")]
pub mod tsv_with_names;
#[cfg(feature = "with-tsv")]
pub mod tsv_with_names_and_types;

#[cfg(feature = "with-tsv")]
pub use self::{
    tsv::TSVOutput, tsv_raw::TSVRawOutput, tsv_with_names::TSVWithNamesOutput,
    tsv_with_names_and_types::TSVWithNamesAndTypesOutput,
};

#[cfg(feature = "with-tsv")]
pub type TabSeparatedOutput<T> = self::tsv::TSVOutput<T>;
#[cfg(feature = "with-tsv")]
pub type TabSeparatedRawOutput<T> = self::tsv_raw::TSVRawOutput<T>;
#[cfg(feature = "with-tsv")]
pub type TabSeparatedWithNamesOutput<T> = self::tsv_with_names::TSVWithNamesOutput<T>;
#[cfg(feature = "with-tsv")]
pub type TabSeparatedWithNamesAndTypesOutput<T> =
    self::tsv_with_names_and_types::TSVWithNamesAndTypesOutput<T>;

//
#[cfg(feature = "with-json")]
pub mod json;
#[cfg(feature = "with-json")]
pub mod json_compact;
#[cfg(feature = "with-json")]
pub mod json_compact_strings;
#[cfg(feature = "with-json")]
pub mod json_strings;

#[cfg(feature = "with-json")]
pub use self::{
    json::{GeneralJSONOutput, JSONOutput},
    json_compact::{GeneralJSONCompactOutput, JSONCompactOutput},
    json_compact_strings::{GeneralJSONCompactStringsOutput, JSONCompactStringsOutput},
    json_strings::{GeneralJSONStringsOutput, JSONStringsOutput},
};

//
#[cfg(feature = "with-json")]
pub mod json_each_row;
#[cfg(feature = "with-json")]
pub mod json_strings_each_row;

#[cfg(feature = "with-json")]
pub use self::{
    json_each_row::{GeneralJSONEachRowOutput, JSONEachRowOutput},
    json_strings_each_row::{GeneralJSONStringsEachRowOutput, JSONStringsEachRowOutput},
};

pub trait Output {
    type Row;
    type Info;
    type Error;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error>;
}

#[cfg(test)]
pub(crate) mod test_helpers;
