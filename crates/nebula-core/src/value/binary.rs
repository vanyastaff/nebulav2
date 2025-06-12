use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Deref, DerefMut};

/// Binary data value type with various conversion capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BinaryValue(#[serde(with = "serde_bytes")] Vec<u8>);

impl BinaryValue {
    /// Creates a new binary value from bytes
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

    /// Returns reference to inner bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consumes self and returns inner Vec<u8>
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Returns length of the binary data in bytes
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks if the binary data is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for BinaryValue {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<u8>> for BinaryValue {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
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

impl From<String> for BinaryValue {
    fn from(data: String) -> Self {
        Self(data.into_bytes())
    }
}

impl From<BinaryValue> for Vec<u8> {
    fn from(value: BinaryValue) -> Vec<u8> {
        value.0
    }
}

impl From<BinaryValue> for serde_json::Value {
    fn from(value: BinaryValue) -> serde_json::Value {
        serde_json::Value::String(value.to_base64())
    }
}

impl Deref for BinaryValue {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BinaryValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for BinaryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_base64())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_value_operations() {
        let data = b"hello world";
        let bv = BinaryValue::from(&data[..]);

        assert_eq!(bv.len(), 11);
        assert!(!bv.is_empty());
        assert_eq!(bv.as_bytes(), data);

        let base64 = bv.to_base64();
        assert_eq!(base64, "aGVsbG8gd29ybGQ=");

        let bv2 = BinaryValue::from_base64(&base64).unwrap();
        assert_eq!(bv, bv2);

        let vec: Vec<u8> = bv.into_bytes();
        assert_eq!(vec, data);
    }

    #[test]
    fn test_serialization() {
        let bv = BinaryValue::from("test");
        let json = serde_json::to_string(&bv).unwrap();
        assert_eq!(json, r#""dGVzdA==""#);

        let bv2: BinaryValue = serde_json::from_str(&json).unwrap();
        assert_eq!(bv, bv2);
    }

    #[test]
    fn test_display() {
        let bv = BinaryValue::from("display");
        assert_eq!(format!("{}", bv), "ZGlzcGxheQ==");
    }
}