# Solana Validator Config Library

Rust library for extracting validator configuration data from Solana RPC endpoints.

[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](#)
[![Solana](https://img.shields.io/badge/Solana-9945FF?style=flat&logo=solana&logoColor=white)](#)

## Features

- Extracts validator identity keys and metadata from program accounts
- Direct RPC calls without Solana SDK dependency
- Supports mainnet, testnet, devnet, and custom RPC endpoints
- Input validation and error handling
- Configurable timeout and concurrency limits

## Quick Start

### Add to your Cargo.toml:
```toml
[dependencies]
solana-validator-config = { git = "https://github.com/matsuro-hadouken/solana-validator-config-data-lib" }
tokio = { version = "1.0", features = ["full"] }
```

### Implementation:
```rust
use solana_validator_config::{ValidatorConfigClient, SolanaNetwork};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
    let validators = client.fetch_all_validators().await?;
    
    println!("Found {} validators", validators.len());
    
    for validator in validators.iter().take(3) {
        if let Some(name) = &validator.name {
            if let Some(identity) = &validator.validator_identity {
                println!("• {} → {}", name, identity);
            }
        }
    }
    
    Ok(())
}
```

### Execution:
```bash
cargo run
```

**Output:**
```
Found 3265 validators
• ART3MIS.CLOUD → GwHH8ciFhR8vejWCqmg8FWZUCNtubPY2esALvy5tBvji
• Farben → farbZXR7aBQSMCYiUXzoS4pRUsvuCZ38f6AXMXiKACf  
```

## Data Structure

```rust
pub struct ValidatorInfo {
    pub validator_identity: Option<String>,   // Validator identity key
    pub name: Option<String>,                 // Display name  
    pub website: Option<String>,              // Website URL
    pub details: Option<String>,              // Description
    pub keybase_username: Option<String>,     // Keybase identity
}
```

## Development

```bash
./dev.sh help           # Show all commands
./dev.sh example        # Run quick example
./dev.sh validator-test # Test specific validators
./dev.sh test           # Run all tests  
./dev.sh check          # Full quality check
./dev.sh benchmark      # Performance test
```

## Configuration

### Custom RPC endpoints:

```rust
// Private RPC
let client = ValidatorConfigClient::new_custom(
    "https://your-endpoint.quiknode.pro/token/"
);

// Alchemy
let client = ValidatorConfigClient::new_custom(
    "https://solana-mainnet.g.alchemy.com/v2/your-api-key"
);

// Helius
let client = ValidatorConfigClient::new_custom(
    "https://rpc.helius.xyz/?api-key=your-api-key"
);
```

### Networks:

```rust
// Mainnet
let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);

let client = ValidatorConfigClient::new(SolanaNetwork::Testnet);

// Devnet
let client = ValidatorConfigClient::new(SolanaNetwork::Devnet);
```

## Usage

### Statistics:
```rust
let stats = client.get_validator_stats().await?;
println!("Total: {} | With names: {} | With websites: {}", 
    stats.total_validators, stats.with_names, stats.with_websites);
```

### Filtering:
```rust
let validators = client.fetch_all_validators().await?;

// Named validators only
let named_validators: Vec<_> = validators
    .iter()
    .filter(|v| v.name.is_some())
    .collect();

// Search by name
let art3mis = validators
    .iter()
    .find(|v| v.name.as_ref().map_or(false, |n| n.contains("ART3MIS")));
```

### Methods:
```rust
// Check if validator has configuration
if validator.has_config() {
    println!("Validator has metadata");
}

// Get display name with fallback
let display_name = validator.display_name();
let description = validator.display_description();
```

## Performance

- Fetches 3200+ validators from mainnet in 2-5 seconds
- No Solana SDK dependency
- Handles corrupted data
- Memory efficient processing
- Consider caching for frequent calls

## Error Handling

- Input sanitization (500 char limits)
- Null byte replacement
- Control character filtering
- Malformed JSON parsing
- UTF-8 validation
- Multiple parse attempts for corrupted data

## Examples

```bash
./dev.sh example        # Simple usage example
./dev.sh test-ids       # Test validator identity extraction  
./dev.sh validator-test # Quick validation test
```

Or:
```bash
cargo run --example simple_usage
cargo run --example test_validator_ids
```

## Contributing

1. Fork
2. Feature branch
3. Run `./dev.sh check`
4. Submit PR

## License

MIT License
