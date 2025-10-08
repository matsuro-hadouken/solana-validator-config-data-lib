# 🔧 Production Integration Guide# 🔧 Production Integration Guide



> **Advanced patterns, real-world examples, and production-ready setups**> **Advanced patterns, real-world examples, and production-ready setups**



This guide covers advanced integration patterns for production applications, performance optimization, and real-world use cases.This guide covers advanced integration patterns for production applications, performance optimization, and real-world use cases.



## 🚀 Production Checklist## 🚀 Production Checklist



### ✅ Essential Setup### ✅ Essential Setup

- [ ] Use private RPC endpoints (never public in production)- [ ] Use private RPC endpoints (never public in production)

- [ ] Implement proper error handling and retries- [ ] Implement proper error handling and retries

- [ ] Add caching for frequent calls- [ ] Add caching for frequent calls

- [ ] Monitor RPC usage and costs- [ ] Monitor RPC usage and costs

- [ ] Set up logging and metrics- [ ] Set up logging and metrics



### ✅ Security Considerations  ### ✅ Security Considerations  

- [ ] Validate all user inputs before filtering- [ ] Validate all user inputs before filtering

- [ ] Rate limit API calls if exposing publicly- [ ] Rate limit API calls if exposing publicly

- [ ] Sanitize output before displaying to users- [ ] Sanitize output before displaying to users

- [ ] Keep RPC endpoints in environment variables- [ ] Keep RPC endpoints in environment variables



## 🏗️ Real-World Integration Patterns## 🏗️ Real-World Integration Patterns



### Pattern 1: Validator Directory Service### Pattern 1: Validator Directory Service



```rust```rust

use solana_validator_config::{ValidatorConfigClient, SolanaNetwork};use solana_validator_config::{ValidatorConfigClient, SolanaNetwork};

use std::collections::HashMap;use std::collections::HashMap;

use tokio::time::{Duration, interval};use tokio::time::{Duration, interval};



pub struct ValidatorDirectory {pub struct ValidatorDirectory {

    client: ValidatorConfigClient,    client: ValidatorConfigClient,

    cache: HashMap<String, ValidatorInfo>,    cache: HashMap<String, ValidatorInfo>,

    last_update: std::time::Instant,    last_update: std::time::Instant,

    cache_duration: Duration,    cache_duration: Duration,

}}



impl ValidatorDirectory {impl ValidatorDirectory {

    pub fn new(rpc_url: &str) -> Self {    pub fn new(rpc_url: &str) -> Self {

        Self {        Self {

            client: ValidatorConfigClient::new_custom(rpc_url),            client: ValidatorConfigClient::new_custom(rpc_url),

            cache: HashMap::new(),            cache: HashMap::new(),

            last_update: std::time::Instant::now(),            last_update: std::time::Instant::now(),

            cache_duration: Duration::from_secs(300), // 5 minutes            cache_duration: Duration::from_secs(300), // 5 minutes

        }        }

    }    }



    pub async fn get_validator(&mut self, identity: &str) -> Result<Option<ValidatorInfo>, Box<dyn std::error::Error>> {    pub async fn get_validator(&mut self, identity: &str) -> Result<Option<ValidatorInfo>, Box<dyn std::error::Error>> {

        if self.needs_refresh() {        if self.needs_refresh() {

            self.refresh_cache().await?;            self.refresh_cache().await?;

        }        }

                

        Ok(self.cache.get(identity).cloned())        Ok(self.cache.get(identity).cloned())

    }    }



    pub async fn search_validators(&mut self, query: &str) -> Result<Vec<ValidatorInfo>, Box<dyn std::error::Error>> {    pub async fn search_validators(&mut self, query: &str) -> Result<Vec<ValidatorInfo>, Box<dyn std::error::Error>> {

        if self.needs_refresh() {        if self.needs_refresh() {

            self.refresh_cache().await?;            self.refresh_cache().await?;

        }        }



        let results = self.cache        let results = self.cache

            .values()            .values()

            .filter(|v| {            .filter(|v| {

                v.name.as_ref().map_or(false, |n| n.to_lowercase().contains(&query.to_lowercase())) ||                v.name.as_ref().map_or(false, |n| n.to_lowercase().contains(&query.to_lowercase())) ||

                v.details.as_ref().map_or(false, |d| d.to_lowercase().contains(&query.to_lowercase()))                v.details.as_ref().map_or(false, |d| d.to_lowercase().contains(&query.to_lowercase()))

            })            })

            .cloned()            .cloned()

            .collect();            .collect();



        Ok(results)        Ok(results)

    }    }



