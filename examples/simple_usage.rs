// examples/simple_usage.rs
//! Simple example showing how to integrate the Solana validator config library
//! into your own project.
//!
//! IMPORTANT: This library now returns the actual validator identity public keys
//! that can be used to connect to validators, not Config Program account keys.

use solana_validator_config::{SolanaNetwork, ValidatorConfigClient, ValidatorInfo};
use std::collections::HashMap;

/// Example struct showing how you might integrate validator data into your own types
#[derive(Debug, Clone)]
struct MyValidatorData {
    pub validator_identity: Option<String>, // The actual validator identity key
    pub name: String,
    pub website: Option<String>,
    pub description: Option<String>,
    pub verified: bool, // Has Keybase verification
}

impl From<ValidatorInfo> for MyValidatorData {
    fn from(info: ValidatorInfo) -> Self {
        let name = info.display_name().unwrap_or("Unknown").to_string();
        let description = info.display_description().map(std::string::ToString::to_string);
        let verified = info.keybase_username.is_some();

        Self {
            validator_identity: info.validator_identity,
            name,
            website: info.website,
            description,
            verified,
        }
    }
}

/// Example function showing how to process validator data for your application
async fn process_validators_for_my_app() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Fetch validator data
    let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
    let raw_validators = client.fetch_all_validators().await?;

    // 2. Convert to your app's data structures
    let my_validators: Vec<MyValidatorData> = raw_validators
        .into_iter()
        .map(MyValidatorData::from)
        .collect();

    // 3. Create useful data structures
    let mut by_name: HashMap<String, MyValidatorData> = HashMap::new();
    let mut verified_validators = Vec::new();

    for validator in my_validators {
        // Collect verified validators first
        if validator.verified {
            verified_validators.push(validator.clone());
        }

        // Index by name for quick lookup
        by_name.insert(validator.name.clone(), validator);
    }

    // 4. Use the data in your application
    println!("Total validators: {}", by_name.len());
    println!("Verified validators: {}", verified_validators.len());

    // Example: Find a specific validator and use all fields
    if let Some(validator) = by_name.get("Solana Foundation") {
        println!("Found Solana Foundation validator:");
        if let Some(identity) = &validator.validator_identity {
            println!("  Validator Identity: {identity}");
        }
        println!("  Name: {}", validator.name);
        if let Some(website) = &validator.website {
            println!("  Website: {website}");
        }
        if let Some(description) = &validator.description {
            println!("  Description: {description}");
        }
        println!("  Verified: {}", validator.verified);
    }

    // Example: Show validators with websites
    println!("\n--- Validators with websites ---");
    for (_, validator) in by_name.iter().take(5) {
        if let Some(website) = &validator.website {
            let identity = validator.validator_identity.as_deref().unwrap_or("Unknown");
            println!("• {} - {} ({})", validator.name, website, identity);
        }
    }

    // Example: Show validator descriptions
    println!("\n--- Validators with descriptions ---");
    for (_, validator) in by_name.iter().take(3) {
        if let Some(description) = &validator.description {
            let truncated = if description.len() > 80 {
                format!("{}...", &description[..77])
            } else {
                description.clone()
            };
            println!("• {}: {}", validator.name, truncated);
        }
    }

    Ok(())
}

/// Example of how you might cache validator data
use std::time::{Duration, Instant};

struct ValidatorCache {
    data: Vec<ValidatorInfo>,
    last_updated: Instant,
    cache_duration: Duration,
}

impl ValidatorCache {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            last_updated: Instant::now().checked_sub(Duration::from_secs(3600)).unwrap(), // Force initial fetch
            cache_duration: Duration::from_secs(300),                 // 5 minutes
        }
    }

    async fn get_validators(&mut self) -> Result<&[ValidatorInfo], Box<dyn std::error::Error>> {
        if self.last_updated.elapsed() > self.cache_duration {
            println!("Cache expired, fetching fresh data...");
            let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
            self.data = client.fetch_all_validators().await?;
            self.last_updated = Instant::now();
        } else {
            println!("Using cached data");
        }

        Ok(&self.data)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple Integration Example ===\n");

    // Example 1: Basic usage
    process_validators_for_my_app().await?;

    println!("\n=== Caching Example ===\n");

    // Example 2: Using cache
    let mut cache = ValidatorCache::new();

    // First call - will fetch from network
    let validators1 = cache.get_validators().await?;
    println!("First call returned {} validators", validators1.len());

    // Second call - will use cache
    let validators2 = cache.get_validators().await?;
    println!("Second call returned {} validators", validators2.len());

    Ok(())
}
