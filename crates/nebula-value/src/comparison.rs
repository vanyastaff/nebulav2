//! Value comparison utilities for nebula-value

use crate::{NumberValue, Value, ValueError, ValueResult};

/// Result of a value comparison operation
#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonResult {
    /// Comparison returned true
    True,
    /// Comparison returned false
    False,
    /// Comparison failed with an error
    Error(ValueError),
}

impl ComparisonResult {
    /// Returns true if the result is True
    #[inline]
    pub fn is_true(&self) -> bool {
        matches!(self, Self::True)
    }

    /// Returns true if the result is False
    #[inline]
    pub fn is_false(&self) -> bool {
        matches!(self, Self::False)
    }

    /// Returns true if the result is an Error
    #[inline]
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    /// Converts to bool, treating errors as false
    #[inline]
    pub fn to_bool(&self) -> bool {
        matches!(self, Self::True)
    }

    /// Converts to Result<bool, ValueError>
    #[inline]
    pub fn to_result(&self) -> ValueResult<bool> {
        match self {
            Self::True => Ok(true),
            Self::False => Ok(false),
            Self::Error(e) => Err(e.clone()),
        }
    }
}

impl From<bool> for ComparisonResult {
    #[inline]
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

impl From<ValueError> for ComparisonResult {
    #[inline]
    fn from(error: ValueError) -> Self {
        Self::Error(error)
    }
}

impl From<ValueResult<bool>> for ComparisonResult {
    fn from(result: ValueResult<bool>) -> Self {
        match result {
            Ok(true) => Self::True,
            Ok(false) => Self::False,
            Err(e) => Self::Error(e),
        }
    }
}

/// Comparison utilities for Value types
#[derive(Debug, Clone)]
pub struct ValueComparison;

impl ValueComparison {
    // === Methods returning bool (for backwards compatibility) ===

    /// Checks if two values are equal
    pub fn equals(value: &Value, expected: &Value) -> bool {
        value == expected
    }

    /// Checks if two values are not equal
    pub fn not_equals(value: &Value, expected: &Value) -> bool {
        !Self::equals(value, expected)
    }

    /// Checks if value is greater than expected
    pub fn gt_simple(value: &Value, expected: &Value) -> bool {
        Self::compare_values(value, expected).map(|ord| ord.is_gt()).unwrap_or(false)
    }

    /// Checks if value is less than expected
    pub fn lt_simple(value: &Value, expected: &Value) -> bool {
        Self::compare_values(value, expected).map(|ord| ord.is_lt()).unwrap_or(false)
    }

    /// Checks if value is greater than or equal to expected
    pub fn gte_simple(value: &Value, expected: &Value) -> bool {
        Self::compare_values(value, expected).map(|ord| ord.is_ge()).unwrap_or(false)
    }

    /// Checks if value is less than or equal to expected
    pub fn lte_simple(value: &Value, expected: &Value) -> bool {
        Self::compare_values(value, expected).map(|ord| ord.is_le()).unwrap_or(false)
    }

    /// Checks if value is contained in the provided list
    pub fn in_list(value: &Value, list: &[Value]) -> bool {
        list.iter().any(|item| Self::equals(value, item))
    }

    /// Checks if value is not contained in the provided list
    pub fn not_in_list(value: &Value, list: &[Value]) -> bool {
        !Self::in_list(value, list)
    }

    /// Checks if a value is considered "empty"
    pub fn is_empty(value: &Value) -> bool {
        match value {
            Value::String(s) => s.is_empty(),
            Value::Array(arr) => arr.is_empty(),
            Value::Object(obj) => obj.is_empty(),
            Value::Number(num) => num.is_zero(),
            Value::Boolean(b) => !**b,
            Value::Null => true,
            Value::Binary(bin) => bin.len() == 0,
            Value::File(_) => false,
            Value::DateTime(_) => false,
            Value::Duration(dur) => dur.is_zero(),
            Value::Mode(_) => false,
            Value::Expression(expr) => expr.template().is_empty(),
            Value::Regex(_) => false,
            Value::Color(_) => false,
            Value::Cron(_) => false,
        }
    }

    /// Checks if a value is not empty
    pub fn is_not_empty(value: &Value) -> bool {
        !Self::is_empty(value)
    }

    // === Methods returning ComparisonResult (new API) ===

    /// Checks if two values are equal, returning ComparisonResult
    pub fn equals_result(value: &Value, expected: &Value) -> ComparisonResult {
        ComparisonResult::from(Self::equals(value, expected))
    }

    /// Checks if two values are not equal, returning ComparisonResult
    pub fn not_equals_result(value: &Value, expected: &Value) -> ComparisonResult {
        ComparisonResult::from(Self::not_equals(value, expected))
    }

    /// Checks if value is greater than expected, returning ComparisonResult
    pub fn gt_result(value: &Value, expected: &Value) -> ComparisonResult {
        match Self::compare_values(value, expected) {
            Some(ord) => ComparisonResult::from(ord.is_gt()),
            None => ComparisonResult::Error(ValueError::incompatible_comparison(
                value.type_name(),
                expected.type_name(),
            )),
        }
    }

