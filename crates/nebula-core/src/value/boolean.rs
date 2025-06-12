use serde::{Deserialize, Serialize};
use std::ops::{Not, BitAnd, BitOr, BitXor};
use std::str::FromStr;
use crate::value::{ValueError, ValueResult};

/// Boolean value type with extended functionality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BooleanValue(bool);

impl BooleanValue {
    // Creates a new boolean value from a raw bool (const fn)
    /// Represents a constant true value
    pub const TRUE: Self = Self(true);
    /// Represents a constant false value
    pub const FALSE: Self = Self(false);
    
    /// Creates a new boolean value (const fn)
    pub const fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the underlying bool value
    pub const fn value(&self) -> bool {
        self.0
    }

    /// Returns the negated value
    pub const fn not(&self) -> Self {
        Self(!self.0)
    }

    /// Logical AND operation
    pub const fn and(&self, other: Self) -> Self {
        Self(self.0 && other.0)
    }

    /// Logical OR operation
    pub const fn or(&self, other: Self) -> Self {
        Self(self.0 || other.0)
    }

    /// Logical XOR operation
    pub const fn xor(&self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Converts to a static string slice
    pub const fn as_str(&self) -> &'static str {
        if self.0 { "true" } else { "false" }
    }

    /// Attempts to parse from string
    pub fn parse(s: &str) -> ValueResult<Self> {
        match s.to_lowercase().as_str() {
            "true" | "t" | "1" | "yes" | "y" => Ok(Self(true)),
            "false" | "f" | "0" | "no" | "n" => Ok(Self(false)),
            _ => Err(ValueError::invalid_boolean(s)),
        }
    }

    /// Returns Some if both values are true
    pub const fn and_then(&self, other: Self) -> Option<Self> {
        if self.0 && other.0 {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Returns Some if either value is true
    pub const fn or_else(&self, other: Self) -> Option<Self> {
        if self.0 || other.0 {
            Some(Self(true))
        } else {
            None
        }
    }
}

// Core trait implementations
impl Not for BooleanValue {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}

impl BitAnd for BooleanValue {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitOr for BooleanValue {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitXor for BooleanValue {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

// Conversion traits
impl From<bool> for BooleanValue {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<BooleanValue> for bool {
    fn from(value: BooleanValue) -> bool {
        value.0
    }
}

impl From<BooleanValue> for serde_json::Value {
    fn from(value: BooleanValue) -> Self {
        serde_json::Value::Bool(value.0)
    }
}

impl TryFrom<serde_json::Value> for BooleanValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::Bool(b) => Ok(Self(b)),
            other => Err(ValueError::type_conversion_with_value(
                other.to_string(),
                "BooleanValue",
                format!("{:?}", other)
            )),
        }
    }
}

// Display and string conversion
impl std::fmt::Display for BooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for BooleanValue {
    type Err = ValueError;

    fn from_str(s: &str) -> ValueResult<Self> {
        Self::parse(s)
    }
}

// Comparison with other types
impl PartialEq<bool> for BooleanValue {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

impl PartialEq<BooleanValue> for bool {
    fn eq(&self, other: &BooleanValue) -> bool {
        *self == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        const TRUE: BooleanValue = BooleanValue::new(true);
        const FALSE: BooleanValue = BooleanValue::new(false);

        assert!(TRUE.value());
        assert!(!FALSE.value());
    }

    #[test]
    fn test_operations() {
        let t = BooleanValue::new(true);
        let f = BooleanValue::new(false);

        assert_eq!(t.not(), f);
        assert_eq!(t.and(f), f);
        assert_eq!(t.or(f), t);
        assert_eq!(t.xor(f), t);
        assert_eq!(t.and_then(f), None);
        assert_eq!(t.or_else(f), Some(t));
    }

    #[test]
    fn test_parsing() {
        assert_eq!(BooleanValue::parse("true").unwrap(), true);
        assert_eq!(BooleanValue::parse("YES").unwrap(), true);
        assert_eq!(BooleanValue::parse("0").unwrap(), false);
        assert!(BooleanValue::parse("maybe").is_err());
    }

    #[test]
    fn test_json_conversion() {
        let t = BooleanValue::new(true);
        let json: serde_json::Value = t.into();
        assert_eq!(json, serde_json::Value::Bool(true));

        let back = BooleanValue::try_from(json).unwrap();
        assert_eq!(back, t);
    }
}