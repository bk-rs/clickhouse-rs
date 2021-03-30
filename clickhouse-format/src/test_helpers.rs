use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct TestRow {
    pub(crate) array1: Vec<usize>,
    pub(crate) array2: Vec<String>,
    pub(crate) tuple1: (usize, String),
    pub(crate) tuple2: (usize, Option<String>),
    pub(crate) map1: HashMap<String, String>,
}

#[allow(dead_code)]
pub(crate) const TEST_ROW_1: Lazy<TestRow> = Lazy::new(|| TestRow {
    array1: vec![1, 2],
    array2: vec!["a".into(), "b".into()],
    tuple1: (1, "a".into()),
    tuple2: (1, None),
    map1: vec![
        ("1".into(), "Ready".into()),
        ("2".into(), "Steady".into()),
        ("3".into(), "Go".into()),
    ]
    .into_iter()
    .collect(),
});
#[allow(dead_code)]
pub(crate) const TEST_ROW_2: Lazy<TestRow> = Lazy::new(|| TestRow {
    array1: vec![3, 4],
    array2: vec!["c".into(), "d".into()],
    tuple1: (2, "b".into()),
    tuple2: (2, Some("b".into())),
    map1: vec![].into_iter().collect(),
});

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub(crate) struct TestStringsRow {
    pub(crate) array1: String,
    pub(crate) array2: String,
    pub(crate) tuple1: String,
    pub(crate) tuple2: String,
    pub(crate) map1: String,
}

#[allow(dead_code)]
pub(crate) const TEST_STRINGS_ROW_1: Lazy<TestStringsRow> = Lazy::new(|| TestStringsRow {
    array1: "[1,2]".into(),
    array2: "['a','b']".into(),
    tuple1: "(1,'a')".into(),
    tuple2: "(1,NULL)".into(),
    map1: "{'1':'Ready','2':'Steady','3':'Go'}".into(),
});
#[allow(dead_code)]
pub(crate) const TEST_STRINGS_ROW_2: Lazy<TestStringsRow> = Lazy::new(|| TestStringsRow {
    array1: "[3,4]".into(),
    array2: "['c','d']".into(),
    tuple1: "(2,'b')".into(),
    tuple2: "(2,'b')".into(),
    map1: "{}".into(),
});
