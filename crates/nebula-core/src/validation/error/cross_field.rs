use thiserror::Error;

/// Errors for cross-field operations
#[derive(Debug, Error, Clone, PartialEq)]
pub enum CrossFieldError {
    #[error("Field '{field}' does not equal field '{other_field}'")]
    FieldsNotEqual { field: String, other_field: String },

    #[error("Field '{field}' equals field '{other_field}' but should not")]
    FieldsEqual { field: String, other_field: String },

    #[error("Field '{field}' is not greater than field '{other_field}'")]
    FieldNotGreater { field: String, other_field: String },

    #[error("Field '{field}' is not less than field '{other_field}'")]
    FieldNotLess { field: String, other_field: String },

    #[error("Referenced field '{field}' not found in validation context")]
    FieldNotFound { field: String },

    #[error("Cannot compare field '{field}' of type {field_type} with field '{other_field}' of type {other_type}")]
    IncompatibleFieldTypes {
        field: String,
        field_type: String,
        other_field: String,
        other_type: String
    },
}

impl CrossFieldError {
    /// Creates a new "fields not equal" error
    pub fn fields_not_equal(field: impl Into<String>, other_field: impl Into<String>) -> Self {
        Self::FieldsNotEqual {
            field: field.into(),
            other_field: other_field.into(),
        }
    }

    /// Creates a new "field not found" error
    pub fn field_not_found(field: impl Into<String>) -> Self {
        Self::FieldNotFound {
            field: field.into(),
        }
    }
}