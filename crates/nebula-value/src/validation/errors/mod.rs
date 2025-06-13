//! Error types for validation operations
//!
//! Each validation category has its own specific error type for precise
//! error handling and better user experience.

pub mod comparison;
pub mod string;
pub mod regex;
pub mod set;
pub mod range;
pub mod emptiness;
pub mod cross_field;
pub mod conditional;
pub mod logical;

// Re-exports
pub use comparison::ComparisonError;
pub use string::StringError;
pub use regex::RegexError;
pub use set::SetError;
pub use range::RangeError;
pub use emptiness::EmptinessError;
pub use cross_field::CrossFieldError;
pub use conditional::ConditionalError;
pub use logical::LogicalError;

use thiserror::Error;
use crate::ValueError;

/// Comprehensive validation error that wraps all specific error types
#[derive(Debug, Error, Clone)]
pub enum ValidationError {
    #[error("Comparison error for field '{field}': {error}")]
    Comparison { field: String, error: ComparisonError },

    #[error("String operation error for field '{field}': {error}")]
    String { field: String, error: StringError },

    #[error("Regex error for field '{field}': {error}")]
    Regex { field: String, error: RegexError },

    #[error("Set operation error for field '{field}': {error}")]
    Set { field: String, error: SetError },

    #[error("Range error for field '{field}': {error}")]
    Range { field: String, error: RangeError },

    #[error("Emptiness check error for field '{field}': {error}")]
    Emptiness { field: String, error: EmptinessError },

    #[error("Cross-field validation error for field '{field}': {error}")]
    CrossField { field: String, error: CrossFieldError },

    #[error("Conditional validation error for field '{field}': {error}")]
    Conditional { field: String, error: ConditionalError },

    #[error("Logical operation error for field '{field}': {error}")]
    Logical { field: String, error: LogicalError },

    #[error("Value constraint error for field '{field}': {error}")]
    ValueConstraint { field: String, error: ValueError },

    #[error("Type mismatch error for field '{field}': expected {expected_type}, got {actual_type}")]
    TypeMismatch { field: String, expected_type: String, actual_type: String },

    #[error("Unsupported operation for field '{field}': {operation} cannot be applied to {value_type}")]
    UnsupportedOperation { field: String, operation: String, value_type: String },
}

impl ValidationError {
    /// Returns the field that caused the validation error
    pub fn field(&self) -> &String {
        match self {
            Self::Comparison { field, .. } => field,
            Self::String { field, .. } => field,
            Self::Regex { field, .. } => field,
            Self::Set { field, .. } => field,
            Self::Range { field, .. } => field,
            Self::Emptiness { field, .. } => field,
            Self::CrossField { field, .. } => field,
            Self::Conditional { field, .. } => field,
            Self::Logical { field, .. } => field,
            Self::ValueConstraint { field, .. } => field,
            Self::TypeMismatch { field, .. } => field,
            Self::UnsupportedOperation { field, .. } => field,
        }
    }

    /// Returns the error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            Self::Comparison { .. } => "comparison",
            Self::String { .. } => "string",
            Self::Regex { .. } => "regex",
            Self::Set { .. } => "set",
            Self::Range { .. } => "range",
            Self::Emptiness { .. } => "emptiness",
            Self::CrossField { .. } => "cross_field",
            Self::Conditional { .. } => "conditional",
            Self::Logical { .. } => "logical",
            Self::ValueConstraint { .. } => "value_constraint",
            Self::TypeMismatch { .. } => "type_mismatch",
            Self::UnsupportedOperation { .. } => "unsupported_operation",
        }
    }

    /// Returns true if this is a user input error (not a system error)
    pub fn is_user_error(&self) -> bool {
        matches!(self,
            Self::Comparison { .. } |
            Self::String { .. } |
            Self::Regex { .. } |
            Self::Set { .. } |
            Self::Range { .. } |
            Self::Emptiness { .. } |
            Self::CrossField { .. } |
            Self::Conditional { .. } |
            Self::ValueConstraint { .. }
        )
    }

    /// Returns true if this is a system/configuration error
    pub fn is_system_error(&self) -> bool {
        matches!(self,
            Self::TypeMismatch { .. } |
            Self::UnsupportedOperation { .. } |
            Self::Logical { .. }
        )
    }
}