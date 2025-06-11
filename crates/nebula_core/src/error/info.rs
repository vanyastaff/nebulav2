use std::borrow::Cow;
use serde::Serialize;
use super::{ErrorSeverity, ErrorCategory, ErrorContext};

/// Complete error information structure
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct ErrorInfo {
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub message: Cow<'static, str>,
    #[serde(flatten)] // Flatten context into parent object
    pub context: ErrorContext,
}

impl Default for ErrorInfo {
    fn default() -> Self {
        Self {
            severity: ErrorSeverity::Error,
            category: ErrorCategory::General,
            message: Cow::Borrowed("Unknown error"),
            context: ErrorContext::default(),
        }
    }
}

impl ErrorInfo {
    /// Creates new error
    #[inline]
    pub fn new<S>(severity: ErrorSeverity, message: S) -> Self
    where
        S: Into<Cow<'static, str>>
    {
        Self {
            severity,
            message: message.into(),
            category: ErrorCategory::General,
            context: ErrorContext::default(),
        }
    }

    /// Creates const error for static definitions
    pub const fn new_const(
        severity: ErrorSeverity,
        category: ErrorCategory,
        message: &'static str,
    ) -> Self {
        Self {
            severity,
            category,
            message: Cow::Borrowed(message),
            context: ErrorContext::default(),
        }
    }

    /// Creates error with full context
    pub fn with_full_context(
        severity: ErrorSeverity,
        category: ErrorCategory,
        message: impl Into<Cow<'static, str>>,
        context: ErrorContext,
    ) -> Self {
        Self {
            severity,
            category,
            message: message.into(),
            context,
        }
    }

    /// Sets context
    #[inline]
    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = context;
        self
    }

    /// Sets category
    #[inline]
    pub fn with_category(mut self, category: ErrorCategory) -> Self {
        self.category = category;
        self
    }

    /// Adds context data
    #[inline]
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context = self.context.with_data(key, value);
        self
    }

    /// Checks if error should be logged at given level
    #[inline]
    pub fn should_log(&self, min_severity: ErrorSeverity) -> bool {
        self.severity.should_log(min_severity)
    }

    /// Returns short error code for logging
    pub fn error_code(&self) -> String {
        format!("{}_{}", self.category.as_str(), self.severity.as_str())
    }
}

impl std::fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}] {}",
               self.category.as_str(),
               self.severity.as_str(),
               self.message
        )
    }
}

impl std::error::Error for ErrorInfo {}