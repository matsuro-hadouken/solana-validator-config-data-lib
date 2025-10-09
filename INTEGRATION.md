# Production Integration

Production patterns for serious applications.

## Essential Setup

```rust
use solana_validator_config::{ValidatorConfigClient, SolanaNetwork, ClientConfig};

// Production client with private RPC
let client = ValidatorConfigClient::new_custom("https://your-private-rpc.com");

// Custom configuration
let config = ClientConfig::new()
    .with_timeout(30).unwrap()
    .with_max_concurrent_requests(10).unwrap();

let client = ValidatorConfigClient::with_config(SolanaNetwork::Mainnet, config);
```

## Error Handling

```rust
use solana_validator_config::ValidatorConfigError;
use tokio::time::{sleep, Duration};

async fn fetch_with_retry(
    client: &ValidatorConfigClient,
    max_retries: u32,
) -> Result<Vec<(String, ValidatorInfo)>, ValidatorConfigError> {
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match client.fetch_all_validators().await {
            Ok(validators) => return Ok(validators),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries {
                    let delay = Duration::from_secs(2_u64.pow(attempt));
                    sleep(delay).await;
                }
            }
        }
    }
    
    Err(last_error.unwrap())
}
```

## Caching Implementation

```rust
use std::time::{Duration, Instant};
use std::collections::HashMap;

pub struct ValidatorCache {
    data: Option<Vec<(String, ValidatorInfo)>>,
    last_update: Option<Instant>,
    ttl: Duration,
}

impl ValidatorCache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            data: None,
            last_update: None,
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub async fn get_validators(
        &mut self,
        client: &ValidatorConfigClient
    ) -> Result<&[(String, ValidatorInfo)], Box<dyn std::error::Error>> {
        if self.is_stale() {
            self.data = Some(client.fetch_all_validators().await?);
            self.last_update = Some(Instant::now());
        }
        
        Ok(self.data.as_ref().unwrap())
    }

    fn is_stale(&self) -> bool {
        self.last_update
            .map(|time| time.elapsed() > self.ttl)
            .unwrap_or(true)
    }
}
```

## Multi-Network Client

```rust
use std::collections::HashMap;

pub struct MultiNetworkClient {
    clients: HashMap<String, ValidatorConfigClient>,
}

impl MultiNetworkClient {
    pub fn new() -> Self {
        let mut clients = HashMap::new();
        
        clients.insert("mainnet".to_string(),
            ValidatorConfigClient::new(SolanaNetwork::Mainnet));
        clients.insert("testnet".to_string(),
            ValidatorConfigClient::new(SolanaNetwork::Testnet));
        clients.insert("devnet".to_string(),
            ValidatorConfigClient::new(SolanaNetwork::Devnet));

        Self { clients }
    }
    
    pub fn add_network(&mut self, name: &str, rpc_url: &str) {
        self.clients.insert(
            name.to_string(),
            ValidatorConfigClient::new_custom(rpc_url)
        );
    }

    pub async fn get_validator_across_networks(
        &self,
        identity: &str
    ) -> Result<HashMap<String, Option<ValidatorInfo>>, Box<dyn std::error::Error>> {
        let mut results = HashMap::new();

        for (network, client) in &self.clients {
            let validators = client.fetch_all_validators().await?;
            let validator = validators
                .iter()
                .find(|(pubkey, _)| pubkey == identity)
                .map(|(_, info)| info.clone());
            
            results.insert(network.clone(), validator);
        }

        Ok(results)
    }
}
```

## Environment Configuration

```rust
use std::env;

pub fn create_production_client() -> Result<ValidatorConfigClient, Box<dyn std::error::Error>> {
    let rpc_url = env::var("SOLANA_RPC_URL")
        .or_else(|_| env::var("RPC_URL"))
        .map_err(|_| "RPC_URL environment variable not set")?;
    
    if !rpc_url.starts_with("http") {
        return Err("RPC_URL must start with http or https".into());
    }
    
    Ok(ValidatorConfigClient::new_custom(&rpc_url))
}
```

## Rate Limiting

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct RateLimitedClient {
    client: ValidatorConfigClient,
    semaphore: Arc<Semaphore>,
}

impl RateLimitedClient {
    pub fn new(rpc_url: &str, max_concurrent: usize) -> Self {
        Self {
            client: ValidatorConfigClient::new_custom(rpc_url),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn fetch_validators(&self) -> Result<Vec<(String, ValidatorInfo)>, Box<dyn std::error::Error>> {
        let _permit = self.semaphore.acquire().await?;
        self.client.fetch_all_validators().await.map_err(Into::into)
    }
}
```

## Performance Monitoring

```rust
use std::time::Instant;

#[derive(Debug)]
pub struct Metrics {
    pub fetch_duration: Duration,
    pub total_validators: usize,
    pub with_identities: usize,
}

pub async fn fetch_with_metrics(
    client: &ValidatorConfigClient
) -> Result<(Vec<(String, ValidatorInfo)>, Metrics), Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    let validators = client.fetch_all_validators().await?;
    let fetch_duration = start.elapsed();

    let with_identities = validators.len();
    
    let metrics = Metrics {
        fetch_duration,
        total_validators: validators.len(),
        with_identities,
    };
    
    Ok((validators, metrics))
}
```

## Production Checklist

- [ ] Use private RPC endpoints
- [ ] Implement error handling and retries
- [ ] Add caching for frequent calls
- [ ] Monitor RPC usage and costs
- [ ] Set up logging and metrics
- [ ] Validate all user inputs
- [ ] Rate limit API calls if exposing publicly
- [ ] Sanitize output before displaying
- [ ] Keep RPC endpoints in environment variables

## Security Considerations

- [ ] Validate all user inputs before filtering
- [ ] Rate limit API calls if exposing publicly
- [ ] Sanitize output before displaying to users
- [ ] Keep RPC endpoints in environment variables

## Docker Configuration

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/your-app /usr/local/bin/your-app

ENV SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
ENV RUST_LOG=info

CMD ["your-app"]
```

## Health Checks

```rust
pub async fn health_check(client: &ValidatorConfigClient) -> Result<bool, Box<dyn std::error::Error>> {
    match client.get_validator_stats().await {
        Ok(stats) => Ok(stats.total_validators > 100),
        Err(_) => Ok(false),
    }
}
```

## Input Validation

```rust
pub fn validate_validator_identity(identity: &str) -> Result<(), &'static str> {
    if identity.len() < 32 || identity.len() > 44 {
        return Err("Invalid validator identity length");
    }
    
    if !identity.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err("Invalid characters in validator identity");
    }
    
    Ok(())
}

pub fn sanitize_query(query: &str) -> String {
    query
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || ".-_".contains(*c))
        .take(100)
        .collect()
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_fetch_validators() {
        let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
        let validators = client.fetch_all_validators().await.unwrap();
        assert!(!validators.is_empty());
    }
    
    #[tokio::test]
    async fn test_cache() {
        let mut cache = ValidatorCache::new(300);
        let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
        
        let validators1 = cache.get_validators(&client).await.unwrap();
        let validators2 = cache.get_validators(&client).await.unwrap();
        
        assert_eq!(validators1.len(), validators2.len());
    }
}
```
