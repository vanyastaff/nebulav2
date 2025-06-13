#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::str::FromStr;
use crate::{ValueError, ValueResult};

/// Boolean value type with extended functionality and optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BooleanValue(bool);

impl BooleanValue {
    /// Constant true value
    pub const TRUE: Self = Self(true);

    /// Constant false value
    pub const FALSE: Self = Self(false);

    /// Creates a new boolean value (const fn)
    #[inline]
    #[must_use]
    pub const fn new(value: bool) -> Self {
        Self(value)
    }

    /// Creates a true value
    #[inline]
    #[must_use]
    pub const fn true_value() -> Self {
        Self::TRUE
    }

    /// Creates a false value
    #[inline]
    #[must_use]
    pub const fn false_value() -> Self {
        Self::FALSE
    }

    /// Returns the underlying bool value
    #[inline]
    #[must_use]
    pub const fn value(&self) -> bool {
        self.0
    }

    /// Returns the underlying bool value (alias for compatibility)
    #[inline]
    #[must_use]
    pub const fn get(&self) -> bool {
        self.0
    }

    /// Checks if the value is true
    #[inline]
    #[must_use]
    pub const fn is_true(&self) -> bool {
        self.0
    }

    /// Checks if the value is false
    #[inline]
    #[must_use]
    pub const fn is_false(&self) -> bool {
        !self.0
    }

    // === Logical Operations ===

    /// Returns the negated value
    #[inline]
    #[must_use]
    pub const fn not(&self) -> Self {
        Self(!self.0)
    }

    /// Logical AND operation
    #[inline]
    #[must_use]
    pub const fn and(&self, other: Self) -> Self {
        Self(self.0 && other.0)
    }

    /// Logical OR operation
    #[inline]
    #[must_use]
    pub const fn or(&self, other: Self) -> Self {
        Self(self.0 || other.0)
    }

    /// Logical XOR operation
    #[inline]
    #[must_use]
    pub const fn xor(&self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Logical NAND operation (NOT AND)
    #[inline]
    #[must_use]
    pub const fn nand(&self, other: Self) -> Self {
        Self(!(self.0 && other.0))
    }

    /// Logical NOR operation (NOT OR)
    #[inline]
    #[must_use]
    pub const fn nor(&self, other: Self) -> Self {
        Self(!(self.0 || other.0))
    }

    /// Logical XNOR operation (NOT XOR) - equivalence
    #[inline]
    #[must_use]
    pub const fn xnor(&self, other: Self) -> Self {
        Self(!(self.0 ^ other.0))
    }

    /// Logical implication (A implies B: !A || B)
    #[inline]
    #[must_use]
    pub const fn implies(&self, other: Self) -> Self {
        Self(!self.0 || other.0)
    }

    /// Logical equivalence (A <=> B: (A && B) || (!A && !B))
    #[inline]
    #[must_use]
    pub const fn equivalent(&self, other: Self) -> Self {
        Self((self.0 && other.0) || (!self.0 && !other.0))
    }

    // === Conditional Operations ===

    /// Returns Some(self) if both values are true, None otherwise
    #[inline]
    #[must_use]
    pub const fn and_then(&self, other: Self) -> Option<Self> {
        if self.0 && other.0 {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Returns Some(true) if either value is true, None if both false
    #[inline]
    #[must_use]
    pub const fn or_else(&self, other: Self) -> Option<Self> {
        if self.0 || other.0 {
            Some(Self(true))
        } else {
            None
        }
    }

    /// Returns the first value if true, otherwise the second value
    #[inline]
    #[must_use]
    pub const fn then_else(&self, if_true: Self, if_false: Self) -> Self {
        if self.0 { if_true } else { if_false }
    }

    /// Conditional execution: returns Some(value) if self is true
    #[inline]
    #[must_use]
    pub fn then_some<T>(&self, value: T) -> Option<T> {
        if self.0 { Some(value) } else { None }
    }

    // === String Operations ===

    /// Converts to a static string slice
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        if self.0 { "true" } else { "false" }
    }

    /// Converts to uppercase string ("TRUE" or "FALSE")
    #[inline]
    #[must_use]
    pub const fn as_upper_str(&self) -> &'static str {
        if self.0 { "TRUE" } else { "FALSE" }
    }

    /// Converts to short string ("T" or "F")
    #[inline]
    #[must_use]
    pub const fn as_short_str(&self) -> &'static str {
        if self.0 { "T" } else { "F" }
    }

