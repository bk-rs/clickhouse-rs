# clickhouse-data-type

* [ClickHouse Doc](https://clickhouse.tech/docs/en/sql-reference/data-types/)
* [Cargo package](https://crates.io/crates/clickhouse-data-type)

## Dev

```
cargo clippy -p clickhouse-data-type --all-features -- -D clippy::all
cargo +nightly clippy -p clickhouse-data-type --all-features -- -D clippy::all

cargo fmt -p clickhouse-data-type -- --check

cargo build-all-features -p clickhouse-data-type
cargo test-all-features -p clickhouse-data-type -- --nocapture
```
