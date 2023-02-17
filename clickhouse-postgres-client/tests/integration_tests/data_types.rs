use core::str::FromStr as _;

use super::helpers::*;

#[tokio::test]
async fn test_dt_int8() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    // int8 only support 0-9
    assert_eq!(
        fetch_one_and_get_data(get_sql("int-uint/select_Int8_0-9"), &mut conn).await?,
        vec![
            ("a_val".into(), ('0' as i8).into()),
            ("a_ty".into(), "Int8".into()),
            ("b_val".into(), ('9' as i8).into()),
            ("b_ty".into(), "Int8".into())
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_int16() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("int-uint/select_Int16"), &mut conn).await?,
        vec![
            ("min_val".into(), i16::MIN.into()),
            ("min_ty".into(), "Int16".into()),
            ("max_val".into(), i16::MAX.into()),
            ("max_ty".into(), "Int16".into()),
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_int32() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("int-uint/select_Int32"), &mut conn).await?,
        vec![
            ("min_val".into(), i32::MIN.into()),
            ("min_ty".into(), "Int32".into()),
            ("max_val".into(), i32::MAX.into()),
            ("max_ty".into(), "Int32".into()),
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_int64() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("int-uint/select_Int64"), &mut conn).await?,
        vec![
            ("min_val".into(), i64::MIN.into()),
            ("min_ty".into(), "Int64".into()),
            ("max_val".into(), i64::MAX.into()),
            ("max_ty".into(), "Int64".into()),
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_int128() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("int-uint/select_Int128"), &mut conn).await?,
        vec![
            (
                "min_val".into(),
                "-170141183460469231731687303715884105728".into()
            ),
            ("min_ty".into(), "Int128".into()),
            (
                "max_val".into(),
                "170141183460469231731687303715884105727".into()
            ),
            ("max_ty".into(), "Int128".into()),
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_int256() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("int-uint/select_Int256"), &mut conn).await?,
        vec![
            (
                "min_val".into(),
                "-57896044618658097711785492504343953926634992332820282019728792003956564819968"
                    .into()
            ),
            ("min_ty".into(), "Int256".into()),
            (
                "max_val".into(),
                "57896044618658097711785492504343953926634992332820282019728792003956564819967"
                    .into()
            ),
            ("max_ty".into(), "Int256".into()),
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_uint8() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("int-uint/select_UInt8"), &mut conn).await?,
        vec![
            ("min_val".into(), u8::MIN.into()),
            ("min_ty".into(), "UInt8".into()),
            ("max_val".into(), u8::MAX.into()),
            ("max_ty".into(), "UInt8".into()),
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_uint16() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("int-uint/select_UInt16"), &mut conn).await?,
        vec![
            ("min_val".into(), u16::MIN.into()),
            ("min_ty".into(), "UInt16".into()),
            ("max_val".into(), u16::MAX.into()),
            ("max_ty".into(), "UInt16".into()),
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_uint32() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("int-uint/select_UInt32"), &mut conn).await?,
        vec![
            ("min_val".into(), u32::MIN.into()),
            ("min_ty".into(), "UInt32".into()),
            ("max_val".into(), u32::MAX.into()),
            ("max_ty".into(), "UInt32".into()),
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_uint64() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("int-uint/select_UInt64"), &mut conn).await?,
        vec![
            ("min_val".into(), format!("{}", u64::MIN).into()),
            ("min_ty".into(), "UInt64".into()),
            ("max_val".into(), format!("{}", u64::MAX).into()),
            ("max_ty".into(), "UInt64".into()),
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_uint256() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("int-uint/select_UInt256"), &mut conn).await?,
        vec![
            ("min_val".into(), "0".into()),
            ("min_ty".into(), "UInt256".into()),
            (
                "max_val".into(),
                "115792089237316195423570985008687907853269984665640564039457584007913129639935"
                    .into()
            ),
            ("max_ty".into(), "UInt256".into()),
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_float32() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("float/select_Float32"), &mut conn).await?,
        vec![
            ("val".into(), 0.1_f32.into()),
            ("ty".into(), "Float32".into()),
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_float64() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("float/select_Float64"), &mut conn).await?,
        vec![
            ("val".into(), 0.09999999999999998_f64.into()),
            ("ty".into(), "Float64".into()),
        ],
    );

    Ok(())
}

