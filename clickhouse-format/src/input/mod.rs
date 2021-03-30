#[cfg(feature = "with-json")]
pub mod json_compact_each_row;

#[cfg(feature = "with-json")]
pub use self::json_compact_each_row::JSONCompactEachRowInput;

pub trait Input {
    type Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Error>;
}
