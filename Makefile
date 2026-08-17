.PHONY: build dev check check-all test fmt run clean fix

build:
	cargo build --release

dev:
	RUST_LOG=debug cargo run

check:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings
	cargo test

check-all:
	cargo fmt --all -- --check
	cargo clippy --workspace -- -D warnings
	cargo test --workspace

test:
	cargo test

fmt:
	cargo fmt --all

run:
	cargo run --release

fix:
	cargo fmt --all
	cargo clippy --fix --allow-dirty --allow-staged

clean:
	cargo clean
