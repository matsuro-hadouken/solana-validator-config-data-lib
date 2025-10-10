//! Example demonstrating improved error handling and retry logic

use solana_validator_info::{SolanaNetwork, ValidatorConfigClient, ValidatorConfigError};
use std::time::Duration;
use tokio::time::sleep;

async fn fetch_with_retry(
    client: &ValidatorConfigClient,
    max_retries: u32,
) -> Result<Vec<solana_validator_info::ValidatorInfo>, ValidatorConfigError> {
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match client.fetch_all_validators().await {
            Ok(validators) => {
                if attempt > 0 {
                    println!("[SUCCESS] Completed after {} retries!", attempt);
                }
                return Ok(validators);
            }
            Err(e) => {
                println!("[ATTEMPT {}] Failed: {}", attempt + 1, e);

                // Check if error is retryable
                if !e.is_retryable() {
                    println!("[ERROR] Non-retryable error, giving up");
                    return Err(e);
                }

                // Get suggested retry delay
                if let Some(delay) = e.retry_delay() {
                    println!("[RETRY] Waiting {} seconds before retry...", delay);
                    sleep(Duration::from_secs(delay)).await;
                } else {
                    // Default exponential backoff
                    let delay = 2_u64.pow(attempt);
                    println!(
                        "[BACKOFF] Exponential backoff: waiting {} seconds...",
                        delay
                    );
                    sleep(Duration::from_secs(delay)).await;
                }

                last_error = Some(e);
            }
        }
    }

    Err(last_error.unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Error Handling Demo ===\n");

    // Test with mainnet (should work)
    println!("1. Testing with Mainnet (should succeed):");
    let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);

    match fetch_with_retry(&client, 3).await {
        Ok(validators) => {
            println!("[OK] Successfully fetched {} validators", validators.len());
        }
        Err(e) => {
            println!("[FAILED] Failed after retries: {}", e);
            match &e {
                ValidatorConfigError::RateLimitExceeded { retry_after, .. } => {
                    if let Some(delay) = retry_after {
                        println!("  [INFO] Server suggests waiting {} seconds", delay);
                    }
                }
                ValidatorConfigError::HttpError { status, .. } => {
                    println!("  [INFO] HTTP status: {}", status);
                }
                ValidatorConfigError::RpcError { code, .. } => {
                    println!("  [INFO] RPC error code: {}", code);
                }
                _ => {}
            }
        }
    }

    println!("\n2. Testing error categorization:");

    // Test with invalid URL (should be non-retryable)
    println!("Testing with invalid URL:");
    let bad_client =
        ValidatorConfigClient::new_custom("http://invalid-url-that-does-not-exist.fake");

    match bad_client.fetch_all_validators().await {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            println!("Error: {}", e);
            println!("Is retryable: {}", e.is_retryable());
            if let Some(delay) = e.retry_delay() {
                println!("Suggested retry delay: {} seconds", delay);
            } else {
                println!("No retry suggested (non-retryable error)");
            }
        }
    }

    println!("\n[COMPLETED] Error handling demo completed!");
    Ok(())
}