    async fn refresh_cache(&mut self) -> Result<(), Box<dyn std::error::Error>> {    async fn refresh_cache(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let validators = self.client.fetch_all_validators().await?;        let validators = self.client.fetch_all_validators().await?;

                

        self.cache.clear();        self.cache.clear();

        for validator in validators {        for validator in validators {

            if let Some(identity) = &validator.validator_identity {            if let Some(identity) = &validator.validator_identity {

                self.cache.insert(identity.clone(), validator);                self.cache.insert(identity.clone(), validator);

            }            }

        }        }

                

        self.last_update = std::time::Instant::now();        self.last_update = std::time::Instant::now();

        Ok(())        Ok(())

    }    }



    fn needs_refresh(&self) -> bool {    fn needs_refresh(&self) -> bool {

        self.last_update.elapsed() > self.cache_duration        self.last_update.elapsed() > self.cache_duration

    }    }

}}

``````



### Pattern 2: Validator Monitoring Dashboard### Pattern 2: Validator Monitoring Dashboard



```rust```rust

use solana_validator_config::{ValidatorConfigClient, SolanaNetwork};use solana_validator_config::{ValidatorConfigClient, SolanaNetwork};

use serde::{Deserialize, Serialize};use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]#[derive(Debug, Serialize, Deserialize)]

pub struct ValidatorHealth {pub struct ValidatorHealth {

    pub identity: String,    pub identity: String,

    pub name: Option<String>,    pub name: Option<String>,

    pub website: Option<String>,    pub website: Option<String>,

    pub last_seen: chrono::DateTime<chrono::Utc>,    pub last_seen: chrono::DateTime<chrono::Utc>,

    pub is_active: bool,    pub is_active: bool,

    pub stake: Option<u64>,    pub stake: Option<u64>,

}}



pub struct ValidatorMonitor {pub struct ValidatorMonitor {

    config_client: ValidatorConfigClient,    config_client: ValidatorConfigClient,

    monitored_validators: Vec<String>,    monitored_validators: Vec<String>,

}}



impl ValidatorMonitor {impl ValidatorMonitor {

    pub fn new(rpc_url: &str, validator_identities: Vec<String>) -> Self {    pub fn new(rpc_url: &str, validator_identities: Vec<String>) -> Self {

        Self {        Self {

            config_client: ValidatorConfigClient::new_custom(rpc_url),            config_client: ValidatorConfigClient::new_custom(rpc_url),

            monitored_validators: validator_identities,            monitored_validators: validator_identities,

        }        }

    }    }



    pub async fn get_health_report(&self) -> Result<Vec<ValidatorHealth>, Box<dyn std::error::Error>> {    pub async fn get_health_report(&self) -> Result<Vec<ValidatorHealth>, Box<dyn std::error::Error>> {

        let all_validators = self.config_client.fetch_all_validators().await?;        let all_validators = self.config_client.fetch_all_validators().await?;

        let mut report = Vec::new();        let mut report = Vec::new();



        for identity in &self.monitored_validators {        for identity in &self.monitored_validators {

            let validator = all_validators            let validator = all_validators

                .iter()                .iter()

                .find(|v| v.validator_identity.as_ref() == Some(identity));                .find(|v| v.validator_identity.as_ref() == Some(identity));



            let health = match validator {            let health = match validator {

                Some(v) => ValidatorHealth {                Some(v) => ValidatorHealth {

                    identity: identity.clone(),                    identity: identity.clone(),

                    name: v.name.clone(),                    name: v.name.clone(),

                    website: v.website.clone(),                    website: v.website.clone(),

                    last_seen: chrono::Utc::now(),                    last_seen: chrono::Utc::now(),

                    is_active: true,                    is_active: true,

                    stake: None, // You'd get this from stake accounts                    stake: None, // You'd get this from stake accounts

                },                },

                None => ValidatorHealth {                None => ValidatorHealth {

                    identity: identity.clone(),                    identity: identity.clone(),

                    name: None,                    name: None,

                    website: None,                    website: None,

                    last_seen: chrono::Utc::now(),                    last_seen: chrono::Utc::now(),

                    is_active: false,                    is_active: false,

                    stake: None,                    stake: None,

                },                },

            };            };



            report.push(health);            report.push(health);

        }        }



        Ok(report)        Ok(report)

    }    }

}}

