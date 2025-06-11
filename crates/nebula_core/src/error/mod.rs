#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod backtrace;
mod category;
mod context;
mod info;
mod macros;
mod severity;
mod traits;

pub use backtrace::BacktraceExt;
pub use category::ErrorCategory;
pub use context::ErrorContext;
pub use info::ErrorInfo;
pub use severity::ErrorSeverity;
pub use traits::{ErrorChain, HasSeverity, AnyError, ToErrorInfo};

#[doc(inline)]
pub use crate::{error, error_context, impl_has_severity, critical_error};

/// Main result type with backtrace support
pub type AnyResult<T, E = Box<dyn AnyError>> = Result<T, E>;

/// Result type for infallible operations that might have warnings
pub type WarningResult<T> = Result<T, Vec<ErrorInfo>>;

/// Commonly used error constants
pub mod consts {
    use super::*;

    pub const INTERNAL_ERROR: ErrorInfo = ErrorInfo::new_const(
        ErrorSeverity::Critical,
        ErrorCategory::Internal,
        "Internal server error"
    );

    pub const PARAMETER_ERROR: ErrorInfo = ErrorInfo::new_const(
        ErrorSeverity::Error,
        ErrorCategory::Parameter,
        "Parameter validation failed"
    );

    pub const NETWORK_ERROR: ErrorInfo = ErrorInfo::new_const(
        ErrorSeverity::Warning,
        ErrorCategory::Network,
        "Network operation failed"
    );
}

/// Utility functions for error handling
pub mod utils {
    use super::*;

    /// Logs error chain with appropriate severity
    pub fn log_error_chain(error: &dyn AnyError) {
        #[cfg(feature = "tracing")]
        {
            for (i, err) in error.error_chain().iter().enumerate() {
                if i == 0 {
                    err.record_span();
                } else {
                    tracing::debug!(
                        cause = %err,
                        "Error cause #{}", i
                    );
                }
            }
        }

        #[cfg(not(feature = "tracing"))]
        {
            eprintln!("Error: {}", error);
            for (i, cause) in error.error_chain().iter().skip(1).enumerate() {
                eprintln!("  Cause #{}: {}", i + 1, cause);
            }
        }
    }

    /// Converts any error to ErrorInfo with context
    #[track_caller]
    pub fn wrap_error<E: std::error::Error + 'static>(
        error: E,
        severity: ErrorSeverity,
        category: ErrorCategory,
    ) -> ErrorInfo {
        ErrorInfo::with_full_context(
            severity,
            category,
            error.to_string().into(),
            ErrorContext::at_caller()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context_optimization() {
        let ctx = ErrorContext::default()
            .with_data("key", "value")
            .with_data("user_id", "user123")
            .with_data("request_id", "req456");

        assert_eq!(ctx.data.get("key").map(|s| s.as_ref()), Some("value"));
        assert_eq!(ctx.data.get("user_id").map(|s| s.as_ref()), Some("user123"));
        assert_eq!(ctx.data.get("request_id").map(|s| s.as_ref()), Some("req456"));
    }

    #[test]
    fn test_severity_const_methods() {
        assert!(ErrorSeverity::Critical.is_critical());
        assert!(ErrorSeverity::Error.is_error_or_above());
        assert!(ErrorSeverity::Warning.is_warning_or_above());
        assert!(!ErrorSeverity::Info.is_error_or_above());
    }

    #[test]
    fn test_error_macros() {
        let err = error!("Test error");
        assert_eq!(err.severity, ErrorSeverity::Error);
        assert_eq!(err.message, "Test error");

        let critical_err = critical_error!("Critical issue");
        assert_eq!(critical_err.severity, ErrorSeverity::Critical);
    }
}