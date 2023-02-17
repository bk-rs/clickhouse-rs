## Examples

* [ClickHouse Postgres Client - Connection](demos/postgres_client/src/conn.rs)
* [ClickHouse Postgres Client - Pool](demos/postgres_client/src/pool.rs)

* [ClickHouse HTTP Client - Simple](demos/http_client/src/main.rs)
* [ClickHouse HTTP Client - CURD](clickhouse-http-client/tests/integration_tests/curd.rs)

## Dev

```
cargo clippy -p clickhouse-data-type -p clickhouse-data-value -p clickhouse-format -p clickhouse-http-client --all-features --tests -- -D clippy::all
cargo +nightly clippy -p clickhouse-data-type -p clickhouse-data-value -p clickhouse-format -p clickhouse-http-client --all-features --tests -- -D clippy::all

cargo clippy -p clickhouse-postgres-client --features _integration_tests -- -D clippy::all
cargo +nightly clippy -p clickhouse-postgres-client --features _integration_tests -- -D clippy::all

cargo clippy -p sqlx-clickhouse-ext --features postgres,all-types,runtime-tokio-native-tls --tests -- -D clippy::all
cargo +nightly clippy -p sqlx-clickhouse-ext --features postgres,all-types,runtime-tokio-native-tls --tests -- -D clippy::all



cargo fmt -- --check



cargo test -p clickhouse-data-type -p clickhouse-data-value -- --nocapture
cargo test -p clickhouse-format --features with-all -- --nocapture
cargo test -p clickhouse-http-client --features with-format-all -- --nocapture
cargo test -p clickhouse-postgres-client --features all-types,runtime-tokio-native-tls,num-bigint -- --nocapture
cargo test -p sqlx-clickhouse-ext --features postgres,all-types,runtime-tokio-native-tls -- --nocapture
```
