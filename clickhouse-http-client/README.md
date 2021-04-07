# clickhouse-http-client

* [ClickHouse Doc](https://clickhouse.tech/docs/en/interfaces/http/)
* [Cargo package](https://crates.io/crates/clickhouse-http-client)

## Dev

```
cargo clippy -p clickhouse-http-client --all-features -- -D clippy::all
cargo +nightly clippy -p clickhouse-http-client --all-features -- -D clippy::all

cargo fmt -p clickhouse-http-client -- --check

cargo build-all-features -p clickhouse-http-client
cargo test-all-features -p clickhouse-http-client -- --nocapture
```
