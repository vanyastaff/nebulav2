use std::{
    error::Error as StdError,
    fmt::{self, Display},
};
use super::{ErrorSeverity, ErrorCategory, ErrorContext, ErrorInfo};

/// Core error trait (object-safe)
pub trait AnyError: HasSeverity + StdError + Send + Sync + 'static {
    /// Gets associated context (if any)
    fn context(&self) -> Option<&ErrorContext> {
        None
    }

    /// Returns standardized error code
    fn error_code(&self) -> ErrorCode {
        ErrorCode::new(self.category(), self.severity())
    }

    /// Records error using configured logging
    fn record(&self) {
        #[cfg(feature = "tracing")]
        self.record_tracing();

        #[cfg(not(feature = "tracing"))]
        log::error!("{}", self);
    }

    /// Tracing-specific recording
    #[cfg(feature = "tracing")]
    fn record_tracing(&self) {
        use tracing::{event, span, Level};

        let level = self.severity().to_tracing_level();
        let span = span!(
            Level::ERROR,
            "error",
            code = %self.error_code(),
            category = %self.category(),
            severity = %self.severity(),
            error = %self
        );

        let _enter = span.enter();
        event!(level, "Error occurred");

        if let Some(ctx) = self.context() {
            event!(Level::DEBUG, context = ?ctx);
        }
    }
}

/// Standardized error code with optimized display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorCode {
    pub category: ErrorCategory,
    pub severity: ErrorSeverity,
}

impl ErrorCode {
    pub const fn new(category: ErrorCategory, severity: ErrorSeverity) -> Self {
        Self { category, severity }
    }

    pub const fn as_str(&self) -> &'static str {
        // Compile-time generated strings
        match (self.category, self.severity) {
            (ErrorCategory::General, ErrorSeverity::Debug) => "general_debug",
            (ErrorCategory::General, ErrorSeverity::Info) => "general_info",
            // ... other combinations
            _ => "unknown_error",
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Enhanced error severity and categorization
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

    /// Checks if error should be logged based on default log level
    #[inline]
    fn should_log(&self) -> bool {
        self.severity().should_log(ErrorSeverity::default_log_level())
    }

    /// Checks if error should be logged with custom minimum severity
    #[inline]
    fn should_log_at(&self, min_severity: ErrorSeverity) -> bool {
        self.severity().should_log(min_severity)
    }

    /// Checks if error should be shown to users
    #[inline]
    fn is_user_facing(&self) -> bool {
        self.category().is_user_facing() && !self.severity().is_debug()
    }
}

/// Enhanced error chain analysis
pub trait ErrorChain: StdError {
    /// Finds the root cause error
    fn root_cause(&self) -> &(dyn StdError + 'static) {
        let mut cause = self;
        while let Some(source) = cause.source() {
            cause = source;
        }
        cause
    }

    /// Returns a full error chain as iterator
    fn chain(&self) -> ErrorChainIter<'_> {
        ErrorChainIter { current: Some(self) }
    }

    /// Checks if an error chain contains a specific type
    fn has_error<T: StdError + 'static>(&self) -> bool {
        self.chain().any(|e| e.is::<T>())
    }

    /// Finds first error of type T in chain
    fn find_error<T: StdError + 'static>(&self) -> Option<&T> {
        self.chain().find_map(|e| e.downcast_ref::<T>())
    }
}

/// Iterator over error chain
pub struct ErrorChainIter<'a> {
    current: Option<&'a (dyn StdError + 'static)>,
}

impl<'a> Iterator for ErrorChainIter<'a> {
    type Item = &'a (dyn StdError + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = current.source();
        Some(current)
    }
}

impl<T: StdError + ?Sized> ErrorChain for T {}

/// Enhanced conversion to ErrorInfo
pub trait ToErrorInfo {
    /// Converts to structured error info
    fn to_error_info(&self) -> ErrorInfo;

    /// Converts with additional context
    fn with_error_info<C>(self, context: C) -> ErrorInfo
    where
        C: Into<ErrorContext>,
        Self: Sized,
    {
        let mut info = self.to_error_info();
        info.context = info.context.merge(context.into());
        info
    }
}

impl<T: HasSeverity + StdError> ToErrorInfo for T {
    fn to_error_info(&self) -> ErrorInfo {
        let context = if let Some(any_err) = (self as &dyn StdError).downcast_ref::<dyn AnyError>() {
            any_err.context().cloned().unwrap_or_default()
        } else {
            ErrorContext::default()
        };

        ErrorInfo {
            severity: self.severity(),
            category: self.category(),
            message: self.to_string().into(),
            context,
            source: Some(Box::new(self)),
        }
    }
}

/// Enhanced Result extensions
pub trait ResultExt<T, E> {
    /// Adds context to error result
    fn err_context<C>(self, context: C) -> Result<T, ErrorInfo>
    where
        C: Into<ErrorContext>;

    /// Converts error to ErrorInfo with context
    fn map_err_info<C>(self, context: C) -> Result<T, ErrorInfo>
    where
        C: Into<ErrorContext>,
        E: ToErrorInfo;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: ToErrorInfo,
{
    fn err_context<C>(self, context: C) -> Result<T, ErrorInfo>
    where
        C: Into<ErrorContext>,
    {
        self.map_err(|e| e.with_error_info(context))
    }

    fn map_err_info<C>(self, context: C) -> Result<T, ErrorInfo>
    where
        C: Into<ErrorContext>,
        E: ToErrorInfo,
    {
        self.map_err(|e| e.with_error_info(context))
    }
}

/// Extension for Option
pub trait OptionExt<T> {
    /// Converts None to error with context
    fn none_context<C>(self, context: C) -> Result<T, ErrorInfo>
    where
        C: Into<ErrorContext>;
}

impl<T> OptionExt<T> for Option<T> {
    fn none_context<C>(self, context: C) -> Result<T, ErrorInfo>
    where
        C: Into<ErrorContext>,
    {
        self.ok_or_else(|| {
            ErrorInfo::new(ErrorSeverity::Error, "None value")
                .with_context(context.into())
        })
    }
}