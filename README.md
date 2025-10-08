# 🚀 Solana Validator Config Library

> **Get actual validator identities and metadata directly from the Solana blockchain**

A clean, fast Rust library that fetches validator configuration data including **real validator identity keys** you can use to connect to validators. No SDK bloat, pure efficiency.

[![Made with ❤️](https://img.shields.io/badge/Made%20with-%E2%9D%A4%EF%B8%8F-red.svg)](#)
[![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)](#)
[![Solana](https://img.shields.io/badge/Solana-9945FF?style=flat&logo=solana&logoColor=white)](#)

## ✨ What Makes This Special

- **🎯 Real Validator Identities** - Returns actual validator identity keys you can connect to (not just config account addresses)
- **⚡ Pure Speed** - No Solana SDK dependency, just fast HTTP calls and math
- **🔒 Production Ready** - Input sanitization, private RPC support, comprehensive error handling  
- **🧹 Clean API** - Simple, type-safe interface that just works
- **🌐 Multi-Network** - Mainnet, Testnet, Devnet, or any custom RPC
- **🛡️ Battle Tested** - Handles corrupted data, malformed JSON, and edge cases gracefully

## 🚀 Quick Start (2 minutes)

### Add to your Cargo.toml:
```toml
[dependencies]
solana-validator-config = { git = "https://github.com/matsuro-hadouken/solana-validator-config-data-lib" }
tokio = { version = "1.0", features = ["full"] }
```

### Use it:
```rust
use solana_validator_config::{ValidatorConfigClient, SolanaNetwork};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use your private RPC in production
    let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
    let validators = client.fetch_all_validators().await?;
    
    println!("🎯 Found {} validators", validators.len());
    
    for validator in validators.iter().take(3) {
        if let Some(name) = &validator.name {
            if let Some(identity) = &validator.validator_identity {
                println!("• {} → {}", name, identity);
                println!("  ✅ This is the real validator key you can connect to!");
            }
        }
    }
    
    Ok(())
}
```

### Run it:
```bash
cargo run
```

**Expected output:**
```
🎯 Found 3265 validators
• ART3MIS.CLOUD ☘️ → GwHH8ciFhR8vejWCqmg8FWZUCNtubPY2esALvy5tBvji
  ✅ This is the real validator key you can connect to!
• Farben → farbZXR7aBQSMCYiUXzoS4pRUsvuCZ38f6AXMXiKACf  
  ✅ This is the real validator key you can connect to!
```

## 📋 What You Get

Each validator returns this clean data structure:

```rust
pub struct ValidatorInfo {
    pub validator_identity: Option<String>,   // 🎯 The actual validator identity key
    pub name: Option<String>,                 // 📝 Display name  
    pub website: Option<String>,              // 🌐 Website URL
    pub details: Option<String>,              // 📄 Description
    pub keybase_username: Option<String>,     // 🔐 Keybase identity
}
```

## 🔧 Development Tools

We include a powerful dev script for easy development:

```bash
./dev.sh help           # 📚 Show all commands
./dev.sh example        # 🚀 Run quick example
./dev.sh validator-test # 🧪 Test specific validators
./dev.sh test           # ✅ Run all tests  
./dev.sh check          # 🔍 Full quality check
./dev.sh benchmark      # ⚡ Performance test
```
        ## 🌐 Production Setup

### Private RPC Endpoints (Recommended)

```rust
// QuickNode - Fast & reliable
let client = ValidatorConfigClient::new_custom(
    "https://your-endpoint.quiknode.pro/token/"
);

// Alchemy - Great free tier  
let client = ValidatorConfigClient::new_custom(
    "https://solana-mainnet.g.alchemy.com/v2/your-api-key"
);

// Helius - High performance
let client = ValidatorConfigClient::new_custom(
    "https://rpc.helius.xyz/?api-key=your-api-key"
);
```

### Different Networks

```rust
// Mainnet (default)
let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);

// Testnet  
let client = ValidatorConfigClient::new(SolanaNetwork::Testnet);

// Devnet
let client = ValidatorConfigClient::new(SolanaNetwork::Devnet);
```

## 📊 Advanced Usage

### Get Statistics
```rust
let stats = client.get_validator_stats().await?;
println!("📈 Total: {} | With names: {} | With websites: {}", 
    stats.total_validators, stats.with_names, stats.with_websites);
```

### Filter & Search
```rust
let validators = client.fetch_all_validators().await?;

// Find validators with specific criteria
let named_validators: Vec<_> = validators
    .iter()
    .filter(|v| v.name.is_some())
    .collect();

// Search by name
let art3mis = validators
    .iter()
    .find(|v| v.name.as_ref().map_or(false, |n| n.contains("ART3MIS")));
```

### Helper Methods
```rust
// Check if validator has any configuration
if validator.has_config() {
    println!("📋 Validator has metadata");
}

// Get display name with fallback
let display_name = validator.display_name();
let description = validator.display_description();
```

## ⚡ Performance

- **Fetches 3200+ validators** from Mainnet in 2-5 seconds
- **Zero Solana SDK overhead** - pure HTTP + math
- **Handles corrupted data** gracefully  
- **Memory efficient** - processes data in chunks
- **Consider caching** results for frequent calls

## 🛡️ Security Features

- **Input sanitization** (500 char limits)
- **Null byte replacement** 
- **Control character filtering**
- **Unicode emoji support** ☘️
- **Malformed JSON handling**
- **UTF-8 validation**
- **Multiple JSON parsing attempts** for corrupted data

## 🧪 Examples

Check out the examples in the repo:

```bash
./dev.sh example        # Simple usage example
./dev.sh test-ids       # Test validator identity extraction  
./dev.sh validator-test # Quick validation test
```

Or run directly:
```bash
cargo run --example simple_usage
cargo run --example test_validator_ids
```

## 🤝 Contributing

1. Fork the repo
2. Create a feature branch
3. Run `./dev.sh check` before committing  
4. Submit a PR

## 📄 License

MIT License - build amazing things! 🚀

---

**Made with ❤️ for the Solana ecosystem**
```

## Development Tools

This library includes a convenient development script:

```bash
./dev.sh help           # Show all available commands
./dev.sh example        # Run the simple integration example  
./dev.sh test-ids       # Test validator identity extraction
./dev.sh validator-test # Quick test (just shows Farben & KitBull found)
./dev.sh test           # Run all unit tests
./dev.sh check          # Full quality checks (format, lint, test, build)
./dev.sh benchmark      # Performance test
./dev.sh clean          # Clean build artifacts
```

## Validator Data Structure

Each validator returns this information:

```rust
pub struct ValidatorInfo {
    pub validator_identity: Option<String>,      // Actual validator identity key
    pub name: Option<String>,                    // Display name
    pub website: Option<String>,                 // Website URL
    pub details: Option<String>,                 // Description/details
    pub keybase_username: Option<String>,        // Keybase identity
}
```

**Helper methods:**
- `validator.display_name()` - Gets name or keybase_username
- `validator.display_description()` - Gets details
- `validator.has_config()` - True if validator has any data

## Private RPC Endpoints

For production use, connect to private RPC providers:

```rust
// QuickNode
let client = ValidatorConfigClient::new_custom("https://your-endpoint.quiknode.pro/token/");

// Alchemy
let client = ValidatorConfigClient::new_custom("https://solana-mainnet.g.alchemy.com/v2/your-api-key");

// Helius
let client = ValidatorConfigClient::new_custom("https://rpc.helius.xyz/?api-key=your-api-key");

// Any custom RPC
let client = ValidatorConfigClient::new_custom("https://your-private-rpc.com");
```

## Different Networks

```rust
// Mainnet (default)
let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);

// Testnet
let client = ValidatorConfigClient::new(SolanaNetwork::Testnet);

// Devnet
let client = ValidatorConfigClient::new(SolanaNetwork::Devnet);

// Custom RPC
let client = ValidatorConfigClient::new_custom("https://your-rpc.com");
```

## Get Statistics

```rust
let stats = client.get_validator_stats().await?;
println!("Total validators: {}", stats.total_validators);
println!("Have names: {}", stats.with_names);
println!("Have websites: {}", stats.with_websites);
println!("Have Keybase: {}", stats.with_keybase);
```

## Filter Validators

```rust
let validators = client.fetch_all_validators().await?;

// Find validators with websites
let with_websites: Vec<_> = validators
    .iter()
    .filter(|v| v.website.is_some())
    .collect();

// Find validators by name
let solana_validators: Vec<_> = validators
    .iter()
    .filter(|v| {
        v.name.as_ref()
            .map(|name| name.to_lowercase().contains("solana"))
            .unwrap_or(false)
    })
    .collect();

// Find validators with identities
let with_identities: Vec<_> = validators
    .iter()
    .filter(|v| v.validator_identity.is_some())
    .collect();
```

## Advanced Configuration

```rust
use solana_validator_config::{ValidatorConfigClient, ClientConfig};

let config = ClientConfig::new()
    .with_timeout(60).unwrap()           // Custom timeout
    .with_user_agent("my-app/1.0");      // Custom user agent

let client = ValidatorConfigClient::new_custom_with_config(
    "https://your-rpc.com",
    config
);
```

## Error Handling

```rust
use solana_validator_config::ValidatorConfigError;

match client.fetch_all_validators().await {
    Ok(validators) => println!("Got {} validators", validators.len()),
    Err(ValidatorConfigError::Network(e)) => eprintln!("Network error: {}", e),
    Err(ValidatorConfigError::JsonParse(e)) => eprintln!("Parse error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Examples

Run the included examples:

```bash
# Development script (recommended)
./dev.sh example        # Simple integration example
./dev.sh test-ids       # Test validator identity extraction

# Direct cargo commands
cargo run --example simple_usage
cargo run --example test_validator_ids
cargo test
```

## Performance

- Fetches 2800+ validators from mainnet
- Takes 2-5 seconds depending on RPC speed
- Extracts validator identities from all config accounts
- Consider caching results for frequent calls

## Security

The library includes robust protections:

- String length limits (500 characters max)
- Null byte replacement with spaces
- Control character sanitization
- Unicode emoji support
- Malformed JSON handling
- UTF-8 validation

## License

MIT License
