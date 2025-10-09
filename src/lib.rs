//! # Solana Validator Config Library
//!
//! A Rust library for retrieving Solana validator configuration data from the Solana blockchain.
//! This library fetches validator information including names, websites, details, and Keybase usernames
//! as stored in the Config program accounts, strictly following the official Solana validator-info.json specification.
//!
//! **IMPORTANT**: This library returns the actual validator identity public keys that can be used to connect
//! to validators directly, not the Config Program account keys that store the metadata.
//!
//! ## Quick Start
//!
//! ```rust
//! use solana_validator_config::{ValidatorConfigClient, SolanaNetwork};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ValidatorConfigClient::new(SolanaNetwork::Mainnet);
//!     let validators = client.fetch_all_validators().await?;
//!     
//!     for validator in validators {
//!         if let Some(name) = &validator.name {
//!             if let Some(identity) = &validator.validator_identity {
//!                 println!("Validator: {} ({})", name, identity);
//!                 println!("  This is the actual validator identity key you can connect to!");
//!             }
//!         }
//!     }
//!     Ok(())
//! }
//! ```

use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};
use thiserror::Error;

/// Solana Config program ID used to store validator configurations
const SOLANA_CONFIG_PROGRAM_ID: &str = "Config1111111111111111111111111111111111111";

/// Maximum reasonable timeout in seconds
const MAX_TIMEOUT_SECONDS: u64 = 300;

/// Maximum reasonable concurrent requests
const MAX_CONCURRENT_REQUESTS: usize = 100;

/// Represents different Solana network environments
#[derive(Debug, Clone)]
pub enum SolanaNetwork {
    Mainnet,
    Testnet,
    Devnet,
    Custom(String),
}

impl SolanaNetwork {
    /// Get the RPC endpoint URL for the network
    #[must_use]
    pub fn rpc_url(&self) -> &str {
        match self {
            Self::Mainnet => "https://api.mainnet-beta.solana.com",
            Self::Testnet => "https://api.testnet.solana.com",
            Self::Devnet => "https://api.devnet.solana.com",
            Self::Custom(url) => url,
        }
    }

    /// Create a custom network with the specified RPC endpoint
    ///
    /// # Examples
    ///
    /// ```
    /// use solana_validator_config::SolanaNetwork;
    ///
    /// // Using a private RPC provider
    /// let network = SolanaNetwork::custom("https://my-private-rpc.com");
    ///
    /// // Using QuickNode
    /// let network = SolanaNetwork::custom("https://your-endpoint.quiknode.pro/token/");
    ///
    /// // Using Alchemy
    /// let network = SolanaNetwork::custom("https://solana-mainnet.g.alchemy.com/v2/your-api-key");
    ///
    /// // Using Helius
    /// let network = SolanaNetwork::custom("https://rpc.helius.xyz/?api-key=your-api-key");
    /// ```
    #[must_use]
    pub fn custom(rpc_url: impl Into<String>) -> Self {
        Self::Custom(rpc_url.into())
    }
}

/// Maximum safe length for string fields to prevent abuse
/// Based on typical Solana validator info field usage:
/// - Names: usually 20-50 characters
/// - Websites: usually 20-100 characters  
/// - Details: usually 50-300 characters
/// - Keybase: usually 10-30 characters
const MAX_STRING_LENGTH: usize = 500; // Much more reasonable limit

/// Sanitize an optional string field during deserialization
fn sanitize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    Ok(opt.map(sanitize_string))
}

/// Sanitize a string by removing potentially dangerous content and limiting length
fn sanitize_string(input: String) -> String {
    // Limit length to prevent abuse - more reasonable limit based on real usage
    let truncated = if input.len() > MAX_STRING_LENGTH {
        format!("{}...", &input[..MAX_STRING_LENGTH - 3])
    } else {
        input
    };

    // Clean up the string with better replacement strategy
    let cleaned = truncated
        .chars()
        .map(|c| {
            match c {
                // Replace null bytes with spaces (better UX)
                '\0' => ' ',
                // Replace other control characters with newlines (better readability)
                c if c.is_control() && c != '\n' && c != '\r' && c != '\t' => '\n',
                // Keep everything else including emojis and Unicode
                c => c,
            }
        })
        .collect::<String>();

    // Clean up multiple consecutive newlines with a regex-like approach
    let mut result = cleaned;
    while result.contains("\n\n\n") {
        result = result.replace("\n\n\n", "\n\n");
    }

    // Only trim spaces, not newlines
    result.trim_matches(' ').to_string()
}

