use std::{env, error, time::Duration};

use clickhouse_http_client::{
    isahc::config::Configurable as _, Client, ClientBuilder, ClientConfigLocation,
};

pub(super) fn get_client() -> Result<Client, Box<dyn error::Error>> {
    let mut client_builder = ClientBuilder::new()
        .configurable(|http_client_builder| http_client_builder.timeout(Duration::from_secs(1)));
    if let Ok(http_url) = env::var("CLICKHOUSE_HTTP_URL") {
        client_builder.set_url(http_url).unwrap();
    }
    client_builder.set_credentials("default", "xxx".to_owned(), ClientConfigLocation::Header);

    Ok(client_builder.build()?)
}

pub(super) fn get_anonymous_client() -> Result<Client, Box<dyn error::Error>> {
    let mut client_builder = ClientBuilder::new()
        .configurable(|http_client_builder| http_client_builder.timeout(Duration::from_secs(1)));
    if let Ok(http_url) = env::var("CLICKHOUSE_HTTP_URL") {
        client_builder.set_url(http_url).unwrap();
    }

    Ok(client_builder.build()?)
}

pub(super) fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}
