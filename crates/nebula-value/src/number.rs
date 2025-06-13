#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::str::FromStr;
use crate::{ValueError, ValueResult};

/// Number value type supporting both integers and floating-point numbers
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NumberValue {
    /// 64-bit signed integer
    Integer(i64),
    /// 64-bit floating-point number
    Float(f64),
}

impl NumberValue {
    // === Constants ===

    /// Constant zero (integer)
    pub const ZERO: Self = Self::Integer(0);

    /// Constant one (integer)
    pub const ONE: Self = Self::Integer(1);

    /// Constant negative one (integer)
    pub const NEG_ONE: Self = Self::Integer(-1);

    /// Mathematical constant π (pi)
    pub const PI: Self = Self::Float(std::f64::consts::PI);

    /// Mathematical constant e (Euler's number)
    pub const E: Self = Self::Float(std::f64::consts::E);

    /// Constant representing positive infinity
    pub const INFINITY: Self = Self::Float(f64::INFINITY);

    /// Constant representing negative infinity
    pub const NEG_INFINITY: Self = Self::Float(f64::NEG_INFINITY);

    // === Constructors ===

    /// Creates a new integer value
    #[inline]
    #[must_use]
    pub const fn new_int(value: i64) -> Self {
        Self::Integer(value)
    }

    /// Creates a new float value
    #[inline]
    #[must_use]
    pub const fn new_float(value: f64) -> Self {
        Self::Float(value)
    }

    /// Creates an integer from a smaller integer type
    #[inline]
    #[must_use]
    pub fn from_int<T>(value: T) -> Self
    where
        T: Into<i64>,
    {
        Self::Integer(value.into())
    }

    /// Creates NumberValue from i32 (const version)
    #[inline]
    #[must_use]
    pub const fn from_i32(value: i32) -> Self {
        Self::Integer(value as i64)
    }

    /// Creates NumberValue from i16 (const version)
    #[inline]
    #[must_use]
    pub const fn from_i16(value: i16) -> Self {
        Self::Integer(value as i64)
    }

    /// Creates NumberValue from i8 (const version)
    #[inline]
    #[must_use]
    pub const fn from_i8(value: i8) -> Self {
        Self::Integer(value as i64)
    }

    /// Creates NumberValue from u32 (const version)
    #[inline]
    #[must_use]
    pub const fn from_u32(value: u32) -> Self {
        Self::Integer(value as i64)
    }

    /// Creates NumberValue from u16 (const version)
    #[inline]
    #[must_use]
    pub const fn from_u16(value: u16) -> Self {
        Self::Integer(value as i64)
    }

