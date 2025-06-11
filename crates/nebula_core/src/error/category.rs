use serde::{Deserialize, Serialize};

/// Error categories for better organization and filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// General uncategorized errors
    General,
    /// Parameter definition and validation errors
    Parameter,
    /// Workflow execution errors
    Workflow,
    /// Node processing errors
    Node,
    /// Action execution errors
    Action,
    /// Credential and authentication data errors
    Credential,
    /// General execution errors
    Execution,
    /// Authentication and authorization errors
    Auth,
    /// Network and external service errors
    Network,
    /// File system and storage errors
    Storage,
    /// Configuration errors
    Configuration,
    /// Internal system errors
    Internal,
}

impl ErrorCategory {
    /// Returns the category as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCategory::General => "general",
            ErrorCategory::Parameter => "parameter",
            ErrorCategory::Workflow => "workflow",
            ErrorCategory::Node => "node",
            ErrorCategory::Action => "action",
            ErrorCategory::Credential => "credential",
            ErrorCategory::Execution => "execution",
            ErrorCategory::Auth => "auth",
            ErrorCategory::Network => "network",
            ErrorCategory::Storage => "storage",
            ErrorCategory::Configuration => "configuration",
            ErrorCategory::Internal => "internal",
        }
    }

    /// Returns a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            ErrorCategory::General => "General system errors",
            ErrorCategory::Parameter => "Parameter validation and configuration",
            ErrorCategory::Workflow => "Workflow definition and orchestration",
            ErrorCategory::Node => "Node operation and computation",
            ErrorCategory::Action => "Action execution and processing",
            ErrorCategory::Credential => "Credential management and validation",
            ErrorCategory::Execution => "Runtime execution errors",
            ErrorCategory::Auth => "Authentication and authorization",
            ErrorCategory::Network => "Network communication and external services",
            ErrorCategory::Storage => "File system and data storage",
            ErrorCategory::Configuration => "System configuration and settings",
            ErrorCategory::Internal => "Internal system errors",
        }
    }
}

impl std::fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}