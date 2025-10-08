# Integration Guide - How to Use This Library

Simple guide to get the Solana Validator Config library working in your project.

**For production applications, always use private RPC endpoints.** Public endpoints have severe rate limits and are unreliable.

### Quick Start with Private RPC

```rust
use solana_validator_config::ValidatorConfigClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // PRODUCTION: Use your private RPC endpoint
    let client = ValidatorConfigClient::new_custom("https://your-private-rpc.com");
    
    // Get all validators
    let validators = client.fetch_all_validators().await?;
    println!("Found {} validators", validators.len());
    
    Ok(())
}
```

## Choose Your Method

### Method 1: Add as Git Dependency (Recommended)

1. **Add to your `Cargo.toml`:**
```toml
[dependencies]
solana-validator-config = { git = "https://github.com/matsuro-hadouken/solana-validator-config-data-lib" }
tokio = { version = "1.0", features = ["full"] }
```

**Alternative options:**
```toml
# Use specific branch
solana-validator-config = { git = "https://github.com/matsuro-hadouken/solana-validator-config-data-lib", branch = "main" }

# Use specific tag (when available)
solana-validator-config = { git = "https://github.com/matsuro-hadouken/solana-validator-config-data-lib", tag = "v0.1.0" }
```

2. **Use in your code:**
```rust
use solana_validator_config::ValidatorConfigClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // PRODUCTION: Use private RPC endpoint
    let client = ValidatorConfigClient::new_custom("https://your-private-rpc.com");
    
    // TESTING ONLY: Use public endpoint (unreliable, rate limited)
    // let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
    
    // Get all validators
    let validators = client.fetch_all_validators().await?;
    
    println!("Found {} validators", validators.len());
    
    // Show first 10 with names
    for (pubkey, info) in validators.iter().take(10) {
        if let Some(name) = &info.name {
            println!("{}: {}", name, pubkey);
        }
    }
    
    Ok(())
}
```

### Method 2: Copy the Code

If you want to copy the library code directly into your project:

1. **Copy `src/lib.rs` to your project**
2. **Add dependencies to your `Cargo.toml`:**
```toml
[dependencies]
reqwest = { version = "0.12.23", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
base64 = "0.22.1"
tokio = { version = "1.0", features = ["full"] }
thiserror = "2.0"
log = "0.4"
```

## Common Use Cases

### Get All Validator Data
```rust
let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
let validators = client.fetch_all_validators().await?;

// You now have Vec<(String, ValidatorInfo)>
// String = validator pubkey
// ValidatorInfo = all the config data
```

### Convert to Your Own Types
```rust
#[derive(Debug)]
struct MyValidator {
    pubkey: String,
    name: String,
    website: Option<String>,
    description: Option<String>,
}

let my_validators: Vec<MyValidator> = validators
    .into_iter()
    .filter_map(|(pubkey, info)| {
        info.name.map(|name| MyValidator {
            pubkey,
            name,
            website: info.website,
            description: info.display_description().map(|s| s.to_string()),
        })
    })
    .collect();

println!("Got {} validators with names", my_validators.len());
```

### Find Specific Validators
```rust
// Validators with websites
let with_websites: Vec<_> = validators
    .iter()
    .filter(|(_, info)| info.website.is_some())
    .collect();

// Verified validators (have Keybase)
let verified: Vec<_> = validators
    .iter()
    .filter(|(_, info)| info.keybase_username.is_some())
    .collect();

// Search by name
let matching: Vec<_> = validators
    .iter()
    .filter(|(_, info)| {
        info.name
            .as_ref()
            .map(|name| name.to_lowercase().contains("search_term"))
            .unwrap_or(false)
    })
    .collect();

println!("Found {} matching validators", matching.len());
```
### Save Data to JSON File
```rust
use std::fs;

let validators = client.fetch_all_validators().await?;
let json_data = serde_json::to_string_pretty(&validators)?;
fs::write("validators.json", json_data)?;
println!("Saved {} validators to validators.json", validators.len());
```

### Use Different Networks

**Private RPC (Recommended for Production):**
```rust
// Any private RPC endpoint
let client = ValidatorConfigClient::new_custom("https://your-private-rpc.com");

// QuickNode
let client = ValidatorConfigClient::new_custom("https://your-endpoint.quiknode.pro/token/");

// Alchemy
let client = ValidatorConfigClient::new_custom("https://solana-mainnet.g.alchemy.com/v2/your-api-key");

// Helius
let client = ValidatorConfigClient::new_custom("https://rpc.helius.xyz/?api-key=your-api-key");
```

**Public RPC (Testing Only - Not Recommended for Production):**
```rust
use solana_validator_config::SolanaNetwork;

// Mainnet (real validators)
let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);

// Testnet
let client = ValidatorConfigClient::new(SolanaNetwork::Testnet);

// Devnet
let client = ValidatorConfigClient::new(SolanaNetwork::Devnet);

// Custom public endpoint
let client = ValidatorConfigClient::new(
    SolanaNetwork::Custom("https://some-public-rpc.com".to_string())
);
```

### Get Statistics
```rust
let stats = client.get_validator_stats().await?;
println!("Total: {}", stats.total_validators);
println!("With names: {}", stats.with_names);
println!("With websites: {}", stats.with_websites);
println!("With Keybase: {}", stats.with_keybase);
```