``````



### Pattern 3: Multi-Network Validator Tracker### Pattern 3: Multi-Network Validator Tracker



```rust```rust

use solana_validator_config::{ValidatorConfigClient, SolanaNetwork};use solana_validator_config::{ValidatorConfigClient, SolanaNetwork};

use std::collections::HashMap;use std::collections::HashMap;



pub struct MultiNetworkTracker {pub struct MultiNetworkTracker {

    clients: HashMap<String, ValidatorConfigClient>,    clients: HashMap<String, ValidatorConfigClient>,

}}



impl MultiNetworkTracker {impl MultiNetworkTracker {

    pub fn new() -> Self {    pub fn new() -> Self {

        let mut clients = HashMap::new();        let mut clients = HashMap::new();

                

        clients.insert("mainnet".to_string(),         clients.insert("mainnet".to_string(), 

            ValidatorConfigClient::new(SolanaNetwork::Mainnet));            ValidatorConfigClient::new(SolanaNetwork::Mainnet));

        clients.insert("testnet".to_string(),         clients.insert("testnet".to_string(), 

            ValidatorConfigClient::new(SolanaNetwork::Testnet));            ValidatorConfigClient::new(SolanaNetwork::Testnet));

        clients.insert("devnet".to_string(),         clients.insert("devnet".to_string(), 

            ValidatorConfigClient::new(SolanaNetwork::Devnet));            ValidatorConfigClient::new(SolanaNetwork::Devnet));



        Self { clients }        Self { clients }

    }    }



    pub fn add_custom_network(&mut self, name: &str, rpc_url: &str) {    pub fn add_custom_network(&mut self, name: &str, rpc_url: &str) {

        self.clients.insert(        self.clients.insert(

            name.to_string(),            name.to_string(),

            ValidatorConfigClient::new_custom(rpc_url)            ValidatorConfigClient::new_custom(rpc_url)

        );        );

    }    }



    pub async fn get_validator_across_networks(&self, identity: &str) -> Result<HashMap<String, Option<ValidatorInfo>>, Box<dyn std::error::Error>> {    pub async fn get_validator_across_networks(&self, identity: &str) -> Result<HashMap<String, Option<ValidatorInfo>>, Box<dyn std::error::Error>> {

        let mut results = HashMap::new();        let mut results = HashMap::new();



        for (network, client) in &self.clients {        for (network, client) in &self.clients {

            let validators = client.fetch_all_validators().await?;            let validators = client.fetch_all_validators().await?;

            let validator = validators            let validator = validators

                .iter()                .iter()

                .find(|v| v.validator_identity.as_ref() == Some(identity))                .find(|v| v.validator_identity.as_ref() == Some(identity))

                .cloned();                .cloned();

                        

            results.insert(network.clone(), validator);            results.insert(network.clone(), validator);

        }        }



        Ok(results)        Ok(results)

    }    }

}}

``````



## 🎯 Advanced Filtering & Analytics## 🎯 Advanced Filtering & Analytics



### Performance-Optimized Filtering### Performance-Optimized Filtering



```rust```rust

use rayon::prelude::*;use rayon::prelude::*;



