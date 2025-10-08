#!/bin/bash

# Solana Validator Config Library - Development Helper Script
# This script provides convenient shortcuts for common development tasks

set -e

echo "=============================================="
echo "Solana Validator Config Library - Dev Tools"
echo "=============================================="

case "${1:-help}" in
    "build")
        echo "Building library..."
        cargo build
        echo "Build completed successfully!"
        ;;
    
    "test")
        echo "Running tests..."
        cargo test
        echo "All tests passed!"
        ;;
    
    "example"|"simple")
        echo "Running simple integration example..."
        cargo run --example simple_usage
        ;;
    
    "test-ids")
        echo "Testing validator identity extraction..."
        cargo run --example test_validator_ids
        ;;
    
    "run")
        echo "Running simple example with info logging..."
        RUST_LOG=info cargo run --example simple_usage
        ;;
    
    "test-rpc")
        echo "Testing RPC endpoints..."
        echo ""
        echo "1. Testing with Mainnet public RPC (may be rate limited)..."
        RUST_LOG=warn timeout 30s cargo run --example simple_usage || echo "Public RPC test completed (may have failed due to rate limits)"
        echo ""
        echo "RPC testing completed!"
        ;;
    
    "clean")
        echo "Cleaning build artifacts..."
        cargo clean
        echo "Clean completed!"
        ;;
    
    "check")
        echo "Running comprehensive quality checks..."
        echo "  ├─ Formatting..."
        cargo fmt --check
        echo "  ├─ Linting..."
        cargo clippy -- -D warnings
        echo "  ├─ Testing..."
        cargo test
        echo "  └─ Building..."
        cargo build
        echo "All quality checks passed!"
        ;;
    
    "benchmark")
        echo "Running quick benchmark..."
        echo "RUST_LOG=info cargo run --example simple_usage --release"
        time RUST_LOG=info cargo run --example simple_usage --release
        ;;
    
    "release")
        echo "Building release version..."
        cargo build --release
        echo "Release build completed!"
        ;;
    
    "validator-test")
        echo "Quick validator identity test..."
        echo "Looking for test validators (including your specified one)..."
        cargo run --example test_validator_ids | grep -E "(Found|Total|Extracted|Not found)"
        ;;
    
    "help"|*)
        echo "Solana Validator Config Development Tool"
        echo "Usage: $0 {run|example|test-ids|test-rpc|test|check|benchmark|release|validator-test|clean|help}"
        echo ""
        echo "Commands:"
        echo "  run           - Run the example with info logging"
        echo "  example       - Run the simple integration example"
        echo "  test-ids      - Test validator identity extraction"
        echo "  validator-test- Quick test to verify known validators"
        echo "  test-rpc      - Test RPC endpoints"
        echo "  test          - Run all unit tests"
        echo "  check         - Run comprehensive quality checks"
        echo "  benchmark     - Run quick performance benchmark"
        echo "  release       - Build optimized release version"
        echo "  clean         - Clean build artifacts"
        echo "  help          - Show this help message"
        echo ""
        echo "Library Usage:"
        echo "  This is now a clean library that returns ValidatorInfo structs"
        echo "  with validator_identity field containing actual validator keys!"
        echo ""
        echo "Quick Test:"
        echo "  ./dev.sh validator-test  # Test validator identity extraction with your specified key"
        echo "  ./dev.sh example         # Run full integration example"
        echo "  ./dev.sh check           # Quality assurance"
        ;;
esac