#[cfg(feature = "bigdecimal")]
#[tokio::test]
async fn test_dt_decimal32() -> Result<(), Box<dyn std::error::Error>> {
    use sqlx_clickhouse_ext::sqlx_core::types::BigDecimal as SqlxBigDecimal;

    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("decimal/select_Decimal32"), &mut conn).await?,
        vec![
            (
                "val".into(),
                SqlxBigDecimal::from_str("-1.111")
                    .unwrap()
                    .with_scale(5)
                    .into()
            ),
            ("ty".into(), "Decimal(9, 5)".into())
        ],
    );

    Ok(())
}

#[cfg(feature = "bigdecimal")]
#[tokio::test]
async fn test_dt_decimal64() -> Result<(), Box<dyn std::error::Error>> {
    use sqlx_clickhouse_ext::sqlx_core::types::BigDecimal as SqlxBigDecimal;

    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("decimal/select_Decimal64"), &mut conn).await?,
        vec![
            (
                "val".into(),
                SqlxBigDecimal::from_str("-1.111")
                    .unwrap()
                    .with_scale(5)
                    .into()
            ),
            ("ty".into(), "Decimal(18, 5)".into())
        ],
    );

    Ok(())
}

#[cfg(feature = "bigdecimal")]
#[tokio::test]
async fn test_dt_decimal128() -> Result<(), Box<dyn std::error::Error>> {
    use sqlx_clickhouse_ext::sqlx_core::types::BigDecimal as SqlxBigDecimal;

    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("decimal/select_Decimal128"), &mut conn).await?,
        vec![
            (
                "val".into(),
                SqlxBigDecimal::from_str("-1.111")
                    .unwrap()
                    .with_scale(5)
                    .into()
            ),
            ("ty".into(), "Decimal(38, 5)".into())
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_decimal256() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("decimal/select_Decimal256"), &mut conn).await?,
        vec![
            ("val".into(), "-1.111".into(),),
            ("ty".into(), "Decimal(76, 5)".into())
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_boolean() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    execute(get_sql("boolean/create_table"), &mut conn).await?;
    execute(get_sql("boolean/insert"), &mut conn).await?;
    assert_eq!(
        fetch_one_and_get_data(get_sql("boolean/select"), &mut conn).await?,
        vec![
            ("true_val".into(), ('1' as i8).into()),
            ("true_ty".into(), "Int8".into()),
            ("false_val".into(), ('0' as i8).into()),
            ("false_ty".into(), "Int8".into())
        ],
    );
    execute(get_sql("boolean/drop_table"), &mut conn).await?;

    Ok(())
}

#[tokio::test]
async fn test_dt_string() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("string/select_f_toString_date"), &mut conn).await?,
        vec![
            ("val".into(), "2021-03-01".into(),),
            ("ty".into(), "String".into())
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_fixedstring() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("fixedstring/select"), &mut conn).await?,
        vec![
            ("val".into(), "foo\0\0\0\0\0".into(),),
            ("ty".into(), "FixedString(8)".into())
        ],
    );

    Ok(())
}

#[cfg(feature = "uuid")]
#[tokio::test]
async fn test_dt_uuid() -> Result<(), Box<dyn std::error::Error>> {
    use sqlx_clickhouse_ext::sqlx_core::types::Uuid as SqlxUuid;

    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("uuid/select"), &mut conn).await?,
        vec![
            (
                "val".into(),
                SqlxUuid::parse_str("61f0c404-5cb3-11e7-907b-a6006ad3dba0")
                    .unwrap()
                    .into()
            ),
            ("ty".into(), "UUID".into())
        ],
    );

    Ok(())
}