    /// Converts to numeric string ("1" or "0")
    #[inline]
    #[must_use]
    pub const fn as_numeric_str(&self) -> &'static str {
        if self.0 { "1" } else { "0" }
    }

    /// Converts to yes/no string
    #[inline]
    #[must_use]
    pub const fn as_yes_no(&self) -> &'static str {
        if self.0 { "yes" } else { "no" }
    }

    /// Converts to on/off string
    #[inline]
    #[must_use]
    pub const fn as_on_off(&self) -> &'static str {
        if self.0 { "on" } else { "off" }
    }

    // === Parsing and Conversion ===

    /// Attempts to parse from string with comprehensive support
    pub fn parse(s: &str) -> ValueResult<Self> {
        let trimmed = s.trim();
        let lower = trimmed.to_lowercase();

        match lower.as_str() {
            // Standard boolean values
            "true" | "t" => Ok(Self(true)),
            "false" | "f" => Ok(Self(false)),

            // Numeric representations
            "1" => Ok(Self(true)),
            "0" => Ok(Self(false)),

            // Yes/No variants
            "yes" | "y" => Ok(Self(true)),
            "no" | "n" => Ok(Self(false)),

            // On/Off variants
            "on" => Ok(Self(true)),
            "off" => Ok(Self(false)),

            // Enable/Disable variants
            "enable" | "enabled" => Ok(Self(true)),
            "disable" | "disabled" => Ok(Self(false)),

            // Active/Inactive variants
            "active" => Ok(Self(true)),
            "inactive" => Ok(Self(false)),

            // Positive/Negative variants
            "positive" | "pos" => Ok(Self(true)),
            "negative" | "neg" => Ok(Self(false)),

            // Other common representations
            "ok" | "okay" => Ok(Self(true)),
            "cancel" | "cancelled" => Ok(Self(false)),

            // High/Low (for binary states)
            "high" | "hi" => Ok(Self(true)),
            "low" | "lo" => Ok(Self(false)),

            _ => Err(ValueError::custom(format!(
                "Cannot parse '{}' as boolean. Valid values: true/false, 1/0, yes/no, on/off, etc.",
                s
            ))),
        }
    }

    /// Attempts to parse from string (lenient - case insensitive)
    pub fn parse_lenient(s: &str) -> ValueResult<Self> {
        Self::parse(s)
    }

    /// Attempts to parse from string (strict - exact match)
    pub fn parse_strict(s: &str) -> ValueResult<Self> {
        match s {
            "true" => Ok(Self(true)),
            "false" => Ok(Self(false)),
            _ => Err(ValueError::custom(format!(
                "Strict parsing failed for '{}'. Only 'true' and 'false' are allowed.",
                s
            ))),
        }
    }

    // === Numeric Conversion ===

    /// Converts to integer (1 for true, 0 for false)
    #[inline]
    #[must_use]
    pub const fn as_int(&self) -> i32 {
        if self.0 { 1 } else { 0 }
    }

    /// Converts to unsigned integer (1 for true, 0 for false)
    #[inline]
    #[must_use]
    pub const fn as_uint(&self) -> u32 {
        if self.0 { 1 } else { 0 }
    }

    /// Converts to float (1.0 for true, 0.0 for false)
    #[inline]
    #[must_use]
    pub const fn as_float(&self) -> f64 {
        if self.0 { 1.0 } else { 0.0 }
    }

    /// Creates from integer (0 is false, everything else is true)
    #[inline]
    #[must_use]
    pub const fn from_int(value: i32) -> Self {
        Self(value != 0)
    }

    /// Creates from float (0.0 is false, everything else is true)
    #[inline]
    #[must_use]
    pub fn from_float(value: f64) -> Self {
        Self(value != 0.0 && !value.is_nan())
    }

    // === Utility Methods ===

    /// Flips the boolean value in place
    #[inline]
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }

    /// Sets the value to true
    #[inline]
    pub fn set_true(&mut self) {
        self.0 = true;
    }

    /// Sets the value to false
    #[inline]
    pub fn set_false(&mut self) {
        self.0 = false;
    }

    /// Sets the value
    #[inline]
    pub fn set(&mut self, value: bool) {
        self.0 = value;
    }

    /// Returns a copy with the value toggled
    #[inline]
    #[must_use]
    pub const fn toggled(&self) -> Self {
        Self(!self.0)
    }

    // === Collection Operations ===

    /// Performs logical AND on a slice of boolean values
    #[must_use]
    pub fn all(values: &[Self]) -> Self {
        let result = values.iter().all(|b| b.0);
        Self(result)
    }

    /// Performs logical OR on a slice of boolean values
    #[must_use]
    pub fn any(values: &[Self]) -> Self {
        let result = values.iter().any(|b| b.0);
        Self(result)
    }

    /// Counts the number of true values in a slice
    #[must_use]
    pub fn count_true(values: &[Self]) -> usize {
        values.iter().filter(|b| b.0).count()
    }

    /// Counts the number of false values in a slice
    #[must_use]
    pub fn count_false(values: &[Self]) -> usize {
        values.iter().filter(|b| !b.0).count()
    }

    /// Returns the majority value from a slice (true if more trues than falses)
    #[must_use]
    pub fn majority(values: &[Self]) -> Option<Self> {
        if values.is_empty() {
            return None;
        }

        let true_count = Self::count_true(values);
        let false_count = values.len() - true_count;

        if true_count > false_count {
            Some(Self(true))
        } else if false_count > true_count {
            Some(Self(false))
        } else {
            None // Tie
        }
    }
}

