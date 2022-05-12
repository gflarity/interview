Interview
=========

## Install Rust

Install rustup:

```
curl https://sh.rustup.rs -sSf | sh
rustup install 1.34.0
rustup default 1.34.0
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
* `CONFIG_FILE`, location of the main app config file to load. Defaults to `config.yaml`.
* `REDIS_SECRETS_FILE`, location of a "secrets" file for Redis. Defaults to `redis.yaml`.
* `STARTUP_WAIT_MILLIS`, amount of time to wait before the app begins serving traffic.
  Defaults to `30000`.

There are secrets in `redis.yaml`, which should be overwritten but not committed. This can
lead into discussions about secrets management.

