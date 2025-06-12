// crates/nebula_core/src/value/number.rs

use crate::value::{ValueError, ValueResult};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

/// Numeric value type supporting integers and floating-point numbers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberValue {
    /// Integer value
    Integer(i64),
    /// Floating-point value
    Float(f64),
}

impl NumberValue {
    /// Creates a new integer number
    pub fn new_int(value: i64) -> Self {
        Self::Integer(value)
    }

    /// Creates a new floating-point number
    pub fn new_float(value: f64) -> Self {
        Self::Float(value)
    }

    /// Creates a number from any numeric type
    pub fn new<T>(value: T) -> Self
    where
        T: Into<NumberValue>,
    {
        value.into()
    }

    /// Returns true if this is an integer
    pub fn is_integer(&self) -> bool {
        matches!(self, NumberValue::Integer(_))
    }

    /// Returns true if this is a float
    pub fn is_float(&self) -> bool {
        matches!(self, NumberValue::Float(_))
    }

    /// Returns true if the number is zero
    pub fn is_zero(&self) -> bool {
        match self {
            NumberValue::Integer(i) => *i == 0,
            NumberValue::Float(f) => *f == 0.0,
        }
    }

    /// Returns true if the number is positive
    pub fn is_positive(&self) -> bool {
        match self {
            NumberValue::Integer(i) => *i > 0,
            NumberValue::Float(f) => *f > 0.0,
        }
    }

    /// Returns true if the number is negative
    pub fn is_negative(&self) -> bool {
        match self {
            NumberValue::Integer(i) => *i < 0,
            NumberValue::Float(f) => *f < 0.0,
        }
    }

    /// Returns true if the number is finite (not NaN or infinite)
    pub fn is_finite(&self) -> bool {
        match self {
            NumberValue::Integer(_) => true,
            NumberValue::Float(f) => f.is_finite(),
        }
    }

    /// Returns true if the number is NaN
    pub fn is_nan(&self) -> bool {
        match self {
            NumberValue::Integer(_) => false,
            NumberValue::Float(f) => f.is_nan(),
        }
    }

    /// Converts to f64 (always possible)
    pub fn as_f64(&self) -> f64 {
        match self {
            NumberValue::Integer(i) => *i as f64,
            NumberValue::Float(f) => *f,
        }
    }

    /// Attempts to convert to i64
    pub fn as_i64(&self) -> ValueResult<i64> {
        match self {
            NumberValue::Integer(i) => Ok(*i),
            NumberValue::Float(f) => {
                if f.fract() == 0.0 && f.is_finite() {
                    let i = *f as i64;
                    if (i as f64) == *f {
                        Ok(i)
                    } else {
                        Err(ValueError::number_out_of_range(
                            f.to_string(),
                            i64::MIN.to_string(),
                            i64::MAX.to_string(),
                        ))
                    }
                } else {
                    Err(ValueError::type_conversion_with_value(
                        "Float",
                        "Integer",
                        f.to_string(),
                    ))
                }
            }
        }
    }

    /// Attempts to convert to u64
    pub fn as_u64(&self) -> ValueResult<u64> {
        match self {
            NumberValue::Integer(i) => {
                if *i >= 0 {
                    Ok(*i as u64)
                } else {
                    Err(ValueError::number_out_of_range(
                        i.to_string(),
                        "0".to_string(),
                        u64::MAX.to_string(),
                    ))
                }
            }
            NumberValue::Float(f) => {
                if f.fract() == 0.0 && *f >= 0.0 && f.is_finite() {
                    let u = *f as u64;
                    if (u as f64) == *f {
                        Ok(u)
                    } else {
                        Err(ValueError::number_out_of_range(
                            f.to_string(),
                            "0".to_string(),
                            u64::MAX.to_string(),
                        ))
                    }
                } else {
                    Err(ValueError::type_conversion_with_value(
                        "Float",
                        "UnsignedInteger",
                        f.to_string(),
                    ))
                }
            }
        }
    }