    /// Checks if value is less than expected, returning ComparisonResult
    pub fn lt_result(value: &Value, expected: &Value) -> ComparisonResult {
        match Self::compare_values(value, expected) {
            Some(ord) => ComparisonResult::from(ord.is_lt()),
            None => ComparisonResult::Error(ValueError::incompatible_comparison(
                value.type_name(),
                expected.type_name(),
            )),
        }
    }

    /// Checks if value is greater than or equal to expected, returning
    /// ComparisonResult
    pub fn gte_result(value: &Value, expected: &Value) -> ComparisonResult {
        match Self::compare_values(value, expected) {
            Some(ord) => ComparisonResult::from(ord.is_ge()),
            None => ComparisonResult::Error(ValueError::incompatible_comparison(
                value.type_name(),
                expected.type_name(),
            )),
        }
    }

    /// Checks if value is less than or equal to expected, returning
    /// ComparisonResult
    pub fn lte_result(value: &Value, expected: &Value) -> ComparisonResult {
        match Self::compare_values(value, expected) {
            Some(ord) => ComparisonResult::from(ord.is_le()),
            None => ComparisonResult::Error(ValueError::incompatible_comparison(
                value.type_name(),
                expected.type_name(),
            )),
        }
    }

    /// Checks if value is contained in the provided list, returning
    /// ComparisonResult
    pub fn in_list_result(value: &Value, list: &[Value]) -> ComparisonResult {
        ComparisonResult::from(Self::in_list(value, list))
    }

    /// Checks if value is not contained in the provided list, returning
    /// ComparisonResult
    pub fn not_in_list_result(value: &Value, list: &[Value]) -> ComparisonResult {
        ComparisonResult::from(Self::not_in_list(value, list))
    }

    /// Checks if a value is considered "empty", returning ComparisonResult
    pub fn is_empty_result(value: &Value) -> ComparisonResult {
        ComparisonResult::from(Self::is_empty(value))
    }

    /// Checks if a value is not empty, returning ComparisonResult
    pub fn is_not_empty_result(value: &Value) -> ComparisonResult {
        ComparisonResult::from(Self::is_not_empty(value))
    }

