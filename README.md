## Setup & Building
```bash
cargo install cargo-watch
cargo build
```

## Run server locally (Manually)
```bash
cargo watch -q -c -w src/ -w assets/ -x run
```

## Run server locally (Docker)
```bash
docker compose build
docker compose up
```

visit http://localhost:3000
