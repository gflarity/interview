FROM rust:1.42.0-stretch as build

WORKDIR /
COPY . .

# If we wanted to use a specific toolchain version,
# RUN rustup toolchain install 1.36.0
# RUN rustup default 1.36.0

# If we build on an OS that doesn't support dynamically linked libs,
#RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build

RUN cargo build
# RUN mv target/debug/interview /interview

FROM ubuntu:20.04
WORKDIR /
COPY --from=build target/debug/interview .
COPY --from=build config.yaml .
COPY --from=build redis.yaml .

ENTRYPOINT /interview
