# Makefile for Solana Validator Config Library

.PHONY: help build test example clean doc publish

help: ## Show this help message
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the library
	cargo build

test: ## Run all tests
	cargo test

example: ## Run the main example
	cargo run --bin validator-config-example

simple-example: ## Run the simple integration example
	cargo run --example simple_usage

doc: ## Generate and open documentation
	cargo doc --open

clean: ## Clean build artifacts
	cargo clean

check: ## Check code formatting and lints
	cargo fmt --check
	cargo clippy -- -D warnings

format: ## Format code
	cargo fmt

release: ## Build in release mode
	cargo build --release

publish: check test ## Publish to crates.io (dry run)
	cargo publish --dry-run

# Development helpers
watch: ## Watch for changes and rebuild
	cargo watch -x build

watch-test: ## Watch for changes and run tests
	cargo watch -x test