    /// Safe addition with overflow detection
    pub fn add(&self, other: &NumberValue) -> ValueResult<NumberValue> {
        match (self, other) {
            (NumberValue::Integer(a), NumberValue::Integer(b)) => {
                a.checked_add(*b)
                    .map(NumberValue::Integer)
                    .ok_or_else(|| {
                        ValueError::number_out_of_range(
                            format!("{} + {}", a, b),
                            i64::MIN.to_string(),
                            i64::MAX.to_string(),
                        )
                    })
            }
            _ => {
                let result = self.as_f64() + other.as_f64();
                if result.is_finite() {
                    Ok(NumberValue::Float(result))
                } else {
                    Err(ValueError::number_out_of_range(
                        format!("{} + {}", self.as_f64(), other.as_f64()),
                        "finite number".to_string(),
                        "finite number".to_string(),
                    ))
                }
            }
        }
    }

    /// Safe subtraction with overflow detection
    pub fn subtract(&self, other: &NumberValue) -> ValueResult<NumberValue> {
        match (self, other) {
            (NumberValue::Integer(a), NumberValue::Integer(b)) => {
                a.checked_sub(*b)
                    .map(NumberValue::Integer)
                    .ok_or_else(|| {
                        ValueError::number_out_of_range(
                            format!("{} - {}", a, b),
                            i64::MIN.to_string(),
                            i64::MAX.to_string(),
                        )
                    })
            }
            _ => {
                let result = self.as_f64() - other.as_f64();
                if result.is_finite() {
                    Ok(NumberValue::Float(result))
                } else {
                    Err(ValueError::number_out_of_range(
                        format!("{} - {}", self.as_f64(), other.as_f64()),
                        "finite number".to_string(),
                        "finite number".to_string(),
                    ))
                }
            }
        }
    }

    /// Safe multiplication with overflow detection
    pub fn multiply(&self, other: &NumberValue) -> ValueResult<NumberValue> {
        match (self, other) {
            (NumberValue::Integer(a), NumberValue::Integer(b)) => {
                a.checked_mul(*b)
                    .map(NumberValue::Integer)
                    .ok_or_else(|| {
                        ValueError::number_out_of_range(
                            format!("{} * {}", a, b),
                            i64::MIN.to_string(),
                            i64::MAX.to_string(),
                        )
                    })
            }
            _ => {
                let result = self.as_f64() * other.as_f64();
                if result.is_finite() {
                    Ok(NumberValue::Float(result))
                } else {
                    Err(ValueError::number_out_of_range(
                        format!("{} * {}", self.as_f64(), other.as_f64()),
                        "finite number".to_string(),
                        "finite number".to_string(),
                    ))
                }
            }
        }
    }

    /// Safe division with zero-check
    pub fn divide(&self, other: &NumberValue) -> ValueResult<NumberValue> {
        if other.is_zero() {
            return Err(ValueError::DivisionByZero);
        }

        match (self, other) {
            (NumberValue::Integer(a), NumberValue::Integer(b)) => {
                if a % b == 0 {
                    // Exact division, keep as integer
                    Ok(NumberValue::Integer(a / b))
                } else {
                    // Non-exact division, convert to float
                    let result = (*a as f64) / (*b as f64);
                    Ok(NumberValue::Float(result))
                }
            }
            _ => {
                let result = self.as_f64() / other.as_f64();
                if result.is_finite() {
                    Ok(NumberValue::Float(result))
                } else {
                    Err(ValueError::number_out_of_range(
                        format!("{} / {}", self.as_f64(), other.as_f64()),
                        "finite number".to_string(),
                        "finite number".to_string(),
                    ))
                }
            }
        }
    }

    /// Modulo operation
    pub fn modulo(&self, other: &NumberValue) -> ValueResult<NumberValue> {
        if other.is_zero() {
            return Err(ValueError::DivisionByZero);
        }

        match (self, other) {
            (NumberValue::Integer(a), NumberValue::Integer(b)) => {
                Ok(NumberValue::Integer(a % b))
            }
            _ => {
                let result = self.as_f64() % other.as_f64();
                if result.is_finite() {
                    Ok(NumberValue::Float(result))
                } else {
                    Err(ValueError::number_out_of_range(
                        format!("{} % {}", self.as_f64(), other.as_f64()),
                        "finite number".to_string(),
                        "finite number".to_string(),
                    ))
                }
            }
        }
    }

