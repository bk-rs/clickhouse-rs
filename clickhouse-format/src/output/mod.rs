#[cfg(feature = "with-csv")]
pub mod csv;
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
pub mod tab_separated {
    pub type TabSeparated = super::tsv::TSVOutput;
}

pub trait Output {
    type Row;
    type Info;
    type Error;

    fn deserialize(&self, slice: &[u8]) -> Result<(Vec<Self::Row>, Self::Info), Self::Error>;
}
