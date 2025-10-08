# Solana Validator Config Library

A simple Rust library for retrieving Solana validator configuration data directly from the blockchain. Get validator names, websites, descriptions, and metadata with just a few lines of code.

**🚀 Private RPC Ready**: Built specifically for production use with private RPC endpoints (QuickNode, Alchemy, Helius, etc.)

👉 **[Quick Start Guide](GETTING_STARTED.md)** - Get up and running in 2 minutes!

## What This Library Does

This library connects to the Solana blockchain and fetches validator configuration information that validators have published about themselves. You can:

- Get all validator names, websites, and details (descriptions)
- Access Keybase usernames for identity verification
- Work with any Solana network (Mainnet, Testnet, Devnet, or custom)
- Filter and analyze validator data
- Export data to JSON

*Note: This library strictly follows the official Solana validator-info.json specification.*

## Key Features

- **🔒 Private RPC Support** - Works seamlessly with QuickNode, Alchemy, Helius, and other private providers
- **🛡️ Input Sanitization** - Automatically handles emojis, special characters, and malicious content
- **Simple API** - Just a few functions to get all validator data
- **Multiple networks** - Mainnet, Testnet, Devnet, or custom RPC
- **Type safe** - Full Rust type safety with proper error handling
- **Async support** - Non-blocking operations with Tokio
- **Built-in stats** - Get counts and summaries automatically
- **Well tested** - Comprehensive test coverage including security scenarios
- **Type safe** - Full Rust type safety with proper error handling
- **Async support** - Non-blocking operations with Tokio
- **Built-in stats** - Get counts and summaries automatically
- **Well tested** - Comprehensive test coverage

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
solana-validator-config = { git = "https://github.com/matsuro-hadouken/solana-validator-config-data-lib" }
tokio = { version = "1.0", features = ["full"] }
```

**Alternative installation options:**
```toml
# Use specific branch
solana-validator-config = { git = "https://github.com/matsuro-hadouken/solana-validator-config-data-lib", branch = "main" }

# Use specific tag (recommended for production)
solana-validator-config = { git = "https://github.com/matsuro-hadouken/solana-validator-config-data-lib", tag = "v0.1.0" }
```

## Simple Example

Here's how to get all validator information in just a few lines:

```rust
use solana_validator_config::ValidatorConfigClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // For production, use a private RPC endpoint:
    let client = ValidatorConfigClient::new_custom("https://your-private-rpc.com");
    
    // For testing, you can use public endpoints:
    // let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
    
    // Get all validator info
    let validators = client.fetch_all_validators().await?;
    
    println!("Found {} validators", validators.len());
    
    // Show first 5 validators with names
    for (pubkey, info) in validators.iter().take(5) {
        if let Some(name) = &info.name {
            println!("Validator: {}", name);
            println!("  Pubkey: {}", pubkey);
            
            if let Some(website) = &info.website {
                println!("  Website: {}", website);
            }
            
            if let Some(description) = info.display_description() {
                println!("  Description: {}", description);
            }
            println!();
        }
    }
    
    Ok(())
}
```

**Note**: Public RPC endpoints are included for easy testing and getting started, but you should use private RPC endpoints for production applications.

## 📦 Publishing Status

This library is currently available as a **GitHub repository**. It's not yet published to crates.io, so you need to reference it via Git URL in your `Cargo.toml`.

### Future Plans
- [ ] Publish to crates.io for easier installation (`cargo add solana-validator-config`)
- [ ] Add more comprehensive examples
- [ ] Performance optimizations

## 🛡️ Security & Data Handling

This library includes robust protections against problematic validator data:

### **Input Sanitization**
- **Length limits**: Strings are truncated to 500 characters to prevent abuse
- **Character replacement**: 
  - Null bytes (`\0`) are replaced with spaces for better readability
  - Control characters are replaced with newlines (except `\n`, `\r`, `\t`)
  - Maximum 2 consecutive newlines to prevent formatting abuse
- **Unicode support**: Properly handles emojis and international characters
- **Encoding safety**: Validates UTF-8 encoding from blockchain data

### **What's Protected**
```rust
// ✅ These work safely:
"🚀 Rocket Validator 💎"           // Emojis preserved
"Café Münchën Validator"           // International characters
"Validator\nWith\nNewlines"        // Reasonable whitespace

// 🛡️ These are sanitized:
"Validator\0WithNullBytes"         // → "ValidatorWithNullBytes"
"A".repeat(2000)                   // → Truncated to 1000 chars + "..."
"Bad\x01\x02ControlChars"          // → "BadControlChars"
```

### **Error Handling**
- Graceful handling of malformed JSON from blockchain
- UTF-8 conversion errors are caught and reported
- Base64 decoding failures don't crash the application
- Invalid validator data is silently filtered out

**Your application is protected** from malicious or malformed validator data automatically.

## Different Networks

You can connect to any Solana network:

```rust
// Mainnet (default - real validators)
let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);

// Testnet (test network)
let client = ValidatorConfigClient::new(SolanaNetwork::Testnet);

// Devnet (development network)
let client = ValidatorConfigClient::new(SolanaNetwork::Devnet);

