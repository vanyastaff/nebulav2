//! String validation errors

use thiserror::Error;

/// Errors for string operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum StringError {
    #[error("String '{value}' does not contain required substring '{substring}'")]
    DoesNotContain { value: String, substring: String },

    #[error("String '{value}' contains forbidden substring '{substring}'")]
    ContainsForbidden { value: String, substring: String },

    #[error("String '{value}' does not start with required prefix '{prefix}'")]
    DoesNotStartWith { value: String, prefix: String },

    #[error("String '{value}' does not end with the required suffix '{suffix}'")]
    DoesNotEndWith { value: String, suffix: String },

    #[error("String is too short: {actual} characters, minimum {min} required")]
    TooShort { actual: usize, min: usize },

    #[error("String is too long: {actual} characters, maximum {max} allowed")]
    TooLong { actual: usize, max: usize },

    #[error("String length is not exact: {actual} characters, expected exactly {expected}")]
    WrongLength { actual: usize, expected: usize },

    #[error("Value is not a string: {actual_type}")]
    NotAString { actual_type: String },

    #[error("String is empty but should not be")]
    UnexpectedlyEmpty,

    #[error("String is not empty but should be")]
    UnexpectedlyNotEmpty,

    #[error("String contains forbidden characters: {chars}")]
    ForbiddenCharacters { chars: String },

    #[error("String encoding is invalid: {reason}")]
    InvalidEncoding { reason: String },
}

impl StringError {
    /// Creates a new "does not contain" error
    pub fn does_not_contain(value: impl Into<String>, substring: impl Into<String>) -> Self {
        Self::DoesNotContain {
            value: value.into(),
            substring: substring.into(),
        }
    }

    /// Creates a new "contains forbidden" error
    pub fn contains_forbidden(value: impl Into<String>, substring: impl Into<String>) -> Self {
        Self::ContainsForbidden {
            value: value.into(),
            substring: substring.into(),
        }
    }

    /// Creates a new "too short" error
    pub fn too_short(actual: usize, min: usize) -> Self {
        Self::TooShort { actual, min }
    }

    /// Creates a new "too long" error
    pub fn too_long(actual: usize, max: usize) -> Self {
        Self::TooLong { actual, max }
    }

    /// Returns the problematic value if available
    pub fn value(&self) -> Option<&str> {
        match self {
            Self::DoesNotContain { value, .. } => Some(value),
            Self::ContainsForbidden { value, .. } => Some(value),
            Self::DoesNotStartWith { value, .. } => Some(value),
            Self::DoesNotEndWith { value, .. } => Some(value),
            _ => None,
        }
    }

    /// Returns true if this is a length-related error
    pub fn is_length_error(&self) -> bool {
        matches!(self, Self::TooShort { .. } | Self::TooLong { .. } | Self::WrongLength { .. })
    }

    /// Returns true if this is a content-related error
    pub fn is_content_error(&self) -> bool {
        matches!(self,
            Self::DoesNotContain { .. } |
            Self::ContainsForbidden { .. } |
            Self::DoesNotStartWith { .. } |
            Self::DoesNotEndWith { .. } |
            Self::ForbiddenCharacters { .. }
        )
    }
}