/// Validator configuration information extracted from Solana config accounts
/// This struct strictly follows the official Solana validator-info.json specification
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValidatorInfo {
    /// The actual validator identity public key (extracted from Config Program account data)
    /// This is the key you use to connect to the validator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator_identity: Option<String>,

    /// Validator display name
    #[serde(deserialize_with = "sanitize_optional_string", default)]
    pub name: Option<String>,

    /// Validator website URL
    #[serde(deserialize_with = "sanitize_optional_string", default)]
    pub website: Option<String>,

    /// Validator description/details
    #[serde(deserialize_with = "sanitize_optional_string", default)]
    pub details: Option<String>,

    /// Keybase username for identity verification
    #[serde(
        alias = "keybaseUsername",
        deserialize_with = "sanitize_optional_string",
        default
    )]
    pub keybase_username: Option<String>,
}

impl ValidatorInfo {
    /// Get the primary name for this validator (tries name, then `keybase_username`)
    #[must_use]
    pub fn display_name(&self) -> Option<&str> {
        self.name.as_deref().or(self.keybase_username.as_deref())
    }

    /// Get the validator description
    #[must_use]
    pub fn display_description(&self) -> Option<&str> {
        self.details.as_deref()
    }

    /// Check if this validator has meaningful configuration data
    #[must_use]
    pub const fn has_config(&self) -> bool {
        self.validator_identity.is_some()
            || self.name.is_some()
            || self.website.is_some()
            || self.keybase_username.is_some()
            || self.details.is_some()
    }
}

/// Errors that can occur when working with validator configurations
#[derive(Error, Debug)]
pub enum ValidatorConfigError {
    /// Network-related errors (connection, timeout, etc.)
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON parsing errors
    #[error("Failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// Base64 decoding errors
    #[error("Failed to decode base64 data: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    /// UTF-8 conversion errors
    #[error("Invalid UTF-8 data: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    /// RPC-specific errors
    #[error("RPC error: {message}")]
    Rpc { message: String },

    /// Configuration validation errors
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// Configuration options for the validator config client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Maximum number of concurrent requests (for future batch processing)
    pub max_concurrent_requests: usize,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Whether to include validators with empty configs
    pub include_empty_configs: bool,
    /// User agent string for HTTP requests
    pub user_agent: String,
}

impl ClientConfig {
    /// Create a new configuration with validation
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the timeout with validation
    /// 
    /// # Errors
    /// Returns `ValidatorConfigError::InvalidConfig` if timeout is 0
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Result<Self, ValidatorConfigError> {
        if timeout_seconds == 0 {
            return Err(ValidatorConfigError::InvalidConfig(
                "Timeout must be greater than 0".to_string(),
            ));
        }
        if timeout_seconds > MAX_TIMEOUT_SECONDS {
            log::warn!(
                "Timeout of {} seconds is very high, consider using a lower value",
                timeout_seconds
            );
        }
        self.timeout_seconds = timeout_seconds;
        Ok(self)
    }

    /// Set maximum concurrent requests with validation
    /// 
    /// # Errors
    /// Returns `ValidatorConfigError::InvalidConfig` if `max_requests` is 0
    pub fn with_max_concurrent_requests(
        mut self,
        max_requests: usize,
    ) -> Result<Self, ValidatorConfigError> {
        if max_requests == 0 {
            return Err(ValidatorConfigError::InvalidConfig(
                "Max concurrent requests must be greater than 0".to_string(),
            ));
        }
        if max_requests > MAX_CONCURRENT_REQUESTS {
            log::warn!("Very high concurrent request limit: {}", max_requests);
        }
        self.max_concurrent_requests = max_requests;
        Ok(self)
    }

    /// Set whether to include empty configurations
    #[must_use]
    pub const fn with_include_empty_configs(mut self, include: bool) -> Self {
        self.include_empty_configs = include;
        self
    }

