# Examples

Code examples demonstrating current library usage.

## Running Examples

```bash
# Basic usage with caching demonstration
cargo run --example simple_usage

# Test specific validator identity extraction
cargo run --example test_validator_ids

# Demonstrate improved error handling and retry logic
cargo run --example error_handling_demo
```

## Example Descriptions

### `simple_usage.rs`
Demonstrates:
- Fetching all validators from mainnet
- Filtering by website/verification status
- Basic caching implementation
- Error handling patterns

### `test_validator_ids.rs`
Validates:
- Known validator public key extraction
- Identity key accuracy
- Debugging failed extractions

### `error_handling_demo.rs`
Shows:
- Improved error categorization
- Retry logic with backoff
- Rate limit handling
- Retryable vs non-retryable errors

## Network Requirements

Examples use public RPC endpoints by default. To avoid rate limiting (HTTP 429), use a private RPC provider for production or frequent testing.

## Execution Time

Initial fetch: 2-5 seconds depending on network conditions. Cached operations: <100ms.