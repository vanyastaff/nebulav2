use super::{ErrorCategory, ErrorContext, ErrorSeverity};
use serde::{Deserialize, Serialize};

/// Structured error information for telemetry and debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// Error severity level
    pub severity: ErrorSeverity,
    /// Error code for programmatic handling
    pub code: String,
    /// Error category
    pub category: ErrorCategory,
    /// Human-readable error message
    pub message: String,
    /// User-friendly message (if different from a technical message)
    pub user_message: Option<String>,
    /// Additional context information
    pub context: ErrorContext,
    /// Suggestions for fixing the error
    pub suggestions: Vec<String>,
    /// Related documentation links
    pub help_links: Vec<String>,
    /// Whether this error is recoverable
    pub recoverable: bool,
    /// Tags for filtering and grouping
    pub tags: Vec<String>,
}

impl ErrorInfo {
    /// Creates a new error info
    pub fn new(
        severity: ErrorSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            category: ErrorCategory::General,
            message: message.into(),
            user_message: None,
            context: ErrorContext::new(),
            suggestions: Vec::new(),
            help_links: Vec::new(),
            recoverable: false,
            tags: Vec::new(),
        }
    }

    /// Builder pattern methods
    pub fn with_category(mut self, category: ErrorCategory) -> Self {
        self.category = category;
        self
    }

    pub fn with_user_message(mut self, message: impl Into<String>) -> Self {
        self.user_message = Some(message.into());
        self
    }

    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = context;
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    pub fn with_suggestions<I, S>(mut self, suggestions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.suggestions.extend(suggestions.into_iter().map(Into::into));
        self
    }

    pub fn with_help_link(mut self, link: impl Into<String>) -> Self {
        self.help_links.push(link.into());
        self
    }

    pub fn with_help_links<I, S>(mut self, links: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.help_links.extend(links.into_iter().map(Into::into));
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn with_tags<I, S>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags.extend(tags.into_iter().map(Into::into));
        self
    }

    pub fn recoverable(mut self) -> Self {
        self.recoverable = true;
        self
    }

    /// Returns the display message (user message if available, otherwise technical message)
    pub fn display_message(&self) -> &str {
        self.user_message.as_ref().unwrap_or(&self.message)
    }

    /// Returns true if this error should be shown to the user
    pub fn should_show_to_user(&self) -> bool {
        !matches!(self.severity, ErrorSeverity::Debug) && !self.tags.contains(&"internal".to_string())
    }

    /// Returns true if this error should be logged
    pub fn should_log(&self) -> bool {
        self.severity.should_log()
    }

    /// Returns true if this error should stop execution
    pub fn should_stop_execution(&self) -> bool {
        self.severity.is_serious() && !self.recoverable
    }
}