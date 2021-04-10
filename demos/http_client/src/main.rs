/*
cargo run -p clickhouse-demo-http-client -- "http://127.0.0.1:8123" default xxx system
*/

use std::{env, error, time::Duration};

use clickhouse_http_client::{
    clickhouse_format::output::JsonCompactEachRowWithNamesAndTypesOutput,
    isahc::config::Configurable, ClientBuilder,
};
use futures_executor::block_on;
use serde::Deserialize;

fn main() -> Result<(), Box<dyn error::Error>> {
    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let url = env::args().nth(1);
    let username = env::args().nth(2);
    let password = env::args().nth(3);
    let database = env::args().nth(4);

    let mut client_builder = ClientBuilder::new()
        .configurable(|http_client_builder| http_client_builder.timeout(Duration::from_secs(1)));
    if let Some(url) = url {
        client_builder.set_url(url)?;
    }
    if let Some(username) = username {
        client_builder.set_username_to_header(username)?;
    }
    if let Some(password) = password {
        client_builder.set_password_to_header(password)?;
    }
    if let Some(database) = database {
        client_builder.set_database_to_header(database)?;
    }
    let client = client_builder.build()?;

    let (databases, _) = client
        .select_with_format(
            "show databases",
            JsonCompactEachRowWithNamesAndTypesOutput::<Database>::new(),
            None,
        )
        .await?;

    println!("databases: {:?}", databases);

    println!("done");

    Ok(())
}

#[derive(Deserialize, Debug)]
struct Database {
    name: String,
}
