.EXPORT_ALL_VARIABLES:

DATABASE_URL := sqlite:db

watch:
	cargo watch -x test
watch-doc:
	cargo watch -s 'cargo doc --no-deps --all-features --document-private-items'
doc:
	cargo doc --no-deps --all-features --document-private-items --open
bench:
	cargo bench --all-features
test:
	cargo test --all-features
test-fuzz:
	ARBTEST_BUDGET_MS=6000000 cargo test --all-features --release
check:
	cargo check --workspace
build:
	cargo build --workspace --timings
build-static:
	DATABASE_URL=sqlite:db \
	TARGET_CC=x86_64-linux-musl-gcc \
	OPENSSL_DIR=/usr/local/opt/openssl@3 \
	RUSTFLAGS="-C linker=x86_64-linux-musl-gcc" \
	cargo build --workspace --target=x86_64-unknown-linux-musl --timings
server:
	cargo run -p whatlang -- serve ./db 8080
db:
	rm -f db
	cat db.sql | sqlite3 db
