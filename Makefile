.PHONY: fmt clippy test run ci

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all-features

run:
	cargo run

ci: fmt clippy test
