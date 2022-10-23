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
