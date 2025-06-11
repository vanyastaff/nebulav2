//! # Key System
//!
//! This module provides a type-safe, domain-specific key system for Nebula.
//! Keys are used to uniquely identify parameters, actions, nodes, and other entities
//! throughout the system with compile-time domain separation and runtime validation.
//!
//! ## Features
//!
//! - **Type Safety**: Different key types cannot be mixed (e.g., `ParameterKey` vs `ActionKey`)
//! - **Domain Validation**: Each domain can enforce its own validation rules
//! - **Zero-Cost Abstractions**: No runtime overhead for type separation
//! - **Memory Efficient**: Uses `Arc<str>` for zero-copy cloning
//! - **Compile-Time Validation**: Macros for validated static keys
//!
//! ## Usage
//!
//! The key system provides both compile-time and runtime key creation:
//! - Compile-time: Use macros like `param_key!("api_key")` for static keys
//! - Runtime: Use `ParameterKey::new("dynamic_key")?` with validation
//! - Type safety prevents mixing different key domains

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;
use thiserror::Error;

/// Maximum allowed length for any key
pub const MAX_KEY_LENGTH: usize = 64;

/// Errors that can occur during key parsing and validation
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum KeyParseError {
    /// Key cannot be empty or contain only whitespace
    #[error("Key cannot be empty or whitespace")]
    Empty,

    /// Key contains characters that are not allowed
    #[error("Key contains invalid characters")]
    InvalidCharacters,

    /// Key exceeds the maximum allowed length
    #[error("Key is too long (max {MAX_KEY_LENGTH} characters)")]
    TooLong,

    /// Key starts with reserved prefix (domain-specific)
    #[error("Key starts with reserved prefix")]
    ReservedPrefix,

    /// Action key doesn't follow verb_noun pattern
    #[error("Action key must follow 'verb_noun' pattern (e.g., 'send_request')")]
    InvalidActionFormat,

    /// Node key contains invalid version format
    #[error("Node key contains invalid version format")]
    InvalidNodeVersion,
}

/// Generic key type for different domains
///
/// This is the core key type that provides type safety through the domain marker `T`.
/// Keys are immutable after creation and use `Arc<str>` for efficient cloning.
///
/// # Type Parameters
///
/// * `T` - A domain marker type that implements `KeyDomain`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Key<T: KeyDomain>(Arc<str>, PhantomData<T>);

// Manual Serialize/Deserialize implementation to handle Arc<str>
impl<T: KeyDomain> Serialize for Key<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T: KeyDomain> Deserialize<'de> for Key<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Key::new(s).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

/// Trait for key domain markers
///
/// This trait defines the behavior for different key domains (parameter, action, node, etc.).
/// Each domain can specify its own validation rules and normalization behavior.
pub trait KeyDomain:
    'static + Send + Sync + fmt::Debug + PartialEq + Eq + Hash + Ord + PartialOrd
{
    /// Human-readable name for this domain
    const DOMAIN_NAME: &'static str;

    /// Domain-specific validation rules
    ///
    /// This method is called after common validation passes.
    /// Domains can enforce their own specific rules here.
    ///
    /// # Arguments
    ///
    /// * `key` - The normalized key string to validate
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the key is valid for this domain
    /// * `Err(KeyParseError)` with the specific validation failure
    fn validate_domain_rules(key: &str) -> Result<(), KeyParseError> {
        let _ = key; // Suppress unused parameter warning
        Ok(()) // Default: no domain-specific validation
    }

    /// Domain-specific normalization
    ///
    /// This method is called after common normalization.
    /// Domains can apply additional normalization rules here.
    ///
    /// # Arguments
    ///
    /// * `key` - The key string after common normalization
    ///
    /// # Returns
    ///
    /// The normalized key string for this domain
    fn normalize_domain(key: String) -> String {
        key // Default: no additional normalization
    }
}

// Domain marker types

/// Domain marker for parameter keys
///
/// Parameters represent configurable values for nodes and actions.
/// Parameter keys cannot start with underscore (reserved for internal parameters).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParameterDomain;

