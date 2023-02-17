# sqlx-clickhouse-ext

* [Cargo package](https://crates.io/crates/sqlx-clickhouse-ext)

## Dev

```
cargo clippy -p sqlx-clickhouse-ext --features postgres,all-types,runtime-tokio-native-tls --tests -- -D clippy::all
cargo +nightly clippy -p sqlx-clickhouse-ext --features postgres,all-types,runtime-tokio-native-tls --tests -- -D clippy::all

cargo fmt -p sqlx-clickhouse-ext -- --check

cargo test -p sqlx-clickhouse-ext --features postgres,all-types,runtime-tokio-native-tls -- --nocapture
```

```
cargo publish --features runtime-tokio-native-tls --dry-run
```
