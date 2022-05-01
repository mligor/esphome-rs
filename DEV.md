# Generate `api.rs`

- download latest version of `api.proto` and `api_options.proto` from https://github.com/esphome/esphome/tree/dev/esphome/components/api

- clone and build release version of `rust-protobuf` (https://github.com/stepancheg/rust-protobuf) in the same root folder where `esphome-rs` is located:

```shell
git clone https://github.com/stepancheg/rust-protobuf.git
cd rust-protobuf
cargo build -r
```

- regenerate api.rs from `esphome-rs/src` folder:

```shell
protoc  --plugin=protoc-gen-rust=../../rust-protobuf/target/release/protoc-gen-rust --rust_out=./  api.proto
```
