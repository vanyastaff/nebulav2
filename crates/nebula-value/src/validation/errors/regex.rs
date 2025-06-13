//! Regex validation errors

use thiserror::Error;

/// Errors for regex operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum RegexError {
    #[error("String '{value}' does not match regex pattern '{pattern}'")]
    PatternMismatch { value: String, pattern: String },

    #[error("String '{value}' matches forbidden regex pattern '{pattern}'")]
    PatternMatch { value: String, pattern: String },

    #[error("Invalid regex pattern '{pattern}': {reason}")]
    InvalidPattern { pattern: String, reason: String },

    #[error("Regex compilation failed for pattern '{pattern}': {error}")]
    CompilationFailed { pattern: String, error: String },

    #[error("Regex execution timeout for a pattern '{pattern}' on input '{input}'")]
    ExecutionTimeout { pattern: String, input: String },

    #[error("Empty regex pattern provided")]
    EmptyPattern,
}

impl RegexError {
    /// Creates a new pattern mismatch error
    pub fn pattern_mismatch(value: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self::PatternMismatch {
            value: value.into(),
            pattern: pattern.into(),
        }
    }

    /// Creates a new pattern match error (for not_matches validation)
    pub fn pattern_match(value: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self::PatternMatch {
            value: value.into(),
            pattern: pattern.into(),
        }
    }

    /// Creates a new invalid pattern error
    pub fn invalid_pattern(pattern: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidPattern {
            pattern: pattern.into(),
            reason: reason.into(),
        }
    }

    /// Creates a new compilation failed error
    pub fn compilation_failed(pattern: impl Into<String>, error: impl Into<String>) -> Self {
        Self::CompilationFailed {
            pattern: pattern.into(),
            error: error.into(),
        }
    }

    /// Returns the regex pattern that caused the error
    pub fn pattern(&self) -> Option<&str> {
        match self {
            Self::PatternMismatch { pattern, .. } => Some(pattern),
            Self::PatternMatch { pattern, .. } => Some(pattern),
            Self::InvalidPattern { pattern, .. } => Some(pattern),
            Self::CompilationFailed { pattern, .. } => Some(pattern),
            Self::ExecutionTimeout { pattern, .. } => Some(pattern),
            Self::EmptyPattern => None,
        }
    }

    /// Returns true if this is a pattern definition error
    pub fn is_pattern_error(&self) -> bool {
        matches!(self,
            Self::InvalidPattern { .. } |
            Self::CompilationFailed { .. } |
            Self::EmptyPattern
        )
    }

    /// Returns true if this is a pattern matching error
    pub fn is_matching_error(&self) -> bool {
        matches!(self,
            Self::PatternMismatch { .. } |
            Self::PatternMatch { .. } |
            Self::ExecutionTimeout { .. }
        )
    }
}