//! Range validation errors

use thiserror::Error;

/// Errors for range operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum RangeError {
    #[error("Value {value} is not between {min} and {max}")]
    NotInRange { value: String, min: String, max: String },

    #[error("Value {value} is between {min} and {max} but should not be")]
    InForbiddenRange { value: String, min: String, max: String },

    #[error("Invalid range: minimum {min} is greater than maximum {max}")]
    InvalidRange { min: String, max: String },

    #[error("Cannot perform range validation on type {value_type}")]
    UnsupportedType { value_type: String },

    #[error("Value {value} is not positive")]
    NotPositive { value: String },

    #[error("Value {value} is not negative")]
    NotNegative { value: String },

    #[error("Value {value} is not zero")]
    NotZero { value: String },

    #[error("Value {value} is zero but should not be")]
    IsZero { value: String },
}

impl RangeError {
    /// Creates a new "not in range" error
    pub fn not_in_range(value: impl Into<String>, min: impl Into<String>, max: impl Into<String>) -> Self {
        Self::NotInRange {
            value: value.into(),
            min: min.into(),
            max: max.into(),
        }
    }

    /// Creates a new "in forbidden range" error
    pub fn in_forbidden_range(value: impl Into<String>, min: impl Into<String>, max: impl Into<String>) -> Self {
        Self::InForbiddenRange {
            value: value.into(),
            min: min.into(),
            max: max.into(),
        }
    }

    /// Creates a new "invalid range" error
    pub fn invalid_range(min: impl Into<String>, max: impl Into<String>) -> Self {
        Self::InvalidRange {
            min: min.into(),
            max: max.into(),
        }
    }

    /// Creates a new "not positive" error
    pub fn not_positive(value: impl Into<String>) -> Self {
        Self::NotPositive {
            value: value.into(),
        }
    }
}