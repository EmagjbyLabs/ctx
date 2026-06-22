.PHONY: fmt clippy test run install ci

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all-features

run:
	cargo run

i install:
	cargo install --path .

ci: fmt clippy test
