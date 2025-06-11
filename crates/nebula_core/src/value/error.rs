use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum ValueError {
    #[error("Invalid regex pattern '{pattern}': {error}")]
    InvalidRegex { pattern: String, error: String },

    #[error("Invalid number format: {0}")]
    InvalidNumber(String),

    #[error("Invalid date format: {0}")]
    InvalidDate(String),

    #[error("Invalid duration format: {0}")]
    InvalidDuration(String),

    #[error("Invalid expression syntax: {0}")]
    InvalidExpression(String),

    #[error("Type conversion error: cannot convert {from} to {to}")]
    TypeConversion { from: String, to: String },

    #[error("Value out of range: {value} is not within {min}..{max}")]
    OutOfRange {
        value: String,
        min: String,
        max: String,
    },

    #[error("Invalid enum variant: '{variant}' is not a valid {enum_name}")]
    InvalidEnumVariant { variant: String, enum_name: String },

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Value error: {0}")]
    Custom(String),
}

impl ValueError {
    pub fn invalid_regex(pattern: impl Into<String>, error: impl Into<String>) -> Self {
        Self::InvalidRegex {
            pattern: pattern.into(),
            error: error.into(),
        }
    }

    pub fn type_conversion(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self::TypeConversion {
            from: from.into(),
            to: to.into(),
        }
    }

    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom(message.into())
    }
}

impl From<regex::Error> for ValueError {
    fn from(err: regex::Error) -> Self {
        Self::InvalidRegex {
            pattern: "unknown".to_string(),
            error: err.to_string(),
        }
    }
}

impl From<chrono::ParseError> for ValueError {
    fn from(err: chrono::ParseError) -> Self {
        Self::InvalidDate(err.to_string())
    }
}

impl From<serde_json::Error> for ValueError {
    fn from(err: serde_json::Error) -> Self {
        if err.is_syntax() {
            Self::Deserialization(err.to_string())
        } else {
            Self::Serialization(err.to_string())
        }
    }
}
