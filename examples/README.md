# Examples

Code examples demonstrating library usage.

## Running Examples

```bash
# Basic usage with caching demonstration
cargo run --example simple_usage

# Test specific validator identity extraction
cargo run --example test_validator_ids
```

## Example Descriptions

### `simple_usage.rs`
Demonstrates core functionality:
- Fetching all validators from mainnet
- Filtering by website/verification status
- Basic caching implementation
- Error handling patterns

### `test_validator_ids.rs`
Validates validator identity extraction:
- Tests known validator public keys
- Verifies identity key accuracy
- Debugging failed extractions

## Network Requirements

Examples use public RPC endpoints by default. Production applications should use private RPC providers to avoid rate limiting.

## Execution Time

Initial fetch: 2-5 seconds depending on network conditions.
Cached operations: <100ms.