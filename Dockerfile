FROM rust:1.68-slim-buster as builder
WORKDIR /app/src

# Force crates.io init for better docker caching
RUN cargo search --limit 0

COPY . .
RUN cargo build --release

EXPOSE 3100
ENTRYPOINT [ "/app/src/target/release/agartex-authentication" ]
