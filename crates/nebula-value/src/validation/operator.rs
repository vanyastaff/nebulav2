
//! Type-safe validation operators

use std::fmt::Debug;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Value, validation::{ValidationResult, ValidatorContext}};

/// Trait for custom validation functions
pub trait CustomValidator: Send + Sync + 'static + Debug {
    fn validate(&self, value: &Value, context: &ValidatorContext) -> ValidationResult;
    fn name(&self) -> &str;
    fn clone_box(&self) -> Box<dyn CustomValidator>;
}

// Helper trait for cloning trait objects
impl Clone for Box<dyn CustomValidator> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Type-safe validation operators with specific error types
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
pub enum ValidationOperator {
    Eq(Value),
    NotEq(Value),
    Gt(Value),
    Gte(Value),
    Lt(Value),
    Lte(Value),

    // Set operators
    In(Vec<Value>),
    NotIn(Vec<Value>),

    // String operators
    Contains(String),
    NotContains(String),
    StartsWith(String),
    EndsWith(String),

    // Length operators
    MinLength(usize),
    MaxLength(usize),
    ExactLength(usize),

    // Regex operators
    Matches(String),
    NotMatches(String),

    // Range operators
    Between { min: Value, max: Value },
    NotBetween { min: Value, max: Value },

    // Emptiness operators
    Empty,
    NotEmpty,
    Null,
    NotNull,

    // Numeric operators
    Positive,
    Negative,
    Zero,
    NonZero,

    // Cross-field operators
    EqualsField(String),
    NotEqualsField(String),
    GreaterThanField(String),
    LessThanField(String),

    // Conditional operators
    RequiredIf(String, Box<ValidationOperator>),
    ForbiddenIf(String, Box<ValidationOperator>),

    // Logical operators
    And(Vec<ValidationOperator>),
    Or(Vec<ValidationOperator>),
    Not(Box<ValidationOperator>),

    // Custom validator
    Custom {
        name: String,
        #[cfg_attr(feature = "serde", serde(skip))]
        validator: Option<Box<dyn CustomValidator>>,
    },
}

// Manual PartialEq implementation to handle Custom variant
impl PartialEq for ValidationOperator {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // For Custom variant, compare only by name since trait objects can't be compared
            (Self::Custom { name: name_a, .. }, Self::Custom { name: name_b, .. }) => name_a == name_b,
            // For all other variants, use derived comparison (will be automatically handled)
            _ => std::mem::discriminant(self) == std::mem::discriminant(other) && {
                // This is a bit of a hack, but we'll delegate to the derived comparison
                // by converting to a comparable representation
                format!("{:?}", self) == format!("{:?}", other)
            }
        }
    }
}

impl ValidationOperator {
    // Convenience constructors for common patterns

    /// Creates an AND combination of validators
    pub fn and(validators: Vec<ValidationOperator>) -> Self {
        Self::And(validators)
    }

    /// Creates an OR combination of validators  
    pub fn or(validators: Vec<ValidationOperator>) -> Self {
        Self::Or(validators)
    }

    /// Creates a NOT wrapper
    pub fn not(validator: ValidationOperator) -> Self {
        Self::Not(Box::new(validator))
    }

    /// Creates required validation (not empty and not null)
    pub fn required() -> Self {
        Self::and(vec![Self::NotNull, Self::NotEmpty])
    }

    /// Creates optional validation (can be null, but if not null must pass validation)
    pub fn optional(validator: ValidationOperator) -> Self {
        Self::or(vec![Self::Null, validator])
    }

    /// Creates string length range validation
    pub fn length_between(min: usize, max: usize) -> Self {
        Self::and(vec![
            Self::MinLength(min),
            Self::MaxLength(max),
        ])
    }

    /// Creates numeric range validation
    pub fn range(min: impl Into<Value>, max: impl Into<Value>) -> Self {
        Self::Between {
            min: min.into(),
            max: max.into(),
        }
    }

    /// Creates positive number validation
    pub fn positive_number() -> Self {
        Self::and(vec![
            Self::NotNull,
            Self::Positive,
        ])
    }
}