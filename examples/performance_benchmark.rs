// examples/performance_benchmark.rs
//! Performance benchmark for validator config processing
//!
//! This benchmark measures the current performance baseline for:
//! - Network request time
//! - Data parsing and processing time
//! - Memory allocation patterns
//! - Overall throughput

use solana_validator_info::{SolanaNetwork, ValidatorConfigClient};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Validator Config Performance Benchmark");
    println!("=====================================");

    // Use testnet for consistent, repeatable benchmarks
    let client = ValidatorConfigClient::new(SolanaNetwork::Testnet);

    // Warmup request to establish connection
    println!("Performing warmup request...");
    let _warmup = client.fetch_all_validators().await?;

    // Benchmark multiple runs
    let runs = 5;
    let mut total_time = 0u128;
    let mut validator_counts = Vec::new();

    println!("Running {} benchmark iterations...", runs);

    for run in 1..=runs {
        let start = Instant::now();
        let validators = client.fetch_all_validators().await?;
        let duration = start.elapsed();

        total_time += duration.as_millis();
        validator_counts.push(validators.len());

        println!(
            "Run {}: {}ms, {} validators",
            run,
            duration.as_millis(),
            validators.len()
        );

        // Add small delay between runs
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    let avg_time = total_time / runs as u128;
    let avg_validators = validator_counts.iter().sum::<usize>() / validator_counts.len();

    println!("\nBenchmark Results:");
    println!("Average time: {}ms", avg_time);
    println!("Average validators: {}", avg_validators);
    println!(
        "Throughput: {:.2} validators/second",
        (avg_validators as f64 * 1000.0) / avg_time as f64
    );

    // Memory usage estimation
    let validators = client.fetch_all_validators().await?;
    let estimated_memory = estimate_memory_usage(&validators);
    println!(
        "Estimated memory usage: {:.2} MB",
        estimated_memory / 1_048_576.0
    );

    // Performance breakdown
    println!("\nPerformance Analysis:");
    benchmark_string_operations();
    benchmark_json_parsing().await;

    Ok(())
}

fn estimate_memory_usage(validators: &[solana_validator_info::ValidatorInfo]) -> f64 {
    validators
        .iter()
        .map(|v| {
            let mut size = std::mem::size_of::<solana_validator_info::ValidatorInfo>() as f64;

            if let Some(ref s) = v.validator_identity {
                size += s.len() as f64;
            }
            if let Some(ref s) = v.name {
                size += s.len() as f64;
            }
            if let Some(ref s) = v.website {
                size += s.len() as f64;
            }
            if let Some(ref s) = v.details {
                size += s.len() as f64;
            }
            if let Some(ref s) = v.keybase_username {
                size += s.len() as f64;
            }

            size
        })
        .sum()
}

fn benchmark_string_operations() {
    println!("String operations benchmark:");

    let test_string = "Test\0string\nwith\rcontrol\tchars\n\n\nand multiple newlines";
    let iterations = 10000;

    let start = Instant::now();
    for _ in 0..iterations {
        let _result = clean_json_string_current(test_string);
    }
    let current_time = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        let _result = clean_json_string_optimized(test_string);
    }
    let optimized_time = start.elapsed();

    println!(
        "Current string cleaning: {:.2}ms for {} iterations",
        current_time.as_secs_f64() * 1000.0,
        iterations
    );
    println!(
        "Optimized string cleaning: {:.2}ms for {} iterations",
        optimized_time.as_secs_f64() * 1000.0,
        iterations
    );

    let improvement = (current_time.as_secs_f64() - optimized_time.as_secs_f64())
        / current_time.as_secs_f64()
        * 100.0;
    println!("Improvement: {:.1}%", improvement);
}

async fn benchmark_json_parsing() {
    println!("JSON parsing benchmark:");

    // This would require access to internal functions, so we'll simulate
    let test_data =
        r#"{"name":"Test Validator","website":"https://test.com","details":"Test details"}"#;
    let iterations = 1000;

    let start = Instant::now();
    for _ in 0..iterations {
        let _result: Result<solana_validator_info::ValidatorInfo, _> =
            serde_json::from_str(test_data);
    }
    let parse_time = start.elapsed();

    println!(
        "JSON parsing: {:.2}ms for {} iterations",
        parse_time.as_secs_f64() * 1000.0,
        iterations
    );
}

// Simulate current string cleaning implementation
fn clean_json_string_current(json_str: &str) -> String {
    let mut result = json_str
        .trim()
        .replace('\0', "")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");

    while result.contains("\n\n\n") {
        result = result.replace("\n\n\n", "\n\n");
    }

    result
}

// Optimized string cleaning implementation
fn clean_json_string_optimized(json_str: &str) -> String {
    let trimmed = json_str.trim();
    let mut result = String::with_capacity(trimmed.len() + 20);

    for ch in trimmed.chars() {
        match ch {
            '\0' => {} // Skip null bytes
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c => result.push(c),
        }
    }

    result
}
