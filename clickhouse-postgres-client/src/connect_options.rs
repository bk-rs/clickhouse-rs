use core::{
    ops::{Deref, DerefMut},
    str::FromStr,
};
use std::env::{VarError, set_var, var};

use sqlx::postgres::PgConnectOptions;
use sqlx_clickhouse_ext::sqlx_core::error::Error;
use url::Url;

#[derive(Debug, Clone)]
pub struct ClickhousePgConnectOptions {
    pub(crate) inner: PgConnectOptions,
}
impl ClickhousePgConnectOptions {
    pub fn new() -> Self {
        update_env();

        Self {
            inner: PgConnectOptions::new(),
        }
    }

    pub fn into_inner(self) -> PgConnectOptions {
        self.inner
    }
}

impl Default for ClickhousePgConnectOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for ClickhousePgConnectOptions {
    type Target = PgConnectOptions;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ClickhousePgConnectOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl FromStr for ClickhousePgConnectOptions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        update_env();

        let s = update_url(s)?;

        PgConnectOptions::from_str(&s).map(|inner| Self { inner })
    }
}

//
const PORT_DEFAULT_STR: &str = "9005";
const SSL_MODE_PREFER: &str = "prefer";
const SSL_MODE_DISABLE: &str = "disable";

fn update_env() {
    if let Err(VarError::NotPresent) = var("PGPORT") {
        unsafe { set_var("PGPORT", PORT_DEFAULT_STR) }
    }

    match var("PGSSLMODE") {
        Ok(str) if str == SSL_MODE_PREFER => unsafe { set_var("PGSSLMODE", SSL_MODE_DISABLE) },
        Err(VarError::NotPresent) => unsafe { set_var("PGSSLMODE", SSL_MODE_DISABLE) },
        _ => (),
    }
}

fn update_url(s: &str) -> Result<String, Error> {
    let mut url: Url = s
        .parse()
        .map_err(|err: url::ParseError| Error::Configuration(err.into()))?;

    url.query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<Vec<_>>()
        .into_iter()
        .fold(url.query_pairs_mut().clear(), |ser, (key, value)| {
            match key.as_ref() {
                "sslmode" => {
                    if value == SSL_MODE_PREFER {
                        ser.append_pair(&key, SSL_MODE_DISABLE);
                    } else {
                        ser.append_pair(&key, &value);
                    }
                }
                "ssl-mode" => {
                    if value == SSL_MODE_PREFER {
                        ser.append_pair(&key, SSL_MODE_DISABLE);
                    } else {
                        ser.append_pair(&key, &value);
                    }
                }
                _ => {
                    ser.append_pair(&key, &value);
                }
            };
            ser
        });

    Ok(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::remove_var;

    #[test]
    fn test_update_env() {
        unsafe {
            remove_var("PGPORT");
        }
        unsafe {
            remove_var("PGSSLMODE");
        }
        update_env();
        assert_eq!(var("PGPORT").unwrap(), "9005");
        assert_eq!(var("PGSSLMODE").unwrap(), "disable");

        unsafe {
            remove_var("PGPORT");
        }
        unsafe {
            remove_var("PGSSLMODE");
        }
        unsafe {
            set_var("PGSSLMODE", "prefer");
        }
        update_env();
        assert_eq!(var("PGPORT").unwrap(), "9005");
        assert_eq!(var("PGSSLMODE").unwrap(), "disable");
    }

    #[test]
    fn test_update_url() {
        let uri = "postgres:///?sslmode=prefer";
        assert_eq!(update_url(uri).unwrap(), "postgres:///?sslmode=disable");

        let uri = "postgres:///?ssl-mode=prefer";
        assert_eq!(update_url(uri).unwrap(), "postgres:///?ssl-mode=disable");
    }
}
