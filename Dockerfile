FROM rust:1.42.0-stretch

WORKDIR /

COPY . .

# RUN rustup toolchain install 1.36.0
# RUN rustup default 1.36.0

#RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build
RUN cargo build
RUN mv target/debug/interview /interview

ENTRYPOINT ./interview
