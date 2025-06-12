// crates/nebula_core/src/value/error.rs

use thiserror::Error;
use crate::error::{ErrorSeverity, HasSeverity};

/// Errors that can occur when working with Value types
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ValueError {
    /// Invalid regex pattern
    #[error("Invalid regex pattern '{pattern}': {error}")]
    InvalidRegex { pattern: String, error: String },

    /// Invalid number format
    #[error("Invalid number format: {input}")]
    InvalidNumber { input: String },

    /// Number out of valid range
    #[error("Number out of range: {value} (valid range: {min}..={max})")]
    NumberOutOfRange {
        value: String,
        min: String,
        max: String,
    },

    /// Division by zero
    #[error("Division by zero")]
    DivisionByZero,
    
    #[error("Invalid boolean: {input}")]
    InvalidBoolean { input: String },

    /// Invalid date/time format
    #[error("Invalid date format: {input}")]
    InvalidDate { input: String },

    /// Invalid time format
    #[error("Invalid time format: {input}")]
    InvalidTime { input: String },

    /// Invalid datetime format
    #[error("Invalid datetime format: {input}")]
    InvalidDateTime { input: String },

    /// Invalid duration format
    #[error("Invalid duration format: {input}")]
    InvalidDuration { input: String },

    /// Invalid expression syntax
    #[error("Invalid expression syntax: {input} - {reason}")]
    InvalidExpression { input: String, reason: String },

    /// Expression variable not found
    #[error("Expression variable '{variable}' not found in context")]
    ExpressionVariableNotFound { variable: String },

    /// Expression evaluation error
    #[error("Expression evaluation failed: {reason}")]
    ExpressionEvaluationFailed { reason: String },

    /// Type conversion error
    #[error("Type conversion failed: cannot convert {from_type} to {to_type}")]
    TypeConversion { from_type: String, to_type: String },

    /// Type conversion with value details
    #[error("Type conversion failed: cannot convert {from_type} '{value}' to {to_type}")]
    TypeConversionWithValue {
        from_type: String,
        to_type: String,
        value: String,
    },

    /// Array index out of bounds
    #[error("Array index out of bounds: index {index}, length {length}")]
    IndexOutOfBounds { index: usize, length: usize },

    /// Object key not found
    #[error("Object key '{key}' not found")]
    KeyNotFound { key: String },

    /// Invalid enum variant
    #[error("Invalid enum variant '{variant}' for type {enum_name}")]
    InvalidEnumVariant { variant: String, enum_name: String },

    /// Invalid UTF-8 sequence
    #[error("Invalid UTF-8 sequence: {reason}")]
    InvalidUtf8 { reason: String },

    /// Binary data decoding error
    #[error("Binary data decoding failed: {reason}")]
    BinaryDecodingFailed { reason: String },

    /// Binary data encoding error
    #[error("Binary data encoding failed: {reason}")]
    BinaryEncodingFailed { reason: String },

    /// JSON serialization error
    #[error("JSON serialization failed: {reason}")]
    JsonSerialization { reason: String },

    /// JSON deserialization error
    #[error("JSON deserialization failed: {reason}")]
    JsonDeserialization { reason: String },

    /// Invalid format for specific value type
    #[error("Invalid {value_type} format: {input}")]
    InvalidFormat { value_type: String, input: String },

    /// Operation not supported for this value type
    #[error("Operation '{operation}' not supported for {value_type}")]
    UnsupportedOperation {
        operation: String,
        value_type: String,
    },

    /// Comparison not possible between different types
    #[error("Cannot compare {left_type} with {right_type}")]
    IncompatibleComparison {
        left_type: String,
        right_type: String,
    },

    /// Custom validation error
    #[error("Validation failed: {reason}")]
    ValidationFailed { reason: String },

    /// Generic custom error
    #[error("{message}")]
    Custom { message: String },
}

impl ValueError {
    // Constructors for common error patterns

    /// Creates an invalid regex error
    pub fn invalid_regex(pattern: impl Into<String>, error: impl Into<String>) -> Self {
        Self::InvalidRegex {
            pattern: pattern.into(),
            error: error.into(),
        }
    }

