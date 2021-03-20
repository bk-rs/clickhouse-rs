#[cfg(feature = "with-csv")]
pub mod csv_formats;
#[cfg(feature = "with-json")]
pub mod json_formats;
#[cfg(feature = "with-tsv")]
pub mod tsv_formats;

#[cfg(feature = "with-tsv")]
pub mod tab_separated_formats {
    pub type TabSeparated = crate::tsv_formats::TSV;
    pub type TabSeparatedRaw = crate::tsv_formats::TSVRaw;
}

pub mod input_format;
pub mod output_format;
