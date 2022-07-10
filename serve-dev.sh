export DATABASE_URL=postgres://rocket:rocket@localhost:1010/rocket
docker-compose -f serve-db.yaml up -d
cargo fmt
cargo build
cargo run