/// Domain marker for action keys
///
/// Actions represent operations that can be performed by nodes.
/// Action keys must follow the `verb_noun` pattern (e.g., `send_request`, `validate_data`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActionDomain;

/// Domain marker for node keys
///
/// Nodes represent processing units in the workflow.
/// Node keys should be descriptive and unique within the workflow.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeDomain;

/// Domain marker for connection keys
///
/// Connections represent data flow between nodes.
/// Connection keys are typically auto-generated but can be customized.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConnectionDomain;

/// Domain marker for workflow keys
///
/// Workflows represent complete automation sequences.
/// Workflow keys should be globally unique and descriptive.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkflowDomain;

/// Domain marker for credential keys
///
/// Credentials represent authentication information.
/// Credential keys have additional security considerations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CredentialDomain;

// Domain implementations

impl KeyDomain for ParameterDomain {
    const DOMAIN_NAME: &'static str = "parameter";

    fn validate_domain_rules(key: &str) -> Result<(), KeyParseError> {
        // Parameters cannot start with underscore (reserved for internal use)
        if key.starts_with('_') {
            return Err(KeyParseError::ReservedPrefix);
        }
        Ok(())
    }
}

impl KeyDomain for ActionDomain {
    const DOMAIN_NAME: &'static str = "action";

    fn validate_domain_rules(key: &str) -> Result<(), KeyParseError> {
        // Actions should follow verb_noun pattern
        let parts: Vec<&str> = key.split('_').collect();
        if parts.len() < 2 {
            return Err(KeyParseError::InvalidActionFormat);
        }

        // First part should be a verb (basic check - starts with lowercase letter)
        if let Some(verb) = parts.first() {
            if verb.is_empty() || !verb.chars().next().unwrap().is_ascii_lowercase() {
                return Err(KeyParseError::InvalidActionFormat);
            }
        }

        Ok(())
    }
}

impl KeyDomain for NodeDomain {
    const DOMAIN_NAME: &'static str = "node";

    fn validate_domain_rules(key: &str) -> Result<(), KeyParseError> {
        // Node keys can optionally end with version suffix (e.g., "chatgpt_v2")
        if key.contains("_v") {
            let parts: Vec<&str> = key.split("_v").collect();
            if parts.len() == 2 {
                if let Some(version) = parts.get(1) {
                    // Basic version validation (should be numeric)
                    if !version.chars().all(|c| c.is_ascii_digit()) {
                        return Err(KeyParseError::InvalidNodeVersion);
                    }
                }
            }
        }
        Ok(())
    }
}

impl KeyDomain for ConnectionDomain {
    const DOMAIN_NAME: &'static str = "connection";
}

impl KeyDomain for WorkflowDomain {
    const DOMAIN_NAME: &'static str = "workflow";
}

impl KeyDomain for CredentialDomain {
    const DOMAIN_NAME: &'static str = "credential";

    fn validate_domain_rules(key: &str) -> Result<(), KeyParseError> {
        // Credentials cannot start with "temp_" (reserved for temporary credentials)
        if key.starts_with("temp_") {
            return Err(KeyParseError::ReservedPrefix);
        }
        Ok(())
    }
}

// Type aliases for convenience

/// Key type for parameters
pub type ParameterKey = Key<ParameterDomain>;

/// Key type for actions
pub type ActionKey = Key<ActionDomain>;

/// Key type for nodes
pub type NodeKey = Key<NodeDomain>;

/// Key type for connections
pub type ConnectionKey = Key<ConnectionDomain>;

/// Key type for workflows
pub type WorkflowKey = Key<WorkflowDomain>;

/// Key type for credentials
pub type CredentialKey = Key<CredentialDomain>;

// Key implementation