    /// Power operation
    pub fn power(&self, exponent: &NumberValue) -> ValueResult<NumberValue> {
        let base = self.as_f64();
        let exp = exponent.as_f64();
        let result = base.powf(exp);

        if result.is_finite() {
            Ok(NumberValue::Float(result))
        } else {
            Err(ValueError::number_out_of_range(
                format!("{} ^ {}", base, exp),
                "finite number".to_string(),
                "finite number".to_string(),
            ))
        }
    }

    /// Absolute value
    pub fn abs(&self) -> NumberValue {
        match self {
            NumberValue::Integer(i) => NumberValue::Integer(i.abs()),
            NumberValue::Float(f) => NumberValue::Float(f.abs()),
        }
    }

    /// Square root
    pub fn sqrt(&self) -> ValueResult<NumberValue> {
        let value = self.as_f64();
        if value < 0.0 {
            Err(ValueError::number_out_of_range(
                value.to_string(),
                "0".to_string(),
                "positive number".to_string(),
            ))
        } else {
            Ok(NumberValue::Float(value.sqrt()))
        }
    }

    /// Ceiling
    pub fn ceil(&self) -> NumberValue {
        match self {
            NumberValue::Integer(i) => NumberValue::Integer(*i),
            NumberValue::Float(f) => NumberValue::Float(f.ceil()),
        }
    }

    /// Floor
    pub fn floor(&self) -> NumberValue {
        match self {
            NumberValue::Integer(i) => NumberValue::Integer(*i),
            NumberValue::Float(f) => NumberValue::Float(f.floor()),
        }
    }

    /// Round to nearest integer
    pub fn round(&self) -> NumberValue {
        match self {
            NumberValue::Integer(i) => NumberValue::Integer(*i),
            NumberValue::Float(f) => NumberValue::Float(f.round()),
        }
    }

    /// Truncate (remove fractional part)
    pub fn trunc(&self) -> NumberValue {
        match self {
            NumberValue::Integer(i) => NumberValue::Integer(*i),
            NumberValue::Float(f) => NumberValue::Float(f.trunc()),
        }
    }

    /// Validates that the number is within a range
    pub fn validate_range(&self, min: Option<f64>, max: Option<f64>) -> ValueResult<()> {
        let value = self.as_f64();

        if let Some(min_val) = min {
            if value < min_val {
                return Err(ValueError::number_out_of_range(
                    value.to_string(),
                    min_val.to_string(),
                    max.map_or("∞".to_string(), |m| m.to_string()),
                ));
            }
        }

        if let Some(max_val) = max {
            if value > max_val {
                return Err(ValueError::number_out_of_range(
                    value.to_string(),
                    min.map_or("-∞".to_string(), |m| m.to_string()),
                    max_val.to_string(),
                ));
            }
        }

        Ok(())
    }
}

// Trait implementations

impl Default for NumberValue {
    fn default() -> Self {
        NumberValue::Integer(0)
    }
}

impl fmt::Display for NumberValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberValue::Integer(i) => write!(f, "{}", i),
            NumberValue::Float(fl) => write!(f, "{}", fl),
        }
    }
}

impl FromStr for NumberValue {
    type Err = ValueError;

    fn from_str(s: &str) -> ValueResult<Self> {
        let trimmed = s.trim();

        // Try parsing as integer first
        if let Ok(i) = trimmed.parse::<i64>() {
            return Ok(NumberValue::Integer(i));
        }

        // Try parsing as float
        if let Ok(f) = trimmed.parse::<f64>() {
            return if f.is_finite() {
                Ok(NumberValue::Float(f))
            } else {
                Err(ValueError::invalid_number(s))
            }
        }

        Err(ValueError::invalid_number(s))
    }
}

// Ordering implementation
impl PartialOrd for NumberValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self.as_f64();
        let b = other.as_f64();
        a.partial_cmp(&b)
    }
}