    /// Creates an invalid number error
    pub fn invalid_number(input: impl Into<String>) -> Self {
        Self::InvalidNumber {
            input: input.into(),
        }
    }

    /// Creates a number out of range error
    pub fn number_out_of_range(
        value: impl Into<String>,
        min: impl Into<String>,
        max: impl Into<String>,
    ) -> Self {
        Self::NumberOutOfRange {
            value: value.into(),
            min: min.into(),
            max: max.into(),
        }
    }

    /// Creates a type conversion error
    pub fn type_conversion(from_type: impl Into<String>, to_type: impl Into<String>) -> Self {
        Self::TypeConversion {
            from_type: from_type.into(),
            to_type: to_type.into(),
        }
    }

    /// Creates a type conversion error with value details
    pub fn type_conversion_with_value(
        from_type: impl Into<String>,
        to_type: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self::TypeConversionWithValue {
            from_type: from_type.into(),
            to_type: to_type.into(),
            value: value.into(),
        }
    }

    /// Creates an index out of bounds error
    pub fn index_out_of_bounds(index: usize, length: usize) -> Self {
        Self::IndexOutOfBounds { index, length }
    }

    /// Creates a key not found error
    pub fn key_not_found(key: impl Into<String>) -> Self {
        Self::KeyNotFound { key: key.into() }
    }
    
    /// Invalid boolean error
    pub fn invalid_boolean(input: impl Into<String>) -> Self {
        Self::InvalidBoolean {
            input: input.into(),
        }
    }

    /// Creates an invalid expression error
    pub fn invalid_expression(input: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidExpression {
            input: input.into(),
            reason: reason.into(),
        }
    }

    /// Creates an expression variable not found error
    pub fn expression_variable_not_found(variable: impl Into<String>) -> Self {
        Self::ExpressionVariableNotFound {
            variable: variable.into(),
        }
    }

    /// Creates an unsupported operation error
    pub fn unsupported_operation(
        operation: impl Into<String>,
        value_type: impl Into<String>,
    ) -> Self {
        Self::UnsupportedOperation {
            operation: operation.into(),
            value_type: value_type.into(),
        }
    }

    /// Creates an incompatible comparison error
    pub fn incompatible_comparison(
        left_type: impl Into<String>,
        right_type: impl Into<String>,
    ) -> Self {
        Self::IncompatibleComparison {
            left_type: left_type.into(),
            right_type: right_type.into(),
        }
    }

    /// Creates a validation failed error
    pub fn validation_failed(reason: impl Into<String>) -> Self {
        Self::ValidationFailed {
            reason: reason.into(),
        }
    }

    /// Creates a custom error
    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom {
            message: message.into(),
        }
    }

    /// Creates an invalid format error
    pub fn invalid_format(value_type: impl Into<String>, input: impl Into<String>) -> Self {
        Self::InvalidFormat {
            value_type: value_type.into(),
            input: input.into(),
        }
    }

    /// Creates a JSON serialization error
    pub fn json_serialization(reason: impl Into<String>) -> Self {
        Self::JsonSerialization {
            reason: reason.into(),
        }
    }

    /// Creates a JSON deserialization error
    pub fn json_deserialization(reason: impl Into<String>) -> Self {
        Self::JsonDeserialization {
            reason: reason.into(),
        }
    }
}

// Conversions from common error types

impl From<regex::Error> for ValueError {
    fn from(err: regex::Error) -> Self {
        Self::InvalidRegex {
            pattern: "unknown".to_string(),
            error: err.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for ValueError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::InvalidNumber {
            input: err.to_string(),
        }
    }
}

impl From<std::num::ParseFloatError> for ValueError {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::InvalidNumber {
            input: err.to_string(),
        }
    }
}

impl From<chrono::ParseError> for ValueError {
    fn from(err: chrono::ParseError) -> Self {
        Self::InvalidDateTime {
            input: err.to_string(),
        }
    }
}