// Your own RPC endpoint
let client = ValidatorConfigClient::new(
    SolanaNetwork::Custom("https://your-rpc-endpoint.com".to_string())
);
```

## 🔒 Private RPC Endpoints (Recommended for Production)

For production applications, you should use private RPC endpoints for better reliability, rate limits, and performance:

```rust
use solana_validator_config::ValidatorConfigClient;

// QuickNode
let client = ValidatorConfigClient::new_custom("https://your-endpoint.quiknode.pro/token/");

// Alchemy
let client = ValidatorConfigClient::new_custom("https://solana-mainnet.g.alchemy.com/v2/your-api-key");

// Helius
let client = ValidatorConfigClient::new_custom("https://rpc.helius.xyz/?api-key=your-api-key");

// GenesysGo
let client = ValidatorConfigClient::new_custom("https://ssc-dao.genesysgo.net/");

// Any other private RPC
let client = ValidatorConfigClient::new_custom("https://your-private-rpc.com");

// Get validator data
let validators = client.fetch_all_validators().await?;
```

### Private RPC with Custom Configuration

```rust
use solana_validator_config::{ValidatorConfigClient, ClientConfig};

let config = ClientConfig::new()
    .with_timeout(60).unwrap()           // Longer timeout for reliability
    .with_user_agent("my-app/1.0");      // Custom user agent

let client = ValidatorConfigClient::new_custom_with_config(
    "https://your-private-rpc.com",
    config
);
```

## Get Statistics

Want to know how many validators have names, websites, etc?

```rust
let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);

let stats = client.get_validator_stats().await?;
println!("Total validators: {}", stats.total_validators);
println!("Have names: {}", stats.with_names);
println!("Have websites: {}", stats.with_websites);
println!("Have Keybase: {}", stats.with_keybase);
```

## Filter and Search

Find specific validators:

```rust
let validators = client.fetch_all_validators().await?;

// Find validators with websites
let with_websites: Vec<_> = validators
    .iter()
    .filter(|(_, info)| info.website.is_some())
    .collect();

println!("Validators with websites: {}", with_websites.len());

// Find validators by name
let solana_validators: Vec<_> = validators
    .iter()
    .filter(|(_, info)| {
        info.name
            .as_ref()
            .map(|name| name.to_lowercase().contains("solana"))
            .unwrap_or(false)
    })
    .collect();

println!("Validators with 'solana' in name: {}", solana_validators.len());
```

## Save Data

Export to JSON file:

```rust
use std::fs;

let validators = client.fetch_all_validators().await?;
let json_data = serde_json::to_string_pretty(&validators)?;
fs::write("validators.json", json_data)?;
println!("Saved {} validators to validators.json", validators.len());
```

## Advanced Configuration

Need custom timeouts or settings?

```rust
use solana_validator_config::{ValidatorConfigClient, SolanaNetwork, ClientConfig};

let config = ClientConfig {
    max_concurrent_requests: 20,        // Future use
    timeout_seconds: 60,                // 60 second timeout
    include_empty_configs: true,        // Include validators with no data
};

let client = ValidatorConfigClient::with_config(SolanaNetwork::Mainnet, config);
```

## What Data You Get

Each validator can have this information (following the official Solana validator-info.json specification):

```rust
pub struct ValidatorInfo {
    pub name: Option<String>,                    // Display name
    pub website: Option<String>,                 // Website URL
    pub details: Option<String>,                 // Description/details
    pub keybase_username: Option<String>,        // Keybase identity
}
```

**Helper methods:**
- `info.display_name()` - Gets name or keybase_username
- `info.display_description()` - Gets details
- `info.has_config()` - True if validator has any data

## Error Handling

Handle errors properly:

```rust
use solana_validator_config::ValidatorConfigError;

match client.fetch_all_validators().await {
    Ok(validators) => {
        println!("Got {} validators", validators.len());
    }
    Err(ValidatorConfigError::NetworkError(msg)) => {
        eprintln!("Network problem: {}", msg);
    }
    Err(ValidatorConfigError::ParseError(msg)) => {
        eprintln!("Data parsing problem: {}", msg);
    }
    Err(ValidatorConfigError::RpcError(msg)) => {
        eprintln!("RPC problem: {}", msg);
    }
}
```

## Running Examples

Try the included examples:

```bash
# Basic example
cargo run --bin validator-config-example

# Simple integration example  
cargo run --example simple_usage

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Performance Notes

- Usually fetches 2000+ validators from mainnet
- Takes 2-5 seconds depending on RPC speed
- Consider caching results if calling frequently
- Use `include_empty_configs: false` to skip validators without data

## How to Add to Your Project

### Option 1: As a dependency (when published)
```toml
[dependencies]
solana-validator-config = "0.1.0"
```

### Option 2: As a local dependency
```toml
[dependencies]
solana-validator-config = { path = "../path/to/this/library" }
```

### Option 3: Copy the code
Just copy `src/lib.rs` and add the dependencies from this project's `Cargo.toml`

## Need Help?

- Check the examples in the `examples/` folder
- Run `cargo doc --open` for full API documentation
- All functions are documented with examples

## License

MIT License
