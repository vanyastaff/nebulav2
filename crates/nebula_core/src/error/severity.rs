use serde::{Deserialize, Serialize};
use std::fmt;

/// Severity level for errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Critical errors that prevent the system from functioning
    Critical,
    /// Regular errors that prevent current operation
    Error,
    /// Warnings that don't prevent operation but indicate issues
    Warning,
    /// Informational messages for debugging/monitoring
    Info,
    /// Debug-level information
    Debug,
}

impl ErrorSeverity {
    /// Returns true if this severity level indicates a serious problem
    pub fn is_serious(&self) -> bool {
        matches!(self, ErrorSeverity::Critical | ErrorSeverity::Error)
    }

    /// Returns true if this severity should be logged by default
    pub fn should_log(&self) -> bool {
        matches!(
            self,
            ErrorSeverity::Critical | ErrorSeverity::Error | ErrorSeverity::Warning
        )
    }

    /// Returns the severity as a string for logging/telemetry
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorSeverity::Critical => "critical",
            ErrorSeverity::Error => "error",
            ErrorSeverity::Warning => "warning",
            ErrorSeverity::Info => "info",
            ErrorSeverity::Debug => "debug",
        }
    }

    /// Returns color code for UI display
    pub fn color_code(&self) -> &'static str {
        match self {
            ErrorSeverity::Critical => "#FF0000", // Red
            ErrorSeverity::Error => "#FF4444",    // Light Red
            ErrorSeverity::Warning => "#FFA500",  // Orange
            ErrorSeverity::Info => "#0066CC",     // Blue
            ErrorSeverity::Debug => "#666666",    // Gray
        }
    }

    /// Returns the priority level (higher number = higher priority)
    pub fn priority(&self) -> u8 {
        match self {
            ErrorSeverity::Critical => 5,
            ErrorSeverity::Error => 4,
            ErrorSeverity::Warning => 3,
            ErrorSeverity::Info => 2,
            ErrorSeverity::Debug => 1,
        }
    }
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PartialOrd for ErrorSeverity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ErrorSeverity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}