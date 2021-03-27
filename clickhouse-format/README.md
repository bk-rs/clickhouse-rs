# clickhouse-format

* [ClickHouse Doc](https://clickhouse.tech/docs/en/interfaces/formats/)
* [Cargo package](https://crates.io/crates/clickhouse-format)

## Dev

```
cargo clippy -p clickhouse-format --all-features -- -D clippy::all
cargo +nightly clippy -p clickhouse-format --all-features -- -D clippy::all

cargo fmt -p clickhouse-format -- --check

cargo build-all-features -p clickhouse-format
cargo test-all-features -p clickhouse-format -- --nocapture
```

```
cargo publish --dry-run
```
