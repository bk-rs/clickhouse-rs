use core::num::ParseIntError;
use std::net::{AddrParseError, Ipv4Addr};

#[cfg(feature = "chrono")]
use clickhouse_data_value::datetime::{
    NaiveDateTime as DatetimeNaiveDateTime, ParseError as DatetimeParseError,
};
#[cfg(feature = "num-bigint")]
use num_bigint::{BigInt, BigUint, ParseBigIntError};
use sqlx_clickhouse_ext::sqlx_core::error::Error;
#[cfg(feature = "bigdecimal")]
use sqlx_clickhouse_ext::sqlx_core::types::BigDecimal;
#[cfg(feature = "uuid")]
use sqlx_clickhouse_ext::sqlx_core::types::Uuid;
#[cfg(feature = "chrono")]
use sqlx_clickhouse_ext::sqlx_core::types::chrono::{NaiveDate, NaiveDateTime};

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
                index: format!("{index:?}"),
                source: format!("unknown SQL type `{}`, should enable chrono feature", s).into(),
            }),
            #[cfg(feature = "bigdecimal")]
            "NUMERIC" => Ok(Self::Numeric),
            #[cfg(not(feature = "bigdecimal"))]
            "NUMERIC" => Err(Error::ColumnDecode {
                index: format!("{index:?}"),
                source: format!("unknown SQL type `{}`, should enable bigdecimal feature", s)
                    .into(),
            }),
            #[cfg(feature = "uuid")]
            "UUID" => Ok(Self::Uuid),
            #[cfg(not(feature = "uuid"))]
            "UUID" => Err(Error::ColumnDecode {
                index: format!("{index:?}"),
                source: format!("unknown SQL type `{}`, should enable uuid feature", s).into(),
            }),
            _ => Err(Error::ColumnDecode {
                index: format!("{index:?}"),
                source: format!("unknown SQL type `{s}`").into(),
            }),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ClickhousePgValue {
    Bool(bool),
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
impl From<bool> for ClickhousePgValue {
    fn from(val: bool) -> Self {
        Self::Bool(val)
    }
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

impl ClickhousePgValue {
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Self::Bool(v) => Some(v),
            Self::Char(v) if v == '1' as i8 => Some(true),
            Self::Char(v) if v == '0' as i8 => Some(false),
            _ => self.as_u8().and_then(|v| match v {
                1 => Some(true),
                0 => Some(false),
                _ => None,
            }),
        }
    }
    pub fn as_char(&self) -> Option<i8> {
        match *self {
            Self::Char(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_u8(&self) -> Option<u8> {
        match *self {
            Self::I16(v) if (u8::MIN as i16..=u8::MAX as i16).contains(&v) => Some(v as u8),
            _ => None,
        }
    }
    pub fn as_i16(&self) -> Option<i16> {
        match *self {
            Self::I16(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_u16(&self) -> Option<u16> {
        match *self {
            Self::I32(v) if (u16::MIN as i32..=u16::MAX as i32).contains(&v) => Some(v as u16),
            _ => None,
        }
    }
    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            Self::I32(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_u32(&self) -> Option<u32> {
        match *self {
            Self::I64(v) if (u32::MIN as i64..=u32::MAX as i64).contains(&v) => Some(v as u32),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Self::I64(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<Result<u64, ParseIntError>> {
        match *self {
            Self::String(ref v) => Some(v.parse()),
            _ => None,
        }
    }
    pub fn as_i128(&self) -> Option<Result<i128, ParseIntError>> {
        match *self {
            Self::String(ref v) => Some(v.parse()),
            _ => None,
        }
    }
    pub fn as_u128(&self) -> Option<Result<u128, ParseIntError>> {
        match *self {
            Self::String(ref v) => Some(v.parse()),
            _ => None,
        }
    }
    #[cfg(feature = "num-bigint")]
    pub fn as_big_int(&self) -> Option<Result<BigInt, ParseBigIntError>> {
        match *self {
            Self::String(ref v) => Some(v.parse()),
            _ => None,
        }
    }
    #[cfg(feature = "num-bigint")]
    pub fn as_big_uint(&self) -> Option<Result<BigUint, ParseBigIntError>> {
        match *self {
            Self::String(ref v) => Some(v.parse()),
            _ => None,
        }
    }
    pub fn as_f32(&self) -> Option<f32> {
        match *self {
            Self::F32(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Self::F64(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Self::String(ref v) => Some(v),
            _ => None,
        }
    }
    #[cfg(feature = "chrono")]
    pub fn as_naive_date(&self) -> Option<&NaiveDate> {
        match *self {
            Self::NaiveDate(ref v) => Some(v),
            _ => None,
        }
    }
    #[cfg(feature = "bigdecimal")]
    pub fn as_big_decimal(&self) -> Option<&BigDecimal> {
        match *self {
            Self::BigDecimal(ref v) => Some(v),
            _ => None,
        }
    }
    #[cfg(feature = "uuid")]
    pub fn as_uuid(&self) -> Option<&Uuid> {
        match *self {
            Self::Uuid(ref v) => Some(v),
            _ => None,
        }
    }

    #[cfg(feature = "chrono")]
    pub fn as_naive_date_time(&self) -> Option<Result<NaiveDateTime, DatetimeParseError>> {
        match *self {
            Self::String(ref v) => Some(v.parse::<DatetimeNaiveDateTime>().map(|x| x.0)),
            _ => None,
        }
    }

    pub fn as_ipv4_addr(&self) -> Option<Result<Ipv4Addr, AddrParseError>> {
        match *self {
            Self::String(ref v) => Some(v.parse()),
            _ => self.as_u32().map(|v| Ok(v.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_bool() {
        assert_eq!(ClickhousePgValue::from(false).as_bool(), Some(false));
        assert_eq!(ClickhousePgValue::from(true).as_bool(), Some(true));

        assert_eq!(ClickhousePgValue::from('0' as i8).as_bool(), Some(false));
        assert_eq!(ClickhousePgValue::from('1' as i8).as_bool(), Some(true));
        assert_eq!(ClickhousePgValue::from('2' as i8).as_bool(), None);

        assert_eq!(ClickhousePgValue::from(0_u8).as_bool(), Some(false));
        assert_eq!(ClickhousePgValue::from(1_u8).as_bool(), Some(true));
        assert_eq!(ClickhousePgValue::from(2_u8).as_bool(), None);
    }
    #[test]
    fn test_as_char() {
        assert_eq!(
            ClickhousePgValue::from('3' as i8).as_char(),
            Some('3' as i8)
        );
    }
    #[test]
    fn test_as_u8() {
        assert_eq!(ClickhousePgValue::from(u8::MIN).as_u8(), Some(u8::MIN));
        assert_eq!(ClickhousePgValue::from(u8::MAX).as_u8(), Some(u8::MAX));
    }
    #[test]
    fn test_as_i16() {
        assert_eq!(ClickhousePgValue::from(i16::MIN).as_i16(), Some(i16::MIN));
        assert_eq!(ClickhousePgValue::from(i16::MAX).as_i16(), Some(i16::MAX));
    }
    #[test]
    fn test_as_u16() {
        assert_eq!(ClickhousePgValue::from(u16::MIN).as_u16(), Some(u16::MIN));
        assert_eq!(ClickhousePgValue::from(u16::MAX).as_u16(), Some(u16::MAX));
    }
    #[test]
    fn test_as_i32() {
        assert_eq!(ClickhousePgValue::from(i32::MIN).as_i32(), Some(i32::MIN));
        assert_eq!(ClickhousePgValue::from(i32::MAX).as_i32(), Some(i32::MAX));
    }
    #[test]
    fn test_as_u32() {
        assert_eq!(ClickhousePgValue::from(u32::MIN).as_u32(), Some(u32::MIN));
        assert_eq!(ClickhousePgValue::from(u32::MAX).as_u32(), Some(u32::MAX));
    }
    #[test]
    fn test_as_i64() {
        assert_eq!(ClickhousePgValue::from(i64::MIN).as_i64(), Some(i64::MIN));
        assert_eq!(ClickhousePgValue::from(i64::MAX).as_i64(), Some(i64::MAX));
    }
    #[test]
    fn test_as_u64() {
        assert_eq!(
            ClickhousePgValue::from(format!("{}", u64::MIN)).as_u64(),
            Some(Ok(u64::MIN))
        );
        assert_eq!(
            ClickhousePgValue::from(format!("{}", u64::MAX)).as_u64(),
            Some(Ok(u64::MAX))
        );
    }
    #[test]
    fn test_as_i128() {
        assert_eq!(
            ClickhousePgValue::from(format!("{}", i128::MIN)).as_i128(),
            Some(Ok(i128::MIN))
        );
        assert_eq!(
            ClickhousePgValue::from(format!("{}", i128::MAX)).as_i128(),
            Some(Ok(i128::MAX))
        );
    }
    #[test]
    fn test_as_u128() {
        assert_eq!(
            ClickhousePgValue::from(format!("{}", u128::MIN)).as_u128(),
            Some(Ok(u128::MIN))
        );
        assert_eq!(
            ClickhousePgValue::from(format!("{}", u128::MAX)).as_u128(),
            Some(Ok(u128::MAX))
        );
    }
    #[cfg(feature = "num-bigint")]
    #[test]
    fn test_as_big_int() {
        assert_eq!(
            ClickhousePgValue::from(
                "-57896044618658097711785492504343953926634992332820282019728792003956564819968"
            )
            .as_big_int(),
            Some(Ok(BigInt::parse_bytes(
                b"-57896044618658097711785492504343953926634992332820282019728792003956564819968",
                10
            )
            .unwrap()))
        );
        assert_eq!(
            ClickhousePgValue::from(
                "57896044618658097711785492504343953926634992332820282019728792003956564819967"
            )
            .as_big_int(),
            Some(Ok(BigInt::parse_bytes(
                b"57896044618658097711785492504343953926634992332820282019728792003956564819967",
                10
            )
            .unwrap()))
        );
    }
    #[cfg(feature = "num-bigint")]
    #[test]
    fn test_as_big_uint() {
        assert_eq!(
            ClickhousePgValue::from("0").as_big_uint(),
            Some(Ok(BigUint::parse_bytes(b"0", 10).unwrap()))
        );
        assert_eq!(
            ClickhousePgValue::from(
                "115792089237316195423570985008687907853269984665640564039457584007913129639935"
            )
            .as_big_uint(),
            Some(Ok(BigUint::parse_bytes(
                b"115792089237316195423570985008687907853269984665640564039457584007913129639935",
                10
            )
            .unwrap()))
        );
    }
    #[test]
    fn test_as_f32() {
        assert_eq!(ClickhousePgValue::from(f32::MIN).as_f32(), Some(f32::MIN));
        assert_eq!(ClickhousePgValue::from(f32::MAX).as_f32(), Some(f32::MAX));
    }
    #[test]
    fn test_as_f64() {
        assert_eq!(ClickhousePgValue::from(f64::MIN).as_f64(), Some(f64::MIN));
        assert_eq!(ClickhousePgValue::from(f64::MAX).as_f64(), Some(f64::MAX));
    }
    #[test]
    fn test_as_str() {
        assert_eq!(ClickhousePgValue::from("foo").as_str(), Some("foo"));
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_as_naive_date() {
        let naive_date = NaiveDate::from_ymd_opt(2021, 1, 1).expect("");
        assert_eq!(
            ClickhousePgValue::from(naive_date).as_naive_date(),
            Some(&NaiveDate::from_ymd_opt(2021, 1, 1).expect(""))
        );
    }
    #[cfg(feature = "bigdecimal")]
    #[test]
    fn test_as_big_decimal() {
        let big_decimal = BigDecimal::parse_bytes(b"1.1", 10).unwrap();
        assert_eq!(
            ClickhousePgValue::from(big_decimal.clone()).as_big_decimal(),
            Some(&big_decimal)
        );
    }
    #[cfg(feature = "uuid")]
    #[test]
    fn test_as_uuid() {
        let uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
        assert_eq!(ClickhousePgValue::from(uuid).as_uuid(), Some(&uuid));
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_as_naive_date_time() {
        let dt = NaiveDate::from_ymd_opt(2021, 1, 1)
            .expect("")
            .and_hms_nano_opt(0, 0, 0, 123456789)
            .expect("");
        match ClickhousePgValue::from(dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .as_naive_date_time()
        {
            Some(Ok(dt)) => assert_eq!(
                dt,
                NaiveDate::from_ymd_opt(2021, 1, 1)
                    .expect("")
                    .and_hms_opt(0, 0, 0)
                    .expect("")
            ),
            Some(Err(err)) => panic!("{err:?}"),
            None => panic!(),
        }
    }

    #[test]
    fn test_as_ipv4_addr() {
        assert_eq!(
            ClickhousePgValue::from("127.0.0.1").as_ipv4_addr(),
            Some(Ok(Ipv4Addr::new(127, 0, 0, 1)))
        );

        assert_eq!(
            ClickhousePgValue::from(u32::from(Ipv4Addr::new(127, 0, 0, 1))).as_ipv4_addr(),
            Some(Ok(Ipv4Addr::new(127, 0, 0, 1)))
        );
    }
}
