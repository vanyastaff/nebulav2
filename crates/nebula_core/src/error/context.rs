use std::collections::HashMap;
use std::sync::OnceLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Rich error context with optimized storage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)] // Защита от неизвестных полей
pub struct ErrorContext {
    #[serde(default)]
    pub data: HashMap<Box<str>, Box<str>>, // Все метаданные в одном месте

    #[serde(default = "Utc::now")]
    pub timestamp: DateTime<Utc>,

    #[serde(skip)] // Не сериализуем OnceLock
    pub(crate) stack_trace: OnceLock<String>, // Ленивая загрузка, доступ внутри модуля

    #[serde(default)]
    pub source_location: Option<Box<str>>, // Оставляем только location
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            timestamp: Utc::now(),
            stack_trace: OnceLock::new(),
            source_location: None,
        }
    }
}

impl ErrorContext {
    /// Creates new empty context
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a key-value pair with validation
    #[inline]
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let value = value.into();

        // Валидация ключей
        debug_assert!(!key.is_empty(), "Key cannot be empty");
        debug_assert!(!key.contains('\0'), "Key cannot contain null bytes");
        debug_assert!(key.len() <= 256, "Key too long (max 256 chars)");

        self.data.insert(key.into_boxed_str(), value.into_boxed_str());
        self
    }



    /// Gets stack trace (lazy loading)
    pub fn stack_trace(&self) -> Option<&str> {
        self.stack_trace.get().map(|s| s.as_str())
    }

    /// Forces stack trace capture
    pub fn ensure_stack_trace(&self) -> Option<&str> {
        use std::backtrace::Backtrace;

        self.stack_trace.get_or_init(|| {
            Backtrace::force_capture().to_string()
        });

        self.stack_trace()
    }

    /// Merges another context
    pub fn merge(mut self, other: Self) -> Self {
        self.data.extend(other.data);

        // Merge stack trace (prefer existing)
        if self.stack_trace.get().is_none() {
            if let Some(trace) = other.stack_trace.get() {
                let _ = self.stack_trace.set(trace.clone());
            }
        }

        self.source_location = self.source_location.or(other.source_location);
        self
    }

    /// Checks if context has any data
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty() &&
            self.stack_trace.get().is_none() &&
            self.source_location.is_none()
    }

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
        #[cfg(debug_assertions)]
        {
            use std::backtrace::{Backtrace, BacktraceStatus};
            let bt = Backtrace::force_capture();
            if matches!(bt.status(), BacktraceStatus::Captured) {
                let _ = ctx.stack_trace.set(bt.to_string());
            }
        }

        ctx
    }

    /// Forces backtrace capture regardless of build mode
    #[track_caller]
    pub fn with_forced_backtrace() -> Self {
        let mut ctx = Self::at_caller();

        use std::backtrace::{Backtrace, BacktraceStatus};
        let bt = Backtrace::force_capture();
        if matches!(bt.status(), BacktraceStatus::Captured) {
            let _ = ctx.stack_trace.set(bt.to_string());
        }

        ctx
    }
}