    /// Attempts to compare two values, returning an Ordering if possible
    fn compare_values(left: &Value, right: &Value) -> Option<std::cmp::Ordering> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => a.as_f64().partial_cmp(&b.as_f64()),
            (Value::String(a), Value::String(b)) => Some(a.cmp(b)),
            (Value::Array(a), Value::Array(b)) => Some(a.len().cmp(&b.len())),
            (Value::Object(a), Value::Object(b)) => Some(a.len().cmp(&b.len())),
            (Value::Boolean(a), Value::Boolean(b)) => Some((**a).cmp(&**b)),
            (Value::DateTime(a), Value::DateTime(b)) => Some(a.cmp(b)),
            (Value::Duration(a), Value::Duration(b)) => Some(a.cmp(b)),
            (Value::Number(num), other) | (other, Value::Number(num)) => {
                Self::compare_with_numeric_coercion(num, other, left == right)
            },
            _ => None,
        }
    }

    /// Attempts numeric coercion for cross-type comparisons
    fn compare_with_numeric_coercion(
        num: &NumberValue,
        other: &Value,
        is_first_numeric: bool,
    ) -> Option<std::cmp::Ordering> {
        let other_as_number = match other {
            Value::Boolean(b) => Some(if **b { 1.0 } else { 0.0 }),
            Value::String(s) => s.parse::<f64>().ok(),
            _ => None,
        };

        other_as_number.and_then(|other_num| {
            let num_value = num.as_f64();
            if is_first_numeric {
                num_value.partial_cmp(&other_num)
            } else {
                other_num.partial_cmp(&num_value)
            }
        })
    }

    /// Checks if value matches a pattern
    pub fn matches_pattern(value: &Value, pattern: &Value) -> ValueResult<bool> {
        match (value, pattern) {
            (a, b) if a == b => Ok(true),
            (Value::String(text), Value::Regex(regex)) => Ok(regex.is_match(text)),
            (Value::String(text), Value::String(pattern_str)) => Ok(text.contains(pattern_str)),
            (Value::Array(arr), element) => Ok(arr.iter().any(|item| item == element)),
            (Value::Object(obj), Value::String(key)) => Ok(obj.contains_key(key)),
            _ => Ok(false),
        }
    }

    /// Checks if value matches a pattern, returning ComparisonResult
    pub fn matches_pattern_result(value: &Value, pattern: &Value) -> ComparisonResult {
        ComparisonResult::from(Self::matches_pattern(value, pattern))
    }

    /// Performs fuzzy string matching with similarity threshold
    pub fn fuzzy_match(value: &Value, expected: &Value, threshold: f64) -> ValueResult<bool> {
        let threshold = threshold.clamp(0.0, 1.0);

        match (value, expected) {
            (Value::String(a), Value::String(b)) => {
                let similarity = Self::string_similarity(a, b);
                Ok(similarity >= threshold)
            },
            _ => Ok(Self::equals(value, expected)),
        }
    }

    /// Performs fuzzy string matching with similarity threshold, returning
    /// ComparisonResult
    pub fn fuzzy_match_result(value: &Value, expected: &Value, threshold: f64) -> ComparisonResult {
        ComparisonResult::from(Self::fuzzy_match(value, expected, threshold))
    }

    /// Calculates simple character-based similarity between two strings
    fn string_similarity(a: &str, b: &str) -> f64 {
        if a == b {
            return 1.0;
        }
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }

        let max_len = a.len().max(b.len()) as f64;
        let common_chars = a.chars().zip(b.chars()).filter(|(ca, cb)| ca == cb).count() as f64;

        common_chars / max_len
    }

    /// Numeric comparison with tolerance for floating point values
    pub fn numeric_equals_with_tolerance(value: &Value, expected: &Value, tolerance: f64) -> bool {
        match (value, expected) {
            (Value::Number(a), Value::Number(b)) => (a.as_f64() - b.as_f64()).abs() <= tolerance,
            _ => Self::equals(value, expected),
        }
    }

    /// Case-insensitive string comparison
    pub fn string_equals_ignore_case(value: &Value, expected: &Value) -> bool {
        match (value, expected) {
            (Value::String(a), Value::String(b)) => a.to_lowercase() == b.to_lowercase(),
            _ => Self::equals(value, expected),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ArrayValue, ObjectValue};

    #[test]
    fn test_equality_comparisons() {
        let a = Value::string("hello");
        let b = Value::string("hello");
        let c = Value::string("world");

        assert!(ValueComparison::equals(&a, &b));
        assert!(!ValueComparison::equals(&a, &c));
        assert!(ValueComparison::not_equals(&a, &c));
        assert!(!ValueComparison::not_equals(&a, &b));
    }

    #[test]
    fn test_comparison_result() {
        let a = Value::string("hello");
        let b = Value::string("hello");
        let c = Value::string("world");

        // Test ComparisonResult API
        let result1 = ValueComparison::equals_result(&a, &b);
        assert!(result1.is_true());
        assert!(!result1.is_false());
        assert!(!result1.is_error());
        assert_eq!(result1.to_bool(), true);
        assert_eq!(result1.to_result().unwrap(), true);

        let result2 = ValueComparison::equals_result(&a, &c);
        assert!(result2.is_false());
        assert!(!result2.is_true());
        assert!(!result2.is_error());
        assert_eq!(result2.to_bool(), false);
        assert_eq!(result2.to_result().unwrap(), false);
    }

    #[test]
    fn test_comparison_errors() {
        let string_val = Value::string("hello");
        let number_val = Value::number(42);

        // These should return errors when comparing incompatible types
        let result = ValueComparison::gt_result(&string_val, &number_val);
        assert!(result.is_error());
        assert!(!result.is_true());
        assert!(!result.is_false());
        assert_eq!(result.to_bool(), false); // Errors treated as false
        assert!(result.to_result().is_err());
    }

    #[test]
    fn test_numeric_comparisons() {
        let five = Value::number(5);
        let ten = Value::number(10);

        assert!(ValueComparison::lt_simple(&five, &ten));
        assert!(ValueComparison::lte_simple(&five, &ten));
        assert!(ValueComparison::gt_simple(&ten, &five));
        assert!(ValueComparison::gte_simple(&ten, &five));

        let same = Value::number(5);
        assert!(ValueComparison::gte_simple(&five, &same));
        assert!(ValueComparison::lte_simple(&five, &same));

        // Test with ComparisonResult
        let result = ValueComparison::gt_result(&ten, &five);
        assert!(result.is_true());
    }

    #[test]
    fn test_membership() {
        let value = Value::string("apple");
        let list = vec![Value::string("apple"), Value::string("banana"), Value::string("cherry")];

        assert!(ValueComparison::in_list(&value, &list));
        assert!(!ValueComparison::not_in_list(&value, &list));

        let missing = Value::string("grape");
        assert!(!ValueComparison::in_list(&missing, &list));
        assert!(ValueComparison::not_in_list(&missing, &list));

        // Test with ComparisonResult
        let result = ValueComparison::in_list_result(&value, &list);
        assert!(result.is_true());
    }

    #[test]
    fn test_emptiness_checks() {
        assert!(ValueComparison::is_empty(&Value::string("")));
        assert!(ValueComparison::is_empty(&Value::array(ArrayValue::empty())));
        assert!(ValueComparison::is_empty(&Value::object(ObjectValue::new())));
        assert!(ValueComparison::is_empty(&Value::number(0)));
        assert!(ValueComparison::is_empty(&Value::boolean(false)));
        assert!(ValueComparison::is_empty(&Value::null()));

        assert!(ValueComparison::is_not_empty(&Value::string("hello")));
        assert!(ValueComparison::is_not_empty(&Value::array(ArrayValue::new(vec![
            Value::string("item")
        ]))));
        assert!(ValueComparison::is_not_empty(&Value::number(42)));
        assert!(ValueComparison::is_not_empty(&Value::boolean(true)));

        // Test with ComparisonResult
        let result = ValueComparison::is_empty_result(&Value::string(""));
        assert!(result.is_true());
    }
}
