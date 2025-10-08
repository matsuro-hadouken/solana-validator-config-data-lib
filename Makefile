# Makefile for Solana Validator Config Library

.PHONY: help build test example clean check release

help:
	@echo "🚀 Solana Validator Config Library"
	@echo "=================================="
	@echo "Available commands:"
	@echo "  make build    - Build the library"
	@echo "  make test     - Run all tests"
	@echo "  make example  - Run simple example"
	@echo "  make check    - Run quality checks"
	@echo "  make clean    - Clean build artifacts"
	@echo ""
	@echo "💡 Use ./dev.sh for more development commands"

build:
	cargo build

test:
	cargo test

example:
	cargo run --example simple_usage

check:
	cargo fmt --check
	cargo clippy -- -D warnings
	cargo test

clean:
	cargo clean

release:
	cargo build --release
