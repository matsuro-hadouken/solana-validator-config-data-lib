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
        echo "  ├─ Strict linting..."
        cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery
        echo "  ├─ Testing..."
        cargo test
        echo "  ├─ Documentation..."
        cargo doc --no-deps
        echo "  └─ Building..."
        cargo build
        echo "All quality checks passed!"
        ;;
    
    "benchmark")
        echo "Running performance benchmark..."
        cargo run --example performance_benchmark
        ;;
    
    "error-demo")
        echo "Running error handling demonstration..."
        cargo run --example error_handling_demo
        ;;
    
    "doc")
        echo "Building library documentation..."
        cargo doc --no-deps --open
        ;;
    
    "full-check")
        echo "Running complete quality assurance..."
        echo "  ├─ Formatting..."
        cargo fmt --check
        echo "  ├─ Strict linting..."
        cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery
        echo "  ├─ Unit tests..."
        cargo test
        echo "  ├─ Documentation tests..."
        cargo test --doc
        echo "  ├─ Documentation build..."
        cargo doc --no-deps
        echo "  ├─ All examples..."
        cargo run --example simple_usage > /dev/null
        cargo run --example test_validator_ids > /dev/null
        cargo run --example error_handling_demo > /dev/null
        echo "  └─ Release build..."
        cargo build --release
        echo "Complete quality assurance passed!"
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
        echo "Usage: $0 {command}"
        echo ""
        echo "Development Commands:"
        echo "  run           - Run the example with info logging"
        echo "  example       - Run the simple integration example"
        echo "  test-ids      - Test identity extraction"
        echo "  error-demo    - Run error handling demonstration"
        echo "  validator-test- Quick test to verify known validators"
        echo "  test-rpc      - Test RPC endpoints"
        echo "  test          - Run all unit tests"
        echo "  benchmark     - Run performance benchmark"
        echo ""
        echo "Quality Assurance:"
        echo "  check         - Standard quality checks"
        echo "  full-check    - Complete quality assurance"
        echo "  doc           - Build and open documentation"
        echo ""
        echo "Build Commands:"
        echo "  build         - Debug build"
        echo "  release       - Optimized release build"
        echo "  clean         - Clean build artifacts"
        echo "  help          - Show this help message"
        echo ""
        echo "Quick Start:"
        echo "  ./dev.sh example         # Run integration example"
        echo "  ./dev.sh full-check      # Complete quality assurance"
        echo "  ./dev.sh benchmark       # Performance testing"
        ;;
esac