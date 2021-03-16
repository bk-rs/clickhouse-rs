#[cfg(feature = "_integration_tests")]
#[cfg(test)]
mod tests {
    /*
    https://clickhouse.tech/docs/en/sql-reference/functions/type-conversion-functions/
    https://clickhouse.tech/docs/en/sql-reference/functions/ip-address-functions/
    */

    use std::{env::var, error};

    use chrono04::{TimeZone as _, Utc};
    use clickhouse_postgres_client::{ClickhousePgConnection, ClickhousePgValue};

    async fn get_conn() -> Result<ClickhousePgConnection, Box<dyn error::Error>> {
        let mut conn = clickhouse_postgres_client::connect(
            var("CLICKHOUSE_DATABASE_URL")
                .unwrap_or_else(|_| "postgres://default:xxx@127.0.0.1:9005".to_string())
                .as_str(),
        )
        .await?;

        clickhouse_postgres_client::execute("set allow_experimental_map_type = 1;", &mut conn)
            .await?;
        clickhouse_postgres_client::execute("set allow_experimental_geo_types = 1;", &mut conn)
            .await?;

        Ok(conn)
    }

    async fn execute(sql: &str) -> Result<(), Box<dyn error::Error>> {
        let mut conn = get_conn().await?;

        clickhouse_postgres_client::execute(sql, &mut conn).await?;

        Ok(())
    }

    async fn fetch_one_and_get_data(
        sql: &str,
    ) -> Result<Vec<(String, ClickhousePgValue)>, Box<dyn error::Error>> {
        let mut conn = get_conn().await?;

        let row = clickhouse_postgres_client::fetch_one(sql, &mut conn).await?;

        let data = row
            .try_get_data()?
            .into_iter()
            .map(|(name, value)| (name.to_string(), value))
            .collect();

        Ok(data)
    }

