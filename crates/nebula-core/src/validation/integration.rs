//! Integration with Parameter and Value systems

use crate::parameter::{ParameterKey, ParameterValue};
use crate::value::Value;
use crate::validation::{ValidationError, ValidationOperator};
use std::collections::HashMap;

/// Context for validation operations
#[derive(Debug, Clone)]
pub struct ValidatorContext {
    /// All parameter values in the current validation context
    pub values: HashMap<ParameterKey, ParameterValue>,

    /// The current field being validated
    pub current_field: ParameterKey,

    /// Additional context data
    pub metadata: HashMap<String, Value>,
}

impl ValidatorContext {
    /// Creates a new validator context
    pub fn new(values: HashMap<ParameterKey, ParameterValue>, current_field: ParameterKey) -> Self {
        Self {
            values,
            current_field,
            metadata: HashMap::new(),
        }
    }

    /// Gets a value by field key
    pub fn get_value(&self, key: &ParameterKey) -> Option<&ParameterValue> {
        self.values.get(key)
    }

    /// Gets the current field value
    pub fn current_value(&self) -> Option<&ParameterValue> {
        self.get_value(&self.current_field)
    }

    /// Adds metadata to the context
    pub fn with_metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Result type for validation operations
pub type ValidationResult = Result<(), ValidationError>;

/// Trait for types that can be validated
pub trait Validatable {
    /// Validates the value using the given operator and context
    fn validate(&self, operator: &ValidationOperator, context: &ValidatorContext) -> ValidationResult;
}

impl Validatable for ParameterValue {
    fn validate(&self, operator: &ValidationOperator, context: &ValidatorContext) -> ValidationResult {
        match self {
            ParameterValue::Value(value) => value.validate(operator, context),
            ParameterValue::Expression(_) => {
                // TODO: Handle expression validation
                // For now, expressions are considered valid
                Ok(())
            }
            ParameterValue::Mode(_) => {
                // TODO: Handle mode validation
                Ok(())
            }
            ParameterValue::Group(_) => {
                // TODO: Handle group validation
                Ok(())
            }
            ParameterValue::Expirable(_) => {
                // TODO: Handle expirable validation
                Ok(())
            }
        }
    }
}

impl Validatable for Value {
    fn validate(&self, operator: &ValidationOperator, context: &ValidatorContext) -> ValidationResult {
        // This is where the actual validation logic will be implemented
        // For now, return Ok to compile
        Ok(())
    }
}