pub fn analyze_validators_parallel(validators: Vec<ValidatorInfo>) -> ValidatorAnalytics {pub fn analyze_validators_parallel(validators: Vec<ValidatorInfo>) -> ValidatorAnalytics {

    let (with_names, with_websites, with_keybase, total) = validators    let (with_names, with_websites, with_keybase, total) = validators

        .par_iter()        .par_iter()

        .map(|v| (        .map(|v| (

            if v.name.is_some() { 1 } else { 0 },            if v.name.is_some() { 1 } else { 0 },

            if v.website.is_some() { 1 } else { 0 },            if v.website.is_some() { 1 } else { 0 },

            if v.keybase_username.is_some() { 1 } else { 0 },            if v.keybase_username.is_some() { 1 } else { 0 },

            1            1

        ))        ))

        .reduce(        .reduce(

            || (0, 0, 0, 0),            || (0, 0, 0, 0),

            |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)            |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)

        );        );



    ValidatorAnalytics {    ValidatorAnalytics {

        total_validators: total,        total_validators: total,

        with_names,        with_names,

        with_websites,         with_websites, 

        with_keybase,        with_keybase,

        completion_rate: (with_names as f64 / total as f64) * 100.0,        completion_rate: (with_names as f64 / total as f64) * 100.0,

    }    }

}}



#[derive(Debug)]#[derive(Debug)]

pub struct ValidatorAnalytics {pub struct ValidatorAnalytics {

    pub total_validators: usize,    pub total_validators: usize,

    pub with_names: usize,    pub with_names: usize,

    pub with_websites: usize,    pub with_websites: usize,

    pub with_keybase: usize,    pub with_keybase: usize,

    pub completion_rate: f64,    pub completion_rate: f64,

}}

``````



### Smart Caching with Redis### Smart Caching with Redis



```rust```rust

use redis::{Client, Commands};use redis::{Client, Commands};

use serde_json;use serde_json;



pub struct CachedValidatorClient {pub struct CachedValidatorClient {

    config_client: ValidatorConfigClient,    config_client: ValidatorConfigClient,

    redis_client: Client,    redis_client: Client,

    cache_ttl: usize,    cache_ttl: usize,

}}



impl CachedValidatorClient {impl CachedValidatorClient {

    pub fn new(rpc_url: &str, redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {    pub fn new(rpc_url: &str, redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {

        Ok(Self {        Ok(Self {

            config_client: ValidatorConfigClient::new_custom(rpc_url),            config_client: ValidatorConfigClient::new_custom(rpc_url),

            redis_client: Client::open(redis_url)?,            redis_client: Client::open(redis_url)?,

            cache_ttl: 300, // 5 minutes            cache_ttl: 300, // 5 minutes

        })        })

    }    }



    pub async fn fetch_validators_cached(&self) -> Result<Vec<ValidatorInfo>, Box<dyn std::error::Error>> {    pub async fn fetch_validators_cached(&self) -> Result<Vec<ValidatorInfo>, Box<dyn std::error::Error>> {

        let mut conn = self.redis_client.get_connection()?;        let mut conn = self.redis_client.get_connection()?;

        let cache_key = "solana:validators:all";        let cache_key = "solana:validators:all";



        // Try cache first        // Try cache first

        if let Ok(cached_data) = conn.get::<_, String>(cache_key) {        if let Ok(cached_data) = conn.get::<_, String>(cache_key) {

            if let Ok(validators) = serde_json::from_str::<Vec<ValidatorInfo>>(&cached_data) {            if let Ok(validators) = serde_json::from_str::<Vec<ValidatorInfo>>(&cached_data) {

                return Ok(validators);                return Ok(validators);

            }            }

        }        }



        // Cache miss - fetch fresh data        // Cache miss - fetch fresh data

        let validators = self.config_client.fetch_all_validators().await?;        let validators = self.config_client.fetch_all_validators().await?;

                

        // Update cache        // Update cache

        let serialized = serde_json::to_string(&validators)?;        let serialized = serde_json::to_string(&validators)?;

        conn.set_ex(cache_key, serialized, self.cache_ttl)?;        conn.set_ex(cache_key, serialized, self.cache_ttl)?;



        Ok(validators)        Ok(validators)

    }    }

}}

``````



## 🔧 Error Handling Patterns#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {

### Robust Error Handling    // PRODUCTION: Use private RPC endpoint

