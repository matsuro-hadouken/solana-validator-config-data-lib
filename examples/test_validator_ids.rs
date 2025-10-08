//! Test validator identity extraction

use solana_validator_config::{SolanaNetwork, ValidatorConfigClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing Validator Identity Extraction ===");

    let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
    let configs = client.fetch_all_validators().await?;

    println!("Total validators found: {}", configs.len());

    // Look for some known validators
    let known_validators = [
        "GwHH8ciFhR8vejWCqmg8FWZUCNtubPY2esALvy5tBvji", // Your specified test validator  
        "farbZXR7aBQSMCYiUXzoS4pRUsvuCZ38f6AXMXiKACf", // Farben (known active validator)
    ];

    println!("\n--- Looking for specified test validators ---");
    for known in &known_validators {
        if let Some(info) = configs.iter().find(|info| {
            if let Some(identity) = &info.validator_identity {
                identity == known
            } else {
                false
            }
        }) {
            println!(
                "✅ Found {}: {}",
                known,
                info.display_name().unwrap_or("No name")
            );
        } else {
            println!("❌ Not found: {} (may not have published config or be inactive)", known);
        }
    }

    // Show first 10 validator identities to verify format
    println!("\n--- First 10 validator identities ---");
    for (i, info) in configs.iter().take(10).enumerate() {
        let identity = info
            .validator_identity
            .as_deref()
            .unwrap_or("No identity extracted");
        println!(
            "{}. {} ({})",
            i + 1,
            info.display_name().unwrap_or("Unknown"),
            identity
        );
    }

    // Count validators with extracted identities
    let total_with_identities = configs
        .iter()
        .filter(|info| info.validator_identity.is_some())
        .count();

    println!(
        "\n✅ Extracted validator identities for {} out of {} validators!",
        total_with_identities,
        configs.len()
    );

    // Verify extracted identities are properly formatted
    let invalid_keys: Vec<_> = configs
        .iter()
        .filter_map(|info| info.validator_identity.as_ref())
        .filter(|pk| {
            // Base58 encoded 32-byte keys can be 43-44 chars (leading zeros compressed)
            pk.len() < 43
                || pk.len() > 44
                || !pk.chars().all(|c| {
                    "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".contains(c)
                })
        })
        .collect();

    if invalid_keys.is_empty() {
        println!("✅ All extracted validator identities are properly formatted!");
    } else {
        println!(
            "❌ Found {} invalid validator identities:",
            invalid_keys.len()
        );
        for key in invalid_keys.iter().take(5) {
            println!("  - {} (length: {})", key, key.len());
        }
    }

    Ok(())
}
