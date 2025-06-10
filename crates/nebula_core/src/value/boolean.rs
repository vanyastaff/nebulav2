use derive_more::{Deref, Display, From, Into};
use serde::{Deserialize, Serialize};

/// Boolean value type for true/false values
#[derive(
    Debug, Clone, PartialEq, Eq, Hash,
    Serialize, Deserialize, From, Into, Deref, Display
)]
pub struct BooleanValue(bool);

impl BooleanValue {
    /// Creates a new boolean value
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the boolean value
    pub fn value(&self) -> bool {
        self.0
    }

    /// Returns the negated boolean value
    pub fn not(&self) -> BooleanValue {
        BooleanValue(!self.0)
    }
}

impl std::ops::Not for BooleanValue {
    type Output = BooleanValue;

    fn not(self) -> Self::Output {
        BooleanValue(!self.0)
    }
}

impl Into<serde_json::Value> for BooleanValue {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Bool(self.0)
    }
}