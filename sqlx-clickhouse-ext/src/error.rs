use std::any::type_name;

use sqlx_core::{database::Database, error::BoxDynError, type_info::TypeInfo as _, types::Type};

// https://github.com/launchbadge/sqlx/blob/v0.5.1/sqlx-core/src/error.rs#L136
pub(crate) fn mismatched_types<DB: Database, T: Type<DB>>(ty: &DB::TypeInfo) -> BoxDynError {
    // TODO: `#name` only produces `TINYINT` but perhaps we want to show `TINYINT(1)`
    format!(
        "mismatched types; Rust type `{}` (as SQL type `{}`) is not compatible with SQL type `{}`",
        type_name::<T>(),
        T::type_info().name(),
        ty.name()
    )
    .into()
}