// === Default Implementation ===

impl Default for BooleanValue {
    #[inline]
    fn default() -> Self {
        Self::FALSE
    }
}

// === Core Trait Implementations ===

impl Not for BooleanValue {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self::not(&self)
    }
}

impl BitAnd for BooleanValue {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitOr for BooleanValue {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitXor for BooleanValue {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

// === Assignment Operators ===

impl std::ops::BitAndAssign for BooleanValue {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitOrAssign for BooleanValue {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitXorAssign for BooleanValue {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

// === Conversion Traits ===

impl From<bool> for BooleanValue {
    #[inline]
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<BooleanValue> for bool {
    #[inline]
    fn from(value: BooleanValue) -> bool {
        value.0
    }
}

impl From<i32> for BooleanValue {
    #[inline]
    fn from(value: i32) -> Self {
        Self::from_int(value)
    }
}

impl From<f64> for BooleanValue {
    #[inline]
    fn from(value: f64) -> Self {
        Self::from_float(value)
    }
}

// === Deref Implementation ===

impl std::ops::Deref for BooleanValue {
    type Target = bool;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BooleanValue {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// === Display and String Conversion ===

impl std::fmt::Display for BooleanValue {
    #[inline]
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

// === Comparison with Primitives ===

impl PartialEq<bool> for BooleanValue {
    #[inline]
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

impl PartialEq<BooleanValue> for bool {
    #[inline]
    fn eq(&self, other: &BooleanValue) -> bool {
        *self == other.0
    }
}

impl PartialEq<i32> for BooleanValue {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        self.as_int() == *other
    }
}

impl PartialEq<BooleanValue> for i32 {
    #[inline]
    fn eq(&self, other: &BooleanValue) -> bool {
        *self == other.as_int()
    }
}

// === JSON Conversion ===

#[cfg(feature = "json")]
impl From<BooleanValue> for serde_json::Value {
    #[inline]
    fn from(value: BooleanValue) -> Self {
        serde_json::Value::Bool(value.0)
    }
}

#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for BooleanValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::Bool(b) => Ok(Self(b)),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(Self::from_int(i as i32))
                } else if let Some(f) = n.as_f64() {
                    Ok(Self::from_float(f))
                } else {
                    Err(ValueError::custom("Invalid number for boolean conversion"))
                }
            }
            serde_json::Value::String(s) => Self::parse(&s),
            other => Err(ValueError::custom(format!(
                "Cannot convert {:?} to BooleanValue", other
            ))),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(BooleanValue::TRUE.value());
        assert!(!BooleanValue::FALSE.value());

        const TRUE_VAL: BooleanValue = BooleanValue::new(true);
        const FALSE_VAL: BooleanValue = BooleanValue::new(false);

        assert!(TRUE_VAL.value());
        assert!(!FALSE_VAL.value());
    }

    #[test]
    fn test_logical_operations() {
        let t = BooleanValue::TRUE;
        let f = BooleanValue::FALSE;

        // Basic operations
        assert_eq!(t.not(), f);
        assert_eq!(t.and(f), f);
        assert_eq!(t.or(f), t);
        assert_eq!(t.xor(f), t);

        // Extended operations
        assert_eq!(t.nand(f), t);
        assert_eq!(t.nor(f), f);
        assert_eq!(t.xnor(f), f);
        assert_eq!(t.implies(f), f);
        assert_eq!(t.equivalent(t), t);
    }