impl From<std::str::Utf8Error> for ValueError {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::InvalidUtf8 {
            reason: err.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for ValueError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::InvalidUtf8 {
            reason: err.to_string(),
        }
    }
}

impl From<base64::DecodeError> for ValueError {
    fn from(err: base64::DecodeError) -> Self {
        Self::BinaryDecodingFailed {
            reason: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for ValueError {
    fn from(err: serde_json::Error) -> Self {
        if err.is_syntax() || err.is_data() {
            Self::JsonDeserialization {
                reason: err.to_string(),
            }
        } else {
            Self::JsonSerialization {
                reason: err.to_string(),
            }
        }
    }
}

// Helper trait for easy error conversion
pub trait ValueErrorExt<T> {
    /// Converts a Result to ValueError with custom context
    fn with_value_context(self, context: &str) -> Result<T, ValueError>;

    /// Converts a Result to ValueError with type conversion context
    fn with_conversion_context(self, from_type: &str, to_type: &str) -> Result<T, ValueError>;
}

impl<T, E> ValueErrorExt<T> for Result<T, E>
where
    E: std::error::Error,
{
    fn with_value_context(self, context: &str) -> Result<T, ValueError> {
        self.map_err(|e| ValueError::custom(format!("{}: {}", context, e)))
    }

    fn with_conversion_context(self, from_type: &str, to_type: &str) -> Result<T, ValueError> {
        self.map_err(|_| ValueError::type_conversion(from_type, to_type))
    }
}

impl HasSeverity for ValueError {
    fn severity(&self) -> ErrorSeverity {
        match self {
            // Critical errors - system cannot function
            ValueError::DivisionByZero => ErrorSeverity::Critical,
            ValueError::IndexOutOfBounds { .. } => ErrorSeverity::Critical,
            ValueError::InvalidUtf8 { .. } => ErrorSeverity::Critical,

            // Regular errors - operation failed
            ValueError::InvalidRegex { .. } => ErrorSeverity::Error,
            ValueError::InvalidNumber { .. } => ErrorSeverity::Error,
            ValueError::NumberOutOfRange { .. } => ErrorSeverity::Error,
            ValueError::InvalidDate { .. } => ErrorSeverity::Error,
            ValueError::InvalidTime { .. } => ErrorSeverity::Error,
            ValueError::InvalidDateTime { .. } => ErrorSeverity::Error,
            ValueError::InvalidDuration { .. } => ErrorSeverity::Error,
            ValueError::TypeConversion { .. } => ErrorSeverity::Error,
            ValueError::TypeConversionWithValue { .. } => ErrorSeverity::Error,
            ValueError::KeyNotFound { .. } => ErrorSeverity::Error,
            ValueError::InvalidEnumVariant { .. } => ErrorSeverity::Error,
            ValueError::BinaryDecodingFailed { .. } => ErrorSeverity::Error,
            ValueError::BinaryEncodingFailed { .. } => ErrorSeverity::Error,
            ValueError::JsonSerialization { .. } => ErrorSeverity::Error,
            ValueError::JsonDeserialization { .. } => ErrorSeverity::Error,
            ValueError::InvalidFormat { .. } => ErrorSeverity::Error,
            ValueError::UnsupportedOperation { .. } => ErrorSeverity::Error,
            ValueError::IncompatibleComparison { .. } => ErrorSeverity::Error,
            ValueError::ValidationFailed { .. } => ErrorSeverity::Error,

            // Warnings - something unexpected but can continue
            ValueError::InvalidExpression { .. } => ErrorSeverity::Warning,
            ValueError::ExpressionVariableNotFound { .. } => ErrorSeverity::Warning,
            ValueError::ExpressionEvaluationFailed { .. } => ErrorSeverity::Warning,

            // Custom errors - severity depends on context, default to Error
            ValueError::Custom { .. } => ErrorSeverity::Error,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            ValueError::InvalidRegex { .. } => "VALUE_INVALID_REGEX",
            ValueError::InvalidNumber { .. } => "VALUE_INVALID_NUMBER",
            ValueError::NumberOutOfRange { .. } => "VALUE_NUMBER_OUT_OF_RANGE",
            ValueError::DivisionByZero => "VALUE_DIVISION_BY_ZERO",
            ValueError::InvalidDate { .. } => "VALUE_INVALID_DATE",
            ValueError::InvalidTime { .. } => "VALUE_INVALID_TIME",
            ValueError::InvalidDateTime { .. } => "VALUE_INVALID_DATETIME",
            ValueError::InvalidDuration { .. } => "VALUE_INVALID_DURATION",
            ValueError::InvalidExpression { .. } => "VALUE_INVALID_EXPRESSION",
            ValueError::ExpressionVariableNotFound { .. } => "VALUE_EXPRESSION_VAR_NOT_FOUND",
            ValueError::ExpressionEvaluationFailed { .. } => "VALUE_EXPRESSION_EVAL_FAILED",
            ValueError::TypeConversion { .. } => "VALUE_TYPE_CONVERSION",
            ValueError::TypeConversionWithValue { .. } => "VALUE_TYPE_CONVERSION_WITH_VALUE",
            ValueError::IndexOutOfBounds { .. } => "VALUE_INDEX_OUT_OF_BOUNDS",
            ValueError::KeyNotFound { .. } => "VALUE_KEY_NOT_FOUND",
            ValueError::InvalidEnumVariant { .. } => "VALUE_INVALID_ENUM_VARIANT",
            ValueError::InvalidUtf8 { .. } => "VALUE_INVALID_UTF8",
            ValueError::BinaryDecodingFailed { .. } => "VALUE_BINARY_DECODE_FAILED",
            ValueError::BinaryEncodingFailed { .. } => "VALUE_BINARY_ENCODE_FAILED",
            ValueError::JsonSerialization { .. } => "VALUE_JSON_SERIALIZATION",
            ValueError::JsonDeserialization { .. } => "VALUE_JSON_DESERIALIZATION",
            ValueError::InvalidFormat { .. } => "VALUE_INVALID_FORMAT",
            ValueError::UnsupportedOperation { .. } => "VALUE_UNSUPPORTED_OPERATION",
            ValueError::IncompatibleComparison { .. } => "VALUE_INCOMPATIBLE_COMPARISON",
            ValueError::ValidationFailed { .. } => "VALUE_VALIDATION_FAILED",
            ValueError::Custom { .. } => "VALUE_CUSTOM",
        }
    }
}

// Result type alias for convenience
pub type ValueResult<T> = Result<T, ValueError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_constructors() {
        let err = ValueError::invalid_number("abc");
        assert!(matches!(err, ValueError::InvalidNumber { .. }));

        let err = ValueError::type_conversion("string", "number");
        assert!(matches!(err, ValueError::TypeConversion { .. }));

        let err = ValueError::index_out_of_bounds(5, 3);
        assert!(matches!(err, ValueError::IndexOutOfBounds { index: 5, length: 3 }));
    }

    #[test]
    fn test_error_messages() {
        let err = ValueError::type_conversion("string", "number");
        assert_eq!(
            err.to_string(),
            "Type conversion failed: cannot convert string to number"
        );

        let err = ValueError::invalid_expression("{{ invalid", "unclosed braces");
        assert_eq!(
            err.to_string(),
            "Invalid expression syntax: {{ invalid - unclosed braces"
        );
    }

    #[test]
    fn test_from_conversions() {
        let parse_err = "abc".parse::<i32>().unwrap_err();
        let value_err: ValueError = parse_err.into();
        assert!(matches!(value_err, ValueError::InvalidNumber { .. }));
    }

    #[test]
    fn test_value_error_ext() {
        let result: Result<i32, std::num::ParseIntError> = "abc".parse();
        let value_result = result.with_conversion_context("string", "integer");

        assert!(value_result.is_err());
        if let Err(ValueError::TypeConversion { from_type, to_type }) = value_result {
            assert_eq!(from_type, "string");
            assert_eq!(to_type, "integer");
        }
    }

    #[test]
    fn test_error_equality() {
        let err1 = ValueError::custom("test");
        let err2 = ValueError::custom("test");
        let err3 = ValueError::custom("other");

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }
}