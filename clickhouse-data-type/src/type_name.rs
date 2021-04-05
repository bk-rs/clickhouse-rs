use std::collections::HashMap;

use chrono_tz::Tz;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammars/type_name.pest"]
struct TypeNameParser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeName {
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt256,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Int256,
    Float32,
    Float64,
    Decimal { precision: u8, scale: u8 },
    String,
    FixedString { n: usize },
    UUID,
    Date,
    DateTime { timezone: Tz },
    DateTime64 { precision: u8, timezone: Tz },
    Enum8(HashMap<String, i8>),
    Enum16(HashMap<String, i16>),
}
