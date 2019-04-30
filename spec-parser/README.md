# spec-parser

Generates `model.rs` for `kafka-protocol` by parsing `Kafka` [protocol specifications](https://kafka.apache.org/protocol.html).

## Usage

To generate `../kafka-protocol/src/model.rs` based on the latest specs:

``` shell
cargo run
```

To change output:

``` shell
cargo run -- -o /some/path/model.rs
cargo run -- -o -
```

For more options see:

``` shell
cargo run -- --help
```

## Tests

Download the latest Kafka protocol specs before running the tests:

``` shell
wget -P src/ https://kafka.apache.org/protocol.html

cargo test
```