## Performance Tips

### 1. Cache Results (Recommended)
Network calls take 2-5 seconds, so cache the data:

```rust
use std::time::{Duration, Instant};

struct ValidatorCache {
    data: Option<Vec<(String, ValidatorInfo)>>,
    last_update: Option<Instant>,
}

impl ValidatorCache {
    fn new() -> Self {
        Self { data: None, last_update: None }
    }
    
    fn is_stale(&self) -> bool {
        self.last_update
            .map(|time| time.elapsed() > Duration::from_secs(300)) // 5 minutes
            .unwrap_or(true)
    }
    
    async fn get_validators(&mut self) -> Result<&[(String, ValidatorInfo)], Box<dyn std::error::Error>> {
        if self.is_stale() {
            let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
            self.data = Some(client.fetch_all_validators().await?);
            self.last_update = Some(Instant::now());
        }
        
        Ok(self.data.as_ref().unwrap())
    }
}

// Usage
let mut cache = ValidatorCache::new();
let validators = cache.get_validators().await?;
```

### 2. Filter Early
```rust
use solana_validator_config::ClientConfig;

let config = ClientConfig {
    max_concurrent_requests: 20,
    timeout_seconds: 30,
    include_empty_configs: false,  // Skip validators with no data
};

let client = ValidatorConfigClient::with_config(SolanaNetwork::Mainnet, config);
```

## What Data You Get

Each validator has this information (all optional):

```rust
pub struct ValidatorInfo {
    pub name: Option<String>,                    // Display name
    pub website: Option<String>,                 // Website URL  
    pub details: Option<String>,                 // Description
    pub description: Option<String>,             // Alternative description
    pub keybase_username: Option<String>,        // Keybase verification
    pub icon_url: Option<String>,                // Logo/icon URL
    pub domain: Option<String>,                  // Domain name
    pub contact: Option<String>,                 // Contact info
    pub twitter: Option<String>,                 // Twitter handle
    pub discord: Option<String>,                 // Discord info
}
```

**Helper methods:**
- `info.display_name()` - Returns name or keybase_username
- `info.display_description()` - Returns details or description  
- `info.has_config()` - True if validator has any data

## Typical Numbers (Mainnet)

- **Total validators:** ~2,800
- **With names:** ~2,800 (almost all)
- **With websites:** ~1,700 (60%)
- **With Keybase:** ~1,000 (35%)
- **With icons:** ~1,200 (43%)

## Error Handling

```rust
use solana_validator_config::ValidatorConfigError;

match client.fetch_all_validators().await {
    Ok(validators) => {
        println!("Got {} validators", validators.len());
        // Process data...
    }
    Err(ValidatorConfigError::NetworkError(msg)) => {
        eprintln!("Network problem: {}", msg);
        // Maybe retry or use cached data
    }
    Err(ValidatorConfigError::ParseError(msg)) => {
        eprintln!("Data parsing problem: {}", msg);
        // Data format issue
    }
    Err(ValidatorConfigError::RpcError(msg)) => {
        eprintln!("RPC problem: {}", msg);
        // Solana RPC issue
    }
}
```

## Testing Your Integration

### Quick Test
```bash
# In this library directory
cargo run --bin validator-config-example

# Should show validator data
```

### Integration Test
```rust
#[tokio::test]
async fn test_integration() {
    use solana_validator_config::{ValidatorConfigClient, SolanaNetwork};
    
    let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
    let validators = client.fetch_all_validators().await.unwrap();
    
    assert!(!validators.is_empty());
    println!("Integration test passed! Got {} validators", validators.len());
}
```

## Complete Working Example

Here's a complete example you can copy and run:

```rust
use solana_validator_config::{ValidatorConfigClient, SolanaNetwork};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching Solana validator data...");
    
    let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
    let validators = client.fetch_all_validators().await?;
    
    // Get statistics
    let stats = client.get_validator_stats().await?;
    println!("Statistics:");
    println!("  Total validators: {}", stats.total_validators);
    println!("  With names: {}", stats.with_names);
    println!("  With websites: {}", stats.with_websites);
    
    // Group by first letter of name
    let mut by_letter: HashMap<char, u32> = HashMap::new();
    for (_, info) in &validators {
        if let Some(name) = &info.name {
            let first_char = name.chars().next().unwrap_or('?').to_ascii_uppercase();
            *by_letter.entry(first_char).or_insert(0) += 1;
        }
    }
    
    println!("\nValidators by first letter:");
    let mut letters: Vec<_> = by_letter.iter().collect();
    letters.sort_by_key(|(letter, _)| **letter);
    for (letter, count) in letters {
        println!("  {}: {}", letter, count);
    }
    
    // Find some interesting validators
    println!("\nSome interesting validators:");
    for (pubkey, info) in validators.iter().take(5) {
        if let Some(name) = &info.name {
            println!("  {}", name);
            if let Some(website) = &info.website {
                println!("    Website: {}", website);
            }
        }
    }
    
    Ok(())
}
```

## Need Help?

- Check `README.md` for full documentation
- Look at `examples/simple_usage.rs` for more examples  
- Run `cargo doc --open` for API documentation
- Run the examples to see it working

That's it! You now have access to all Solana validator configuration data.