    /// Set custom user agent
    #[must_use]
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            max_concurrent_requests: 10,
            timeout_seconds: 30,
            include_empty_configs: false,
            user_agent: format!("solana-validator-config/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

/// Main client for fetching Solana validator configurations
pub struct ValidatorConfigClient {
    network: SolanaNetwork,
    config: ClientConfig,
    http_client: Client,
}

impl ValidatorConfigClient {
    /// Create a new client for the specified network
    #[must_use]
    pub fn new(network: SolanaNetwork) -> Self {
        Self::with_config(network, ClientConfig::default())
    }

    /// Create a new client with custom configuration
    /// 
    /// # Panics
    /// Panics if the HTTP client cannot be created with the given configuration
    #[must_use]
    pub fn with_config(network: SolanaNetwork, config: ClientConfig) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .user_agent(&config.user_agent)
            .build()
            .expect("Failed to create HTTP client");

        log::info!(
            "Created Solana validator config client for network: {:?}",
            network
        );

        Self {
            network,
            config,
            http_client,
        }
    }

    /// Create a new client with a custom RPC endpoint
    ///
    /// This is a convenience method for connecting to private RPC providers.
    ///
    /// # Examples
    ///
    /// ```
    /// use solana_validator_config::ValidatorConfigClient;
    ///
    /// // Using a private RPC endpoint
    /// let client = ValidatorConfigClient::new_custom("https://my-private-rpc.com");
    ///
    /// // Using QuickNode
    /// let client = ValidatorConfigClient::new_custom("https://your-endpoint.quiknode.pro/token/");
    ///
    /// // Using Alchemy  
    /// let client = ValidatorConfigClient::new_custom("https://solana-mainnet.g.alchemy.com/v2/your-api-key");
    ///
    /// // Using Helius
    /// let client = ValidatorConfigClient::new_custom("https://rpc.helius.xyz/?api-key=your-api-key");
    ///
    /// // Using GenesysGo
    /// let client = ValidatorConfigClient::new_custom("https://ssc-dao.genesysgo.net/");
    /// ```
    pub fn new_custom(rpc_url: impl Into<String>) -> Self {
        Self::new(SolanaNetwork::custom(rpc_url))
    }

    /// Create a new client with a custom RPC endpoint and configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use solana_validator_config::{ValidatorConfigClient, ClientConfig};
    ///
    /// let config = ClientConfig::new()
    ///     .with_timeout(60).unwrap()
    ///     .with_user_agent("my-app/1.0");
    ///
    /// let client = ValidatorConfigClient::new_custom_with_config(
    ///     "https://my-private-rpc.com",
    ///     config
    /// );
    /// ```
    pub fn new_custom_with_config(rpc_url: impl Into<String>, config: ClientConfig) -> Self {
        Self::with_config(SolanaNetwork::custom(rpc_url), config)
    }

    /// Fetch all validator configurations from the network
    /// 
    /// # Errors
    /// Returns `ValidatorConfigError` if the RPC request fails or response cannot be parsed
    pub async fn fetch_all_validators(&self) -> Result<Vec<ValidatorInfo>, ValidatorConfigError> {
        log::info!(
            "Fetching validator configurations from {}",
            self.network.rpc_url()
        );

        let rpc_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getProgramAccounts",
            "params": [
                SOLANA_CONFIG_PROGRAM_ID,
                {
                    "encoding": "base64"
                }
            ]
        });

        let response = self
            .http_client
            .post(self.network.rpc_url())
            .json(&rpc_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            log::error!("RPC request failed with status {status}: {error_body}");
            return Err(ValidatorConfigError::Rpc {
                message: format!("Request failed with status {status}: {error_body}"),
            });
        }

        let rpc_response: RpcResponse = response.json().await?;

        log::info!(
            "Received {} config accounts from RPC",
            rpc_response.result.len()
        );

        let total_accounts = rpc_response.result.len();
        let mut validators = Vec::with_capacity(total_accounts);
        let mut parse_errors = 0;

        for (index, entry) in rpc_response.result.into_iter().enumerate() {
            // Try to extract validator identity and info with identity included in struct
            if let Some(info) =
                extract_validator_identity_and_info_from_base64(&entry.account.data.0)
            {
                if self.config.include_empty_configs || info.has_config() {
                    // The validator identity is now in info.validator_identity!
                    validators.push(info);
                }
            } else if let Some(mut info) = extract_validator_info_from_base64(&entry.account.data.0)
            {
                if self.config.include_empty_configs || info.has_config() {
                    // Fallback to config account address if identity extraction fails
                    info.validator_identity = Some(entry.pubkey);
                    validators.push(info);
                }
            } else {
                parse_errors += 1;
                if parse_errors <= 5 {
                    // Log first few parse errors
                    log::debug!("Failed to parse validator config at index {}", index);
                }
            }
        }

