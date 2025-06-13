//! Logical validation errors

use thiserror::Error;

/// Errors for logical operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum LogicalError {
    #[error("All conditions in AND operation must be met")]
    AndConditionFailed { failed_conditions: Vec<String> },

    #[error("At least one condition in OR operation must be met")]
    OrConditionFailed { failed_conditions: Vec<String> },

    #[error("NOT condition failed - the negated condition was met")]
    NotConditionFailed { negated_condition: String },

    #[error("Empty condition list provided for logical operation")]
    EmptyConditionList,

    #[error("Logical operation failed: {reason}")]
    LogicalOperationFailed { reason: String },
}

impl LogicalError {
    /// Creates a new "AND condition failed" error
    pub fn and_condition_failed(failed_conditions: Vec<String>) -> Self {
        Self::AndConditionFailed { failed_conditions }
    }

    /// Creates a new "OR condition failed" error
    pub fn or_condition_failed(failed_conditions: Vec<String>) -> Self {
        Self::OrConditionFailed { failed_conditions }
    }

    /// Creates a new "NOT condition failed" error
    pub fn not_condition_failed(negated_condition: impl Into<String>) -> Self {
        Self::NotConditionFailed {
            negated_condition: negated_condition.into(),
        }
    }
}