// From implementations for common numeric types
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

// Arithmetic operators (safe versions)
impl Add for NumberValue {
    type Output = ValueResult<NumberValue>;

    fn add(self, rhs: NumberValue) -> Self::Output {
        self.add(&rhs)
    }
}

impl Add for &NumberValue {
    type Output = ValueResult<NumberValue>;

    fn add(self, rhs: &NumberValue) -> Self::Output {
        NumberValue::add(self, rhs)
    }
}

impl Sub for NumberValue {
    type Output = ValueResult<NumberValue>;

    fn sub(self, rhs: NumberValue) -> Self::Output {
        self.subtract(&rhs)
    }
}

impl Sub for &NumberValue {
    type Output = ValueResult<NumberValue>;

    fn sub(self, rhs: &NumberValue) -> Self::Output {
        NumberValue::subtract(self, rhs)
    }
}

impl Mul for NumberValue {
    type Output = ValueResult<NumberValue>;

    fn mul(self, rhs: NumberValue) -> Self::Output {
        self.multiply(&rhs)
    }
}

impl Mul for &NumberValue {
    type Output = ValueResult<NumberValue>;

    fn mul(self, rhs: &NumberValue) -> Self::Output {
        NumberValue::multiply(self, rhs)
    }
}

impl Div for NumberValue {
    type Output = ValueResult<NumberValue>;

    fn div(self, rhs: NumberValue) -> Self::Output {
        self.divide(&rhs)
    }
}

impl Div for &NumberValue {
    type Output = ValueResult<NumberValue>;

    fn div(self, rhs: &NumberValue) -> Self::Output {
        NumberValue::divide(self, rhs)
    }
}

impl Neg for NumberValue {
    type Output = NumberValue;

    fn neg(self) -> Self::Output {
        match self {
            NumberValue::Integer(i) => NumberValue::Integer(-i),
            NumberValue::Float(f) => NumberValue::Float(-f),
        }
    }
}

impl Neg for &NumberValue {
    type Output = NumberValue;

    fn neg(self) -> Self::Output {
        (*self).clone().neg()
    }
}

// JSON conversion
impl From<NumberValue> for serde_json::Value {
    fn from(value: NumberValue) -> Self {
        match value {
            NumberValue::Integer(i) => serde_json::Value::Number(serde_json::Number::from(i)),
            NumberValue::Float(f) => {
                if let Some(num) = serde_json::Number::from_f64(f) {
                    serde_json::Value::Number(num)
                } else {
                    serde_json::Value::Null // Handle NaN/Infinity
                }
            }
        }
    }
}

