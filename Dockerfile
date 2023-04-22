FROM rust:1.68-slim-buster as builder
WORKDIR /app/src

# Force crates.io init for better docker caching
COPY docker/caching.rs src/main.rs
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo build --release

COPY . .
RUN cargo build --release

EXPOSE 3100
ENTRYPOINT [ "/app/src/target/release/agartex-authentication" ]