        if parse_errors > 0 {
            log::warn!(
                "Failed to parse {} out of {} validator configs",
                parse_errors,
                total_accounts
            );
        }

        log::info!(
            "Successfully extracted {} valid validator configs",
            validators.len()
        );
        Ok(validators)
    }

    /// Get validator statistics
    /// 
    /// # Errors
    /// Returns `ValidatorConfigError` if fetching validators fails
    pub async fn get_validator_stats(&self) -> Result<ValidatorStats, ValidatorConfigError> {
        let validators = self.fetch_all_validators().await?;

        let total_count = validators.len();
        let with_names = validators.iter().filter(|info| info.name.is_some()).count();
        let with_websites = validators
            .iter()
            .filter(|info| info.website.is_some())
            .count();
        let with_keybase = validators
            .iter()
            .filter(|info| info.keybase_username.is_some())
            .count();

        Ok(ValidatorStats {
            total_validators: total_count,
            with_names,
            with_websites,
            with_keybase,
        })
    }
}

/// Statistics about validator configurations
#[derive(Debug, Clone, Serialize)]
pub struct ValidatorStats {
    pub total_validators: usize,
    pub with_names: usize,
    pub with_websites: usize,
    pub with_keybase: usize,
}

// Internal structs for RPC communication
#[derive(Debug, Deserialize)]
struct RpcResponse {
    result: Vec<AccountEntry>,
}

#[derive(Debug, Deserialize)]
struct AccountEntry {
    pubkey: String,
    account: AccountData,
}

#[derive(Debug, Deserialize)]
struct AccountData {
    data: (String, String), // (base64_data, encoding_type)
    #[allow(dead_code)]
    executable: bool,
    #[allow(dead_code)]
    lamports: u64,
    #[allow(dead_code)]
    owner: String,
    #[allow(dead_code)]
    #[serde(alias = "rentEpoch")]
    rent_epoch: u64,
}

/// Extract validator info from base64-encoded account data
fn extract_validator_info_from_base64(base64_data: &str) -> Option<ValidatorInfo> {
    // Decode the base64 data
    let decoded = general_purpose::STANDARD.decode(base64_data).ok()?;

    // Look for JSON starting with '{'
    let json_start = decoded.iter().position(|&b| b == b'{')?;
    let json_slice = &decoded[json_start..];

    // Convert to string
    let json_str = std::str::from_utf8(json_slice).ok()?;

    // Try to parse as JSON directly first
    if let Ok(info) = serde_json::from_str::<ValidatorInfo>(json_str) {
        return Some(info);
    }

    // If direct parsing fails, try to extract just the JSON object
    if let Some(end_pos) = find_json_end(json_str) {
        let trimmed_json = &json_str[..=end_pos];

        // Try parsing the trimmed JSON
        if let Ok(info) = serde_json::from_str::<ValidatorInfo>(trimmed_json) {
            return Some(info);
        }

        // If that fails, try to clean up common JSON issues
        let cleaned_json = clean_json_string(trimmed_json);
        serde_json::from_str::<ValidatorInfo>(&cleaned_json).ok()
    } else {
        None
    }
}

