#[derive(strum::Display, strum::EnumString, PartialEq, Eq, Debug, Clone)]
pub enum FormatName {
    //
    #[strum(serialize = "JSON")]
    Json,
    #[strum(serialize = "JSONStrings")]
    JsonStrings,
    #[strum(serialize = "JSONCompact")]
    JsonCompact,
    #[strum(serialize = "JSONCompactStrings")]
    JsonCompactStrings,
    //
    #[strum(serialize = "TSV")]
    Tsv,
    #[strum(serialize = "TSVRaw")]
    TsvRaw,
    #[strum(serialize = "TSVWithNames")]
    TsvWithNames,
    #[strum(serialize = "TSVWithNamesAndTypes")]
    TsvWithNamesAndTypes,
    //
    #[strum(serialize = "JSONEachRow")]
    JsonEachRow,
    #[strum(serialize = "JSONStringsEachRow")]
    JsonStringsEachRow,
    #[strum(serialize = "JSONCompactEachRow")]
    JsonCompactEachRow,
    #[strum(serialize = "JSONCompactStringsEachRow")]
    JsonCompactStringsEachRow,
    #[strum(serialize = "JSONEachRowWithProgress")]
    JsonEachRowWithProgress,
    #[strum(serialize = "JSONStringsEachRowWithProgress")]
    JsonStringsEachRowWithProgress,
    #[strum(serialize = "JSONCompactEachRowWithNamesAndTypes")]
    JsonCompactEachRowWithNamesAndTypes,
    #[strum(serialize = "JSONCompactStringsEachRowWithNamesAndTypes")]
    JsonCompactStringsEachRowWithNamesAndTypes,
    #[strum(serialize = "JSONCompactEachRowWithNames")]
    JsonCompactEachRowWithNames,
    #[strum(serialize = "JSONCompactStringsEachRowWithNames")]
    JsonCompactStringsEachRowWithNames,
}
