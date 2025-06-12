use std::backtrace::{Backtrace, BacktraceStatus};
use std::sync::OnceLock;

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
            BacktraceStatus::Captured => Some(format!("{bt}")),
            _ => None,
        }
    }

    fn is_captured() -> bool {
        static CACHED: OnceLock<bool> = OnceLock::new();
        *CACHED.get_or_init(|| {
            matches!(Backtrace::force_capture().status(), BacktraceStatus::Captured)
        })
    }

    #[cfg(debug_assertions)]
    fn capture_debug() -> Option<String> {
        <Self as BacktraceExt>::capture()
    }

    #[cfg(not(debug_assertions))]
    fn capture_debug() -> Option<String> {
        None
    }
}