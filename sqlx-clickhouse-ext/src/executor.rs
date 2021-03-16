use futures_core::future::BoxFuture;
use sqlx_core::{database::Database, error::Error, executor::Executor};

pub trait ExecutorExt<'c, 'q, 'async_trait, T>: Executor<'c>
where
    'c: 'async_trait,
    'q: 'async_trait,
    T: 'async_trait,
    T: From<<Self::Database as Database>::Row>,
    Self: Sync + 'async_trait,
{
    fn execute(self, sql: &'q str) -> BoxFuture<'async_trait, Result<(), Error>> {
        Box::pin(async move { Executor::<'c>::execute(self, sql).await.map(|_| ()) })
    }

    fn fetch_all(self, sql: &'q str) -> BoxFuture<'async_trait, Result<Vec<T>, Error>> {
        Box::pin(async move {
            Executor::<'c>::fetch_all(self, sql)
                .await
                .map(|rows| rows.into_iter().map(|row| row.into()).collect())
        })
    }

    fn fetch_one(self, sql: &'q str) -> BoxFuture<'async_trait, Result<T, Error>> {
        Box::pin(async move {
            Executor::<'c>::fetch_one(self, sql)
                .await
                .map(|row| row.into())
        })
    }

    fn fetch_optional(self, sql: &'q str) -> BoxFuture<'async_trait, Result<Option<T>, Error>> {
        Box::pin(async move {
            Executor::<'c>::fetch_optional(self, sql)
                .await
                .map(|option_row| option_row.map(|row| row.into()))
        })
    }
}