impl TryFrom<serde_json::Value> for NumberValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::Number(num) => {
                if let Some(i) = num.as_i64() {
                    Ok(NumberValue::Integer(i))
                } else if let Some(f) = num.as_f64() {
                    if f.is_finite() {
                        Ok(NumberValue::Float(f))
                    } else {
                        Err(ValueError::invalid_number(f.to_string()))
                    }
                } else {
                    Err(ValueError::invalid_number(num.to_string()))
                }
            }
            other => Err(ValueError::type_conversion_with_value(
                format!("{:?}", other.to_string()),
                "NumberValue",
                format!("{:?}", other),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let int_val = NumberValue::new_int(42);
        let float_val = NumberValue::new_float(3.14);

        assert_eq!(int_val, NumberValue::Integer(42));
        assert_eq!(float_val, NumberValue::Float(3.14));

        assert!(int_val.is_integer());
        assert!(float_val.is_float());
    }

    #[test]
    fn test_conversions() {
        let int_val = NumberValue::new_int(42);
        let float_val = NumberValue::new_float(42.0);

        assert_eq!(int_val.as_f64(), 42.0);
        assert_eq!(int_val.as_i64().unwrap(), 42);
        assert_eq!(float_val.as_i64().unwrap(), 42);

        let non_int_float = NumberValue::new_float(3.14);
        assert!(non_int_float.as_i64().is_err());
    }

    #[test]
    fn test_arithmetic() {
        let a = NumberValue::new_int(10);
        let b = NumberValue::new_int(3);

        assert_eq!(a.add(&b).unwrap(), NumberValue::Integer(13));
        assert_eq!(a.subtract(&b).unwrap(), NumberValue::Integer(7));
        assert_eq!(a.multiply(&b).unwrap(), NumberValue::Integer(30));
        assert_eq!(a.divide(&b).unwrap(), NumberValue::Float(10.0 / 3.0));
        assert_eq!(a.modulo(&b).unwrap(), NumberValue::Integer(1));
    }

    #[test]
    fn test_division_by_zero() {
        let a = NumberValue::new_int(10);
        let zero = NumberValue::new_int(0);

        assert!(matches!(a.divide(&zero), Err(ValueError::DivisionByZero)));
        assert!(matches!(a.modulo(&zero), Err(ValueError::DivisionByZero)));
    }

    #[test]
    fn test_overflow() {
        let max_int = NumberValue::new_int(i64::MAX);
        let one = NumberValue::new_int(1);

        assert!(max_int.add(&one).is_err());
    }

    #[test]
    fn test_math_functions() {
        let num = NumberValue::new_float(-3.7);

        assert_eq!(num.abs(), NumberValue::Float(3.7));
        assert_eq!(num.ceil(), NumberValue::Float(-3.0));
        assert_eq!(num.floor(), NumberValue::Float(-4.0));
        assert_eq!(num.round(), NumberValue::Float(-4.0));
        assert_eq!(num.trunc(), NumberValue::Float(-3.0));

        let positive = NumberValue::new_float(9.0);
        assert_eq!(positive.sqrt().unwrap(), NumberValue::Float(3.0));

        let negative = NumberValue::new_float(-4.0);
        assert!(negative.sqrt().is_err());
    }

    #[test]
    fn test_range_validation() {
        let num = NumberValue::new_float(5.0);

        assert!(num.validate_range(Some(0.0), Some(10.0)).is_ok());
        assert!(num.validate_range(Some(6.0), None).is_err());
        assert!(num.validate_range(None, Some(4.0)).is_err());
    }

    #[test]
    fn test_parsing() {
        assert_eq!("42".parse::<NumberValue>().unwrap(), NumberValue::Integer(42));
        assert_eq!("3.14".parse::<NumberValue>().unwrap(), NumberValue::Float(3.14));
        assert!("not_a_number".parse::<NumberValue>().is_err());
        assert!("NaN".parse::<NumberValue>().is_err());
    }

    #[test]
    fn test_from_implementations() {
        assert_eq!(NumberValue::from(42i32), NumberValue::Integer(42));
        assert_eq!(NumberValue::from(3.14f64), NumberValue::Float(3.14));
    }

    #[test]
    fn test_operators() {
        let a = NumberValue::new_int(10);
        let b = NumberValue::new_int(3);

        assert_eq!((a + b).unwrap(), NumberValue::Integer(13));
        assert_eq!((a - b).unwrap(), NumberValue::Integer(7));
        assert_eq!((a * b).unwrap(), NumberValue::Integer(30));
        assert!((a / b).is_ok());

        assert_eq!(-a, NumberValue::Integer(-10));
    }

    #[test]
    fn test_json_conversion() {
        let int_val = NumberValue::new_int(42);
        let json_val: serde_json::Value = int_val.clone().into();

        assert_eq!(json_val, serde_json::Value::Number(serde_json::Number::from(42)));

        let back = NumberValue::try_from(json_val).unwrap();
        assert_eq!(back, int_val);

        // Test error case
        let string_val = serde_json::Value::String("not_a_number".to_string());
        assert!(NumberValue::try_from(string_val).is_err());
    }

    #[test]
    fn test_comparison() {
        let a = NumberValue::new_int(5);
        let b = NumberValue::new_float(5.0);
        let c = NumberValue::new_int(10);

        assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
        assert_eq!(a.partial_cmp(&c), Some(Ordering::Less));
        assert_eq!(c.partial_cmp(&a), Some(Ordering::Greater));
    }
}