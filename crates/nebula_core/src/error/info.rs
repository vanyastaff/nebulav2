use std::{
    borrow::Cow,
    error::Error as StdError,
    fmt,
    sync::OnceLock,
};
use serde::{Serialize, Deserialize};
use super::{ErrorSeverity, ErrorCategory, ErrorContext, HasSeverity};

/// Complete error information structure with optimized serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[must_use = "ErrorInfo should be handled or logged"]
pub struct ErrorInfo {
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    #[serde(borrow)]
    pub message: Cow<'static, str>,
    #[serde(default, skip_serializing_if = "ErrorContext::is_empty")]
    pub context: ErrorContext,
    #[serde(skip)]
    pub source: Option<Box<dyn StdError + Send + Sync>>,
}

impl Default for ErrorInfo {
    fn default() -> Self {
        Self {
            severity: ErrorSeverity::Error,
            category: ErrorCategory::General,
            message: Cow::Borrowed("Unknown error"),
            context: ErrorContext::default(),
            source: None,
        }
    }
}

impl ErrorInfo {
    /// Creates new error with severity and message
    pub fn new<S>(severity: ErrorSeverity, message: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self {
            severity,
            message: message.into(),
            ..Default::default()
        }
    }

    /// Creates const error for static definitions (no allocations)
    pub const fn new_const(
        severity: ErrorSeverity,
        category: ErrorCategory,
        message: &'static str,
    ) -> Self {
        Self {
            severity,
            category,
            message: Cow::Borrowed(message),
            context: ErrorContext::new(),
            source: None,
        }
    }

    /// Creates error with automatic caller location
    #[track_caller]
    pub fn at_caller<S>(severity: ErrorSeverity, message: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self {
            severity,
            message: message.into(),
            context: ErrorContext::at_caller(),
            ..Default::default()
        }
    }

    /// Builder method to set context
    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = context;
        self
    }

    /// Builder method to set category
    pub fn with_category(mut self, category: ErrorCategory) -> Self {
        self.category = category;
        self
    }

    /// Builder method to add context data
    pub fn with_data<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: ToString,
    {
        self.context = self.context.with_data(key, value);
        self
    }

    /// Adds backtrace to context (lazy capture)
    pub fn with_backtrace(mut self) -> Self {
        self.context = self.context.capture_backtrace();
        self
    }

    /// Builder method to set source error
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Checks if error is critical
    pub const fn is_critical(&self) -> bool {
        matches!(self.severity, ErrorSeverity::Critical)
    }

    /// Checks if error should be logged
    pub const fn should_log(&self) -> bool {
        self.severity.priority() >= ErrorSeverity::Warning.priority()
    }

    /// Checks if error should be shown to users
    pub fn is_user_facing(&self) -> bool {
        self.category.is_user_facing() && !self.severity.is_debug()
    }

    /// Gets short error summary
    pub fn summary(&self) -> Cow<'static, str> {
        if self.message.is_empty() {
            Cow::Borrowed("Unknown error")
        } else {
            Cow::Borrowed(&self.message)
        }
    }

    /// Gets detailed error description
    pub fn detailed(&self) -> String {
        let mut details = String::with_capacity(128);
        details.push_str(&self.message);

        if !self.context.is_empty() {
            details.push_str(" | ");
            details.push_str(&self.context.format_compact());
        }

        if let Some(source) = &self.source {
            details.push_str("\nCaused by: ");
            details.push_str(&source.to_string());
        }

        details
    }

    /// Records error to appropriate logging system
    pub fn record(&self) {
        #[cfg(feature = "tracing")]
        {
            use tracing::{event, Level};
            event!(
                target: "error",
                level = self.severity.to_tracing_level(),
                error = ?self,
                "Error occurred"
            );
        }

        #[cfg(not(feature = "tracing"))]
        eprintln!("{}", self.detailed());
    }

    /// Converts to anyhow::Error
    pub fn into_anyhow(self) -> anyhow::Error {
        anyhow::Error::new(self)
    }

    /// Creates from std error with automatic classification
    pub fn from_std_error<E>(error: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        let (severity, category) = Self::classify_error(&error);
        Self::new(severity, error.to_string())
            .with_category(category)
            .with_source(error)
    }

    /// Classifies standard errors into severity/category
    fn classify_error<E: StdError>(error: &E) -> (ErrorSeverity, ErrorCategory) {
        let msg = error.to_string().to_ascii_lowercase();

        let severity = if msg.contains("fatal") || msg.contains("critical") {
            ErrorSeverity::Critical
        } else if msg.contains("warning") {
            ErrorSeverity::Warning
        } else {
            ErrorSeverity::Error
        };
        

        let category = match () {
            _ if msg.contains("network") => ErrorCategory::Network,
            _ if msg.contains("file") || msg.contains("io") => ErrorCategory::Storage,
            _ if msg.contains("parse") => ErrorCategory::Parameter,
            _ if msg.contains("auth") => ErrorCategory::Auth,
            _ if msg.contains("config") => ErrorCategory::Configuration,
            _ => ErrorCategory::General,
        };

        (severity, category)
    }
}

// Formatting implementations
impl fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.detailed())
    }
}

impl StdError for ErrorInfo {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_deref()
    }
}

impl HasSeverity for ErrorInfo {
    fn severity(&self) -> ErrorSeverity {
        self.severity
    }

    fn category(&self) -> ErrorCategory {
        self.category
    }
}

// Conversion implementations
impl From<&'static str> for ErrorInfo {
    fn from(message: &'static str) -> Self {
        Self::new(ErrorSeverity::Error, Cow::Borrowed(message))
    }
}

impl From<String> for ErrorInfo {
    fn from(message: String) -> Self {
        Self::new(ErrorSeverity::Error, Cow::Owned(message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_error_creation() {
        let err = ErrorInfo::new(ErrorSeverity::Error, "test");
        assert_eq!(err.message, "test");
        assert_eq!(err.severity, ErrorSeverity::Error);
    }

    #[test]
    fn test_from_std_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = ErrorInfo::from_std_error(io_err);
        assert_eq!(err.category, ErrorCategory::Storage);
        assert!(err.source.is_some());
    }

    #[test]
    fn test_error_chaining() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = ErrorInfo::new(ErrorSeverity::Error, "operation failed")
            .with_source(io_err);

        assert!(err.source().is_some());
        assert!(err.to_string().contains("Caused by:"));
    }
}