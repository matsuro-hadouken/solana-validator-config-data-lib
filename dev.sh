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
    
    "example")
        echo "Running main example..."
        cargo run --bin validator-config-example
        ;;
    
    "simple")
        echo "Running simple integration example..."
        cargo run --example simple_usage
        ;;
    
    "run")
        echo "Running default example with info logging..."
        RUST_LOG=info cargo run --bin validator-config-example
        ;;
    
    "run-private")
        if [ -z "$SOLANA_RPC_URL" ]; then
            echo "ERROR: SOLANA_RPC_URL environment variable not set"
            echo ""
            echo "To test with a private RPC endpoint, set the environment variable:"
            echo "  export SOLANA_RPC_URL='https://your-private-rpc.com'"
            echo "  ./dev.sh run-private"
            echo ""
            echo "Examples of private RPC providers:"
            echo "  QuickNode: https://your-endpoint.quiknode.pro/token/"
            echo "  Alchemy:   https://solana-mainnet.g.alchemy.com/v2/your-api-key"
            echo "  Helius:    https://rpc.helius.xyz/?api-key=your-api-key"
            echo "  GenesysGo: https://ssc-dao.genesysgo.net/"
            exit 1
        fi
        echo "Running example with private RPC endpoint: $SOLANA_RPC_URL"
        RUST_LOG=info cargo run --bin validator-config-example
        ;;
    
    "test-rpc")
        echo "Testing different RPC endpoints..."
        echo ""
        echo "1. Testing with Mainnet public RPC (may be rate limited)..."
        RUST_LOG=warn timeout 30s cargo run --example simple_usage || echo "Public RPC test completed (may have failed due to rate limits)"
        echo ""
        
        if [ -n "$SOLANA_RPC_URL" ]; then
            echo "2. Testing with your private RPC: $SOLANA_RPC_URL"
            RUST_LOG=warn timeout 30s cargo run --bin validator-config-example || echo "Private RPC test failed"
        else
            echo "2. Skipping private RPC test (SOLANA_RPC_URL not set)"
            echo "   To test private RPC: export SOLANA_RPC_URL='https://your-private-rpc.com'"
        fi
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
        echo "RUST_LOG=info cargo run --bin validator-config-example --release"
        time RUST_LOG=info cargo run --bin validator-config-example --release
        ;;
    
    "release")
        echo "Building release version..."
        cargo build --release
        echo "Release build completed!"
        ;;
    
    "install")
        echo "Installing as local binary..."
        cargo install --path .
        echo "Installation completed!"
        ;;
    
    "help"|*)
        echo "Solana Validator Config Development Tool"
        echo "Usage: $0 {run|run-private|test-rpc|test|check|benchmark|release|install|clean|help}"
        echo ""
        echo "Commands:"
        echo "  run        - Run the example with public RPC (for testing only)"
        echo "  run-private- Run the example with private RPC (requires SOLANA_RPC_URL)"
        echo "  test-rpc   - Test both public and private RPC endpoints"
        echo "  test       - Run all unit tests"
        echo "  check      - Run comprehensive quality checks"
        echo "  benchmark  - Run quick performance benchmark"
        echo "  release    - Build optimized release version"
        echo "  install    - Install as local binary"
        echo "  clean      - Clean build artifacts"
        echo "  help       - Show this help message"
        echo ""
        echo "Private RPC Examples:"
        echo "  export SOLANA_RPC_URL='https://your-endpoint.quiknode.pro/token/'"
        echo "  ./dev.sh run-private"
        echo ""
        echo "  export SOLANA_RPC_URL='https://solana-mainnet.g.alchemy.com/v2/your-api-key'"
        echo "  ./dev.sh test-rpc"
        echo ""
        echo "Production Usage:"
        echo "  ./dev.sh run          # Public RPC (testing only - rate limited)"
        echo "  ./dev.sh run-private  # Private RPC (recommended for production)"
        echo "  ./dev.sh check        # Quality assurance"
        echo "  ./dev.sh benchmark    # Performance test"
        ;;
esac