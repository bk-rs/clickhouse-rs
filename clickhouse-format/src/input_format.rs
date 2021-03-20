pub trait InputFormat {
    type Error;

    fn serialize(&self) -> Result<Vec<u8>, Self::Error>;
}
