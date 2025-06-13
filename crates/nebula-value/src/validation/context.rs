use std::collections::HashMap;
use crate::validation::ValidationError;
use crate::Value;

/// Context for validation operations
#[derive(Debug, Clone)]
pub struct ValidatorContext {
    /// All parameter values in the current validation context
    pub values: HashMap<String, Value>,

    /// The current field being validated
    pub current_field: String,

    /// Additional context data
    pub metadata: HashMap<String, Value>,
}

impl ValidatorContext {
    /// Creates a new validator context
    pub fn new(values: HashMap<String, Value>, current_field: String) -> Self {
        Self {
            values,
            current_field,
            metadata: HashMap::new(),
        }
    }

    /// Gets a value by field key
    pub fn get_value(&self, key: &String) -> Option<&Value> {
        self.values.get(key)
    }

    /// Gets the current field value
    pub fn current_value(&self) -> Option<&Value> {
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