    #[test]
    fn test_parsing() {
        // Standard cases
        assert_eq!(BooleanValue::parse("true").unwrap(), true);
        assert_eq!(BooleanValue::parse("false").unwrap(), false);

        // Numeric
        assert_eq!(BooleanValue::parse("1").unwrap(), true);
        assert_eq!(BooleanValue::parse("0").unwrap(), false);

        // Yes/No
        assert_eq!(BooleanValue::parse("yes").unwrap(), true);
        assert_eq!(BooleanValue::parse("no").unwrap(), false);

        // Case insensitive
        assert_eq!(BooleanValue::parse("TRUE").unwrap(), true);
        assert_eq!(BooleanValue::parse("False").unwrap(), false);

        // Extended variants
        assert_eq!(BooleanValue::parse("enable").unwrap(), true);
        assert_eq!(BooleanValue::parse("disable").unwrap(), false);

        // Invalid cases
        assert!(BooleanValue::parse("maybe").is_err());
        assert!(BooleanValue::parse("invalid").is_err());
    }

    #[test]
    fn test_string_representations() {
        let t = BooleanValue::TRUE;
        let f = BooleanValue::FALSE;

        assert_eq!(t.as_str(), "true");
        assert_eq!(f.as_str(), "false");
        assert_eq!(t.as_upper_str(), "TRUE");
        assert_eq!(t.as_short_str(), "T");
        assert_eq!(t.as_numeric_str(), "1");
        assert_eq!(t.as_yes_no(), "yes");
        assert_eq!(t.as_on_off(), "on");
    }

    #[test]
    fn test_numeric_conversion() {
        let t = BooleanValue::TRUE;
        let f = BooleanValue::FALSE;

        assert_eq!(t.as_int(), 1);
        assert_eq!(f.as_int(), 0);
        assert_eq!(t.as_float(), 1.0);
        assert_eq!(f.as_float(), 0.0);

        assert_eq!(BooleanValue::from_int(42), t);
        assert_eq!(BooleanValue::from_int(0), f);
        assert_eq!(BooleanValue::from_float(3.14), t);
        assert_eq!(BooleanValue::from_float(0.0), f);
    }

    #[test]
    fn test_collection_operations() {
        let values = vec![
            BooleanValue::TRUE,
            BooleanValue::FALSE,
            BooleanValue::TRUE,
        ];

        assert_eq!(BooleanValue::any(&values), BooleanValue::TRUE);
        assert_eq!(BooleanValue::all(&values), BooleanValue::FALSE);
        assert_eq!(BooleanValue::count_true(&values), 2);
        assert_eq!(BooleanValue::count_false(&values), 1);
        assert_eq!(BooleanValue::majority(&values), Some(BooleanValue::TRUE));
    }

    #[test]
    fn test_operators() {
        let t = BooleanValue::TRUE;
        let f = BooleanValue::FALSE;

        // Bitwise operators
        assert_eq!(t & f, f);
        assert_eq!(t | f, t);
        assert_eq!(t ^ f, t);
        assert_eq!(!t, f);

        // Assignment operators
        let mut val = BooleanValue::TRUE;
        val &= BooleanValue::FALSE;
        assert_eq!(val, BooleanValue::FALSE);
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_json_conversion() {
        let t = BooleanValue::TRUE;
        let f = BooleanValue::FALSE;

        // To JSON
        let json_t: serde_json::Value = t.into();
        let json_f: serde_json::Value = f.into();
        assert_eq!(json_t, serde_json::Value::Bool(true));
        assert_eq!(json_f, serde_json::Value::Bool(false));

        // From JSON
        let back_t = BooleanValue::try_from(json_t).unwrap();
        let back_f = BooleanValue::try_from(json_f).unwrap();
        assert_eq!(back_t, t);
        assert_eq!(back_f, f);

        // From JSON string
        let from_str = BooleanValue::try_from(serde_json::Value::String("yes".to_string())).unwrap();
        assert_eq!(from_str, BooleanValue::TRUE);

        // From JSON number
        let from_num = BooleanValue::try_from(serde_json::Value::Number(1.into())).unwrap();
        assert_eq!(from_num, BooleanValue::TRUE);
    }
}