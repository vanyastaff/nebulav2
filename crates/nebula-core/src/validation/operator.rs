//! Type-safe validation operators

use crate::parameter::ParameterKey;
use crate::value::Value;
use serde::{Deserialize, Serialize};
use crate::validation::{ValidationResult, ValidatorContext};

/// Type-safe validation operators with specific error types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ValidationOperator {
    // Comparison operators
    #[serde(rename = "eq")]
    Eq(Value),
    #[serde(rename = "not_eq")]
    NotEq(Value),
    #[serde(rename = "gt")]
    Gt(Value),
    #[serde(rename = "gte")]
    Gte(Value),
    #[serde(rename = "lt")]
    Lt(Value),
    #[serde(rename = "lte")]
    Lte(Value),

    // Set operators
    #[serde(rename = "in")]
    In(Vec<Value>),
    #[serde(rename = "not_in")]
    NotIn(Vec<Value>),

    // String operators
    #[serde(rename = "contains")]
    Contains(String),
    #[serde(rename = "not_contains")]
    NotContains(String),
    #[serde(rename = "starts_with")]
    StartsWith(String),
    #[serde(rename = "ends_with")]
    EndsWith(String),

    // Length operators
    #[serde(rename = "min_length")]
    MinLength(usize),
    #[serde(rename = "max_length")]
    MaxLength(usize),
    #[serde(rename = "exact_length")]
    ExactLength(usize),

    // Regex operators
    #[serde(rename = "matches")]
    Matches(String),
    #[serde(rename = "not_matches")]
    NotMatches(String),

    // Range operators
    #[serde(rename = "between")]
    Between { min: Value, max: Value },
    #[serde(rename = "not_between")]
    NotBetween { min: Value, max: Value },

    // Emptiness operators
    #[serde(rename = "is_empty")]
    IsEmpty,
    #[serde(rename = "is_not_empty")]
    IsNotEmpty,
    #[serde(rename = "is_null")]
    IsNull,
    #[serde(rename = "is_not_null")]
    IsNotNull,

    // Numeric operators
    #[serde(rename = "positive")]
    Positive,
    #[serde(rename = "negative")]
    Negative,
    #[serde(rename = "zero")]
    Zero,
    #[serde(rename = "non_zero")]
    NonZero,

    // Format operators (presets)
    #[serde(rename = "is_email")]
    IsEmail,
    #[serde(rename = "is_url")]
    IsUrl,
    #[serde(rename = "is_uuid")]
    IsUuid,
    #[serde(rename = "is_phone")]
    IsPhoneNumber,
    #[serde(rename = "is_ip")]
    IsIpAddress,

    // Cross-field operators
    #[serde(rename = "equals_field")]
    EqualsField(ParameterKey),
    #[serde(rename = "not_equals_field")]
    NotEqualsField(ParameterKey),
    #[serde(rename = "greater_than_field")]
    GreaterThanField(ParameterKey),
    #[serde(rename = "less_than_field")]
    LessThanField(ParameterKey),

    // Conditional operators
    #[serde(rename = "required_if")]
    RequiredIf(ParameterKey, Box<ValidationOperator>),
    #[serde(rename = "forbidden_if")]
    ForbiddenIf(ParameterKey, Box<ValidationOperator>),

    // Logical operators
    #[serde(rename = "and")]
    And(Vec<ValidationOperator>),
    #[serde(rename = "or")]
    Or(Vec<ValidationOperator>),
    #[serde(rename = "not")]
    Not(Box<ValidationOperator>),

    // Custom validator
    #[serde(rename = "custom")]
    Custom {
        name: String,
        #[serde(skip)]
        validator: Option<Box<dyn CustomValidator>>,
    },
}

/// Trait for custom validation functions
pub trait CustomValidator: Send + Sync {
    fn validate(&self, value: &Value, context: &ValidatorContext) -> ValidationResult;
    fn name(&self) -> &str;
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
        Self::and(vec![Self::IsNotNull, Self::IsNotEmpty])
    }

    /// Creates optional validation (can be null, but if not null must pass validation)
    pub fn optional(validator: ValidationOperator) -> Self {
        Self::or(vec![Self::IsNull, validator])
    }

    /// Creates email validation
    pub fn email() -> Self {
        Self::and(vec![
            Self::required(),
            Self::IsEmail,
            Self::MaxLength(254), // RFC 5321 limit
        ])
    }

    /// Creates URL validation
    pub fn url() -> Self {
        Self::and(vec![
            Self::required(),
            Self::IsUrl,
            Self::MaxLength(2048), // Reasonable URL limit
        ])
    }

    /// Creates UUID validation
    pub fn uuid() -> Self {
        Self::and(vec![
            Self::required(),
            Self::IsUuid,
        ])
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
            Self::IsNotNull,
            Self::Positive,
        ])
    }

    /// Creates strong password validation
    pub fn strong_password() -> Self {
        Self::and(vec![
            Self::MinLength(8),
            Self::MaxLength(128),
            Self::Matches("^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d)(?=.*[@$!%*?&])[A-Za-z\\d@$!%*?&]".to_string()),
        ])
    }
}