    let client = ValidatorConfigClient::new_custom("https://your-private-rpc.com");

```rust    

use solana_validator_config::{ValidatorConfigClient, ValidatorConfigError};    // TESTING ONLY: Use public endpoint (unreliable, rate limited)

use tokio::time::{sleep, Duration};    // let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);

    

pub async fn fetch_with_retry(    // Get all validators

    client: &ValidatorConfigClient,    let validators = client.fetch_all_validators().await?;

    max_retries: u32,    

) -> Result<Vec<ValidatorInfo>, ValidatorConfigError> {    println!("Found {} validators", validators.len());

    let mut last_error = None;    

        // Show first 10 with names

    for attempt in 0..=max_retries {    for (pubkey, info) in validators.iter().take(10) {

        match client.fetch_all_validators().await {        if let Some(name) = &info.name {

            Ok(validators) => return Ok(validators),            println!("{}: {}", name, pubkey);

            Err(e) => {        }

                last_error = Some(e);    }

                if attempt < max_retries {    

                    let delay = Duration::from_secs(2_u64.pow(attempt)); // Exponential backoff    Ok(())

                    println!("Attempt {} failed, retrying in {:?}...", attempt + 1, delay);}

                    sleep(delay).await;```

                }

            }### Method 2: Copy the Code

        }

    }If you want to copy the library code directly into your project:

    

    Err(last_error.unwrap())1. **Copy `src/lib.rs` to your project**

}2. **Add dependencies to your `Cargo.toml`:**

``````toml

[dependencies]

### Environment Configurationreqwest = { version = "0.12.23", features = ["json"] }

serde = { version = "1.0", features = ["derive"] }

```rustserde_json = "1.0"

use std::env;base64 = "0.22.1"

tokio = { version = "1.0", features = ["full"] }

pub fn create_production_client() -> Result<ValidatorConfigClient, Box<dyn std::error::Error>> {thiserror = "2.0"

    let rpc_url = env::var("SOLANA_RPC_URL")log = "0.4"

        .or_else(|_| env::var("RPC_URL"))```

        .map_err(|_| "RPC_URL environment variable not set")?;

    ## Common Use Cases

    // Validate URL format

    if !rpc_url.starts_with("http") {### Get All Validator Data

        return Err("RPC_URL must start with http or https".into());```rust

    }let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);

    let validators = client.fetch_all_validators().await?;

    Ok(ValidatorConfigClient::new_custom(&rpc_url))

}// You now have Vec<(String, ValidatorInfo)>

```// String = validator identity key (the actual validator public key)

// ValidatorInfo = all the config data

## 📊 Monitoring & Metrics```



### Performance Metrics### Convert to Your Own Types

```rust

```rust#[derive(Debug)]

use std::time::Instant;struct MyValidator {

    pubkey: String,

pub struct ValidatorMetrics {    name: String,

    pub fetch_duration: Duration,    website: Option<String>,

    pub total_validators: usize,    description: Option<String>,

    pub successful_extractions: usize,}

    pub cache_hit_rate: f64,

}let my_validators: Vec<MyValidator> = validators

    .into_iter()

pub async fn fetch_with_metrics(client: &ValidatorConfigClient) -> Result<(Vec<ValidatorInfo>, ValidatorMetrics), Box<dyn std::error::Error>> {    .filter_map(|(pubkey, info)| {

    let start = Instant::now();        info.name.map(|name| MyValidator {

                pubkey,

    let validators = client.fetch_all_validators().await?;            name,

    let fetch_duration = start.elapsed();            website: info.website,

                description: info.display_description().map(|s| s.to_string()),

    let successful_extractions = validators        })

        .iter()    })

        .filter(|v| v.validator_identity.is_some())    .collect();

        .count();

    println!("Got {} validators with names", my_validators.len());

    let metrics = ValidatorMetrics {```

        fetch_duration,

        total_validators: validators.len(),### Find Specific Validators

        successful_extractions,```rust

        cache_hit_rate: 0.0, // Implement based on your caching strategy// Validators with websites

    };let with_websites: Vec<_> = validators

        .iter()

    Ok((validators, metrics))    .filter(|(_, info)| info.website.is_some())

}    .collect();

```

// Verified validators (have Keybase)

## 🛡️ Production Securitylet verified: Vec<_> = validators

    .iter()

### Input Validation    .filter(|(_, info)| info.keybase_username.is_some())

    .collect();

```rust

pub fn validate_validator_identity(identity: &str) -> Result<(), &'static str> {// Search by name

    if identity.len() < 32 || identity.len() > 44 {let matching: Vec<_> = validators

        return Err("Invalid validator identity length");    .iter()

    }    .filter(|(_, info)| {

            info.name

    if !identity.chars().all(|c| c.is_ascii_alphanumeric()) {            .as_ref()

        return Err("Invalid characters in validator identity");            .map(|name| name.to_lowercase().contains("search_term"))

    }            .unwrap_or(false)

        })

    Ok(())    .collect();

}

