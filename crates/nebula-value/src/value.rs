#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    ArrayValue, BinaryValue, BooleanValue, ColorValue, CronValue, DateTimeValue, DurationValue,
    ExpressionValue, FileValue, GroupValue, ModeValue, NumberValue, ObjectValue, RegexValue,
    StringValue,
};

/// The main Value enum representing all possible value types in Nebula
///
/// This enum supports both tagged and untagged serialization depending on context.
/// The default serialization uses a tagged format for better type safety.
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "lowercase"))]
pub enum Value {
    /// Text string value
    String(StringValue),
    /// Numeric value (supports integers and floats)
    Number(NumberValue),
    /// Boolean true/false value
    Boolean(BooleanValue),
    /// Binary data
    Binary(BinaryValue),
    /// Array of values
    Array(ArrayValue),
    /// Key-value object
    Object(ObjectValue),
    /// Date and time value
    DateTime(DateTimeValue),
    /// Time duration/interval
    Duration(DurationValue),
    /// Group value (for grouping related parameters)
    Group(GroupValue),
    /// Mode value (for UI modes)
    Mode(ModeValue),
    /// Expression value (for dynamic expressions)
    Expression(ExpressionValue),
    /// Regular expression value
    Regex(RegexValue),
    /// Color value (hex, rgb, etc.)
    Color(ColorValue),
    /// Cron expression value
    Cron(CronValue),
    /// File reference value
    File(FileValue),
    /// Null/empty value
    #[default]
    Null,
}

impl Value {
    // === Constructor methods ===

    /// Creates a string value
    #[inline]
    #[must_use]
    pub fn string(s: impl Into<StringValue>) -> Self {
        Self::String(s.into())
    }

    /// Creates a number value
    #[inline]
    #[must_use]
    pub fn number(n: impl Into<NumberValue>) -> Self {
        Self::Number(n.into())
    }

    /// Creates a boolean value
    #[inline]
    #[must_use]
    pub fn boolean(b: impl Into<BooleanValue>) -> Self {
        Self::Boolean(b.into())
    }

    /// Creates an array value
    #[inline]
    #[must_use]
    pub fn array(a: impl Into<ArrayValue>) -> Self {
        Self::Array(a.into())
    }

    /// Creates an object value
    #[inline]
    #[must_use]
    pub fn object(o: impl Into<ObjectValue>) -> Self {
        Self::Object(o.into())
    }

    /// Creates a binary value
    #[inline]
    #[must_use]
    pub fn binary(b: impl Into<BinaryValue>) -> Self {
        Self::Binary(b.into())
    }

    /// Creates a datetime value
    #[inline]
    #[must_use]
    pub fn datetime(dt: impl Into<DateTimeValue>) -> Self {
        Self::DateTime(dt.into())
    }

    /// Creates a duration value
    #[inline]
    #[must_use]
    pub fn duration(d: impl Into<DurationValue>) -> Self {
        Self::Duration(d.into())
    }

    /// Creates a null value
    #[inline]
    #[must_use]
    pub const fn null() -> Self {
        Self::Null
    }

    // === Type checking methods ===