    /// Creates NumberValue from u8 (const version)
    #[inline]
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        Self::Integer(value as i64)
    }

    /// Creates NumberValue from usize (const version, may truncate on 32-bit systems)
    #[inline]
    #[must_use]
    pub const fn from_usize(value: usize) -> Self {
        Self::Integer(value as i64)
    }

    /// Creates zero value
    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        Self::ZERO
    }

    /// Creates one value
    #[inline]
    #[must_use]
    pub const fn one() -> Self {
        Self::ONE
    }

    // === Type Checks ===

    /// Returns true if this is an integer
    #[inline]
    #[must_use]
    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    /// Returns true if this is a float
    #[inline]
    #[must_use]
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    /// Returns true if the number is positive
    #[inline]
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        match self {
            Self::Integer(i) => *i > 0,
            Self::Float(f) => *f > 0.0,
        }
    }

    /// Returns true if the number is negative
    #[inline]
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        match self {
            Self::Integer(i) => *i < 0,
            Self::Float(f) => *f < 0.0,
        }
    }

    /// Returns true if the number is zero
    #[inline]
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        match self {
            Self::Integer(i) => *i == 0,
            Self::Float(f) => *f == 0.0,
        }
    }

    /// Returns true if this is a finite number (not NaN or infinity)
    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        match self {
            Self::Integer(_) => true,
            Self::Float(f) => f.is_finite(),
        }
    }

    /// Returns true if this is NaN
    #[inline]
    #[must_use]
    pub fn is_nan(&self) -> bool {
        match self {
            Self::Integer(_) => false,
            Self::Float(f) => f.is_nan(),
        }
    }

    /// Returns true if this is infinite
    #[inline]
    #[must_use]
    pub fn is_infinite(&self) -> bool {
        match self {
            Self::Integer(_) => false,
            Self::Float(f) => f.is_infinite(),
        }
    }

    /// Returns true if the number is an even integer
    #[inline]
    #[must_use]
    pub fn is_even(&self) -> bool {
        match self {
            Self::Integer(i) => i % 2 == 0,
            Self::Float(f) => f.fract() == 0.0 && (*f as i64) % 2 == 0,
        }
    }

    /// Returns true if the number is an odd integer
    #[inline]
    #[must_use]
    pub fn is_odd(&self) -> bool {
        match self {
            Self::Integer(i) => i % 2 != 0,
            Self::Float(f) => f.fract() == 0.0 && (*f as i64) % 2 != 0,
        }
    }

    // === Value Accessors ===

    /// Converts to f64 (always possible)
    #[inline]
    #[must_use]
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::Integer(i) => *i as f64,
            Self::Float(f) => *f,
        }
    }

    /// Converts to f32
    #[inline]
    #[must_use]
    pub fn as_f32(&self) -> f32 {
        self.as_f64() as f32
    }

    /// Tries to convert to i64
    pub fn as_i64(&self) -> ValueResult<i64> {
        match self {
            Self::Integer(i) => Ok(*i),
            Self::Float(f) => {
                if f.fract() == 0.0 && f.is_finite() && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 {
                    Ok(*f as i64)
                } else {
                    Err(ValueError::custom(format!(
                        "Cannot convert {} to integer", f
                    )))
                }
            }
        }
    }

    /// Tries to convert to i32
    pub fn as_i32(&self) -> ValueResult<i32> {
        let i64_val = self.as_i64()?;
        if i64_val >= i32::MIN as i64 && i64_val <= i32::MAX as i64 {
            Ok(i64_val as i32)
        } else {
            Err(ValueError::custom(format!(
                "Value {} is out of range for i32", i64_val
            )))
        }
    }

    /// Tries to convert to usize
    pub fn as_usize(&self) -> ValueResult<usize> {
        let i64_val = self.as_i64()?;
        if i64_val >= 0 && i64_val <= usize::MAX as i64 {
            Ok(i64_val as usize)
        } else {
            Err(ValueError::custom(format!(
                "Value {} is out of range for usize", i64_val
            )))
        }
    }

    // === Basic Arithmetic ===

    /// Safe addition
    pub fn add(&self, other: &Self) -> ValueResult<Self> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => {
                a.checked_add(*b)
                    .map(Self::Integer)
                    .ok_or_else(|| ValueError::custom("Integer overflow in addition"))
            }
            _ => Ok(Self::Float(self.as_f64() + other.as_f64())),
        }
    }

    /// Safe subtraction
    pub fn subtract(&self, other: &Self) -> ValueResult<Self> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => {
                a.checked_sub(*b)
                    .map(Self::Integer)
                    .ok_or_else(|| ValueError::custom("Integer overflow in subtraction"))
            }
            _ => Ok(Self::Float(self.as_f64() - other.as_f64())),
        }
    }

    /// Safe multiplication
    pub fn multiply(&self, other: &Self) -> ValueResult<Self> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => {
                a.checked_mul(*b)
                    .map(Self::Integer)
                    .ok_or_else(|| ValueError::custom("Integer overflow in multiplication"))
            }
            _ => Ok(Self::Float(self.as_f64() * other.as_f64())),
        }
    }

    /// Safe division
    pub fn divide(&self, other: &Self) -> ValueResult<Self> {
        if other.is_zero() {
            return Err(ValueError::custom("Division by zero"));
        }

        // Always return float for division to handle fractions
        Ok(Self::Float(self.as_f64() / other.as_f64()))
    }

    /// Modulo operation
    pub fn modulo(&self, other: &Self) -> ValueResult<Self> {
        if other.is_zero() {
            return Err(ValueError::custom("Modulo by zero"));
        }

        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => Ok(Self::Integer(a % b)),
            _ => Ok(Self::Float(self.as_f64() % other.as_f64())),
        }
    }

    /// Exponentiation
    pub fn pow(&self, exponent: &Self) -> ValueResult<Self> {
        let base = self.as_f64();
        let exp = exponent.as_f64();

        // Handle special cases
        if base == 0.0 && exp < 0.0 {
            return Err(ValueError::custom("Cannot raise zero to negative power"));
        }

        let result = base.powf(exp);

        if !result.is_finite() {
            return Err(ValueError::custom("Power operation resulted in non-finite number"));
        }

        // Try to return integer if possible
        if result.fract() == 0.0 && result >= i64::MIN as f64 && result <= i64::MAX as f64 {
            Ok(Self::Integer(result as i64))
        } else {
            Ok(Self::Float(result))
        }
    }

    // === Mathematical Functions ===

    /// Absolute value
    #[inline]
    #[must_use]
    pub fn abs(&self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(i.abs()),
            Self::Float(f) => Self::Float(f.abs()),
        }
    }

    /// Sign of the number (-1, 0, or 1)
    #[inline]
    #[must_use]
    pub fn signum(&self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(i.signum()),
            Self::Float(f) => Self::Float(f.signum()),
        }
    }

    /// Square root
    pub fn sqrt(&self) -> ValueResult<Self> {
        let value = self.as_f64();
        if value < 0.0 {
            return Err(ValueError::custom("Cannot take square root of negative number"));
        }
        Ok(Self::Float(value.sqrt()))
    }

    /// Cube root
    #[must_use]
    pub fn cbrt(&self) -> Self {
        Self::Float(self.as_f64().cbrt())
    }

    /// Natural logarithm (ln)
    pub fn ln(&self) -> ValueResult<Self> {
        let value = self.as_f64();
        if value <= 0.0 {
            return Err(ValueError::custom("Cannot take logarithm of non-positive number"));
        }
        Ok(Self::Float(value.ln()))
    }

    /// Base-10 logarithm
    pub fn log10(&self) -> ValueResult<Self> {
        let value = self.as_f64();
        if value <= 0.0 {
            return Err(ValueError::custom("Cannot take logarithm of non-positive number"));
        }
        Ok(Self::Float(value.log10()))
    }

    /// Base-2 logarithm
    pub fn log2(&self) -> ValueResult<Self> {
        let value = self.as_f64();
        if value <= 0.0 {
            return Err(ValueError::custom("Cannot take logarithm of non-positive number"));
        }
        Ok(Self::Float(value.log2()))
    }

    /// Logarithm with custom base
    pub fn log(&self, base: &Self) -> ValueResult<Self> {
        let value = self.as_f64();
        let base_val = base.as_f64();

        if value <= 0.0 || base_val <= 0.0 || base_val == 1.0 {
            return Err(ValueError::custom("Invalid arguments for logarithm"));
        }

        Ok(Self::Float(value.log(base_val)))
    }

    /// Exponential function (e^x)
    #[must_use]
    pub fn exp(&self) -> Self {
        Self::Float(self.as_f64().exp())
    }

    /// 2^x
    #[must_use]
    pub fn exp2(&self) -> Self {
        Self::Float(self.as_f64().exp2())
    }

    // === Trigonometric Functions ===

    /// Sine
    #[must_use]
    pub fn sin(&self) -> Self {
        Self::Float(self.as_f64().sin())
    }

    /// Cosine
    #[must_use]
    pub fn cos(&self) -> Self {
        Self::Float(self.as_f64().cos())
    }

    /// Tangent
    #[must_use]
    pub fn tan(&self) -> Self {
        Self::Float(self.as_f64().tan())
    }

    /// Arcsine
    pub fn asin(&self) -> ValueResult<Self> {
        let value = self.as_f64();
        if value < -1.0 || value > 1.0 {
            return Err(ValueError::custom("asin domain error: value must be in [-1, 1]"));
        }
        Ok(Self::Float(value.asin()))
    }

    /// Arccosine
    pub fn acos(&self) -> ValueResult<Self> {
        let value = self.as_f64();
        if value < -1.0 || value > 1.0 {
            return Err(ValueError::custom("acos domain error: value must be in [-1, 1]"));
        }
        Ok(Self::Float(value.acos()))
    }

    /// Arctangent
    #[must_use]
    pub fn atan(&self) -> Self {
        Self::Float(self.as_f64().atan())
    }

    /// Two-argument arctangent (atan2)
    #[must_use]
    pub fn atan2(&self, x: &Self) -> Self {
        Self::Float(self.as_f64().atan2(x.as_f64()))
    }

    // === Hyperbolic Functions ===

    /// Hyperbolic sine
    #[must_use]
    pub fn sinh(&self) -> Self {
        Self::Float(self.as_f64().sinh())
    }

    /// Hyperbolic cosine
    #[must_use]
    pub fn cosh(&self) -> Self {
        Self::Float(self.as_f64().cosh())
    }

    /// Hyperbolic tangent
    #[must_use]
    pub fn tanh(&self) -> Self {
        Self::Float(self.as_f64().tanh())
    }

    /// Inverse hyperbolic sine
    #[must_use]
    pub fn asinh(&self) -> Self {
        Self::Float(self.as_f64().asinh())
    }

    /// Inverse hyperbolic cosine
    pub fn acosh(&self) -> ValueResult<Self> {
        let value = self.as_f64();
        if value < 1.0 {
            return Err(ValueError::custom("acosh domain error: value must be >= 1"));
        }
        Ok(Self::Float(value.acosh()))
    }

    /// Inverse hyperbolic tangent
    pub fn atanh(&self) -> ValueResult<Self> {
        let value = self.as_f64();
        if value <= -1.0 || value >= 1.0 {
            return Err(ValueError::custom("atanh domain error: value must be in (-1, 1)"));
        }
        Ok(Self::Float(value.atanh()))
    }

    // === Rounding Functions ===

    /// Round to nearest integer
    #[must_use]
    pub fn round(&self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(*i),
            Self::Float(f) => Self::Float(f.round()),
        }
    }

    /// Round towards positive infinity (ceiling)
    #[must_use]
    pub fn ceil(&self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(*i),
            Self::Float(f) => Self::Float(f.ceil()),
        }
    }

    /// Round towards negative infinity (floor)
    #[must_use]
    pub fn floor(&self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(*i),
            Self::Float(f) => Self::Float(f.floor()),
        }
    }

    /// Round towards zero (truncate)
    #[must_use]
    pub fn trunc(&self) -> Self {
        match self {
            Self::Integer(i) => Self::Integer(*i),
            Self::Float(f) => Self::Float(f.trunc()),
        }
    }

    /// Get fractional part
    #[must_use]
    pub fn fract(&self) -> Self {
        match self {
            Self::Integer(_) => Self::Float(0.0),
            Self::Float(f) => Self::Float(f.fract()),
        }
    }

    /// Round to n decimal places
    #[must_use]
    pub fn round_to(&self, decimal_places: u32) -> Self {
        if decimal_places == 0 {
            return self.round();
        }

        let multiplier = 10_f64.powi(decimal_places as i32);
        let value = self.as_f64();
        Self::Float((value * multiplier).round() / multiplier)
    }

    // === Comparison and Clamping ===

    /// Returns the minimum of two numbers
    #[inline]
    #[must_use]
    pub fn min(&self, other: &Self) -> Self {
        if self.as_f64() <= other.as_f64() { *self } else { *other }
    }

    /// Returns the maximum of two numbers
    #[inline]
    #[must_use]
    pub fn max(&self, other: &Self) -> Self {
        if self.as_f64() >= other.as_f64() { *self } else { *other }
    }

    /// Clamps the number to a range
    #[must_use]
    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        if self.as_f64() < min.as_f64() {
            *min
        } else if self.as_f64() > max.as_f64() {
            *max
        } else {
            *self
        }
    }

    // === Range Validation ===

    /// Validates that the number is within a range
    pub fn validate_range(&self, min: Option<f64>, max: Option<f64>) -> ValueResult<()> {
        let value = self.as_f64();

        if let Some(min_val) = min {
            if value < min_val {
                return Err(ValueError::custom(format!(
                    "Value {} is less than minimum {}", value, min_val
                )));
            }
        }

        if let Some(max_val) = max {
            if value > max_val {
                return Err(ValueError::custom(format!(
                    "Value {} is greater than maximum {}", value, max_val
                )));
            }
        }

        Ok(())
    }

    /// Validates that the number is positive
    pub fn validate_positive(&self) -> ValueResult<()> {
        if !self.is_positive() {
            return Err(ValueError::custom(format!(
                "Value {} must be positive", self.as_f64()
            )));
        }
        Ok(())
    }

    /// Validates that the number is non-negative
    pub fn validate_non_negative(&self) -> ValueResult<()> {
        if self.is_negative() {
            return Err(ValueError::custom(format!(
                "Value {} must be non-negative", self.as_f64()
            )));
        }
        Ok(())
    }

    /// Validates that the number is an integer
    pub fn validate_integer(&self) -> ValueResult<()> {
        match self {
            Self::Integer(_) => Ok(()),
            Self::Float(f) => {
                if f.fract() == 0.0 {
                    Ok(())
                } else {
                    Err(ValueError::custom(format!(
                        "Value {} must be an integer", f
                    )))
                }
            }
        }
    }

    // === Collection Operations ===

    /// Sum of a slice of numbers
    pub fn sum(numbers: &[Self]) -> ValueResult<Self> {
        let mut result = Self::ZERO;
        for num in numbers {
            result = result.add(num);
        }
        Ok(result)
    }

    /// Product of a slice of numbers
    pub fn product(numbers: &[Self]) -> ValueResult<Self> {
        let mut result = Self::ONE;
        for num in numbers {
            result = result.multiply(num)?;
        }
        Ok(result)
    }

    /// Average of a slice of numbers
    pub fn average(numbers: &[Self]) -> ValueResult<Self> {
        if numbers.is_empty() {
            return Err(ValueError::custom("Cannot calculate average of empty slice"));
        }

        let sum = Self::sum(numbers)?;
        let count = Self::new_int(numbers.len() as i64);
        sum.divide(&count)
    }

    /// Minimum value in a slice
    #[must_use]
    pub fn min_of(numbers: &[Self]) -> Option<Self> {
        numbers.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).copied()
    }

    /// Maximum value in a slice
    #[must_use]
    pub fn max_of(numbers: &[Self]) -> Option<Self> {
        numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).copied()
    }
}

