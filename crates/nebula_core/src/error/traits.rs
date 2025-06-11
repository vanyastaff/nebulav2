use std::error::Error as StdError;
use super::{ErrorSeverity, ErrorCategory, ErrorContext, ErrorInfo};

/// Core error trait
pub trait AnyError: HasSeverity + StdError + Send + Sync + 'static {
    /// Gets associated context
    fn context(&self) -> Option<&ErrorContext>;

    /// Records error in tracing span
    fn record_span(&self) {
        use tracing::{error_span, field};

        error_span!(
            "core.error",
            code = self.error_code(),
            severity = %self.severity(),
            category = %self.category()
        ).record("message", field::debug(self.to_string()));
    }
}

/// Error severity marker trait
pub trait HasSeverity {
    /// Returns error severity
    fn severity(&self) -> ErrorSeverity;

    /// Returns error category
    fn category(&self) -> ErrorCategory {
        ErrorCategory::General
    }
}

/// Error chain analysis
pub trait ErrorChain {
    /// Finds root cause
    fn root_cause(&self) -> Option<&dyn AnyError>;
}

impl<T: AnyError> ErrorChain for T {
    fn root_cause(&self) -> Option<&dyn AnyError> {
        let mut current: &dyn AnyError = self;
        while let Some(next) = current.source().and_then(|e| e.downcast_ref()) {
            current = next;
        }
        Some(current)
    }
}

/// Conversion to ErrorInfo
pub trait ToErrorInfo {
    /// Converts to structured error info
    fn to_error_info(&self) -> ErrorInfo;
}

impl<T: HasSeverity + StdError> ToErrorInfo for T {
    fn to_error_info(&self) -> ErrorInfo {
        ErrorInfo {
            severity: self.severity(),
            category: self.category(),
            message: self.to_string().into(),
            ..Default::default()
        }
    }
}