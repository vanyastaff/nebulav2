use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Context information for errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Additional key-value pairs with context information
    pub data: HashMap<String, String>,
    /// Timestamp when the error occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Stack trace if available
    pub stack_trace: Option<String>,
    /// Source location (file:line)
    pub source_location: Option<String>,
    /// User ID or session identifier
    pub user_id: Option<String>,
    /// Request ID for tracing
    pub request_id: Option<String>,
    /// Component or module where error occurred
    pub component: Option<String>,
}

impl ErrorContext {
    /// Creates a new error context
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            timestamp: chrono::Utc::now(),
            stack_trace: None,
            source_location: None,
            user_id: None,
            request_id: None,
            component: None,
        }
    }

    /// Adds a key-value pair to the context
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }

    /// Sets the source location
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.source_location = Some(location.into());
        self
    }

    /// Sets the stack trace
    pub fn with_stack_trace(mut self, trace: impl Into<String>) -> Self {
        self.stack_trace = Some(trace.into());
        self
    }

    /// Sets the user ID
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Sets the request ID
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Sets the component
    pub fn with_component(mut self, component: impl Into<String>) -> Self {
        self.component = Some(component.into());
        self
    }

    /// Adds multiple data entries at once
    pub fn with_data_entries<I, K, V>(mut self, entries: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in entries {
            self.data.insert(key.into(), value.into());
        }
        self
    }

    /// Merges another context into this one
    pub fn merge(mut self, other: ErrorContext) -> Self {
        self.data.extend(other.data);

        if self.stack_trace.is_none() {
            self.stack_trace = other.stack_trace;
        }

        if self.source_location.is_none() {
            self.source_location = other.source_location;
        }

        if self.user_id.is_none() {
            self.user_id = other.user_id;
        }

        if self.request_id.is_none() {
            self.request_id = other.request_id;
        }

        if self.component.is_none() {
            self.component = other.component;
        }

        self
    }

    /// Returns true if the context has any meaningful data
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
            && self.stack_trace.is_none()
            && self.source_location.is_none()
            && self.user_id.is_none()
            && self.request_id.is_none()
            && self.component.is_none()
    }
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}