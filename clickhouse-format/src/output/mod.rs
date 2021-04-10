use crate::format_name::FormatName;

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
    tsv::TsvOutput, tsv_raw::TsvRawOutput, tsv_with_names::TsvWithNamesOutput,
    tsv_with_names_and_types::TsvWithNamesAndTypesOutput,
};

#[cfg(feature = "with-tsv")]
pub type TabSeparatedOutput<T> = self::tsv::TsvOutput<T>;
#[cfg(feature = "with-tsv")]
pub type TabSeparatedRawOutput<T> = self::tsv_raw::TsvRawOutput<T>;
#[cfg(feature = "with-tsv")]
pub type TabSeparatedWithNamesOutput<T> = self::tsv_with_names::TsvWithNamesOutput<T>;
#[cfg(feature = "with-tsv")]
pub type TabSeparatedWithNamesAndTypesOutput<T> =
    self::tsv_with_names_and_types::TsvWithNamesAndTypesOutput<T>;

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
    json::{GeneralJsonOutput, JsonOutput},
    json_compact::{GeneralJsonCompactOutput, JsonCompactOutput},
    json_compact_strings::{GeneralJsonCompactStringsOutput, JsonCompactStringsOutput},
    json_strings::{GeneralJsonStringsOutput, JsonStringsOutput},
};

//
#[cfg(feature = "with-json")]
pub mod json_compact_each_row;
#[cfg(feature = "with-json")]
pub mod json_compact_each_row_with_names_and_types;
#[cfg(feature = "with-json")]
pub mod json_compact_strings_each_row;
#[cfg(feature = "with-json")]
pub mod json_compact_strings_each_row_with_names_and_types;
#[cfg(feature = "with-json")]
pub mod json_each_row;
#[cfg(feature = "with-json")]
pub mod json_each_row_with_progress;
#[cfg(feature = "with-json")]
pub mod json_strings_each_row;
#[cfg(feature = "with-json")]
pub mod json_strings_each_row_with_progress;

#[cfg(feature = "with-json")]
pub use self::{
    json_compact_each_row::{GeneralJsonCompactEachRowOutput, JsonCompactEachRowOutput},
    json_compact_each_row_with_names_and_types::{
        GeneralJsonCompactEachRowWithNamesAndTypesOutput, JsonCompactEachRowWithNamesAndTypesOutput,
    },
    json_compact_strings_each_row::{
        GeneralJsonCompactStringsEachRowOutput, JsonCompactStringsEachRowOutput,
    },
    json_compact_strings_each_row_with_names_and_types::{
        GeneralJsonCompactStringsEachRowWithNamesAndTypesOutput,
        JsonCompactStringsEachRowWithNamesAndTypesOutput,
    },
    json_each_row::{GeneralJsonEachRowOutput, JsonEachRowOutput},
    json_each_row_with_progress::{
        GeneralJsonEachRowWithProgressOutput, JsonEachRowWithProgressOutput,
    },
    json_strings_each_row::{GeneralJsonStringsEachRowOutput, JsonStringsEachRowOutput},
    json_strings_each_row_with_progress::{
        GeneralJsonStringsEachRowWithProgressOutput, JsonStringsEachRowWithProgressOutput,
    },
};

pub trait Output {
    type Row;
    type Info;
    type Error;

    fn format_name() -> FormatName;
    fn deserialize(&self, slice: &[u8]) -> OutputResult<Self::Row, Self::Info, Self::Error>;
}
pub type OutputResult<Row, Info, Error> = Result<(Vec<Row>, Info), Error>;
