use sqlx_clickhouse_ext::{
    sqlx_core::{
        column::Column as _, column::ColumnIndex, error::Error, postgres::PgRow, row::Row as _,
        type_info::TypeInfo as _, value::ValueRef as _,
    },
    value::ValueRefTryGet as _,
};

use crate::type_info::{ClickhousePgType, ClickhousePgValue};

pub struct ClickhousePgRow {
    inner: PgRow,
}
impl From<PgRow> for ClickhousePgRow {
    fn from(row: PgRow) -> Self {
        Self { inner: row }
    }
}
impl ClickhousePgRow {
    pub fn try_get_data(&self) -> Result<Vec<(&str, ClickhousePgValue)>, Error> {
        let mut array = vec![];
        for (i, column) in self.inner.columns().iter().enumerate() {
            array.push((column.name(), self.try_get_value(i)?));
        }
        Ok(array)
    }

    pub fn try_get_value<I>(&self, index: I) -> Result<ClickhousePgValue, Error>
    where
        I: ColumnIndex<PgRow>,
    {
        let index = index.index(&self.inner)?;

        let value = self.inner.try_get_raw(index)?;

        let cpt = ClickhousePgType::try_from((value.type_info().name(), index))?;

        match cpt {
            ClickhousePgType::Char => (value, index).try_get().map(ClickhousePgValue::Char),
            ClickhousePgType::Int2 => (value, index).try_get().map(ClickhousePgValue::I16),
            ClickhousePgType::Int4 => (value, index).try_get().map(ClickhousePgValue::I32),
            ClickhousePgType::Int8 => (value, index).try_get().map(ClickhousePgValue::I64),
            ClickhousePgType::Float4 => (value, index).try_get().map(ClickhousePgValue::F32),
            ClickhousePgType::Float8 => (value, index).try_get().map(ClickhousePgValue::F64),
            ClickhousePgType::Varchar => (value, index).try_get().map(ClickhousePgValue::String),
            #[cfg(feature = "chrono")]
            ClickhousePgType::Date => (value, index).try_get().map(ClickhousePgValue::NaiveDate),
            #[cfg(feature = "bigdecimal")]
            ClickhousePgType::Numeric => {
                (value, index).try_get().map(ClickhousePgValue::BigDecimal)
            }
            #[cfg(feature = "uuid")]
            ClickhousePgType::Uuid => (value, index).try_get().map(ClickhousePgValue::Uuid),
        }
    }
}
