use crate::format_name::FormatName;

#[cfg(feature = "with-json")]
pub mod json_compact_each_row;

#[cfg(feature = "with-json")]
pub use self::json_compact_each_row::JsonCompactEachRowInput;

pub trait Input {
    type Error: std::error::Error;

    fn format_name() -> FormatName;
    fn serialize(&self) -> Result<Vec<u8>, Self::Error>;
}
