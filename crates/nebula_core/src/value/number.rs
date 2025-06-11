use derive_more::{Add, Display, Sub};
use serde::{Deserialize, Serialize};

/// Numeric value type preserving integer vs float distinction
#[derive(Debug, Clone, Serialize, Deserialize, Add, Sub, Display)]
#[serde(untagged)] // JSON: 42 or 3.14 (no type tag)
pub enum NumberValue {
    /// Integer value (preserves exact integer representation)
    Integer(i64),
    /// Floating-point value
    Float(f64),
}

impl NumberValue {
    /// Creates an integer number value
    pub fn integer(value: i64) -> Self {
        NumberValue::Integer(value)
    }

    /// Creates a float number value
    pub fn float(value: f64) -> Self {
        NumberValue::Float(value)
    }

    /// Returns true if this is an integer value
    pub fn is_integer(&self) -> bool {
        matches!(self, NumberValue::Integer(_))
    }

    /// Returns true if this is a float value
    pub fn is_float(&self) -> bool {
        matches!(self, NumberValue::Float(_))
    }

    /// Returns true if the number is finite (not NaN or infinite)
    pub fn is_finite(&self) -> bool {
        match self {
            NumberValue::Integer(_) => true,
            NumberValue::Float(f) => f.is_finite(),
        }
    }

    /// Converts to f64 (always succeeds)
    pub fn as_f64(&self) -> f64 {
        match self {
            NumberValue::Integer(i) => *i as f64,
            NumberValue::Float(f) => *f,
        }
    }

    /// Converts to i64 if possible
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            NumberValue::Integer(i) => Some(*i),
            NumberValue::Float(f) => {
                if f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 {
                    Some(*f as i64)
                } else {
                    None
                }
            }
        }
    }

    /// Returns the absolute value
    pub fn abs(&self) -> NumberValue {
        match self {
            NumberValue::Integer(i) => NumberValue::Integer(i.abs()),
            NumberValue::Float(f) => NumberValue::Float(f.abs()),
        }
    }

    /// Rounds to the nearest integer (returns Integer if input was integer)
    pub fn round(&self) -> NumberValue {
        match self {
            NumberValue::Integer(i) => NumberValue::Integer(*i),
            NumberValue::Float(f) => NumberValue::Integer(f.round() as i64),
        }
    }

    /// Rounds down to the nearest integer
    pub fn floor(&self) -> NumberValue {
        match self {
            NumberValue::Integer(i) => NumberValue::Integer(*i),
            NumberValue::Float(f) => NumberValue::Integer(f.floor() as i64),
        }
    }

    /// Rounds up to the nearest integer
    pub fn ceil(&self) -> NumberValue {
        match self {
            NumberValue::Integer(i) => NumberValue::Integer(*i),
            NumberValue::Float(f) => NumberValue::Integer(f.ceil() as i64),
        }
    }
}

impl From<i8> for NumberValue {
    fn from(value: i8) -> Self {
        NumberValue::Integer(value as i64)
    }
}
impl From<i16> for NumberValue {
    fn from(value: i16) -> Self {
        NumberValue::Integer(value as i64)
    }
}
impl From<i32> for NumberValue {
    fn from(value: i32) -> Self {
        NumberValue::Integer(value as i64)
    }
}
impl From<i64> for NumberValue {
    fn from(value: i64) -> Self {
        NumberValue::Integer(value)
    }
}
impl From<u8> for NumberValue {
    fn from(value: u8) -> Self {
        NumberValue::Integer(value as i64)
    }
}
impl From<u16> for NumberValue {
    fn from(value: u16) -> Self {
        NumberValue::Integer(value as i64)
    }
}
impl From<u32> for NumberValue {
    fn from(value: u32) -> Self {
        NumberValue::Integer(value as i64)
    }
}
impl From<f32> for NumberValue {
    fn from(value: f32) -> Self {
        NumberValue::Float(value as f64)
    }
}
impl From<f64> for NumberValue {
    fn from(value: f64) -> Self {
        NumberValue::Float(value)
    }
}

impl PartialOrd for NumberValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_f64().partial_cmp(&other.as_f64())
    }
}

impl Ord for NumberValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_f64()
            .partial_cmp(&other.as_f64())
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialEq for NumberValue {
    fn eq(&self, other: &Self) -> bool {
        self.as_f64() == other.as_f64()
    }
}

impl Eq for NumberValue {}

impl Into<serde_json::Value> for NumberValue {
    fn into(self) -> serde_json::Value {
        match self {
            NumberValue::Integer(i) => serde_json::Value::Number(serde_json::Number::from(i)),
            NumberValue::Float(f) => {
                serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap())
            }
        }
    }
}
