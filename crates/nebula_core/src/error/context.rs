use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Rich error context with optimized storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    #[serde(default)]
    pub data: HashMap<String, String>,

    #[serde(default = "Utc::now")]
    pub timestamp: DateTime<Utc>,

    #[serde(default)]
    pub stack_trace: Option<String>,

    #[serde(default)]
    pub source_location: Option<String>,

    #[serde(default)]
    pub user_id: Option<String>,

    #[serde(default)]
    pub request_id: Option<String>,

    #[serde(default)]
    pub component: Option<String>,
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            timestamp: Utc::now(),
            stack_trace: None,
            source_location: None,
            user_id: None,
            request_id: None,
            component: None,
        }
    }
}

impl ErrorContext {
    /// Creates new empty context
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a key-value pair
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }

    /// Merges another context
    pub fn merge(mut self, other: Self) -> Self {
        self.data.extend(other.data);
        self.stack_trace = self.stack_trace.or(other.stack_trace);
        self.source_location = self.source_location.or(other.source_location);
        self.user_id = self.user_id.or(other.user_id);
        self.request_id = self.request_id.or(other.request_id);
        self.component = self.component.or(other.component);
        self
    }
}