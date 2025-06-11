// nebula_core/src/value/mod.rs

pub mod array;
pub mod binary;
pub mod boolean;
pub mod datetime;
pub mod duration;
pub mod number;
pub mod object;
pub mod string;
pub mod mode;
pub mod group;
pub mod expression;
pub mod regex;
pub mod error;
mod comparison;

pub use error::ValueError;
pub use array::ArrayValue;
pub use binary::BinaryValue;
pub use boolean::BooleanValue;
pub use datetime::DateTimeValue;
pub use duration::DurationValue;
pub use number::NumberValue;
pub use object::ObjectValue;
pub use string::StringValue;
pub use mode::ModeValue;
pub use group::GroupValue;
pub use expression::ExpressionValue;
pub use regex::RegexValue;
pub use comparison::{ValueComparison, ComparisonResult};

use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, AsRefStr)]
#[serde(tag = "kind", rename_all = "lowercase")]
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
    /// Null/empty value
    #[default]
    Null,
}

impl Value {
    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    pub fn is_regex(&self) -> bool {
        matches!(self, Self::Regex(_))
    }

    // Type checks
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    // Value accessors
    pub fn as_string(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s.as_ref())
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Option<&NumberValue> {
        if let Self::Number(n) = self {
            Some(n)
        } else {
            None
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        if let Self::Boolean(b) = self {
            Some(**b)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&ArrayValue> {
        if let Self::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<&ObjectValue> {
        if let Self::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }

    // Mutable accessors
    pub fn as_array_mut(&mut self) -> Option<&mut ArrayValue> {
        if let Self::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut ObjectValue> {
        if let Self::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }

    pub fn as_group(&self) -> Option<&GroupValue> {
        if let Self::Group(g) = self { Some(g) } else { None }
    }

    pub fn as_regex(&self) -> Option<&RegexValue> {
        if let Self::Regex(r) = self { Some(r) } else { None }
    }

    pub fn try_as_string(&self) -> Result<&str, ValueError> {
        self.as_string().ok_or_else(|| {
            ValueError::type_conversion(self.type_name(), "string")
        })
    }

    pub fn try_as_number(&self) -> Result<f64, ValueError> {
        self.as_number()
            .map(|n| n.as_f64())
            .ok_or_else(|| {
                ValueError::type_conversion(self.type_name(), "number")
            })
    }

    pub fn try_as_boolean(&self) -> Result<bool, ValueError> {
        self.as_boolean().ok_or_else(|| {
            ValueError::type_conversion(self.type_name(), "boolean")
        })
    }

    pub fn try_as_array(&self) -> Result<&ArrayValue, ValueError> {
        self.as_array().ok_or_else(|| {
            ValueError::type_conversion(self.type_name(), "array")
        })
    }

    pub fn try_as_object(&self) -> Result<&ObjectValue, ValueError> {
        self.as_object().ok_or_else(|| {
            ValueError::type_conversion(self.type_name(), "object")
        })
    }

    // Constructors
    pub fn string(s: impl Into<StringValue>) -> Self {
        Self::String(s.into())
    }

    pub fn number(n: impl Into<NumberValue>) -> Self {
        Self::Number(n.into())
    }

    pub fn boolean(b: bool) -> Self {
        Self::Boolean(b.into())
    }

    pub fn array(a: impl Into<ArrayValue>) -> Self {
        Self::Array(a.into())
    }

    pub fn object(o: impl Into<ObjectValue>) -> Self {
        Self::Object(o.into())
    }

    pub fn regex(pattern: impl AsRef<str>) -> Result<Self, ValueError> {
        Ok(Self::Regex(RegexValue::new(pattern)?))
    }

    // Conversion to string
    pub fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.to_string(),
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
            Self::Binary(_) => "[binary data]".to_string(),
            Self::Null => "null".to_string(),
        }
    }

    /// Returns the type name as static string
    pub fn type_name(&self) -> &'static str {
        self.as_ref()
    }
}

// From implementations for basic types
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self::string(s)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::string(s)
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Self::number(n)
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Self::number(n)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Self::number(n)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::boolean(b)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(vec: Vec<T>) -> Self {
        Self::array(vec.into_iter().map(Into::into).collect::<ArrayValue>())
    }
}