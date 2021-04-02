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
pub(crate) const MAX_DATETIME_UNIX_TIMESTAMP: u64 = 4291718400;
