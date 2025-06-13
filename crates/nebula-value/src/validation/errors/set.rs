//! Set validation errors

use thiserror::Error;

/// Errors for set operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum SetError {
    #[error("Value '{value}' is not in allowed list: {allowed_values}")]
    NotInList { value: String, allowed_values: String },

    #[error("Value '{value}' is in forbidden list: {forbidden_values}")]
    InForbiddenList { value: String, forbidden_values: String },

    #[error("Empty value list provided for validation")]
    EmptyValueList,

    #[error("Cannot compare value of type {value_type} with list values of type {list_type}")]
    IncompatibleTypes { value_type: String, list_type: String },
}

impl SetError {
    /// Creates a new "not in list" error
    pub fn not_in_list(value: impl Into<String>, allowed_values: Vec<String>) -> Self {
        Self::NotInList {
            value: value.into(),
            allowed_values: allowed_values.join(", "),
        }
    }

    /// Creates a new "in forbidden list" error
    pub fn in_forbidden_list(value: impl Into<String>, forbidden_values: Vec<String>) -> Self {
        Self::InForbiddenList {
            value: value.into(),
            forbidden_values: forbidden_values.join(", "),
        }
    }

    /// Creates a new "incompatible types" error
    pub fn incompatible_types(value_type: impl Into<String>, list_type: impl Into<String>) -> Self {
        Self::IncompatibleTypes {
            value_type: value_type.into(),
            list_type: list_type.into(),
        }
    }
}