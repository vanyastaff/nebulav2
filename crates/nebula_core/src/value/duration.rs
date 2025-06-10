use derive_more::{Add, Deref, Display, Div, From, Into, Mul, Sub};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Duration value for time intervals
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord,
    Serialize, Deserialize, From, Into, Deref,
    Add, Sub, Mul, Div
)]
pub struct DurationValue(Duration);

impl DurationValue {
    /// Creates a new duration value
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Creates a duration from seconds
    pub fn from_secs(secs: u64) -> Self {
        Self(Duration::from_secs(secs))
    }

    /// Creates a duration from milliseconds
    pub fn from_millis(millis: u64) -> Self {
        Self(Duration::from_millis(millis))
    }

    /// Creates a duration from microseconds
    pub fn from_micros(micros: u64) -> Self {
        Self(Duration::from_micros(micros))
    }

    /// Creates a duration from nanoseconds
    pub fn from_nanos(nanos: u64) -> Self {
        Self(Duration::from_nanos(nanos))
    }

    /// Returns the duration in seconds
    pub fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }

    /// Returns the duration in milliseconds
    pub fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }

    /// Returns the duration in microseconds
    pub fn as_micros(&self) -> u128 {
        self.0.as_micros()
    }

    /// Returns the duration in nanoseconds
    pub fn as_nanos(&self) -> u128 {
        self.0.as_nanos()
    }

    /// Returns true if the duration is zero
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl std::fmt::Display for DurationValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}ms", self.as_millis())
    }
}

impl Into<serde_json::Value> for DurationValue {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Number(
            serde_json::Number::from(self.as_millis() as u64)
        )
    }
}
