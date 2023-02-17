# clickhouse-data-value

* [Cargo package](https://crates.io/crates/clickhouse-data-value)

## Dev

```
cargo clippy -p clickhouse-data-value --all-features --tests -- -D clippy::all
cargo +nightly clippy -p clickhouse-data-value --all-features --tests -- -D clippy::all

cargo fmt -p clickhouse-data-value -- --check

cargo test -p clickhouse-data-value -- --nocapture
```
