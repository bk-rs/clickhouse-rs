pub use sqlx_clickhouse_ext::executor::ExecutorExt as ClickhouseExecutor;

use crate::{
    ClickhousePgConnection, ClickhousePgPool, ClickhousePgPoolConnection, row::ClickhousePgRow,
};

impl<'c, 'q, 'async_trait> ClickhouseExecutor<'c, 'q, 'async_trait, ClickhousePgRow>
    for &'c ClickhousePgPool
where
    'c: 'async_trait,
    'q: 'async_trait,
{
}

impl<'c, 'q, 'async_trait> ClickhouseExecutor<'c, 'q, 'async_trait, ClickhousePgRow>
    for &'c mut ClickhousePgPoolConnection
where
    'c: 'async_trait,
    'q: 'async_trait,
{
}

impl<'c, 'q, 'async_trait> ClickhouseExecutor<'c, 'q, 'async_trait, ClickhousePgRow>
    for &'c mut ClickhousePgConnection
where
    'c: 'async_trait,
    'q: 'async_trait,
{
}
