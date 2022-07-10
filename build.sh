cat .env | grep POSTGRES_DB | export
cargo build
cargo run