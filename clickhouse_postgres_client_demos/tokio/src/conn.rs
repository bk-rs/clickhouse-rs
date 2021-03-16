/*
cargo run -p clickhouse-postgres-client-demo-tokio --bin conn postgres://default:xxx@127.0.0.1:9005
*/

use std::{env, error};

use clickhouse_postgres_client::{connect, execute, fetch_all};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let database_url = env::args().nth(1).unwrap_or_else(|| {
        env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://default:xxx@127.0.0.1:9005".to_owned())
    });

    let mut conn = connect(&database_url).await?;

    execute("use default", &mut conn).await?;

    let rows = fetch_all("show databases", &mut conn).await?;
    for row in rows.iter() {
        println!("data: {:?}", row.try_get_data());
    }

    println!("done");

    Ok(())
}
