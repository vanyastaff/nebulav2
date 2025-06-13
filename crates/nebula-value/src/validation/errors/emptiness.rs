//! Emptiness validation errors

use thiserror::Error;

/// Errors for emptiness checks
#[derive(Debug, Error, Clone, PartialEq)]
pub enum EmptinessError {
    #[error("Value is empty but should not be")]
    UnexpectedlyEmpty,

    #[error("Value is not empty but should be")]
    UnexpectedlyNotEmpty,

    #[error("Value is null but should not be")]
    UnexpectedlyNull,

    #[error("Value is not null but should be")]
    UnexpectedlyNotNull,

    #[error("Cannot check emptiness for type {value_type}")]
    UnsupportedType { value_type: String },
}

impl EmptinessError {
    /// Creates a new "unsupported type" error
    pub fn unsupported_type(value_type: impl Into<String>) -> Self {
        Self::UnsupportedType {
            value_type: value_type.into(),
        }
    }
}