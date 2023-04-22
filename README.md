# agartex-authentication
Authentication Service

## Runbook

To run locally from repository root use

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

Build docker image from repository root
```
docker build .
```