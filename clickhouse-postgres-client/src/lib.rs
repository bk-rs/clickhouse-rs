pub use sqlx_clickhouse_ext;

pub mod connect_options;
pub mod executor;
pub mod row;
pub mod type_info;
pub use self::connect_options::ClickhousePgConnectOptions;
pub use self::executor::ClickhouseExecutor;
pub use self::row::ClickhousePgRow;
pub use self::type_info::ClickhousePgValue;

pub type SqlxError = sqlx_clickhouse_ext::sqlx_core::error::Error;
pub type ClickhousePgPool = sqlx_clickhouse_ext::sqlx_core::postgres::PgPool;
pub type ClickhousePgPoolOptions = sqlx_clickhouse_ext::sqlx_core::postgres::PgPoolOptions;
pub type ClickhousePgPoolConnection = sqlx_clickhouse_ext::sqlx_core::pool::PoolConnection<
    sqlx_clickhouse_ext::sqlx_core::postgres::Postgres,
>;
pub type ClickhousePgConnection = sqlx_clickhouse_ext::sqlx_core::postgres::PgConnection;

//
use sqlx_clickhouse_ext::sqlx_core::{
    connection::ConnectOptions as _, database::Database as SqlxDatabase,
};

pub async fn connect(url: &str) -> Result<ClickhousePgConnection, SqlxError> {
    connect_with(&url.parse()?).await
}

pub async fn connect_with(
    options: &ClickhousePgConnectOptions,
) -> Result<ClickhousePgConnection, SqlxError> {
    options.inner.connect().await
}

pub async fn execute<'c, 'q, 'async_trait, E>(sql: &'q str, executor: E) -> Result<(), SqlxError>
where
    E: ClickhouseExecutor<'c, 'q, 'async_trait, ClickhousePgRow>,
    'c: 'async_trait,
    'q: 'async_trait,
    ClickhousePgRow: From<<E::Database as SqlxDatabase>::Row>,
{
    ClickhouseExecutor::execute(executor, sql).await
}

pub async fn fetch_all<'c, 'q, 'async_trait, E>(
    sql: &'q str,
    executor: E,
) -> Result<Vec<ClickhousePgRow>, SqlxError>
where
    E: ClickhouseExecutor<'c, 'q, 'async_trait, ClickhousePgRow>,
    'c: 'async_trait,
    'q: 'async_trait,
    ClickhousePgRow: From<<E::Database as SqlxDatabase>::Row>,
{
    ClickhouseExecutor::fetch_all(executor, sql).await
}

pub async fn fetch_one<'c, 'q, 'async_trait, E>(
    sql: &'q str,
    executor: E,
) -> Result<ClickhousePgRow, SqlxError>
where
    E: ClickhouseExecutor<'c, 'q, 'async_trait, ClickhousePgRow>,
    'c: 'async_trait,
    'q: 'async_trait,
    ClickhousePgRow: From<<E::Database as SqlxDatabase>::Row>,
{
    ClickhouseExecutor::fetch_one(executor, sql).await
}

pub async fn fetch_optional<'c, 'q, 'async_trait, E>(
    sql: &'q str,
    executor: E,
) -> Result<Option<ClickhousePgRow>, SqlxError>
where
    E: ClickhouseExecutor<'c, 'q, 'async_trait, ClickhousePgRow>,
    'c: 'async_trait,
    'q: 'async_trait,
    ClickhousePgRow: From<<E::Database as SqlxDatabase>::Row>,
{
    ClickhouseExecutor::fetch_optional(executor, sql).await
}
