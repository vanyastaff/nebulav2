use std::borrow::Cow;
use serde::Serialize;
use super::{ErrorSeverity, ErrorCategory, ErrorContext};

/// Complete error information structure
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ErrorInfo {
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub message: Cow<'static, str>,
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

    /// Sets context
    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = context;
        self
    }

    /// Sets category
    pub fn with_category(mut self, category: ErrorCategory) -> Self {
        self.category = category;
        self
    }
}

impl std::fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.severity.as_str(), self.message)
    }
}

impl std::error::Error for ErrorInfo {}