use std::error::Error as StdError;
use super::{ErrorSeverity, ErrorCategory, ErrorContext, ErrorInfo};

/// Core error trait (без Sized чтобы поддерживать trait objects)
pub trait AnyError: HasSeverity + StdError + Send + Sync + 'static {
    /// Gets associated context
    fn context(&self) -> Option<&ErrorContext> {
        None // Default implementation
    }

    /// Returns error code for logging/metrics
    fn error_code(&self) -> String {
        format!("{}_{}", self.category().as_str(), self.severity().as_str())
    }

    /// Records error in tracing span
    #[cfg(feature = "tracing")]
    fn record_span(&self) {
        use tracing::{error_span, field};

        error_span!(
            "nebula.error",
            code = self.error_code(),
            severity = %self.severity(),
            category = %self.category()
        ).in_scope(|| {
            tracing::error!(
                message = %self,
                "Error occurred"
            );
        });
    }

    #[cfg(not(feature = "tracing"))]
    fn record_span(&self) {
        // No-op when tracing is disabled
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

    /// Checks if error is critical
    #[inline]
    fn is_critical(&self) -> bool {
        self.severity().is_critical()
    }

    /// Checks if error should be logged
    #[inline]
    fn should_log(&self, min_severity: ErrorSeverity) -> bool {
        self.severity().should_log(min_severity)
    }
}

/// Error chain analysis
pub trait ErrorChain {
    /// Finds root cause
    fn root_cause(&self) -> Option<&dyn AnyError>;

    /// Collects all errors in chain
    fn error_chain(&self) -> Vec<&dyn AnyError>;
}

impl<T: AnyError> ErrorChain for T {
    fn root_cause(&self) -> Option<&dyn AnyError> {
        let mut current: &dyn AnyError = self;
        while let Some(source) = current.source() {
            if let Some(next) = source.downcast_ref::<dyn AnyError>() {
                current = next;
            } else {
                break;
            }
        }
        Some(current)
    }

    fn error_chain(&self) -> Vec<&dyn AnyError> {
        let mut chain = vec![self];
        let mut current: &dyn AnyError = self;

        while let Some(source) = current.source() {
            if let Some(next) = source.downcast_ref::<dyn AnyError>() {
                chain.push(next);
                current = next;
            } else {
                break;
            }
        }

        chain
    }
}

/// Conversion to ErrorInfo
pub trait ToErrorInfo {
    /// Converts to structured error info
    fn to_error_info(&self) -> ErrorInfo;
}

impl<T: HasSeverity + StdError> ToErrorInfo for T {
    fn to_error_info(&self) -> ErrorInfo {
        let context = if let Some(any_error) = self.downcast_ref::<dyn AnyError>() {
            any_error.context().cloned().unwrap_or_default()
        } else {
            ErrorContext::default()
        };

        ErrorInfo {
            severity: self.severity(),
            category: self.category(),
            message: self.to_string().into(),
            context,
        }
    }
}

// Blanket implementation for ErrorInfo
impl AnyError for ErrorInfo {
    fn context(&self) -> Option<&ErrorContext> {
        Some(&self.context)
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