use super::{ErrorCategory, ErrorInfo, ErrorSeverity};

/// Trait for errors that have severity levels and error codes
pub trait HasSeverity {
    /// Returns the severity level of the error
    fn severity(&self) -> ErrorSeverity;

    /// Returns the error code as string
    fn error_code(&self) -> &'static str;

    /// Returns true if this is a critical error
    fn is_critical(&self) -> bool {
        self.severity() == ErrorSeverity::Critical
    }

    /// Returns true if this error should stop execution
    fn should_stop_execution(&self) -> bool {
        self.severity().is_serious()
    }

    /// Returns error category for grouping/filtering
    fn category(&self) -> ErrorCategory {
        ErrorCategory::General
    }

    /// Returns user-friendly error message
    fn user_message(&self) -> Option<String> {
        None // By default, use the Display implementation
    }

    /// Returns suggestions for fixing the error
    fn suggestions(&self) -> Vec<String> {
        Vec::new()
    }

    /// Returns related documentation links
    fn help_links(&self) -> Vec<String> {
        Vec::new()
    }

    /// Returns tags for filtering
    fn tags(&self) -> Vec<String> {
        Vec::new()
    }

    /// Returns true if this error is recoverable
    fn is_recoverable(&self) -> bool {
        false
    }
}

/// Trait for converting errors to structured error information
pub trait ToErrorInfo {
    fn to_error_info(&self) -> ErrorInfo;
}

/// Blanket implementation for all types that implement HasSeverity
impl<T: HasSeverity + std::fmt::Display> ToErrorInfo for T {
    fn to_error_info(&self) -> ErrorInfo {
        let mut info = ErrorInfo::new(self.severity(), self.error_code(), self.to_string())
            .with_category(self.category());

        if let Some(user_msg) = self.user_message() {
            info = info.with_user_message(user_msg);
        }

        info = info.with_suggestions(self.suggestions());
        info = info.with_help_links(self.help_links());
        info = info.with_tags(self.tags());

        if self.is_recoverable() {
            info = info.recoverable();
        }

        info
    }
}