println!("Found {} matching validators", matching.len());

pub fn sanitize_search_query(query: &str) -> String {```

    query### Save Data to JSON File

        .chars()```rust

        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || ".-_".contains(*c))use std::fs;

        .take(100) // Limit length

        .collect()let validators = client.fetch_all_validators().await?;

}let json_data = serde_json::to_string_pretty(&validators)?;

```fs::write("validators.json", json_data)?;

println!("Saved {} validators to validators.json", validators.len());

### Rate Limiting```



```rust### Use Different Networks

use tokio::sync::Semaphore;

use std::sync::Arc;**Private RPC (Recommended for Production):**

```rust

pub struct RateLimitedClient {// Any private RPC endpoint

    client: ValidatorConfigClient,let client = ValidatorConfigClient::new_custom("https://your-private-rpc.com");

    semaphore: Arc<Semaphore>,

}// QuickNode

let client = ValidatorConfigClient::new_custom("https://your-endpoint.quiknode.pro/token/");

impl RateLimitedClient {

    pub fn new(rpc_url: &str, max_concurrent_requests: usize) -> Self {// Alchemy

        Self {let client = ValidatorConfigClient::new_custom("https://solana-mainnet.g.alchemy.com/v2/your-api-key");

            client: ValidatorConfigClient::new_custom(rpc_url),

            semaphore: Arc::new(Semaphore::new(max_concurrent_requests)),// Helius

        }let client = ValidatorConfigClient::new_custom("https://rpc.helius.xyz/?api-key=your-api-key");

    }```



    pub async fn fetch_validators(&self) -> Result<Vec<ValidatorInfo>, Box<dyn std::error::Error>> {**Public RPC (Testing Only - Not Recommended for Production):**

        let _permit = self.semaphore.acquire().await?;```rust

        self.client.fetch_all_validators().await.map_err(Into::into)use solana_validator_config::SolanaNetwork;

    }

}// Mainnet (real validators)

```let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);



## 🚀 Deployment Patterns// Testnet

let client = ValidatorConfigClient::new(SolanaNetwork::Testnet);

### Docker Configuration

// Devnet

```dockerfilelet client = ValidatorConfigClient::new(SolanaNetwork::Devnet);

FROM rust:1.70 as builder

WORKDIR /app// Custom public endpoint

COPY . .let client = ValidatorConfigClient::new(

RUN cargo build --release    SolanaNetwork::Custom("https://some-public-rpc.com".to_string())

);

FROM debian:bookworm-slim```

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/your-app /usr/local/bin/your-app### Get Statistics

```rust

ENV SOLANA_RPC_URL=https://api.mainnet-beta.solana.comlet stats = client.get_validator_stats().await?;

ENV RUST_LOG=infoprintln!("Total: {}", stats.total_validators);

println!("With names: {}", stats.with_names);

CMD ["your-app"]println!("With websites: {}", stats.with_websites);

```println!("With Keybase: {}", stats.with_keybase);

```

### Kubernetes Deployment

## Performance Tips

