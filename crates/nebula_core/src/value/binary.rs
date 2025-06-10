use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

/// Binary data value type
#[derive(
    Debug, Clone, PartialEq, Eq, Hash,
    Serialize, Deserialize, From, Into, Deref, DerefMut
)]
pub struct BinaryValue(Vec<u8>);

impl BinaryValue {
    /// Creates a new binary value
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    /// Creates from base64 string
    pub fn from_base64(encoded: &str) -> Result<Self, base64::DecodeError> {
        general_purpose::STANDARD.decode(encoded).map(Self)
    }

    /// Converts to base64 string
    pub fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.0)
    }
}

impl From<&[u8]> for BinaryValue {
    fn from(data: &[u8]) -> Self {
        Self(data.to_vec())
    }
}

impl From<&str> for BinaryValue {
    fn from(data: &str) -> Self {
        Self(data.as_bytes().to_vec())
    }
}

impl Default for BinaryValue {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Into<serde_json::Value> for BinaryValue {
    fn into(self) -> serde_json::Value {
        serde_json::Value::String(self.to_base64())
    }
}