    /// Returns true if this is a string value
    #[inline]
    #[must_use]
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Returns true if this is a number value
    #[inline]
    #[must_use]
    pub const fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// Returns true if this is a boolean value
    #[inline]
    #[must_use]
    pub const fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    /// Returns true if this is an array value
    #[inline]
    #[must_use]
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Returns true if this is an object value
    #[inline]
    #[must_use]
    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Returns true if this is a null value
    #[inline]
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns true if this is a binary value
    #[inline]
    #[must_use]
    pub const fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_))
    }

    /// Returns true if this is a datetime value
    #[inline]
    #[must_use]
    pub const fn is_datetime(&self) -> bool {
        matches!(self, Self::DateTime(_))
    }

    /// Returns true if this is a duration value
    #[inline]
    #[must_use]
    pub const fn is_duration(&self) -> bool {
        matches!(self, Self::Duration(_))
    }

    /// Returns true if this is a group value
    #[inline]
    #[must_use]
    pub const fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    /// Returns true if this is a mode value
    #[inline]
    #[must_use]
    pub const fn is_mode(&self) -> bool {
        matches!(self, Self::Mode(_))
    }

    /// Returns true if this is an expression value
    #[inline]
    #[must_use]
    pub const fn is_expression(&self) -> bool {
        matches!(self, Self::Expression(_))
    }

    /// Returns true if this is a regex value
    #[inline]
    #[must_use]
    pub const fn is_regex(&self) -> bool {
        matches!(self, Self::Regex(_))
    }

    /// Returns true if this is a color value
    #[inline]
    #[must_use]
    pub const fn is_color(&self) -> bool {
        matches!(self, Self::Color(_))
    }

    /// Returns true if this is a cron value
    #[inline]
    #[must_use]
    pub const fn is_cron(&self) -> bool {
        matches!(self, Self::Cron(_))
    }

    /// Returns true if this is a file value
    #[inline]
    #[must_use]
    pub const fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    // === Value accessor methods ===

    /// Returns the string value if this is a string
    #[inline]
    #[must_use]
    pub fn as_string(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s.as_ref())
        } else {
            None
        }
    }

    /// Returns the number value if this is a number
    #[inline]
    #[must_use]
    pub fn as_number(&self) -> Option<&NumberValue> {
        if let Self::Number(n) = self {
            Some(n)
        } else {
            None
        }
    }

    /// Returns the boolean value if this is a boolean
    #[inline]
    #[must_use]
    pub fn as_boolean(&self) -> Option<bool> {
        if let Self::Boolean(b) = self {
            Some(**b)
        } else {
            None
        }
    }

    /// Returns the array value if this is an array
    #[inline]
    #[must_use]
    pub fn as_array(&self) -> Option<&ArrayValue> {
        if let Self::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }

    /// Returns the object value if this is an object
    #[inline]
    #[must_use]
    pub fn as_object(&self) -> Option<&ObjectValue> {
        if let Self::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }

    /// Returns the binary value if this is binary
    #[inline]
    #[must_use]
    pub fn as_binary(&self) -> Option<&BinaryValue> {
        if let Self::Binary(b) = self {
            Some(b)
        } else {
            None
        }
    }

    /// Returns the datetime value if this is a datetime
    #[inline]
    #[must_use]
    pub fn as_datetime(&self) -> Option<&DateTimeValue> {
        if let Self::DateTime(dt) = self {
            Some(dt)
        } else {
            None
        }
    }

    /// Returns the duration value if this is a duration
    #[inline]
    #[must_use]
    pub fn as_duration(&self) -> Option<&DurationValue> {
        if let Self::Duration(d) = self {
            Some(d)
        } else {
            None
        }
    }

    // === Mutable accessor methods ===

    /// Returns a mutable reference to the array value if this is an array
    #[inline]
    #[must_use]
    pub fn as_array_mut(&mut self) -> Option<&mut ArrayValue> {
        if let Self::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the object value if this is an object
    #[inline]
    #[must_use]
    pub fn as_object_mut(&mut self) -> Option<&mut ObjectValue> {
        if let Self::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }

    // === Utility methods ===

    /// Returns the type name as a string
    #[inline]
    #[must_use]
    pub fn type_name(&self) -> &str {
        match self {
            Self::String(_) => "string",
            Self::Number(_) => "number",
            Self::Boolean(_) => "boolean",
            Self::Binary(_) => "binary",
            Self::Array(_) => "array",
            Self::Object(_) => "object",
            Self::DateTime(_) => "datetime",
            Self::Duration(_) => "duration",
            Self::Group(_) => "group",
            Self::Mode(_) => "mode",
            Self::Expression(_) => "expression",
            Self::Regex(_) => "regex",
            Self::Color(_) => "color",
            Self::Cron(_) => "cron",
            Self::File(_) => "file",
            Self::Null => "null",
        }
    }

    /// Returns a display string for the value
    #[must_use]
    pub fn display_string(&self) -> String {
        match self {
            Self::String(s) => format!("\"{}\"", s.to_string()),
            Self::Number(n) => n.to_string(),
            Self::Boolean(b) => b.to_string(),
            Self::Array(a) => format!("[array with {} items]", a.len()),
            Self::Object(o) => format!("[object with {} fields]", o.len()),
            Self::DateTime(dt) => dt.to_string(),
            Self::Duration(d) => d.to_string(),
            Self::Group(g) => format!("[group: {}]", g.to_string()),
            Self::Mode(m) => format!("[mode: {}]", m.to_string()),
            Self::Expression(e) => format!("{{{{ {} }}}}", e.to_string()),
            Self::Regex(r) => format!("/{}/", r.pattern()),
            Self::Color(c) => c.to_string(),
            Self::Cron(cr) => cr.to_string(),
            Self::File(f) => f.to_string(),
            Self::Binary(_) => "[binary data]".to_string(),
            Self::Null => "null".to_string(),
        }
    }

    /// Returns true if the value is considered "truthy"
    #[must_use]
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Boolean(b) => **b,
            Self::String(s) => !s.to_string().is_empty(),
            Self::Number(n) => !n.is_zero(),
            Self::Array(a) => !a.is_empty(),
            Self::Object(o) => !o.is_empty(),
            Self::Null => false,
            // All other types are considered truthy if they exist
            _ => true,
        }
    }

    /// Returns true if the value is considered "falsy"
    #[inline]
    #[must_use]
    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }
}

// === Display implementation ===

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_string())
    }
}

// === From implementations for basic types ===

impl From<&str> for Value {
    #[inline]
    fn from(s: &str) -> Self {
        Self::string(s)
    }
}

