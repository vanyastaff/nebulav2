//! Error types for the Nebula template engine

use std::fmt;

/// Result type alias for template operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for the template engine
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Template parsing failed
    #[error("Parse error at position {position}: {message}")]
    ParseError {
        /// Error message
        message: String,
        /// Position in the template where error occurred
        position: usize,
        /// The template content that caused the error
        template: String,
    },

    /// Template evaluation failed
    #[error("Evaluation error: {message}")]
    EvaluationError {
        /// Error message
        message: String,
        /// Optional context about where the error occurred
        context: Option<String>,
    },

    /// Function execution failed
    #[error("Function '{function}' failed: {message}")]
    FunctionError {
        /// Function name
        function: String,
        /// Error message
        message: String,
        /// Arguments passed to the function
        args: Vec<String>,
    },

    /// Type conversion error
    #[error("Type error: cannot convert {from} to {to}")]
    TypeError {
        /// Source type
        from: String,
        /// Target type
        to: String,
        /// Optional context
        context: Option<String>,
    },

    /// Variable or data source not found
    #[error("Data not found: {path}")]
    DataNotFound {
        /// The data path that was not found
        path: String,
        /// Available data sources
        available: Vec<String>,
    },

    /// Invalid function signature
    #[error("Invalid function signature for '{function}': {message}")]
    SignatureError {
        /// Function name
        function: String,
        /// Error message
        message: String,
    },

    /// Division by zero or other math errors
    #[error("Math error: {message}")]
    MathError {
        /// Error message
        message: String,
    },

    /// Index out of bounds
    #[error("Index {index} out of bounds for collection of size {size}")]
    IndexError {
        /// The index that was out of bounds
        index: isize,
        /// Size of the collection
        size: usize,
    },

    /// Invalid regular expression
    #[cfg(feature = "regex")]
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    /// JSON serialization/deserialization error
    #[cfg(feature = "serde")]
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Date/time parsing error
    #[cfg(feature = "chrono")]
    #[error("Date/time error: {0}")]
    ChronoError(#[from] chrono::ParseError),

    /// IO error (for file operations if added later)
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Custom error for user-defined functions
    #[error("Custom error: {message}")]
    CustomError {
        /// Error message
        message: String,
        /// Error code for programmatic handling
        code: Option<String>,
    },
}

impl Error {
    /// Create a parse error
    pub fn parse(message: impl Into<String>, position: usize, template: impl Into<String>) -> Self {
        Self::ParseError {
            message: message.into(),
            position,
            template: template.into(),
        }
    }

    /// Create an evaluation error
    pub fn evaluation(message: impl Into<String>) -> Self {
        Self::EvaluationError {
            message: message.into(),
            context: None,
        }
    }

    /// Create an evaluation error with context
    pub fn evaluation_with_context(
        message: impl Into<String>,
        context: impl Into<String>
    ) -> Self {
        Self::EvaluationError {
            message: message.into(),
            context: Some(context.into()),
        }
    }

    /// Create a function error
    pub fn function(
        function: impl Into<String>,
        message: impl Into<String>,
        args: Vec<String>,
    ) -> Self {
        Self::FunctionError {
            function: function.into(),
            message: message.into(),
            args,
        }
    }

    /// Create a type error
    pub fn type_error(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self::TypeError {
            from: from.into(),
            to: to.into(),
            context: None,
        }
    }

    /// Create a type error with context
    pub fn type_error_with_context(
        from: impl Into<String>,
        to: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        Self::TypeError {
            from: from.into(),
            to: to.into(),
            context: Some(context.into()),
        }
    }

    /// Create a data not found error
    pub fn data_not_found(path: impl Into<String>, available: Vec<String>) -> Self {
        Self::DataNotFound {
            path: path.into(),
            available,
        }
    }

    /// Create a signature error
    pub fn signature(function: impl Into<String>, message: impl Into<String>) -> Self {
        Self::SignatureError {
            function: function.into(),
            message: message.into(),
        }
    }

    /// Create a math error
    pub fn math(message: impl Into<String>) -> Self {
        Self::MathError {
            message: message.into(),
        }
    }