    #[tokio::test]
    async fn test_i8() -> Result<(), Box<dyn error::Error>> {
        execute("CREATE TABLE t_boolean (true_val Boolean, false_val Boolean) ENGINE = Memory;")
            .await?;
        execute("INSERT INTO t_boolean VALUES (1, 0);").await?;
        assert_eq!(
            fetch_one_and_get_data("select true_val, toTypeName(true_val) as true_val_ty, false_val, toTypeName(false_val) as false_val_ty from t_boolean;").await?,
            vec![
                ("true_val".into(), ('1' as i8).into()),
                ("true_val_ty".into(), "Int8".into()),
                ("false_val".into(), ('0' as i8).into()),
                ("false_val_ty".into(), "Int8".into())
            ],
        );

        // only support 0-9
        assert_eq!(
            fetch_one_and_get_data("select toInt8(9) as val, toTypeName(val) as ty;").await?,
            vec![
                ("val".into(), ('9' as i8).into()),
                ("ty".into(), "Int8".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_i16() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            fetch_one_and_get_data("select toInt16(32767) as val, toTypeName(val) as ty;").await?,
            vec![
                ("val".into(), i16::MAX.into()),
                ("ty".into(), "Int16".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data("select toUInt8(255) as val, toTypeName(val) as ty;",).await?,
            vec![
                ("val".into(), u8::MAX.into()),
                ("ty".into(), "UInt8".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data("select toInt16(toInt8('-128')) as val, toTypeName(val) as ty;")
                .await?,
            vec![
                ("val".into(), (-128 as i16).into()),
                ("ty".into(), "Int16".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_i32() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            fetch_one_and_get_data("select toInt32(2147483647) as val, toTypeName(val) as ty;",)
                .await?,
            vec![
                ("val".into(), i32::MAX.into()),
                ("ty".into(), "Int32".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data("select toUInt16(65535) as val, toTypeName(val) as ty;",)
                .await?,
            vec![
                ("val".into(), u16::MAX.into()),
                ("ty".into(), "UInt16".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_i64() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            fetch_one_and_get_data(
                "select toInt64(9223372036854775807) as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), i64::MAX.into()),
                ("ty".into(), "Int64".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data("select toUInt32(4294967295) as val, toTypeName(val) as ty;",)
                .await?,
            vec![
                ("val".into(), u32::MAX.into()),
                ("ty".into(), "UInt32".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data(
                "select toUnixTimestamp64Milli(toDateTime64('2021-01-01 00:00:00.123456', 6, 'UTC')) as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                (
                    "val".into(),
                    Utc.ymd(2021,1,1).and_hms_micro(0,0,0,123456).timestamp_millis().into(),
                ),
                ("ty".into(), "Int64".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_f32() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            fetch_one_and_get_data("select toFloat32(1 - 0.9) as val, toTypeName(val) as ty;",)
                .await?,
            vec![
                ("val".into(), 0.1_f32.into()),
                ("ty".into(), "Float32".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_f64() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            fetch_one_and_get_data("select toFloat64(1 - 0.9) as val, toTypeName(val) as ty;",)
                .await?,
            vec![
                ("val".into(), 0.09999999999999998_f64.into()),
                ("ty".into(), "Float64".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_string() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            fetch_one_and_get_data(
                "select toInt128(9223372036854775807) as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), "9223372036854775807".into()),
                ("ty".into(), "Int128".into())
            ],
        );
        assert_eq!(
            fetch_one_and_get_data(
                "select toInt256(9223372036854775807) as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), "9223372036854775807".into()),
                ("ty".into(), "Int256".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data(
                "select toUInt64(9223372036854775807) as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), "9223372036854775807".into()),
                ("ty".into(), "UInt64".into())
            ],
        );
        assert_eq!(
            fetch_one_and_get_data(
                "select toUInt256(9223372036854775807) as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), "9223372036854775807".into()),
                ("ty".into(), "UInt256".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data(
                "select toDecimal256(toString(-1.111), 5) as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), "-1.11100".into(),),
                ("ty".into(), "Decimal(76, 5)".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data("select now('UTC') as val, toTypeName(val) as ty;",).await?,
            vec![
                (
                    "val".into(),
                    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().into(),
                ),
                ("ty".into(), "DateTime('UTC')".into())
            ],
        );
        assert_eq!(
            fetch_one_and_get_data(
                "select toDateTime('2021-01-01 00:00:00', 'UTC') as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), "2021-01-01 00:00:00".into(),),
                ("ty".into(), "DateTime('UTC')".into())
            ],
        );
        assert_eq!(
            fetch_one_and_get_data(
                "select toDateTime64('2021-01-01 00:00:00.123456', 6, 'UTC') as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), "2021-01-01 00:00:00.123456".into(),),
                ("ty".into(), "DateTime64(6, 'UTC')".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data("select toString(now(), 'UTC') as val, toTypeName(val) as ty;",)
                .await?,
            vec![
                (
                    "val".into(),
                    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().into(),
                ),
                ("ty".into(), "String".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data(
                "select toFixedString('foo', 8) as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), "foo\0\0\0\0\0".into(),),
                ("ty".into(), "FixedString(8)".into())
            ],
        );

        Ok(())
    }

    #[cfg(feature = "chrono")]
    #[tokio::test]
    async fn test_chrono() -> Result<(), Box<dyn error::Error>> {
        use sqlx_clickhouse_ext::sqlx_core::types::chrono::NaiveDate as SqlxNaiveDate;

        assert_eq!(
            fetch_one_and_get_data("select toDate('2021-01-01') as val, toTypeName(val) as ty;",)
                .await?,
            vec![
                ("val".into(), SqlxNaiveDate::from_ymd(2021, 1, 1).into()),
                ("ty".into(), "Date".into())
            ],
        );

        Ok(())
    }

    #[cfg(feature = "bigdecimal")]
    #[tokio::test]
    async fn test_bigdecimal() -> Result<(), Box<dyn error::Error>> {
        use sqlx_clickhouse_ext::sqlx_core::types::BigDecimal as SqlxBigDecimal;
        use std::str::FromStr as _;

        assert_eq!(
            fetch_one_and_get_data(
                "select toDecimal32(toString(-1.111), 5) as val, toTypeName(val) as ty;",
            )
            .await?,
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

        assert_eq!(
            fetch_one_and_get_data(
                "select toDecimal64(toString(-1.111), 5) as val, toTypeName(val) as ty;",
            )
            .await?,
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

        assert_eq!(
            fetch_one_and_get_data(
                "select toDecimal128(toString(-1.111), 5) as val, toTypeName(val) as ty;",
            )
            .await?,
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

    #[cfg(feature = "uuid")]
    #[tokio::test]
    async fn test_uuid() -> Result<(), Box<dyn error::Error>> {
        use sqlx_clickhouse_ext::sqlx_core::types::Uuid as SqlxUuid;

        assert_eq!(
            fetch_one_and_get_data(
                "select toUUID('61f0c404-5cb3-11e7-907b-a6006ad3dba0') as val, toTypeName(val) as ty;",
            ).await?,
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

    #[tokio::test]
    async fn test_data_type_enum() -> Result<(), Box<dyn error::Error>> {
        // https://clickhouse.tech/docs/en/sql-reference/data-types/enum/

        assert_eq!(
            fetch_one_and_get_data(
                "select cast('a', 'Enum(\\'a\\'=1, \\'b\\'=2)') as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), "a".into()),
                ("ty".into(), "Enum8('a' = 1, 'b' = 2)".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data(
                "select cast(cast('b', 'Enum(\\'a\\'=1, \\'b\\'=2)'), 'Int8') as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), ('2' as i8).into()),
                ("ty".into(), "Int8".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data(
                "select cast(cast('b', 'Enum16(\\'a\\'=1, \\'b\\'=2)'), 'Int16') as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), 2_i16.into()),
                ("ty".into(), "Int16".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_data_type_array() -> Result<(), Box<dyn error::Error>> {
        // https://clickhouse.tech/docs/en/sql-reference/data-types/array/

        assert_eq!(
            fetch_one_and_get_data("select array(1, 2) as val, toTypeName(val) as ty;",).await?,
            vec![
                ("val".into(), "[1,2]".into()),
                ("ty".into(), "Array(UInt8)".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_data_type_tuple() -> Result<(), Box<dyn error::Error>> {
        // https://clickhouse.tech/docs/en/sql-reference/data-types/tuple/

        assert_eq!(
            fetch_one_and_get_data("select tuple(1,'a') as val, toTypeName(val) as ty;",).await?,
            vec![
                ("val".into(), "(1,'a')".into()),
                ("ty".into(), "Tuple(UInt8, String)".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_data_type_ipv4() -> Result<(), Box<dyn error::Error>> {
        // https://clickhouse.tech/docs/en/sql-reference/data-types/domains/ipv4/

        assert_eq!(
            fetch_one_and_get_data(
                "select isIPv4String('127.0.0.1') as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![("val".into(), 1_u8.into()), ("ty".into(), "UInt8".into())],
        );

        assert_eq!(
            fetch_one_and_get_data(
                "select IPv4StringToNum('127.0.0.1') as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), 2130706433_i64.into()),
                ("ty".into(), "UInt32".into())
            ],
        );

        // TODO
        // select toIPv4('127.0.0.1') as val, toTypeName(val) as ty;
        // Error: ColumnDecode { index: "0", source: ParseIntError { kind: InvalidDigit } }

        assert_eq!(
            fetch_one_and_get_data(
                "select toUInt32(toIPv4('127.0.0.1')) as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), 2130706433_i64.into()),
                ("ty".into(), "UInt32".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data(
                "select IPv4NumToString(toIPv4('127.0.0.1')) as val, toTypeName(val) as ty;",
            )
            .await?,
            vec![
                ("val".into(), "127.0.0.1".into()),
                ("ty".into(), "String".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_data_type_map() -> Result<(), Box<dyn error::Error>> {
        // https://clickhouse.tech/docs/en/sql-reference/data-types/map/

        assert_eq!(
            fetch_one_and_get_data("select cast(([1, 2, 3], ['Ready', 'Steady', 'Go']), 'Map(UInt8, String)') as val, toTypeName(val) as ty;",).await?,
            vec![
                ("val".into(), "{1:'Ready',2:'Steady',3:'Go'}".into()),
                ("ty".into(), "Map(UInt8,String)".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_data_type_geo() -> Result<(), Box<dyn error::Error>> {
        // https://clickhouse.tech/docs/en/sql-reference/data-types/geo/

        execute("CREATE TABLE t_geo_point (p Point) ENGINE = Memory();").await?;
        execute("INSERT INTO t_geo_point VALUES((10, 10));").await?;

        assert_eq!(
            fetch_one_and_get_data("select p as val, toTypeName(val) as ty from t_geo_point;",)
                .await?,
            vec![
                ("val".into(), "(10,10)".into()),
                ("ty".into(), "Point".into())
            ],
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_data_type_nullable() -> Result<(), Box<dyn error::Error>> {
        // https://clickhouse.tech/docs/en/sql-reference/data-types/nullable/

        // TODO
        // select x as x_val, toTypeName(x_val) as x_ty, y as y_val, toTypeName(y_val) as y_ty from t_null;
        // Error: ColumnDecode { index: "2", source: UnexpectedNullError }

        execute("CREATE TABLE t_null (x Int16, y Nullable(Int16)) ENGINE = Memory();").await?;
        execute("INSERT INTO t_null VALUES (1, NULL);").await?;
        assert_eq!(
            fetch_one_and_get_data("select x as x_val, toTypeName(x_val) as x_ty, ifNull(y, -1) as y_val, toTypeName(y_val) as y_ty from t_null;",)
                .await?,
            vec![
                ("x_val".into(), 1_i16.into()),
                ("x_ty".into(), "Int16".into()),
                ("y_val".into(), (-1 as i16).into()),
                ("y_ty".into(), "Int16".into())
            ],
        );

        assert_eq!(
            fetch_one_and_get_data("select toNullable(10) as val, toTypeName(val) as ty;",).await?,
            vec![
                ("val".into(), "10".into()),
                ("ty".into(), "Nullable(UInt8)".into())
            ],
        );

        Ok(())
    }
}