#[cfg(feature = "chrono")]
#[tokio::test]
async fn test_dt_date() -> Result<(), Box<dyn std::error::Error>> {
    use sqlx_clickhouse_ext::sqlx_core::types::chrono::NaiveDate as SqlxNaiveDate;

    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("date/select"), &mut conn).await?,
        vec![
            (
                "val".into(),
                SqlxNaiveDate::from_ymd_opt(2021, 3, 1).expect("").into()
            ),
            ("ty".into(), "Date".into())
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_datetime() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("datetime/select_with_UTC"), &mut conn).await?,
        vec![
            ("val".into(), "2021-03-01 01:02:03".into(),),
            ("ty".into(), "DateTime('UTC')".into())
        ],
    );

    //
    let mut conn =
        get_conn(&[get_setting_sql("set_date_time_output_format_to_simple").as_str()]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("datetime/select_with_UTC"), &mut conn).await?,
        vec![
            ("val".into(), "2021-03-01 01:02:03".into(),),
            ("ty".into(), "DateTime('UTC')".into())
        ],
    );

    //
    let mut conn =
        get_conn(&[get_setting_sql("set_date_time_output_format_to_iso").as_str()]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("datetime/select_with_UTC"), &mut conn).await?,
        vec![
            ("val".into(), "2021-03-01T01:02:03Z".into(),),
            ("ty".into(), "DateTime('UTC')".into())
        ],
    );

    //
    let mut conn =
        get_conn(&[get_setting_sql("set_date_time_output_format_to_unix_timestamp").as_str()])
            .await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("datetime/select_with_UTC"), &mut conn).await?,
        vec![
            ("val".into(), "1614560523".into(),),
            ("ty".into(), "DateTime('UTC')".into())
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_datetime64() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("datetime64/select_with_micro_and_UTC"), &mut conn).await?,
        vec![
            ("val".into(), "2021-03-01 01:02:03.123456".into(),),
            ("ty".into(), "DateTime64(6, 'UTC')".into())
        ],
    );

    assert_eq!(
        fetch_one_and_get_data(get_sql("datetime64/select_with_milli_and_UTC"), &mut conn).await?,
        vec![
            ("val".into(), "2021-03-01 01:02:03.123".into(),),
            ("ty".into(), "DateTime64(3, 'UTC')".into())
        ],
    );

    assert_eq!(
        fetch_one_and_get_data(get_sql("datetime64/select_with_nano_and_UTC"), &mut conn).await?,
        vec![
            ("val".into(), "2021-03-01 01:02:03.123456789".into(),),
            ("ty".into(), "DateTime64(9, 'UTC')".into())
        ],
    );

    //
    let mut conn =
        get_conn(&[get_setting_sql("set_date_time_output_format_to_simple").as_str()]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("datetime64/select_with_nano_and_UTC"), &mut conn).await?,
        vec![
            ("val".into(), "2021-03-01 01:02:03.123456789".into(),),
            ("ty".into(), "DateTime64(9, 'UTC')".into())
        ],
    );

    //
    let mut conn =
        get_conn(&[get_setting_sql("set_date_time_output_format_to_iso").as_str()]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("datetime64/select_with_nano_and_UTC"), &mut conn).await?,
        vec![
            ("val".into(), "2021-03-01T01:02:03.123456789Z".into(),),
            ("ty".into(), "DateTime64(9, 'UTC')".into())
        ],
    );

    //
    let mut conn =
        get_conn(&[get_setting_sql("set_date_time_output_format_to_unix_timestamp").as_str()])
            .await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("datetime64/select_with_nano_and_UTC"), &mut conn).await?,
        vec![
            ("val".into(), "1614560523.123456789".into(),),
            ("ty".into(), "DateTime64(9, 'UTC')".into())
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_enum() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("enum/select_Enum8"), &mut conn).await?,
        vec![
            ("val".into(), "a".into()),
            ("ty".into(), "Enum8('a' = -128, 'b' = 127)".into())
        ],
    );

    assert_eq!(
        fetch_one_and_get_data(get_sql("enum/select_Enum16"), &mut conn).await?,
        vec![
            ("val".into(), "a".into()),
            ("ty".into(), "Enum16('a' = -32768, 'b' = 32767)".into())
        ],
    );

    assert_eq!(
        fetch_one_and_get_data(get_sql("enum/select_cast_Enum8"), &mut conn).await?,
        vec![
            ("a_val".into(), ('1' as i8).into()),
            ("a_ty".into(), "Int8".into()),
            ("b_val".into(), 2_i16.into()),
            ("b_ty".into(), "Int16".into()),
        ],
    );

    assert_eq!(
        fetch_one_and_get_data(get_sql("enum/select_cast_Enum16"), &mut conn).await?,
        vec![("val".into(), 1_i16.into()), ("ty".into(), "Int16".into()),],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_array() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("array/select_with_UInt8"), &mut conn).await?,
        vec![
            ("val".into(), "[1,2]".into()),
            ("ty".into(), "Array(UInt8)".into())
        ],
    );

    assert_eq!(
        fetch_one_and_get_data(get_sql("array/select_with_String"), &mut conn).await?,
        vec![
            ("val".into(), "['a','b']".into()),
            ("ty".into(), "Array(String)".into())
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_tuple() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("tuple/select_with_UInt8_and_String"), &mut conn).await?,
        vec![
            ("val".into(), "(1,'a')".into()),
            ("ty".into(), "Tuple(UInt8, String)".into())
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_domains_ipv4() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("domains_ipv4/select_f_toUInt32"), &mut conn).await?,
        vec![
            ("val".into(), 2130706433_i64.into()),
            ("ty".into(), "UInt32".into())
        ],
    );

    assert_eq!(
        fetch_one_and_get_data(get_sql("domains_ipv4/select_f_IPv4NumToString"), &mut conn).await?,
        vec![
            ("val".into(), "127.0.0.1".into()),
            ("ty".into(), "String".into())
        ],
    );

    assert_eq!(
        fetch_one_and_get_data(get_sql("domains_ipv4/select_f_isIPv4String"), &mut conn).await?,
        vec![("val".into(), 1_u8.into()), ("ty".into(), "UInt8".into())],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_domains_ipv6() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("domains_ipv6/select_f_hex"), &mut conn).await?,
        vec![
            ("val".into(), "2A02AA08E00031000000000000000002".into()),
            ("ty".into(), "String".into())
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_geo() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn =
        get_conn(&[get_setting_sql("enable_allow_experimental_geo_types").as_str()]).await?;

    execute(get_sql("geo/create_table_Point"), &mut conn).await?;
    execute(get_sql("geo/insert_Point"), &mut conn).await?;
    assert_eq!(
        fetch_one_and_get_data(get_sql("geo/select_Point"), &mut conn).await?,
        vec![
            ("val".into(), "(10,10)".into()),
            ("ty".into(), "Point".into())
        ],
    );
    execute(get_sql("geo/drop_table_Point"), &mut conn).await?;

    Ok(())
}

