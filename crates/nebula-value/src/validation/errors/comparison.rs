//! Comparison validation errors

use thiserror::Error;

/// Errors for comparison operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ComparisonError {
    #[error("Value {actual} does not equal expected {expected}")]
    NotEqual { expected: String, actual: String },

    #[error("Value {actual} equals forbidden {forbidden}")]
    Equals { forbidden: String, actual: String },

    #[error("Value {actual} is not greater than {min}")]
    NotGreaterThan { min: String, actual: String },

    #[error("Value {actual} is not greater than or equal to {min}")]
    NotGreaterThanOrEqual { min: String, actual: String },

    #[error("Value {actual} is not less than {max}")]
    NotLessThan { max: String, actual: String },

    #[error("Value {actual} is not less than or equal to {max}")]
    NotLessThanOrEqual { max: String, actual: String },

    #[error("Cannot compare {type1} with {type2}")]
    IncomparableTypes { type1: String, type2: String },

    #[error("Comparison operation '{operation}' is not supported for type {value_type}")]
    UnsupportedOperation { operation: String, value_type: String },
}

impl ComparisonError {
    /// Creates a new "not equal" error
    pub fn not_equal(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::NotEqual {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Creates a new "equals forbidden" error
    pub fn equals(forbidden: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::Equals {
            forbidden: forbidden.into(),
            actual: actual.into(),
        }
    }

    /// Creates a new "not greater than" error
    pub fn not_greater_than(min: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::NotGreaterThan {
            min: min.into(),
            actual: actual.into(),
        }
    }

    /// Creates a new "incomparable types" error
    pub fn incomparable_types(type1: impl Into<String>, type2: impl Into<String>) -> Self {
        Self::IncomparableTypes {
            type1: type1.into(),
            type2: type2.into(),
        }
    }

    /// Returns the actual value that caused the error
    pub fn actual_value(&self) -> Option<&str> {
        match self {
            Self::NotEqual { actual, .. } => Some(actual),
            Self::Equals { actual, .. } => Some(actual),
            Self::NotGreaterThan { actual, .. } => Some(actual),
            Self::NotGreaterThanOrEqual { actual, .. } => Some(actual),
            Self::NotLessThan { actual, .. } => Some(actual),
            Self::NotLessThanOrEqual { actual, .. } => Some(actual),
            _ => None,
        }
    }

    /// Returns true if this is a type compatibility error
    pub fn is_type_error(&self) -> bool {
        matches!(self,
            Self::IncomparableTypes { .. } |
            Self::UnsupportedOperation { .. }
        )
    }
}