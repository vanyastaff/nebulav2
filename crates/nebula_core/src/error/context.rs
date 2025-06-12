use std::{
    collections::HashMap,
    fmt,
    sync::OnceLock,
};
use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Rich error context with optimized storage and serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
#[must_use = "ErrorContext should not be ignored"]
pub struct ErrorContext {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub data: HashMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<Box<str>>,  // Оптимизированное хранение

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location: Option<Box<str>>,  // Оптимизированное хранение

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<Box<str>>,  // Оптимизированное хранение
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            stack_trace: None,
            source_location: None,
            timestamp: Some(Utc::now()),
            span_id: None,
        }
    }
}

impl ErrorContext {
    /// Creates new empty context with current timestamp
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates context without timestamp
    pub fn empty() -> Self {
        Self {
            timestamp: None,
            ..Default::default()
        }
    }

    /// Builder-style method to add key-value data
    pub fn with_data<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: ToString,
    {
        self.data.insert(key.into(), value.to_string());
        self
    }

    /// Adds multiple key-value pairs from iterator
    pub fn with_data_from_iter<I, K, V>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: ToString,
    {
        self.data.extend(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.to_string())),
        );
        self
    }

    /// Adds span/trace ID for distributed tracing
    pub fn with_span_id<S: Into<String>>(mut self, span_id: S) -> Self {
        self.span_id = Some(span_id.into().into_boxed_str());
        self
    }

    /// Merges with another context (other takes precedence)
    pub fn merge(mut self, other: Self) -> Self {
        self.data.extend(other.data);
        self.stack_trace = other.stack_trace.or(self.stack_trace);
        self.source_location = other.source_location.or(self.source_location);
        self.span_id = other.span_id.or(self.span_id);
        self.timestamp = other.timestamp.or(self.timestamp);
        self
    }

    /// Creates context with caller location
    #[track_caller]
    pub fn at_caller() -> Self {
        let location = std::panic::Location::caller();
        Self {
            source_location: Some(
                format!(
                    "{}:{}:{}",
                    location.file(),
                    location.line(),
                    location.column()
                ).into_boxed_str(),
            ),
            ..Default::default()
        }
    }

    /// Captures backtrace if enabled
    pub fn capture_backtrace(mut self) -> Self {
        #[cfg(feature = "backtrace")]
        {
            use std::backtrace::{Backtrace, BacktraceStatus};

            static BACKTRACE_ENABLED: OnceLock<bool> = OnceLock::new();
            let enabled = *BACKTRACE_ENABLED.get_or_init(|| {
                matches!(Backtrace::force_capture().status(), BacktraceStatus::Captured)
            });

            if enabled {
                let bt = Backtrace::force_capture();
                if matches!(bt.status(), BacktraceStatus::Captured) {
                    self.stack_trace = Some(bt.to_string().into_boxed_str());
                }
            }
        }
        self
    }

    /// Checks if context contains any data
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
            && self.stack_trace.is_none()
            && self.source_location.is_none()
            && self.span_id.is_none()
    }

    /// Gets typed data value
    pub fn get_typed<T: FromStr>(&self, key: &str) -> Option<T> {
        self.data.get(key).and_then(|v| v.parse().ok())
    }

    /// Formats a compact summary of the context
    pub fn format_compact(&self) -> String {
        let mut parts = Vec::with_capacity(3);

        if !self.data.is_empty() {
            parts.push(format!("data[{} keys]", self.data.len()));
        }

        if let Some(loc) = &self.source_location {
            parts.push(format!("loc={}", loc.split('/').last().unwrap_or(loc)));
        }

        if parts.is_empty() {
            "no-context".into()
        } else {
            parts.join(" | ")
        }
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_compact())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_context() {
        let ctx = ErrorContext::empty();
        assert!(ctx.is_empty());
        assert_eq!(ctx.format_compact(), "no-context");
    }

    #[test]
    fn test_data_operations() {
        let ctx = ErrorContext::new()
            .with_data("key1", "value1")
            .with_data("key2", 42);

        assert_eq!(ctx.data.len(), 2);
        assert_eq!(ctx.get_typed::<i32>("key2"), Some(42));
    }

    #[test]
    fn test_merge_contexts() {
        let ctx1 = ErrorContext::new().with_data("a", 1);
        let ctx2 = ErrorContext::new().with_data("b", 2);

        let merged = ctx1.merge(ctx2);
        assert_eq!(merged.data.len(), 2);
    }

    #[test]
    #[cfg(feature = "backtrace")]
    fn test_backtrace_capture() {
        let ctx = ErrorContext::new().capture_backtrace();
        assert!(ctx.stack_trace.is_some());
    }
}