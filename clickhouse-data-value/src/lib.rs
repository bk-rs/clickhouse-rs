pub(crate) mod date_and_time_parser;

pub mod date;
pub mod datetime;
pub mod datetime64;

// 2105-12-31 23:59:59
pub(crate) const MAX_DATETIME_UNIX_TIMESTAMP: u64 = 4291718399;
