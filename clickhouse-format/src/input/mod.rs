#[cfg(feature = "with-tsv")]
pub mod tsv;

pub trait Input {
    type Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Error>;
}
