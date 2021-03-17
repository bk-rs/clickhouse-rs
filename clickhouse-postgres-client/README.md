# clickhouse-postgres-client

* [Cargo package](https://crates.io/crates/clickhouse-postgres-client)

## Dev

```
cargo clippy -p clickhouse-postgres-client --features _integration_tests -- -D clippy::all
cargo +nightly clippy -p clickhouse-postgres-client --features _integration_tests -- -D clippy::all

cargo fmt -p clickhouse-postgres-client -- --check

cargo test -p clickhouse-postgres-client --features all-types,runtime-tokio-native-tls,num-bigint -- --nocapture
```

```
cargo publish --features runtime-tokio-native-tls --dry-run
```
