use std::{env, fs, path::PathBuf};

use clickhouse_postgres_client::{ClickhousePgConnection, ClickhousePgValue};

pub(super) async fn get_conn(
    init_sqls: &[&str],
) -> Result<ClickhousePgConnection, Box<dyn std::error::Error>> {
    let mut conn = clickhouse_postgres_client::connect(
        env::var("CLICKHOUSE_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://default:xxx@127.0.0.1:9005".to_string())
            .as_str(),
    )
    .await?;

    for sql in init_sqls {
        clickhouse_postgres_client::execute(sql, &mut conn).await?;
    }

    Ok(conn)
}

pub(super) async fn execute(
    sql: impl AsRef<str>,
    conn: &mut ClickhousePgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    clickhouse_postgres_client::execute(sql.as_ref(), conn).await?;

    Ok(())
}

pub(super) async fn fetch_one_and_get_data(
    sql: impl AsRef<str>,
    conn: &mut ClickhousePgConnection,
) -> Result<Vec<(String, ClickhousePgValue)>, Box<dyn std::error::Error>> {
    let row = clickhouse_postgres_client::fetch_one(sql.as_ref(), conn).await?;

    let data = row
        .try_get_data()?
        .into_iter()
        .map(|(name, value)| (name.to_string(), value))
        .collect();

    Ok(data)
}

pub(super) fn get_sql(path: &str) -> String {
    fs::read_to_string(PathBuf::new().join(format!("../clickhouse_sqls/data_types/{path}.sql")))
        .unwrap()
}

pub(super) fn get_setting_sql(path: &str) -> String {
    fs::read_to_string(PathBuf::new().join(format!("../clickhouse_sqls/settings/{path}.sql")))
        .unwrap()
}
