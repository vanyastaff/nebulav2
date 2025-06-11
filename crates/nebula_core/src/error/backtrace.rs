use std::backtrace::{Backtrace, BacktraceStatus};
use super::ErrorContext;

/// Extension trait for backtrace functionality
pub trait BacktraceExt {
    /// Captures backtrace if enabled
    fn capture() -> Option<String>;

    /// Checks if backtrace is available
    fn is_captured() -> bool;

    /// Captures backtrace only in debug mode
    fn capture_debug() -> Option<String>;
}

impl BacktraceExt for Backtrace {
    fn capture() -> Option<String> {
        let bt = Backtrace::force_capture();
        match bt.status() {
            BacktraceStatus::Captured => Some(bt.to_string()),
            _ => None,
        }
    }

    fn is_captured() -> bool {
        matches!(Backtrace::force_capture().status(), BacktraceStatus::Captured)
    }

    #[cfg(debug_assertions)]
    fn capture_debug() -> Option<String> {
        let bt = Backtrace::force_capture();
        match bt.status() {
            BacktraceStatus::Captured => Some(bt.to_string()),
            _ => None,
        }
    }

    #[cfg(not(debug_assertions))]
    fn capture_debug() -> Option<String> {
        None
    }
}

impl ErrorContext {
    /// Creates context with caller location (fast)
    #[track_caller]
    #[inline]
    pub fn at_caller() -> Self {
        let location = std::panic::Location::caller();
        Self {
            source_location: Some(format!(
                "{}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            ).into_boxed_str()),
            ..Default::default()
        }
    }

    /// Creates context with caller and captures backtrace in debug
    #[track_caller]
    pub fn with_debug_backtrace() -> Self {
        let mut ctx = Self::at_caller();

        // Only capture in debug builds
        if let Some(trace) = Backtrace::capture_debug() {
            let _ = ctx.stack_trace.set(trace);
        }

        ctx
    }

    /// Forces backtrace capture regardless of build mode
    #[track_caller]
    pub fn with_forced_backtrace() -> Self {
        let mut ctx = Self::at_caller();

        if let Some(trace) = Backtrace::capture() {
            let _ = ctx.stack_trace.set(trace);
        }

        ctx
    }
}