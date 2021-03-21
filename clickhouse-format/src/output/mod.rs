#[cfg(feature = "with-csv")]
pub mod csv;
#[cfg(feature = "with-json")]
pub mod json;
#[cfg(feature = "with-json")]
pub mod json_compact;
#[cfg(feature = "with-json")]
pub mod json_compact_string;
#[cfg(feature = "with-json")]
pub mod json_string;
#[cfg(feature = "with-tsv")]
pub mod tsv;

#[cfg(feature = "with-tsv")]
pub mod tab_separated {
    pub type TabSeparated = super::tsv::TSVOutput;
}

pub trait Output {
    type Value;
    type Error;

    fn deserialize(&self, slice: &[u8]) -> Result<Self::Value, Self::Error>;
}
