pub trait OutputFormat {
    type T;
    type Error;

    fn deserialize(&self, slice: &[u8]) -> Result<Self::T, Self::Error>;
}