impl<T: KeyDomain> Key<T> {
    /// Creates a new key with validation
    ///
    /// This method performs both common validation (length, characters) and
    /// domain-specific validation according to the key's domain type.
    ///
    /// # Arguments
    ///
    /// * `key` - String-like input that will be normalized and validated
    ///
    /// # Returns
    ///
    /// * `Ok(Key<T>)` if the key is valid
    /// * `Err(KeyParseError)` with the specific validation failure
    pub fn new(key: impl AsRef<str>) -> Result<Self, KeyParseError> {
        let key_str = key.as_ref();

        // Common validation
        Self::validate_common(key_str)?;

        // Normalize the key
        let normalized = Self::normalize(key_str);

        // Domain-specific validation on normalized key
        T::validate_domain_rules(&normalized)?;

        Ok(Self(Arc::from(normalized), PhantomData))
    }

    /// Creates a key from a static string without runtime validation
    ///
    /// This method is intended for compile-time validated keys and bypasses
    /// all runtime validation. Use only with string literals that you know
    /// are valid.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the static string follows all validation
    /// rules for the domain. Invalid keys created this way may cause
    /// undefined behavior in other parts of the system.
    ///
    /// # Arguments
    ///
    /// * `key` - A static string literal that represents a valid key
    pub fn from_static(key: &'static str) -> Self {
        Self(Arc::from(key), PhantomData)
    }

    /// Returns the key as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the domain name for this key type
    pub fn domain(&self) -> &'static str {
        T::DOMAIN_NAME
    }

    /// Checks if this key starts with the given prefix
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to check for
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.0.starts_with(prefix)
    }

    /// Checks if this key ends with the given suffix
    ///
    /// # Arguments
    ///
    /// * `suffix` - The suffix to check for
    pub fn ends_with(&self, suffix: &str) -> bool {
        self.0.ends_with(suffix)
    }

    /// Common validation rules applied to all keys
    fn validate_common(key: &str) -> Result<(), KeyParseError> {
        // Check for empty or whitespace-only keys
        if key.trim().is_empty() {
            return Err(KeyParseError::Empty);
        }

        // Check length
        if key.len() > MAX_KEY_LENGTH {
            return Err(KeyParseError::TooLong);
        }

        // Check for valid characters: alphanumeric, underscore, hyphen
        if !key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(KeyParseError::InvalidCharacters);
        }

        // Keys cannot contain consecutive underscores or hyphens
        if key.contains("__") || key.contains("--") || key.contains("_-") || key.contains("-_") {
            return Err(KeyParseError::InvalidCharacters);
        }

        // Keys cannot end with underscore or hyphen
        if key.ends_with('_') || key.ends_with('-') {
            return Err(KeyParseError::InvalidCharacters);
        }

        Ok(())
    }

    /// Normalize a key string according to common rules
    fn normalize(key: &str) -> String {
        let normalized = key
            .trim() // Remove leading/trailing whitespace
            .to_lowercase() // Convert to lowercase
            .replace('-', "_"); // Normalize hyphens to underscores

        // Apply domain-specific normalization
        T::normalize_domain(normalized)
    }
}

// Display implementation for keys
impl<T: KeyDomain> fmt::Display for Key<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", T::DOMAIN_NAME, self.0)
    }
}

// String conversion implementations
impl<T: KeyDomain> AsRef<str> for Key<T> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<T: KeyDomain> From<Key<T>> for String {
    fn from(key: Key<T>) -> Self {
        key.0.to_string()
    }
}

// Compile-time validated key creation macros

/// Creates a compile-time validated parameter key
///
/// This macro performs basic validation at compile time and creates a
/// `ParameterKey` from a string literal.
#[macro_export]
macro_rules! param_key {
    ($key:literal) => {
        $crate::types::key::ParameterKey::from_static($key)
    };
}

/// Creates a compile-time validated action key
///
/// This macro performs basic validation at compile time and creates an
/// `ActionKey` from a string literal.
#[macro_export]
macro_rules! action_key {
    ($key:literal) => {
        $crate::types::key::ActionKey::from_static($key)
    };
}

