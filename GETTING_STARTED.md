# Getting Started with Solana Validator Config Library

## Quick Setup (2 minutes)

### 1. Add to your Rust project

```toml
# Add to Cargo.toml
[dependencies]
solana-validator-config = { git = "https://github.com/matsuro-hadouken/solana-validator-config-data-lib" }
tokio = { version = "1.0", features = ["full"] }
```

### 2. Basic usage

```rust
// src/main.rs
use solana_validator_config::ValidatorConfigClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // PRODUCTION: Use your private RPC endpoint
    let client = ValidatorConfigClient::new_custom("https://your-private-rpc.com");
    
    // Get all validators
    let validators = client.fetch_all_validators().await?;
    
    println!("Found {} validators", validators.len());
    
    // Print first 5 validators with names
    for (pubkey, info) in validators.iter().take(5) {
        if let Some(name) = &info.name {
            println!("• {} ({})", name, &pubkey[..8]);
        }
    }
    
    Ok(())
}
```

### 3. Run it

```bash
cargo run
```

## Private RPC Setup (Recommended)

### Get a private RPC endpoint:
- **[QuickNode](https://www.quicknode.com/)** - Fast setup, good free tier
- **[Alchemy](https://www.alchemy.com/)** - Enterprise-grade, excellent docs  
- **[Helius](https://www.helius.dev/)** - Solana-focused, competitive pricing

### Use it in your code:

```rust
let client = ValidatorConfigClient::new_custom("https://your-private-rpc.com");
```

### Or via environment variable:

```bash
export SOLANA_RPC_URL="https://your-private-rpc.com"
cargo run
```

## That's it! 🎉

You're now ready to fetch Solana validator data. Check out:
- [README.md](README.md) - Full documentation
- [INTEGRATION.md](INTEGRATION.md) - Advanced usage patterns
- [examples/](examples/) - More examples