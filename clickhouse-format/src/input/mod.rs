#[cfg(feature = "with-json")]
pub mod json_compact_each_row;

pub trait Input {
    type Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Error>;
}