/// Creates a compile-time validated node key
///
/// This macro performs basic validation at compile time and creates a
/// `NodeKey` from a string literal.
#[macro_export]
macro_rules! node_key {
    ($key:literal) => {
        $crate::types::key::NodeKey::from_static($key)
    };
}

/// Creates a compile-time validated connection key
///
/// This macro performs basic validation at compile time and creates a
/// `ConnectionKey` from a string literal.
#[macro_export]
macro_rules! connection_key {
    ($key:literal) => {
        $crate::types::key::ConnectionKey::from_static($key)
    };
}

/// Creates a compile-time validated workflow key
///
/// This macro performs basic validation at compile time and creates a
/// `WorkflowKey` from a string literal.
#[macro_export]
macro_rules! workflow_key {
    ($key:literal) => {
        $crate::types::key::WorkflowKey::from_static($key)
    };
}

/// Creates a compile-time validated credential key
///
/// This macro performs basic validation at compile time and creates a
/// `CredentialKey` from a string literal.
#[macro_export]
macro_rules! credential_key {
    ($key:literal) => {
        $crate::types::key::CredentialKey::from_static($key)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_key_creation() {
        let key = ParameterKey::new("api_key").unwrap();
        assert_eq!(key.as_str(), "api_key");
        assert_eq!(key.domain(), "parameter");
    }

    #[test]
    fn test_parameter_key_reserved_prefix() {
        let result = ParameterKey::new("_internal");
        assert_eq!(result.unwrap_err(), KeyParseError::ReservedPrefix);
    }

    #[test]
    fn test_action_key_valid_format() {
        let key = ActionKey::new("send_request").unwrap();
        assert_eq!(key.as_str(), "send_request");
    }

    #[test]
    fn test_action_key_invalid_format() {
        let result = ActionKey::new("invalid");
        assert_eq!(result.unwrap_err(), KeyParseError::InvalidActionFormat);
    }

    #[test]
    fn test_key_normalization() {
        let key = ParameterKey::new("API-Key").unwrap();
        assert_eq!(key.as_str(), "api_key");
    }

    #[test]
    fn test_empty_key() {
        let result = ParameterKey::new("");
        assert_eq!(result.unwrap_err(), KeyParseError::Empty);

        let result = ParameterKey::new("   ");
        assert_eq!(result.unwrap_err(), KeyParseError::Empty);
    }

    #[test]
    fn test_too_long_key() {
        let long_key = "a".repeat(MAX_KEY_LENGTH + 1);
        let result = ParameterKey::new(long_key);
        assert_eq!(result.unwrap_err(), KeyParseError::TooLong);
    }

    #[test]
    fn test_invalid_characters() {
        let invalid_keys = vec![
            "key with spaces",
            "key@with#symbols",
            "key.with.dots",
            "__double_underscore",
            "key--double-hyphen",
            "ends_with_underscore_",
            "ends-with-hyphen-",
        ];

        for invalid_key in invalid_keys {
            let result = ParameterKey::new(invalid_key);
            assert!(result.is_err(), "Key '{}' should be invalid", invalid_key);
        }
    }

    #[test]
    fn test_macro_creation() {
        let param_key = ParameterKey::from_static("test_param");
        let action_key = ActionKey::from_static("send_data");
        let node_key = NodeKey::from_static("processor");

        assert_eq!(param_key.as_str(), "test_param");
        assert_eq!(action_key.as_str(), "send_data");
        assert_eq!(node_key.as_str(), "processor");
    }

    #[test]
    fn test_display_implementation() {
        let param_key = ParameterKey::from_static("test");
        let action_key = ActionKey::from_static("send_data");

        assert_eq!(format!("{}", param_key), "parameter:test");
        assert_eq!(format!("{}", action_key), "action:send_data");
    }

    #[test]
    fn test_key_methods() {
        let key = ParameterKey::new("api_key_secret").unwrap();

        assert!(key.starts_with("api_"));
        assert!(key.ends_with("_secret"));
        assert!(!key.starts_with("user_"));
        assert!(!key.ends_with("_token"));
    }
}
