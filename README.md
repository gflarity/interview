Interview
=========

## Install

Install rustup:

```
curl https://sh.rustup.rs -sSf | sh
```

We need to use an old version of Rust:

```
rustup toolchain install 1.36.0
rustup toolchain default 1.36.0
```

Useful commands:

`rustup show`

## Build the binary

```
# Fast build & run
cargo run

# Debug binary (fast)
cargo build
./target/debug/interview

# Properly optimized release build (slow)
cargo build --release
./target/release/interview
```

To deal with dependencies independently of the app,

```
# Pull dependencies (but do not build them)
cargo fetch

# Or,
# Install cargo extension for independent building of deps
cargo install cargo-build-deps

# Then
RUN cargo build-deps
RUN cargo build

# Or,
RUN cargo build-deps  --release
RUN cargo build  --release
```

## Configuration

The `config.yaml` file contains much of the application config. Some other configurations
are passed as the following environment variables:

* `REGION`, the region in which we're running our app, defaults to `local`
* `ENVIRONMENT`, the named environment our application runs in, defaults to `development`

There are secrets in `redis.yaml`, which should be overwritten but not committed.

