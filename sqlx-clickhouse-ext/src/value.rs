pub use sqlx_core::{
    database::{Database, HasValueRef},
    decode::Decode,
    error::Error,
    type_info::TypeInfo as _,
    types::Type,
    value::ValueRef as _,
};

use crate::error::mismatched_types;

pub trait ValueRefTryGet<'r, DB, T>
where
    DB: Database,
    T: Decode<'r, DB> + Type<DB>,
{
    fn try_get(self) -> Result<T, Error>;
}

impl<'r, DB, T> ValueRefTryGet<'r, DB, T> for (<DB as HasValueRef<'r>>::ValueRef, usize)
where
    DB: Database,
    T: Decode<'r, DB> + Type<DB>,
{
    fn try_get(self) -> Result<T, Error> {
        let (value, index) = self;

        // https://github.com/launchbadge/sqlx/blob/v0.5.1/sqlx-core/src/row.rs#L111
        if !value.is_null() {
            let ty = value.type_info();

            if !ty.is_null() && !T::compatible(&ty) {
                return Err(Error::ColumnDecode {
                    index: format!("{index:?}"),
                    source: mismatched_types::<DB, T>(&ty),
                });
            }
        }

        T::decode(value).map_err(|source| Error::ColumnDecode {
            index: format!("{index:?}"),
            source,
        })
    }
}
