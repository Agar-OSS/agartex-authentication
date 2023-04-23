# agartex-authentication
Authentication Service

## Runbook

To run locally from repository root use (Note that this requires that postgres is running)
```
PGUSER=<> PGPASSWORD=<> PGDATABASE=<> cargo run
```

To run tests use
```
cargo test
```

To run linting use
```
cargo clippy --all-targets --all-features --fix -- -D warnings
```

## Docker

Build docker image from repository root
```
docker build -t agaross.azurecr.io/agar-oss/agartex-authentication .
```