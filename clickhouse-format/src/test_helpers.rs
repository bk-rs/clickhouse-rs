use std::collections::HashMap;

use serde::Deserialize;

#[cfg(test)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TestRow {
    pub(crate) array1: Vec<usize>,
    pub(crate) array2: Vec<String>,
    pub(crate) tuple1: (usize, String),
    pub(crate) tuple2: (usize, Option<String>),
    pub(crate) map1: HashMap<String, String>,
}
#[cfg(test)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct TestStringsRow {
    pub(crate) array1: String,
    pub(crate) array2: String,
    pub(crate) tuple1: String,
    pub(crate) tuple2: String,
    pub(crate) map1: String,
}
