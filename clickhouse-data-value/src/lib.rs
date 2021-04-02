#[cfg(feature = "with-datetime")]
#[macro_use]
extern crate pest_derive;

#[cfg(feature = "with-date")]
pub mod date;
#[cfg(feature = "with-datetime")]
pub mod datetime;
#[cfg(feature = "with-datetime64")]
pub mod datetime64;

#[cfg(feature = "with-datetime")]
// 2105-12-31 23:59:59
pub(crate) const MAX_DATETIME_UNIX_TIMESTAMP: u64 = 4291718399;
