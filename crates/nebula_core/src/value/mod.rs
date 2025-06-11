pub mod array;
pub mod binary;
pub mod boolean;
pub mod datetime;
pub mod duration;
pub mod number;
pub mod object;
pub mod string;
mod mode;
mod group;
mod expression;

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

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(tag = "type", rename_all = "lowercase")]
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
    /// Date and time value (supports datetime, date-only, time-only)
    DateTime(DateTimeValue),
    /// Time duration/interval
    Duration(DurationValue),
    /// Group value (for grouping related parameters)
    Group(GroupValue),
    /// Mode value (for UI modes, e.g. select, text)
    Mode(ModeValue),
    /// Expression value (for dynamic expressions)
    Expression(ExpressionValue),
    /// Null/empty value
    Null,
}

impl Value {
    // Проверки типов
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    pub fn is_datetime(&self) -> bool {
        matches!(self, Value::DateTime(_))
    }

    pub fn is_duration(&self) -> bool {
        matches!(self, Value::Duration(_))
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, Value::Binary(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    // Извлечение значений как ссылки
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s.as_ref()),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&NumberValue> {
        match self {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(**b), // если BooleanValue(bool)
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&ArrayValue> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&ObjectValue> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn as_datetime(&self) -> Option<&DateTimeValue> {
        match self {
            Value::DateTime(dt) => Some(dt),
            _ => None,
        }
    }

    pub fn as_duration(&self) -> Option<&DurationValue> {
        match self {
            Value::Duration(d) => Some(d),
            _ => None,
        }
    }

    pub fn as_binary(&self) -> Option<&BinaryValue> {
        match self {
            Value::Binary(b) => Some(b),
            _ => None,
        }
    }

    // Mutable версии
    pub fn as_array_mut(&mut self) -> Option<&mut ArrayValue> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut ObjectValue> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "string",
            Value::Number(_) => "number",
            Value::Boolean(_) => "boolean",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::DateTime(_) => "datetime",
            Value::Duration(_) => "duration",
            Value::Binary(_) => "binary",
            Value::Group(_) => "group",
            Value::Mode(_) => "mode",
            Value::Expression(_) => "expression",
            Value::Null => "null",
        }
    }

    // Конверсии с fallback
    pub fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.to_string(),
            Value::Number(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::DateTime(dt) => dt.to_string(),
            Value::Duration(d) => d.to_string(),
            Value::Binary(b) => b.to_string(),
            Value::Null => "null".to_string(),
            _ => format!("[{}]", self.type_name()),
        }
    }
}

impl Into<serde_json::Value> for Value {
    fn into(self) -> serde_json::Value {
        match self {
            Value::String(s) => s.into(),
            Value::Number(n) => n.into(),
            Value::Boolean(b) => b.into(),
            Value::Array(a) => a.into(),
            Value::Object(o) => o.into(),
            Value::DateTime(dt) => dt.into(),
            Value::Duration(d) => d.into(),
            Value::Binary(b) => b.into(),
            Value::Group(g) => g.into(),
            Value::Mode(m) => m.into(),
            Value::Expression(e) => e.into(),
            Value::Null => serde_json::Value::Null,
        }
    }
}
