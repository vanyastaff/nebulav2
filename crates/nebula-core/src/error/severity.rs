use serde::{Deserialize, Serialize};
use std::{cmp, fmt, str::FromStr};

/// Error severity levels with serde integration and ordering support
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ErrorSeverity {
    /// Critical errors that require immediate attention
    #[serde(rename = "critical")]
    Critical,
    /// Regular errors indicating problems
    #[serde(rename = "error")]
    Error,
    /// Warnings about potential issues
    #[serde(rename = "warning")]
    Warning,
    /// Informational messages
    #[serde(rename = "info")]
    Info,
    /// Debug-level messages
    #[serde(rename = "debug")]
    Debug,
}

// Константы для часто используемых уровней
impl ErrorSeverity {
    pub const DEFAULT: Self = Self::Error;
    pub const LOGGING_THRESHOLD: Self = Self::Warning;

    /// Все возможные варианты в строковом представлении
    pub const fn variants() -> &'static [&'static str] {
        &["critical", "error", "warning", "info", "debug"]
    }

    /// String representation of the severity level
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Debug => "debug",
        }
    }

    /// Priority level (higher = more severe)
    pub const fn priority(&self) -> u8 {
        match self {
            Self::Critical => 5,
            Self::Error => 4,
            Self::Warning => 3,
            Self::Info => 2,
            Self::Debug => 1,
        }
    }

    /// Parse from string with case-insensitive matching (optimized version)
    pub fn from_str(s: &str) -> Option<Self> {
        if s.eq_ignore_ascii_case("critical") || s.eq_ignore_ascii_case("crit") || s.eq_ignore_ascii_case("fatal") {
            Some(Self::Critical)
        } else if s.eq_ignore_ascii_case("error") || s.eq_ignore_ascii_case("err") {
            Some(Self::Error)
        } else if s.eq_ignore_ascii_case("warning") || s.eq_ignore_ascii_case("warn") {
            Some(Self::Warning)
        } else if s.eq_ignore_ascii_case("info") || s.eq_ignore_ascii_case("information") {
            Some(Self::Info)
        } else if s.eq_ignore_ascii_case("debug") || s.eq_ignore_ascii_case("dbg") {
            Some(Self::Debug)
        } else {
            None
        }
    }

    /// Convert to tracing Level (if tracing feature enabled)
    #[cfg(feature = "tracing")]
    pub const fn to_tracing_level(&self) -> tracing::Level {
        match self {
            Self::Critical => tracing::Level::ERROR,
            Self::Error => tracing::Level::ERROR,
            Self::Warning => tracing::Level::WARN,
            Self::Info => tracing::Level::INFO,
            Self::Debug => tracing::Level::DEBUG,
        }
    }

    /// More precise severity checks
    pub const fn is_critical(&self) -> bool {
        matches!(self, Self::Critical)
    }

    pub const fn is_error_or_worse(&self) -> bool {
        matches!(self, Self::Critical | Self::Error)
    }

    pub const fn is_warning_or_worse(&self) -> bool {
        matches!(self, Self::Critical | Self::Error | Self::Warning)
    }

    /// Check if severity is for debugging purposes
    pub const fn is_debug(&self) -> bool {
        matches!(self, Self::Debug)
    }

    /// Returns true if severity should be logged (>= min_severity)
    pub const fn should_log(&self, min_severity: ErrorSeverity) -> bool {
        self.priority() >= min_severity.priority()
    }

    /// Default minimum log level
    pub const fn default_log_level() -> Self {
        Self::Warning
    }

    // Const instances for pattern matching
    pub const CRITICAL: Self = Self::Critical;
    pub const ERROR: Self = Self::Error;
    pub const WARNING: Self = Self::Warning;
    pub const INFO: Self = Self::Info;
    pub const DEBUG: Self = Self::Debug;
}

// Formatting implementation
impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// FromStr implementation with better error handling
impl FromStr for ErrorSeverity {
    type Err = ParseSeverityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or_else(|| ParseSeverityError::new(s))
    }
}

// Ordering implementation
impl PartialOrd for ErrorSeverity {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ErrorSeverity {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

// Default implementation
impl Default for ErrorSeverity {
    fn default() -> Self {
        Self::DEFAULT
    }
}

// Conversion from log::Level
impl From<log::Level> for ErrorSeverity {
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Error => Self::Error,
            log::Level::Warn => Self::Warning,
            log::Level::Info => Self::Info,
            log::Level::Debug | log::Level::Trace => Self::Debug,
        }
    }
}

/// Error type for severity parsing failures
#[derive(Debug, Clone)]
pub struct ParseSeverityError {
    input: String,
}

impl ParseSeverityError {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

    pub fn input(&self) -> &str {
        &self.input
    }
}

impl fmt::Display for ParseSeverityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid severity level '{}'. Expected one of: {}",
            self.input,
            ErrorSeverity::variants().join(", ")
        )
    }
}

impl std::error::Error for ParseSeverityError {}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("critical", Ok(ErrorSeverity::Critical))]
    #[test_case("ERROR", Ok(ErrorSeverity::Error))]
    #[test_case("warn", Ok(ErrorSeverity::Warning))]
    #[test_case("INFO", Ok(ErrorSeverity::Info))]
    #[test_case("dbg", Ok(ErrorSeverity::Debug))]
    #[test_case("unknown", Err(()))]
    fn test_from_str(input: &str, expected: Result<ErrorSeverity, ()>) {
        assert_eq!(ErrorSeverity::from_str(input).ok_or(()), expected);
    }

    #[test]
    fn test_ordering() {
        assert!(ErrorSeverity::Critical > ErrorSeverity::Error);
        assert!(ErrorSeverity::Error > ErrorSeverity::Warning);
        assert!(ErrorSeverity::Warning > ErrorSeverity::Info);
        assert!(ErrorSeverity::Info > ErrorSeverity::Debug);
    }

    #[test]
    fn test_serde() {
        let severity = ErrorSeverity::Critical;
        let serialized = serde_json::to_string(&severity).unwrap();
        assert_eq!(serialized, "\"critical\"");

        let deserialized: ErrorSeverity = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, severity);
    }

    #[test]
    fn test_default() {
        assert_eq!(ErrorSeverity::default(), ErrorSeverity::DEFAULT);
    }

    #[test]
    fn test_from_log_level() {
        assert_eq!(ErrorSeverity::from(log::Level::Error), ErrorSeverity::Error);
        assert_eq!(ErrorSeverity::from(log::Level::Warn), ErrorSeverity::Warning);
        assert_eq!(ErrorSeverity::from(log::Level::Info), ErrorSeverity::Info);
        assert_eq!(ErrorSeverity::from(log::Level::Debug), ErrorSeverity::Debug);
        assert_eq!(ErrorSeverity::from(log::Level::Trace), ErrorSeverity::Debug);
    }
}