use std::convert::TryFrom;

use sqlx_clickhouse_ext::sqlx_core::error::Error;
#[cfg(feature = "chrono")]
use sqlx_clickhouse_ext::sqlx_core::types::chrono::NaiveDate;
#[cfg(feature = "bigdecimal")]
use sqlx_clickhouse_ext::sqlx_core::types::BigDecimal;
#[cfg(feature = "uuid")]
use sqlx_clickhouse_ext::sqlx_core::types::Uuid;

// https://github.com/ClickHouse/ClickHouse/blob/master/src/Core/PostgreSQLProtocol.cpp
pub(crate) enum ClickhousePgType {
    Char,
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    Varchar,
    #[cfg(feature = "chrono")]
    Date,
    #[cfg(feature = "bigdecimal")]
    Numeric,
    #[cfg(feature = "uuid")]
    Uuid,
}

impl TryFrom<(&str, usize)> for ClickhousePgType {
    type Error = Error;

    fn try_from(t: (&str, usize)) -> Result<Self, Self::Error> {
        let (s, index) = t;

        // https://github.com/launchbadge/sqlx/blob/v0.5.1/sqlx-core/src/postgres/type_info.rs#L447-L541
        match s {
            "\"CHAR\"" => Ok(Self::Char),
            "INT2" => Ok(Self::Int2),
            "INT4" => Ok(Self::Int4),
            "INT8" => Ok(Self::Int8),
            "FLOAT4" => Ok(Self::Float4),
            "FLOAT8" => Ok(Self::Float8),
            "VARCHAR" => Ok(Self::Varchar),
            #[cfg(feature = "chrono")]
            "DATE" => Ok(Self::Date),
            #[cfg(not(feature = "chrono"))]
            "DATE" => Err(Error::ColumnDecode {
                index: format!("{:?}", index),
                source: format!("unknown SQL type `{}`, should enable chrono feature", s).into(),
            }),
            #[cfg(feature = "bigdecimal")]
            "NUMERIC" => Ok(Self::Numeric),
            #[cfg(not(feature = "bigdecimal"))]
            "NUMERIC" => Err(Error::ColumnDecode {
                index: format!("{:?}", index),
                source: format!("unknown SQL type `{}`, should enable bigdecimal feature", s)
                    .into(),
            }),
            #[cfg(feature = "uuid")]
            "UUID" => Ok(Self::Uuid),
            #[cfg(not(feature = "uuid"))]
            "UUID" => Err(Error::ColumnDecode {
                index: format!("{:?}", index),
                source: format!("unknown SQL type `{}`, should enable uuid feature", s).into(),
            }),
            _ => Err(Error::ColumnDecode {
                index: format!("{:?}", index),
                source: format!("unknown SQL type `{}`", s).into(),
            }),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ClickhousePgValue {
    Char(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    #[cfg(feature = "chrono")]
    NaiveDate(NaiveDate),
    #[cfg(feature = "bigdecimal")]
    BigDecimal(BigDecimal),
    #[cfg(feature = "uuid")]
    Uuid(Uuid),
}
impl From<i8> for ClickhousePgValue {
    fn from(val: i8) -> Self {
        Self::Char(val)
    }
}
impl From<i16> for ClickhousePgValue {
    fn from(val: i16) -> Self {
        Self::I16(val)
    }
}
impl From<u8> for ClickhousePgValue {
    fn from(val: u8) -> Self {
        Self::I16(val.into())
    }
}
impl From<i32> for ClickhousePgValue {
    fn from(val: i32) -> Self {
        Self::I32(val)
    }
}
impl From<u16> for ClickhousePgValue {
    fn from(val: u16) -> Self {
        Self::I32(val.into())
    }
}
impl From<i64> for ClickhousePgValue {
    fn from(val: i64) -> Self {
        Self::I64(val)
    }
}
impl From<u32> for ClickhousePgValue {
    fn from(val: u32) -> Self {
        Self::I64(val.into())
    }
}
impl From<f32> for ClickhousePgValue {
    fn from(val: f32) -> Self {
        Self::F32(val)
    }
}
impl From<f64> for ClickhousePgValue {
    fn from(val: f64) -> Self {
        Self::F64(val)
    }
}
impl From<String> for ClickhousePgValue {
    fn from(val: String) -> Self {
        Self::String(val)
    }
}
impl From<&str> for ClickhousePgValue {
    fn from(val: &str) -> Self {
        Self::String(val.into())
    }
}
#[cfg(feature = "chrono")]
impl From<NaiveDate> for ClickhousePgValue {
    fn from(val: NaiveDate) -> Self {
        Self::NaiveDate(val)
    }
}
#[cfg(feature = "bigdecimal")]
impl From<BigDecimal> for ClickhousePgValue {
    fn from(val: BigDecimal) -> Self {
        Self::BigDecimal(val)
    }
}
#[cfg(feature = "uuid")]
impl From<Uuid> for ClickhousePgValue {
    fn from(val: Uuid) -> Self {
        Self::Uuid(val)
    }
}
