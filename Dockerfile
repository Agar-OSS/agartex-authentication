FROM rust:1.68-slim-buster as builder
WORKDIR /app/src

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

# Force crates.io init for better docker caching
COPY docker/caching.rs src/main.rs
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM debian:10.13-slim

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN chmod 777 .

RUN useradd user
USER user

COPY --from=builder /app/src/target/release/agartex-authentication .

EXPOSE 3100
ENTRYPOINT [ "./agartex-authentication" ]