```yaml

apiVersion: apps/v1### 1. Cache Results (Recommended)

kind: DeploymentNetwork calls take 2-5 seconds, so cache the data:

metadata:

  name: validator-service```rust

spec:use std::time::{Duration, Instant};

  replicas: 3

  selector:struct ValidatorCache {

    matchLabels:    data: Option<Vec<(String, ValidatorInfo)>>,

      app: validator-service    last_update: Option<Instant>,

  template:}

    metadata:

      labels:impl ValidatorCache {

        app: validator-service    fn new() -> Self {

    spec:        Self { data: None, last_update: None }

      containers:    }

      - name: validator-service    

        image: your-registry/validator-service:latest    fn is_stale(&self) -> bool {

        env:        self.last_update

        - name: SOLANA_RPC_URL            .map(|time| time.elapsed() > Duration::from_secs(300)) // 5 minutes

          valueFrom:            .unwrap_or(true)

            secretKeyRef:    }

              name: solana-secrets    

              key: rpc-url    async fn get_validators(&mut self) -> Result<&[(String, ValidatorInfo)], Box<dyn std::error::Error>> {

        - name: RUST_LOG        if self.is_stale() {

          value: "info"            let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);

        resources:            self.data = Some(client.fetch_all_validators().await?);

          requests:            self.last_update = Some(Instant::now());

            memory: "128Mi"        }

            cpu: "100m"        

          limits:        Ok(self.data.as_ref().unwrap())

            memory: "512Mi"    }

            cpu: "500m"}

```

// Usage

## 🔍 Troubleshootinglet mut cache = ValidatorCache::new();

let validators = cache.get_validators().await?;

### Common Issues```



1. **Rate Limiting**: Use private RPC endpoints and implement exponential backoff### 2. Filter Early

2. **Memory Usage**: Process validators in chunks for large datasets```rust

3. **Network Timeouts**: Configure appropriate timeouts for your RPC clientuse solana_validator_config::ClientConfig;

4. **Parsing Errors**: Handle corrupted validator data gracefully (library does this automatically)

let config = ClientConfig {

### Debug Mode    max_concurrent_requests: 20,

    timeout_seconds: 30,

```rust    include_empty_configs: false,  // Skip validators with no data

use env_logger;};



fn main() {let client = ValidatorConfigClient::with_config(SolanaNetwork::Mainnet, config);

    env_logger::init();```

    

    // Set RUST_LOG=debug to see detailed parsing information## What Data You Get

    // Library will log when it encounters corrupted data or parsing issues

}Each validator has this information (all optional):

```

```rust

### Health Checkspub struct ValidatorInfo {

    pub name: Option<String>,                    // Display name

```rust    pub website: Option<String>,                 // Website URL  

pub async fn health_check(client: &ValidatorConfigClient) -> Result<bool, Box<dyn std::error::Error>> {    pub details: Option<String>,                 // Description

    match client.get_validator_stats().await {    pub description: Option<String>,             // Alternative description

        Ok(stats) => {    pub keybase_username: Option<String>,        // Keybase verification

            // Basic sanity check - should have hundreds of validators    pub icon_url: Option<String>,                // Logo/icon URL

            Ok(stats.total_validators > 100)    pub domain: Option<String>,                  // Domain name

        }    pub contact: Option<String>,                 // Contact info

        Err(_) => Ok(false),    pub twitter: Option<String>,                 // Twitter handle

    }    pub discord: Option<String>,                 // Discord info

}}

``````



## 📚 Additional Resources**Helper methods:**

- `info.display_name()` - Returns name or keybase_username

- **[Solana Validator Info Spec](https://docs.solana.com/running-validator/validator-info)** - Official specification- `info.display_description()` - Returns details or description  

- **[Config Program](https://github.com/solana-labs/solana/tree/master/programs/config)** - Solana's config program source- `info.has_config()` - True if validator has any data

- **[RPC Providers](https://solana.com/rpc)** - List of Solana RPC providers

## Typical Numbers (Mainnet)

---

- **Total validators:** ~2,800

**Need help?** Open an issue on GitHub or check the examples in the repository.- **With names:** ~2,800 (almost all)
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
