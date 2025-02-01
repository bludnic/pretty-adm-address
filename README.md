# ADAMANT short address generator

## Requirements

- Rust

## Build

Build release version for max performance.

```shell
cargo update
cargo build --release
```

## Run

1. Create an empty file: `accounts.csv`
2. Run the binary: `./target/release/pretty-adm-address`
3. Wait for new accounts

By default it saves addresses with <=12 digits. It can take days to find a single address.
