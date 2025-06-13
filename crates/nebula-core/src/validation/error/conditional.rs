//! Conditional validation errors

use thiserror::Error;

/// Errors for conditional operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ConditionalError {
    #[error("Field '{field}' is required when field '{condition_field}' meets condition")]
    RequiredConditionNotMet { field: String, condition_field: String },

    #[error("Field '{field}' is forbidden when field '{condition_field}' meets condition")]
    ForbiddenConditionMet { field: String, condition_field: String },

    #[error("Condition field '{field}' not found in validation context")]
    ConditionFieldNotFound { field: String },

    #[error("Condition validation failed for field '{field}': {reason}")]
    ConditionValidationFailed { field: String, reason: String },
}

impl ConditionalError {
    /// Creates a new "required condition not met" error
    pub fn required_condition_not_met(field: impl Into<String>, condition_field: impl Into<String>) -> Self {
        Self::RequiredConditionNotMet {
            field: field.into(),
            condition_field: condition_field.into(),
        }
    }

    /// Creates a new "forbidden condition met" error  
    pub fn forbidden_condition_met(field: impl Into<String>, condition_field: impl Into<String>) -> Self {
        Self::ForbiddenConditionMet {
            field: field.into(),
            condition_field: condition_field.into(),
        }
    }
}