    /// Create an index error
    pub fn index(index: isize, size: usize) -> Self {
        Self::IndexError { index, size }
    }

    /// Create a custom error
    pub fn custom(message: impl Into<String>) -> Self {
        Self::CustomError {
            message: message.into(),
            code: None,
        }
    }

    /// Create a custom error with code
    pub fn custom_with_code(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self::CustomError {
            message: message.into(),
            code: Some(code.into()),
        }
    }

    /// Check if this is a parse error
    pub fn is_parse_error(&self) -> bool {
        matches!(self, Self::ParseError { .. })
    }

    /// Check if this is an evaluation error
    pub fn is_evaluation_error(&self) -> bool {
        matches!(self, Self::EvaluationError { .. })
    }

    /// Check if this is a function error
    pub fn is_function_error(&self) -> bool {
        matches!(self, Self::FunctionError { .. })
    }

    /// Check if this is a type error
    pub fn is_type_error(&self) -> bool {
        matches!(self, Self::TypeError { .. })
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        match self {
            Self::ParseError { message, .. } => message,
            Self::EvaluationError { message, .. } => message,
            Self::FunctionError { message, .. } => message,
            Self::TypeError { from, to, .. } => {
                // This is a bit of a hack, but we want to return a string reference
                // In practice, you'd use the Display implementation
                "Type conversion error"
            }
            Self::DataNotFound { path, .. } => path,
            Self::SignatureError { message, .. } => message,
            Self::MathError { message } => message,
            Self::IndexError { .. } => "Index out of bounds",
            Self::CustomError { message, .. } => message,
            #[cfg(feature = "regex")]
            Self::RegexError(e) => e.as_str(),
            #[cfg(feature = "serde")]
            Self::JsonError(_) => "JSON error",
            #[cfg(feature = "chrono")]
            Self::ChronoError(_) => "Date/time error",
            Self::IoError(_) => "IO error",
        }
    }
}

/// Alias for function errors in the functions module
pub type FunctionError = Error;

#[cfg(test)]
mod tests {
    use crate::Error;

    #[test]
    fn test_parse_error() {
        let err = Error::parse("Unexpected token", 10, "{{ invalid");
        assert!(err.is_parse_error());
        assert_eq!(err.message(), "Unexpected token");
    }

    #[test]
    fn test_evaluation_error() {
        let err = Error::evaluation("Variable not found");
        assert!(err.is_evaluation_error());
        assert_eq!(err.message(), "Variable not found");
    }

    #[test]
    fn test_function_error() {
        let err = Error::function("uppercase", "Invalid argument", vec!["123".to_string()]);
        assert!(err.is_function_error());
        assert_eq!(err.message(), "Invalid argument");
    }

    #[test]
    fn test_type_error() {
        let err = Error::type_error("String", "Number");
        assert!(err.is_type_error());
    }

    #[test]
    fn test_custom_error() {
        let err = Error::custom("Something went wrong");
        assert_eq!(err.message(), "Something went wrong");
    }

    #[test]
    fn test_custom_error_with_code() {
        let err = Error::custom_with_code("Validation failed", "VALIDATION_ERROR");
        if let Error::CustomError { code, .. } = err {
            assert_eq!(code, Some("VALIDATION_ERROR".to_string()));
        } else {
            panic!("Expected CustomError");
        }
    }

    #[test]
    fn test_error_display() {
        let err = Error::parse("Unexpected token", 10, "{{ invalid");
        let display = format!("{}", err);
        assert!(display.contains("Parse error at position 10"));
        assert!(display.contains("Unexpected token"));
    }

    #[test]
    fn test_math_error() {
        let err = Error::math("Division by zero");
        assert_eq!(err.message(), "Division by zero");
    }

    #[test]
    fn test_index_error() {
        let err = Error::index(-1, 5);
        if let Error::IndexError { index, size } = err {
            assert_eq!(index, -1);
            assert_eq!(size, 5);
        } else {
            panic!("Expected IndexError");
        }
    }
}