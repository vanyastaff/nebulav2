use serde::{Deserialize, Serialize};
use std::fmt;

/// Error severity levels with const support
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ErrorSeverity {
    Critical,
    Error,
    Warning,
    Info,
    Debug,
}

impl ErrorSeverity {
    /// Returns string representation
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Debug => "debug",
        }
    }

    /// Priority level (higher = more severe)
    pub const fn priority(&self) -> u8 {
        match self {
            Self::Critical => 5,
            Self::Error => 4,
            Self::Warning => 3,
            Self::Info => 2,
            Self::Debug => 1,
        }
    }

    // Const instances
    pub const CRITICAL: Self = Self::Critical;
    pub const ERROR: Self = Self::Error;
    pub const WARNING: Self = Self::Warning;
    pub const INFO: Self = Self::Info;
    pub const DEBUG: Self = Self::Debug;
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}