// === Default Implementation ===

impl Default for NumberValue {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

// === Trait Implementations ===

impl fmt::Display for NumberValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(fl) => {
                // Format floats nicely
                if fl.fract() == 0.0 {
                    write!(f, "{:.0}", fl)
                } else {
                    write!(f, "{}", fl)
                }
            }
        }
    }
}

impl FromStr for NumberValue {
    type Err = ValueError;

    fn from_str(s: &str) -> ValueResult<Self> {
        let trimmed = s.trim();

        // Handle special values
        match trimmed.to_lowercase().as_str() {
            "infinity" | "inf" | "+infinity" | "+inf" => return Ok(Self::INFINITY),
            "-infinity" | "-inf" => return Ok(Self::NEG_INFINITY),
            "nan" => return Err(ValueError::custom("NaN is not a valid number")),
            _ => {}
        }

        // Try parsing as integer first (if no decimal point)
        if !trimmed.contains('.') && !trimmed.contains('e') && !trimmed.contains('E') {
            if let Ok(i) = trimmed.parse::<i64>() {
                return Ok(Self::Integer(i));
            }
        }

        // Try parsing as float
        if let Ok(f) = trimmed.parse::<f64>() {
            if f.is_finite() {
                Ok(Self::Float(f))
            } else {
                Err(ValueError::custom(format!("Invalid number: {}", s)))
            }
        } else {
            Err(ValueError::custom(format!("Cannot parse '{}' as number", s)))
        }
    }
}

