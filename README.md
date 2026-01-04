## Examples

* [ClickHouse Postgres Client - Connection](demos/postgres_client/src/conn.rs)
* [ClickHouse Postgres Client - Pool](demos/postgres_client/src/pool.rs)

* [ClickHouse HTTP Client - Simple](demos/http_client/src/main.rs)
* [ClickHouse HTTP Client - CURD](clickhouse-http-client/tests/integration_tests/curd.rs)

## Dev

```
cargo clippy -p clickhouse-data-type -p clickhouse-data-value -p clickhouse-format -p clickhouse-http-client -p sqlx-clickhouse-ext --all-features --tests -- -D clippy::all
cargo +nightly clippy -p clickhouse-data-type -p clickhouse-data-value -p clickhouse-format -p clickhouse-http-client -p sqlx-clickhouse-ext --all-features --tests -- -D clippy::all

cargo clippy -p clickhouse-postgres-client --features _integration_tests --tests -- -D clippy::all
cargo +nightly clippy -p clickhouse-postgres-client --features _integration_tests --tests -- -D clippy::all


cargo fmt -- --check


cargo test -p clickhouse-data-type -p clickhouse-data-value -- --nocapture
cargo test -p clickhouse-format --features with-all -- --nocapture
cargo test -p clickhouse-http-client --features with-format-all -- --nocapture
cargo test -p clickhouse-postgres-client --features all-types,runtime-tokio,tls-rustls-aws-lc-rs,num-bigint -- --nocapture
cargo test -p sqlx-clickhouse-ext -- --nocapture



RUST_BACKTRACE=1 RUST_LOG=trace ./clickhouse-http-client/tests/run_integration_tests.sh
RUST_BACKTRACE=1 RUST_LOG=trace ./clickhouse-postgres-client/tests/run_integration_tests.sh
```

## Publish order

clickhouse-data-type clickhouse-data-value clickhouse-format

clickhouse-http-client

sqlx-clickhouse-ext

clickhouse-postgres-client
