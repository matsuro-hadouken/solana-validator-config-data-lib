//! Example usage of the Solana Validator Config library
//!
//! This example demonstrates how to fetch and display validator configuration data
//! from the Solana blockchain using both public and private RPC endpoints.
//! 
//! For production use, always use private RPC endpoints (QuickNode, Alchemy, Helius, etc.)
//! for better reliability, rate limits, and performance.

use solana_validator_config::{ClientConfig, SolanaNetwork, ValidatorConfigClient, ValidatorInfo};
use std::error::Error;
use std::env;

/// Display validator information in a formatted way
fn display_validator_info(validators: &[(String, ValidatorInfo)]) {
    println!("\n=== VALIDATOR CONFIGURATIONS ===\n");

    for (i, (pubkey, info)) in validators.iter().enumerate() {
        println!("Validator #{} ({}...)", i + 1, &pubkey[..8]);
        println!("  Public Key: {}", pubkey);

        if let Some(name) = &info.name {
            println!("  Name: {}", name);
        }

        if let Some(website) = &info.website {
            println!("  Website: {}", website);
        }

        if let Some(description) = info.display_description() {
            // Truncate long descriptions for readability
            let truncated = if description.len() > 100 {
                format!("{}...", &description[..97])
            } else {
                description.to_string()
            };
            println!("  Description: {}", truncated);
        }

        if let Some(keybase) = &info.keybase_username {
            println!("  Keybase: {}", keybase);
        }

        println!("  ---");
    }
}

/// Display summary statistics about validators
fn display_summary_stats(validators: &[(String, ValidatorInfo)]) {
    println!("\n=== SUMMARY STATISTICS ===");

    let total = validators.len();
    let with_names = validators
        .iter()
        .filter(|(_, info)| info.name.is_some())
        .count();
    let with_websites = validators
        .iter()
        .filter(|(_, info)| info.website.is_some())
        .count();
    let with_keybase = validators
        .iter()
        .filter(|(_, info)| info.keybase_username.is_some())
        .count();
    let with_descriptions = validators
        .iter()
        .filter(|(_, info)| info.display_description().is_some())
        .count();

    println!("Total validators with configs: {}", total);
    println!(
        "Validators with names: {} ({:.1}%)",
        with_names,
        (with_names as f64 / total as f64) * 100.0
    );
    println!(
        "Validators with websites: {} ({:.1}%)",
        with_websites,
        (with_websites as f64 / total as f64) * 100.0
    );
    println!(
        "Validators with Keybase: {} ({:.1}%)",
        with_keybase,
        (with_keybase as f64 / total as f64) * 100.0
    );
    println!(
        "Validators with descriptions: {} ({:.1}%)",
        with_descriptions,
        (with_descriptions as f64 / total as f64) * 100.0
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Note: Set RUST_LOG=info to see detailed logging information

    println!("Solana Validator Config Library - Example Usage");
    println!("==============================================\n");

    // OPTION 1: Using private RPC (RECOMMENDED for production)
    // Check for custom RPC endpoint in environment variable
    let client = if let Ok(custom_rpc) = env::var("SOLANA_RPC_URL") {
        println!("Using custom RPC endpoint: {}", custom_rpc);
        ValidatorConfigClient::new_custom(custom_rpc)
    } else {
        println!("No SOLANA_RPC_URL environment variable found.");
        println!("For production, use a private RPC endpoint:");
        println!("  export SOLANA_RPC_URL='https://your-private-rpc.com'");
        println!("  cargo run");
        println!("\nExamples of private RPC providers:");
        println!("  QuickNode: https://your-endpoint.quiknode.pro/token/");
        println!("  Alchemy:   https://solana-mainnet.g.alchemy.com/v2/your-api-key");
        println!("  Helius:    https://rpc.helius.xyz/?api-key=your-api-key");
        println!("\nFalling back to public endpoint for demo...\n");
        
        // OPTION 2: Using public RPC (for testing only)
        let config = ClientConfig::new()
            .with_timeout(60)?
            .with_user_agent("solana-validator-config-example/1.0");
        
        ValidatorConfigClient::with_config(SolanaNetwork::Mainnet, config)
    };

    println!("Fetching validator configurations from Solana...");

    // Fetch all validators with proper error handling
    let validators = match client.fetch_all_validators().await {
        Ok(validators) => {
            log::info!("Successfully fetched {} validators", validators.len());
            validators
        }
        Err(e) => {
            eprintln!("Error fetching validator data: {}", e);
            eprintln!("This might be due to:");
            eprintln!("  - Network issues or connectivity problems");
            eprintln!("  - RPC rate limiting (use private RPC to avoid this)");
            eprintln!("  - RPC endpoint being down");
            eprintln!("\nFor production use, set SOLANA_RPC_URL to a private endpoint:");
            eprintln!("  export SOLANA_RPC_URL='https://your-private-rpc.com'");
            return Err(e.into());
        }
    };

    if validators.is_empty() {
        println!("No validator configurations found.");
        return Ok(());
    }

    // Display first 10 validators as example
    let sample_validators: Vec<_> = validators.iter().take(10).cloned().collect();
    display_validator_info(&sample_validators);

    // Display summary statistics
    display_summary_stats(&validators);

    // Example: Find validators with specific criteria
    println!("\n=== VALIDATORS WITH WEBSITES ===");
    let validators_with_websites: Vec<_> = validators
        .iter()
        .filter(|(_, info)| info.website.is_some())
        .take(5)
        .collect();

    for (_pubkey, info) in validators_with_websites {
        println!(
            "• {} - {}",
            info.display_name().unwrap_or("Unknown"),
            info.website.as_ref().unwrap()
        );
    }

    // Example: Find verified validators
    println!("\n=== VERIFIED VALIDATORS (Keybase) ===");
    let verified_validators: Vec<_> = validators
        .iter()
        .filter(|(_, info)| info.keybase_username.is_some())
        .take(5)
        .collect();

    for (_pubkey, info) in verified_validators {
        println!(
            "• {} (keybase: {})",
            info.display_name().unwrap_or("Unknown"),
            info.keybase_username.as_ref().unwrap()
        );
    }

    println!("\nExample completed successfully!");
    println!("Note: You can set RUST_LOG=debug to see more detailed logging information.");

    Ok(())
}