// === Ordering ===

impl PartialOrd for NumberValue {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_f64().partial_cmp(&other.as_f64())
    }
}

// === From implementations ===

impl From<i8> for NumberValue {
    #[inline]
    fn from(value: i8) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i16> for NumberValue {
    #[inline]
    fn from(value: i16) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i32> for NumberValue {
    #[inline]
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i64> for NumberValue {
    #[inline]
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<u8> for NumberValue {
    #[inline]
    fn from(value: u8) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<u16> for NumberValue {
    #[inline]
    fn from(value: u16) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<u32> for NumberValue {
    #[inline]
    fn from(value: u32) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<usize> for NumberValue {
    #[inline]
    fn from(value: usize) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<f32> for NumberValue {
    #[inline]
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
    }
}

impl From<f64> for NumberValue {
    #[inline]
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

// === Arithmetic operators ===

impl Add for NumberValue {
    type Output = NumberValue;

    fn add(self, rhs: NumberValue) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => {
                if let Some(result) = a.checked_add(b) {
                    Self::Integer(result)
                } else {
                    Self::Float(a as f64 + b as f64)
                }
            }
            _ => Self::Float(self.as_f64() + rhs.as_f64()),
        }
    }
}

impl Add<&NumberValue> for NumberValue {
    type Output = NumberValue;

    fn add(self, rhs: &NumberValue) -> Self::Output {
        self + *rhs
    }
}

impl Add<NumberValue> for &NumberValue {
    type Output = NumberValue;

    fn add(self, rhs: NumberValue) -> Self::Output {
        *self + rhs
    }
}

impl Add for &NumberValue {
    type Output = NumberValue;

    fn add(self, rhs: &NumberValue) -> Self::Output {
        *self + *rhs
    }
}

impl Sub for NumberValue {
    type Output = NumberValue;

    fn sub(self, rhs: NumberValue) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => {
                if let Some(result) = a.checked_sub(b) {
                    Self::Integer(result)
                } else {
                    Self::Float(a as f64 - b as f64)
                }
            }
            _ => Self::Float(self.as_f64() - rhs.as_f64()),
        }
    }
}

impl Sub<&NumberValue> for NumberValue {
    type Output = NumberValue;

    fn sub(self, rhs: &NumberValue) -> Self::Output {
        self - *rhs
    }
}

impl Sub<NumberValue> for &NumberValue {
    type Output = NumberValue;

    fn sub(self, rhs: NumberValue) -> Self::Output {
        *self - rhs
    }
}

impl Sub for &NumberValue {
    type Output = NumberValue;

    fn sub(self, rhs: &NumberValue) -> Self::Output {
        *self - *rhs
    }
}

impl Mul for NumberValue {
    type Output = NumberValue;

    fn mul(self, rhs: NumberValue) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => {
                if let Some(result) = a.checked_mul(b) {
                    Self::Integer(result)
                } else {
                    Self::Float(a as f64 * b as f64)
                }
            }
            _ => Self::Float(self.as_f64() * rhs.as_f64()),
        }
    }
}

impl Mul<&NumberValue> for NumberValue {
    type Output = NumberValue;

    fn mul(self, rhs: &NumberValue) -> Self::Output {
        self * *rhs
    }
}

impl Mul<NumberValue> for &NumberValue {
    type Output = NumberValue;

    fn mul(self, rhs: NumberValue) -> Self::Output {
        *self * rhs
    }
}

impl Mul for &NumberValue {
    type Output = NumberValue;

    fn mul(self, rhs: &NumberValue) -> Self::Output {
        *self * *rhs
    }
}

impl Div for NumberValue {
    type Output = NumberValue;

    fn div(self, rhs: NumberValue) -> Self::Output {
        Self::Float(self.as_f64() / rhs.as_f64())
    }
}

impl Div<&NumberValue> for NumberValue {
    type Output = NumberValue;

    fn div(self, rhs: &NumberValue) -> Self::Output {
        self / *rhs
    }
}

impl Div<NumberValue> for &NumberValue {
    type Output = NumberValue;

    fn div(self, rhs: NumberValue) -> Self::Output {
        *self / rhs
    }
}

impl Div for &NumberValue {
    type Output = NumberValue;

    fn div(self, rhs: &NumberValue) -> Self::Output {
        *self / *rhs
    }
}

impl Rem for NumberValue {
    type Output = NumberValue;

    fn rem(self, rhs: NumberValue) -> Self::Output {
        match (self, rhs) {
            (Self::Integer(a), Self::Integer(b)) => Self::Integer(a % b),
            _ => Self::Float(self.as_f64() % rhs.as_f64()),
        }
    }
}

impl Rem<&NumberValue> for NumberValue {
    type Output = NumberValue;

    fn rem(self, rhs: &NumberValue) -> Self::Output {
        self % *rhs
    }
}

impl Rem<NumberValue> for &NumberValue {
    type Output = NumberValue;

    fn rem(self, rhs: NumberValue) -> Self::Output {
        *self % rhs
    }
}

impl Rem for &NumberValue {
    type Output = NumberValue;

    fn rem(self, rhs: &NumberValue) -> Self::Output {
        *self % *rhs
    }
}

impl Neg for NumberValue {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(i) => Self::Integer(-i),
            Self::Float(f) => Self::Float(-f),
        }
    }
}