impl From<String> for Value {
    #[inline]
    fn from(s: String) -> Self {
        Self::string(s)
    }
}

impl From<i32> for Value {
    #[inline]
    fn from(n: i32) -> Self {
        Self::number(n)
    }
}

impl From<i64> for Value {
    #[inline]
    fn from(n: i64) -> Self {
        Self::number(n)
    }
}

impl From<f32> for Value {
    #[inline]
    fn from(n: f32) -> Self {
        Self::number(n)
    }
}

impl From<f64> for Value {
    #[inline]
    fn from(n: f64) -> Self {
        Self::number(n)
    }
}

impl From<bool> for Value {
    #[inline]
    fn from(b: bool) -> Self {
        Self::boolean(b)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(vec: Vec<T>) -> Self {
        Self::array(vec.into_iter().map(Into::into).collect::<ArrayValue>())
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(value) => value.into(),
            None => Self::Null,
        }
    }
}

// === Comparison implementation ===

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        match (self, other) {
            // Same types
            (Self::String(a), Self::String(b)) => a.partial_cmp(b),
            (Self::Number(a), Self::Number(b)) => a.partial_cmp(b),
            (Self::Boolean(a), Self::Boolean(b)) => a.partial_cmp(b),
            (Self::DateTime(a), Self::DateTime(b)) => a.partial_cmp(b),
            (Self::Duration(a), Self::Duration(b)) => a.partial_cmp(b),

            // Null comparisons
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Null, _) => Some(Ordering::Less),
            (_, Self::Null) => Some(Ordering::Greater),

            // Type ordering (for stable sorting)
            _ => self.type_name().partial_cmp(other.type_name()),
        }
    }
}

// === JSON conversion (feature-gated) ===

#[cfg(feature = "json")]
impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::String(s) => serde_json::Value::String(s.to_string()),
            Value::Number(n) => n.into(),
            Value::Boolean(b) => serde_json::Value::Bool(*b),
            Value::Array(a) => a.into(),
            Value::Object(o) => o.into(),
            Value::Null => serde_json::Value::Null,
            // Convert other types to strings for JSON compatibility
            other => serde_json::Value::String(other.to_string()),
        }
    }
}

#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for Value {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::String(s) => Ok(Value::string(s)),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(Value::number(i))
                } else if let Some(f) = n.as_f64() {
                    Ok(Value::number(f))
                } else {
                    Err(ValueError::custom("Invalid number format"))
                }
            }
            serde_json::Value::Bool(b) => Ok(Value::boolean(b)),
            serde_json::Value::Array(arr) => {
                let values: Result<Vec<Value>, ValueError> = arr
                    .into_iter()
                    .map(Value::try_from)
                    .collect();
                Ok(Value::array(ArrayValue::new(values?)))
            }
            serde_json::Value::Object(obj) => {
                let mut object = ObjectValue::new();
                for (key, val) in obj {
                    object.insert(key, Value::try_from(val)?)?;
                }
                Ok(Value::object(object))
            }
            serde_json::Value::Null => Ok(Value::Null),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_creation() {
        let string_val = Value::string("hello");
        assert!(string_val.is_string());
        assert_eq!(string_val.as_string(), Some("hello"));

        let number_val = Value::number(42);
        assert!(number_val.is_number());

        let bool_val = Value::boolean(true);
        assert!(bool_val.is_boolean());
        assert_eq!(bool_val.as_boolean(), Some(true));

        let null_val = Value::null();
        assert!(null_val.is_null());
    }

    #[test]
    fn test_value_conversions() {
        let from_str: Value = "hello".into();
        assert!(from_str.is_string());

        let from_int: Value = 42.into();
        assert!(from_int.is_number());

        let from_bool: Value = true.into();
        assert!(from_bool.is_boolean());

        let from_vec: Value = vec![1, 2, 3].into();
        assert!(from_vec.is_array());
    }

    #[test]
    fn test_truthy_falsy() {
        assert!(Value::boolean(true).is_truthy());
        assert!(Value::boolean(false).is_falsy());
        assert!(Value::string("hello").is_truthy());
        assert!(Value::string("").is_falsy());
        assert!(Value::number(42).is_truthy());
        assert!(Value::number(0).is_falsy());
        assert!(Value::null().is_falsy());
    }

    #[test]
    fn test_type_names() {
        assert_eq!(Value::string("hello").type_name(), "string");
        assert_eq!(Value::number(42).type_name(), "number");
        assert_eq!(Value::boolean(true).type_name(), "boolean");
        assert_eq!(Value::null().type_name(), "null");
    }

    #[cfg(all(feature = "json", feature = "serde"))]
    #[test]
    fn test_json_conversion() {
        let value = Value::string("test");
        let json: serde_json::Value = value.clone().into();
        let back: Value = json.try_into().unwrap();
        assert_eq!(value, back);
    }
}