use std::backtrace::{Backtrace, BacktraceStatus};
use super::ErrorContext;

/// Extension trait for backtrace functionality
pub trait BacktraceExt {
    /// Captures backtrace if enabled
    fn capture() -> Option<String>;

    /// Checks if backtrace is available
    fn is_captured() -> bool;
}

impl BacktraceExt for Backtrace {
    fn capture() -> Option<String> {
        let bt = Backtrace::force_capture();
        match bt.status() {
            BacktraceStatus::Captured | BacktraceStatus::Disabled => Some(bt.to_string()),
            _ => None,
        }
    }

    fn is_captured() -> bool {
        matches!(Backtrace::force_capture().status(), BacktraceStatus::Captured)
    }
}

impl ErrorContext {
    /// Creates context with automatically captured backtrace
    #[track_caller]
    pub fn with_auto_backtrace() -> Self {
        let mut ctx = Self::at_caller();
        ctx.stack_trace = Some(Backtrace::capture().to_string());
        ctx
    }

    /// Creates context with caller location
    #[track_caller]
    pub fn at_caller() -> Self {
        let location = std::panic::Location::caller();
        Self {
            source_location: Some(format!(
                "{}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            )),
            ..Default::default()
        }
    }
}