impl Neg for &NumberValue {
    type Output = NumberValue;

    #[inline]
    fn neg(self) -> Self::Output {
        -*self
    }
}

// === JSON conversion ===
#[cfg(feature = "json")]
impl From<NumberValue> for serde_json::Value {
    fn from(value: NumberValue) -> Self {
        match value {
            NumberValue::Integer(i) => serde_json::Value::Number(i.into()),
            NumberValue::Float(f) => {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            }
        }
    }
}

#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for NumberValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(Self::Integer(i))
                } else if let Some(f) = n.as_f64() {
                    Ok(Self::Float(f))
                } else {
                    Err(ValueError::custom("Invalid JSON number"))
                }
            }
            serde_json::Value::String(s) => Self::from_str(&s),
            other => Err(ValueError::custom(format!(
                "Cannot convert {:?} to NumberValue", other
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(NumberValue::ZERO, NumberValue::Integer(0));
        assert_eq!(NumberValue::ONE, NumberValue::Integer(1));
        assert!(NumberValue::PI.as_f64() > 3.0);
        assert!(NumberValue::E.as_f64() > 2.0);
    }

    #[test]
    fn test_const_constructors() {
        const ZERO: NumberValue = NumberValue::zero();
        const ONE: NumberValue = NumberValue::one();
        const FORTY_TWO: NumberValue = NumberValue::from_i32(42);

        assert_eq!(ZERO, NumberValue::Integer(0));
        assert_eq!(ONE, NumberValue::Integer(1));
        assert_eq!(FORTY_TWO, NumberValue::Integer(42));
    }

    #[test]
    fn test_type_checks() {
        let int_val = NumberValue::new_int(42);
        let float_val = NumberValue::new_float(3.14);
        let zero = NumberValue::ZERO;

        assert!(int_val.is_integer());
        assert!(!int_val.is_float());
        assert!(int_val.is_positive());
        assert!(!int_val.is_negative());
        assert!(!int_val.is_zero());

        assert!(!float_val.is_integer());
        assert!(float_val.is_float());
        assert!(float_val.is_positive());

        assert!(zero.is_zero());
        assert!(!zero.is_positive());
        assert!(!zero.is_negative());
    }

    #[test]
    fn test_arithmetic() {
        let a = NumberValue::new_int(10);
        let b = NumberValue::new_int(3);
        let c = NumberValue::new_float(2.5);

        assert_eq!(a.add(&b), NumberValue::Integer(13));
        assert_eq!(a.subtract(&b).unwrap(), NumberValue::Integer(7));
        assert_eq!(a.multiply(&b).unwrap(), NumberValue::Integer(30));
        assert_eq!(a.divide(&b).unwrap(), NumberValue::Float(10.0 / 3.0));
        assert_eq!(a.modulo(&b).unwrap(), NumberValue::Integer(1));

        assert_eq!(a.add(&c), NumberValue::Float(12.5));
    }

    #[test]
    fn test_operators() {
        let a = NumberValue::new_int(10);
        let b = NumberValue::new_int(3);

        assert_eq!(a + b, NumberValue::Integer(13));
        assert_eq!(a - b, NumberValue::Integer(7));
        assert_eq!(a * b, NumberValue::Integer(30));
        assert_eq!(a / b, NumberValue::Float(10.0 / 3.0));
        assert_eq!(a % b, NumberValue::Integer(1));
        assert_eq!(-a, NumberValue::Integer(-10));
    }

    #[test]
    fn test_mathematical_functions() {
        let num = NumberValue::new_float(9.0);
        assert_eq!(num.sqrt().unwrap(), NumberValue::Float(3.0));

        let negative = NumberValue::new_float(-4.0);
        assert!(negative.sqrt().is_err());

        let pi_half = NumberValue::new_float(std::f64::consts::PI / 2.0);
        assert!((pi_half.sin().as_f64() - 1.0).abs() < 1e-10);

        let e = NumberValue::E;
        assert!((e.ln().unwrap().as_f64() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_rounding() {
        let num = NumberValue::new_float(3.7);

        assert_eq!(num.round(), NumberValue::Float(4.0));
        assert_eq!(num.ceil(), NumberValue::Float(4.0));
        assert_eq!(num.floor(), NumberValue::Float(3.0));
        assert_eq!(num.trunc(), NumberValue::Float(3.0));

        let precise = NumberValue::new_float(std::f64::consts::PI);
        assert_eq!(precise.round_to(2), NumberValue::Float(3.14));
        assert_eq!(precise.round_to(0), NumberValue::Float(3.0));
    }

    #[test]
    fn test_trigonometry() {
        let zero = NumberValue::ZERO;
        let pi = NumberValue::PI;

        assert!(zero.sin().as_f64().abs() < 1e-10);
        assert!((zero.cos().as_f64() - 1.0).abs() < 1e-10);
        assert!(pi.sin().as_f64().abs() < 1e-10);
        assert!((pi.cos().as_f64() + 1.0).abs() < 1e-10);

        let one = NumberValue::ONE;
        assert!((one.asin().unwrap().as_f64() - std::f64::consts::PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_collection_operations() {
        let numbers = vec![
            NumberValue::new_int(1),
            NumberValue::new_int(2),
            NumberValue::new_int(3),
            NumberValue::new_int(4),
        ];

        assert_eq!(NumberValue::sum(&numbers).unwrap(), NumberValue::Integer(10));
        assert_eq!(NumberValue::product(&numbers).unwrap(), NumberValue::Integer(24));
        assert_eq!(NumberValue::average(&numbers).unwrap(), NumberValue::Float(2.5));
        assert_eq!(NumberValue::min_of(&numbers), Some(NumberValue::Integer(1)));
        assert_eq!(NumberValue::max_of(&numbers), Some(NumberValue::Integer(4)));
    }

    #[test]
    fn test_parsing() {
        assert_eq!("42".parse::<NumberValue>().unwrap(), NumberValue::Integer(42));
        assert_eq!("3.14".parse::<NumberValue>().unwrap(), NumberValue::Float(3.14));
        assert_eq!("infinity".parse::<NumberValue>().unwrap(), NumberValue::INFINITY);
        assert_eq!("-inf".parse::<NumberValue>().unwrap(), NumberValue::NEG_INFINITY);

        assert!("not_a_number".parse::<NumberValue>().is_err());
        assert!("NaN".parse::<NumberValue>().is_err());
    }

    #[test]
    fn test_validation() {
        let positive = NumberValue::new_int(5);
        let negative = NumberValue::new_int(-3);
        let zero = NumberValue::ZERO;

        assert!(positive.validate_positive().is_ok());
        assert!(negative.validate_positive().is_err());
        assert!(zero.validate_positive().is_err());

        assert!(positive.validate_non_negative().is_ok());
        assert!(zero.validate_non_negative().is_ok());
        assert!(negative.validate_non_negative().is_err());

        assert!(positive.validate_range(Some(0.0), Some(10.0)).is_ok());
        assert!(negative.validate_range(Some(0.0), Some(10.0)).is_err());
    }

    #[test]
    fn test_from_methods() {
        assert_eq!(NumberValue::from_i8(42i8), NumberValue::Integer(42));
        assert_eq!(NumberValue::from_i16(1000i16), NumberValue::Integer(1000));
        assert_eq!(NumberValue::from_i32(100000i32), NumberValue::Integer(100000));
        assert_eq!(NumberValue::from_u8(255u8), NumberValue::Integer(255));
        assert_eq!(NumberValue::from_u16(65535u16), NumberValue::Integer(65535));
        assert_eq!(NumberValue::from_u32(4000000u32), NumberValue::Integer(4000000));
        assert_eq!(NumberValue::from_usize(1000usize), NumberValue::Integer(1000));

        assert_eq!(NumberValue::from_int(42i32), NumberValue::Integer(42));
        assert_eq!(NumberValue::from_int(123u16), NumberValue::Integer(123));
    }
}