#[tokio::test]
async fn test_dt_map() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn =
        get_conn(&[get_setting_sql("enable_allow_experimental_map_type").as_str()]).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("map/select_with_UInt8_and_String"), &mut conn).await?,
        vec![
            ("val".into(), "{1:'Ready',2:'Steady',3:'Go'}".into()),
            ("ty".into(), "Map(UInt8, String)".into())
        ],
    );

    Ok(())
}

#[tokio::test]
async fn test_dt_nullable() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_conn(&[]).await?;

    execute(get_sql("nullable/create_table"), &mut conn).await?;
    execute(get_sql("nullable/insert"), &mut conn).await?;
    assert_eq!(
        fetch_one_and_get_data(get_sql("nullable/select_f_ifNull"), &mut conn).await?,
        vec![
            ("x_val".into(), 1_i16.into()),
            ("x_ty".into(), "Int16".into()),
            ("y_val".into(), (-1_i16).into()),
            ("y_ty".into(), "Int16".into())
        ],
    );
    execute(get_sql("nullable/drop_table"), &mut conn).await?;

    assert_eq!(
        fetch_one_and_get_data(get_sql("nullable/select_f_toNullable"), &mut conn).await?,
        vec![
            ("val".into(), "10".into()),
            ("ty".into(), "Nullable(UInt8)".into())
        ],
    );

    Ok(())
}