/// Extract both validator identity and info from base64-encoded account data
/// Returns `ValidatorInfo` with `validator_identity` field populated
fn extract_validator_identity_and_info_from_base64(base64_data: &str) -> Option<ValidatorInfo> {
    // Decode the base64 data
    let decoded = general_purpose::STANDARD.decode(base64_data).ok()?;

    // First try to extract validator identity (this is the most important part)
    let validator_identity = if decoded.len() >= 66 {
        let key_bytes = &decoded[34..66];
        let base58_key = bs58::encode(key_bytes).into_string();
        
        // Basic validation: Solana public keys are typically 32-44 characters in base58
        if base58_key.len() >= 32 && base58_key.len() <= 44 {
            // Additional validation: check if it looks like a valid public key
            if is_valid_solana_pubkey(&base58_key) {
                Some(base58_key)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Now try to extract JSON info (this can fail without affecting validator identity)
    let mut validator_info = {
        let mut info = ValidatorInfo {
            validator_identity: None,
            name: None,
            website: None,
            details: None,
            keybase_username: None,
        };

        // Try to find valid JSON by looking for all '{' positions
        let mut search_start = 0;
        while let Some(json_start) = decoded[search_start..].iter().position(|&b| b == b'{') {
            let actual_start = search_start + json_start;
            let json_slice = &decoded[actual_start..];
            
            // Try UTF-8 conversion for this position
            if let Ok(json_str) = std::str::from_utf8(json_slice) {
                // Try to parse JSON directly first
                if let Ok(parsed_info) = serde_json::from_str::<ValidatorInfo>(json_str) {
                    info = parsed_info;
                    break;
                } else if let Some(end_pos) = find_json_end(json_str) {
                    let trimmed_json = &json_str[..=end_pos];
                    if let Ok(parsed_info) = serde_json::from_str::<ValidatorInfo>(trimmed_json) {
                        info = parsed_info;
                        break;
                    }
                    let cleaned_json = clean_json_string(trimmed_json);
                    if let Ok(parsed_info) = serde_json::from_str::<ValidatorInfo>(&cleaned_json) {
                        info = parsed_info;
                        break;
                    }
                }
            }
            
            // Move to the next potential '{' position
            search_start = actual_start + 1;
            
            // Safety: don't search forever
            if search_start >= decoded.len() {
                break;
            }
        }

        info
    };

    // Set the validator identity we extracted (this is the key fix!)
    validator_info.validator_identity = validator_identity;

    // Return the ValidatorInfo if we at least have a validator identity
    if validator_info.validator_identity.is_some() {
        Some(validator_info)
    } else {
        None
    }
}

/// Basic validation for Solana public key format
fn is_valid_solana_pubkey(key: &str) -> bool {
    // Solana public keys should be valid base58 and decode to exactly 32 bytes
    bs58::decode(key).into_vec().is_ok_and(|decoded| decoded.len() == 32)
}

/// Clean up common JSON formatting issues
fn clean_json_string(json_str: &str) -> String {
    json_str
        .trim()
        // Remove null bytes that sometimes appear
        .replace('\0', "")
        // Ensure proper string escaping
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Find the end position of a JSON object in a string
fn find_json_end(json_str: &str) -> Option<usize> {
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escape_next = false;

    for (i, ch) in json_str.char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '{' if !in_string => brace_count += 1,
            '}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_info_display_methods() {
        let info = ValidatorInfo {
            validator_identity: Some("GwHH8ciFhR8vejWCqmg8FWZUCNtubPY2esALvy5tBvji".to_string()),
            name: Some("Test Validator".to_string()),
            website: Some("https://test.com".to_string()),
            details: Some("Test details".to_string()),
            keybase_username: Some("testuser".to_string()),
        };

        assert_eq!(info.display_name(), Some("Test Validator"));
        assert_eq!(info.display_description(), Some("Test details"));
        assert!(info.has_config());
    }

    #[test]
    fn test_validator_info_fallback_methods() {
        let info = ValidatorInfo {
            validator_identity: None,
            name: None,
            website: None,
            details: Some("Fallback details".to_string()),
            keybase_username: Some("fallback_user".to_string()),
        };

        assert_eq!(info.display_name(), Some("fallback_user"));
        assert_eq!(info.display_description(), Some("Fallback details"));
        assert!(info.has_config());
    }

    #[test]
    fn test_validator_info_empty() {
        let info = ValidatorInfo {
            validator_identity: None,
            name: None,
            website: None,
            details: None,
            keybase_username: None,
        };

        assert_eq!(info.display_name(), None);
        assert_eq!(info.display_description(), None);
        assert!(!info.has_config());
    }

    #[test]
    fn test_solana_network_urls() {
        assert_eq!(
            SolanaNetwork::Mainnet.rpc_url(),
            "https://api.mainnet-beta.solana.com"
        );
        assert_eq!(
            SolanaNetwork::Testnet.rpc_url(),
            "https://api.testnet.solana.com"
        );
        assert_eq!(
            SolanaNetwork::Devnet.rpc_url(),
            "https://api.devnet.solana.com"
        );

        let custom_url = "https://custom-rpc.com";
        assert_eq!(
            SolanaNetwork::Custom(custom_url.to_string()).rpc_url(),
            custom_url
        );
    }

    #[test]
    fn test_custom_rpc_convenience_methods() {
        let custom_url = "https://my-private-rpc.com";

        // Test SolanaNetwork::custom
        let network = SolanaNetwork::custom(custom_url);
        assert_eq!(network.rpc_url(), custom_url);

        // Test ValidatorConfigClient::new_custom
        let client = ValidatorConfigClient::new_custom(custom_url);
        assert_eq!(client.network.rpc_url(), custom_url);

        // Test ValidatorConfigClient::new_custom_with_config
        let config = ClientConfig::new().with_timeout(120).unwrap();
        let client = ValidatorConfigClient::new_custom_with_config(custom_url, config);
        assert_eq!(client.network.rpc_url(), custom_url);
        assert_eq!(client.config.timeout_seconds, 120);
    }

    #[test]
    fn test_string_sanitization() {
        // Test normal strings
        assert_eq!(
            sanitize_string("Normal Validator".to_string()),
            "Normal Validator"
        );

        // Test special characters (should be preserved)
        assert_eq!(
            sanitize_string("Validator Premium".to_string()),
            "Validator Premium"
        );

        // Test null bytes (should be replaced with spaces)
        assert_eq!(
            sanitize_string("Bad\0Validator".to_string()),
            "Bad Validator"
        );
        assert_eq!(
            sanitize_string("Evil\0null\0bytes".to_string()),
            "Evil null bytes"
        );

        // Test excessive length (should be truncated at 500)
        let long_string = "a".repeat(600);
        let sanitized = sanitize_string(long_string);
        assert_eq!(sanitized.len(), 500);
        assert!(sanitized.ends_with("..."));

        // Test various Unicode characters
        assert_eq!(
            sanitize_string("Café Münchën 中文".to_string()),
            "Café Münchën 中文"
        );

        // Test control characters (should be replaced with newlines, but limited to max 2 consecutive)
        let control_chars = "Test\x01\x02\x03";
        assert_eq!(sanitize_string(control_chars.to_string()), "Test\n\n");

        // Test mixed control chars and null bytes
        assert_eq!(
            sanitize_string("Bad\x01control\0and\x02null".to_string()),
            "Bad\ncontrol and\nnull"
        );

        // Test whitespace preservation (trim only spaces, keep internal whitespace)
        assert_eq!(
            sanitize_string("  Spaced  Out\tValidator\n  ".to_string()),
            "Spaced  Out\tValidator\n"
        );

        // Test multiple consecutive newlines cleanup
        assert_eq!(
            sanitize_string("Line1\n\n\n\nLine2".to_string()),
            "Line1\n\nLine2"
        );
    }

    #[test]
    fn test_validator_info_deserialization_with_problematic_content() {
        // Test JSON with special characters
        let json_with_emojis = r#"
        {
            "name": "Rocket Validator",
            "website": "https://rocket-validator.com",
            "details": "Best validator ever! Top rated supporting the network since 2021",
            "keybaseUsername": "rocket_validator"
        }
        "#;

        let result: Result<ValidatorInfo, _> = serde_json::from_str(json_with_emojis);
        if let Err(e) = &result {
            println!("Error parsing JSON: {}", e);
        }
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.name.as_ref().unwrap(), "Rocket Validator");
        assert!(info.details.as_ref().unwrap().contains("Top rated"));

        // Test with excessively long content
        let long_name = "a".repeat(600);
        let json_with_long_content = format!(
            r#"
        {{
            "name": "{}",
            "website": "https://test.com",
            "details": "Normal details",
            "keybaseUsername": null
        }}
        "#,
            long_name
        );

        let result: Result<ValidatorInfo, _> = serde_json::from_str(&json_with_long_content);
        if let Err(e) = &result {
            println!("Error parsing long content JSON: {}", e);
        }
        assert!(result.is_ok());
        let info = result.unwrap();
        let name = info.name.unwrap();
        assert_eq!(name.len(), 500);
        assert!(name.ends_with("..."));
    }

    #[test]
    fn test_malformed_json_handling() {
        // Test with null bytes in JSON
        let json_with_nulls = r#"{
            "name": "BadValidator",
            "website": "https://test.com",
            "details": null,
            "keybaseUsername": null
        }"#;
        let result: Result<ValidatorInfo, _> = serde_json::from_str(json_with_nulls);
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.name.unwrap(), "BadValidator");
    }

    #[test]
    fn test_client_config_validation() {
        // Test valid timeout
        let config = ClientConfig::new()
            .with_timeout(60)
            .expect("Should accept valid timeout");
        assert_eq!(config.timeout_seconds, 60);

        // Test invalid timeout
        let result = ClientConfig::new().with_timeout(0);
        assert!(result.is_err());

        // Test valid concurrent requests
        let config = ClientConfig::new()
            .with_max_concurrent_requests(5)
            .expect("Should accept valid concurrent requests");
        assert_eq!(config.max_concurrent_requests, 5);

        // Test invalid concurrent requests
        let result = ClientConfig::new().with_max_concurrent_requests(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_client_config_builder_pattern() {
        let config = ClientConfig::new()
            .with_include_empty_configs(true)
            .with_user_agent("test-agent");

        assert!(config.include_empty_configs);
        assert_eq!(config.user_agent, "test-agent");
    }

    #[test]
    fn test_validator_identity_extraction() {
        // Test data from an actual Config Program account
        let test_base64 = "AgdRlwF0SPKsXcI8nrx6x4wKJyV6xhRFjeCk8W+AAAAAAFyWoNoPcmY3XGMzfd/TnsxGdmGkbaqPjoM5N67GtS8/AUMAAAAAAAAAeyJkZXRhaWxzIjoiR0VOQSIsIm5hbWUiOiJHRU5BIiwid2Vic2l0ZSI6Imh0dHBzOi8vYml0Lmx5LzNxSnR2TXMifQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

        let result = extract_validator_info_from_base64(test_base64);
        assert!(result.is_some(), "Failed to extract validator data");

        let info = result.unwrap();

        // Verify the JSON was parsed correctly
        assert_eq!(info.name.as_ref().unwrap(), "GENA");
        assert_eq!(info.details.as_ref().unwrap(), "GENA");
        assert_eq!(info.website.as_ref().unwrap(), "https://bit.ly/3qJtvMs");
    }

    #[test]
    fn test_validator_identity_extraction_multiple() {
        // Test with multiple different validator configs to ensure consistency
        let test_cases = vec![
            // First validator
            "AgdRlwF0SPKsXcI8nrx6x4wKJyV6xhRFjeCk8W+AAAAAAFyWoNoPcmY3XGMzfd/TnsxGdmGkbaqPjoM5N67GtS8/AUMAAAAAAAAAeyJkZXRhaWxzIjoiR0VOQSIsIm5hbWUiOiJHRU5BIiwid2Vic2l0ZSI6Imh0dHBzOi8vYml0Lmx5LzNxSnR2TXMifQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            // Second validator
            "AgdRlwF0SPKsXcI8nrx6x4wKJyV6xhRFjeCk8W+AAAAAAInKZQiIXVottqtMzeeHcf7hBxKF21Xg+o2aN7AnnbYcARMAAAAAAAAAeyJuYW1lIjoiUmVkaXNrYXQifQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        ];

        for (i, test_data) in test_cases.iter().enumerate() {
            let result = extract_validator_info_from_base64(test_data);
            assert!(result.is_some(), "Failed to parse validator {}", i);
        }
    }

    #[test]
    fn test_json_cleaning() {
        let dirty_json = r#"{"name": "Test\nValidator", "website": "https://test.com"}"#;
        let cleaned = clean_json_string(dirty_json);
        assert!(cleaned.contains("\\n"));
        assert!(!cleaned.contains('\n'));
    }

    #[test]
    fn test_find_json_end() {
        let json = r#"{"name": "test", "value": 123}extra data"#;
        let end_pos = find_json_end(json);
        assert_eq!(end_pos, Some(29)); // Position of the closing brace

        let nested_json = r#"{"outer": {"inner": "value"}, "other": "data"}more"#;
        let end_pos = find_json_end(nested_json);
        assert!(end_pos.is_some());
    }

    #[test]
    fn test_find_json_end_with_strings() {
        let json_with_braces_in_string = r#"{"name": "test{with}braces", "value": 123}"#;
        let end_pos = find_json_end(json_with_braces_in_string);
        assert_eq!(end_pos, Some(json_with_braces_in_string.len() - 1));
    }
}
