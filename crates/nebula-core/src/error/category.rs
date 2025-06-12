use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Categorization of errors for better organization and filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ErrorCategory {
    /// General uncategorized errors
    #[serde(rename = "general")]
    General,
    /// Parameter definition and validation errors
    #[serde(rename = "parameter")]
    Parameter,
    /// Workflow execution errors
    #[serde(rename = "workflow")]
    Workflow,
    /// Node processing errors
    #[serde(rename = "node")]
    Node,
    /// Action execution errors
    #[serde(rename = "action")]
    Action,
    /// Credential and authentication data errors
    #[serde(rename = "credential")]
    Credential,
    /// General execution errors
    #[serde(rename = "execution")]
    Execution,
    /// Authentication and authorization errors
    #[serde(rename = "auth")]
    Auth,
    /// Network and external service errors
    #[serde(rename = "network")]
    Network,
    /// File system and storage errors
    #[serde(rename = "storage")]
    Storage,
    /// Configuration errors
    #[serde(rename = "configuration")]
    Configuration,
    /// Internal system errors
    #[serde(rename = "internal")]
    Internal,
}

impl ErrorCategory {
    /// String representation of the category
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::General => "general",
            Self::Parameter => "parameter",
            Self::Workflow => "workflow",
            Self::Node => "node",
            Self::Action => "action",
            Self::Credential => "credential",
            Self::Execution => "execution",
            Self::Auth => "auth",
            Self::Network => "network",
            Self::Storage => "storage",
            Self::Configuration => "configuration",
            Self::Internal => "internal",
        }
    }

    /// Parse from string with case-insensitive matching and common aliases
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "general" | "generic" => Some(Self::General),
            "parameter" | "param" | "validation" => Some(Self::Parameter),
            "workflow" | "pipeline" => Some(Self::Workflow),
            "node" | "component" => Some(Self::Node),
            "action" | "operation" => Some(Self::Action),
            "credential" | "authdata" | "creds" => Some(Self::Credential),
            "execution" | "runtime" => Some(Self::Execution),
            "auth" | "authentication" | "authorization" => Some(Self::Auth),
            "network" | "http" | "api" => Some(Self::Network),
            "storage" | "filesystem" | "db" => Some(Self::Storage),
            "configuration" | "config" | "settings" => Some(Self::Configuration),
            "internal" | "system" => Some(Self::Internal),
            _ => None,
        }
    }

    /// Returns slice of all possible categories
    pub const fn all() -> &'static [Self] {
        &[
            Self::General,
            Self::Parameter,
            Self::Workflow,
            Self::Node,
            Self::Action,
            Self::Credential,
            Self::Execution,
            Self::Auth,
            Self::Network,
            Self::Storage,
            Self::Configuration,
            Self::Internal,
        ]
    }

    /// Checks if this category represents user-facing errors
    pub const fn is_user_facing(&self) -> bool {
        !self.is_internal()
    }

    /// Checks if this category represents internal system errors
    pub const fn is_internal(&self) -> bool {
        matches!(
            self,
            Self::Internal | Self::Execution | Self::Storage | Self::Configuration
        )
    }

    /// Checks if this category represents business logic errors
    pub const fn is_business_logic(&self) -> bool {
        matches!(
            self,
            Self::Workflow | Self::Node | Self::Action | Self::Parameter
        )
    }

    /// Returns true if the error should be reported to monitoring systems
    pub const fn should_report(&self) -> bool {
        !matches!(self, Self::Parameter | Self::Auth)
    }
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ErrorCategory {
    type Err = ParseCategoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or_else(|| ParseCategoryError::new(s))
    }
}

impl Default for ErrorCategory {
    fn default() -> Self {
        Self::General
    }
}

/// Error type for category parsing failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCategoryError {
    input: String,
    valid_categories: &'static [&'static str],
}

impl ParseCategoryError {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            valid_categories: &[
                "general", "parameter", "workflow", "node",
                "action", "credential", "execution", "auth",
                "network", "storage", "configuration", "internal",
            ],
        }
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn valid_categories(&self) -> &'static [&'static str] {
        self.valid_categories
    }
}

impl fmt::Display for ParseCategoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid error category '{}'. Valid categories are: {}",
            self.input,
            self.valid_categories.join(", ")
        )
    }
}

impl std::error::Error for ParseCategoryError {}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("general", Ok(ErrorCategory::General))]
    #[test_case("PARAMETER", Ok(ErrorCategory::Parameter))]
    #[test_case("config", Ok(ErrorCategory::Configuration))]
    #[test_case("invalid", Err(()))]
    fn test_from_str(input: &str, expected: Result<ErrorCategory, ()>) {
        assert_eq!(ErrorCategory::from_str(input).ok_or(()), expected);
    }

    #[test]
    fn test_default() {
        assert_eq!(ErrorCategory::default(), ErrorCategory::General);
    }

    #[test]
    fn test_display() {
        assert_eq!(ErrorCategory::Network.to_string(), "network");
    }

    #[test]
    fn test_is_user_facing() {
        assert!(ErrorCategory::Parameter.is_user_facing());
        assert!(!ErrorCategory::Internal.is_user_facing());
    }

    #[test]
    fn test_parse_error_display() {
        let err = ParseCategoryError::new("invalid");
        assert!(err.to_string().contains("invalid"));
        assert!(err.to_string().contains("general"));
    }
}