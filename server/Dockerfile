FROM rust as cacher
WORKDIR /app
RUN rustup component add rustfmt
COPY Cargo.toml .
COPY Cargo.lock .
COPY src/bin/noop.rs src/bin/noop.rs
RUN cargo build --release --bin noop

FROM cacher as builder
WORKDIR /app
RUN rm -rf src
COPY build.rs .
COPY proto proto
COPY src src
RUN cargo build --release --bin app

FROM rust as runtime
WORKDIR /app
COPY --from=builder /app/target/release/app .
ENV RUST_LOG "app=info,pingquery=trace"
ENTRYPOINT ["./app"]
