//! Value types for the template engine

use crate::error::{Error, Result};
use std::{collections::HashMap, fmt};

/// A value that can be used in template expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Null value
    Null,
    /// Boolean value
    Bool(bool),
    /// Integer number
    Integer(i64),
    /// Floating point number
    Float(f64),
    /// String value
    String(String),
    /// Array of values
    Array(Vec<Value>),
    /// Object/map of string keys to values
    Object(HashMap<String, Value>),
}

impl Value {
    /// Create a null value
    pub fn null() -> Self {
        Self::Null
    }

    /// Create a boolean value
    pub fn bool(value: bool) -> Self {
        Self::Bool(value)
    }

    /// Create an integer value
    pub fn integer(value: i64) -> Self {
        Self::Integer(value)
    }

    /// Create a float value
    pub fn float(value: f64) -> Self {
        Self::Float(value)
    }

    /// Create a string value
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    /// Create an array value
    pub fn array(values: Vec<Value>) -> Self {
        Self::Array(values)
    }

    /// Create an object value
    pub fn object(map: HashMap<String, Value>) -> Self {
        Self::Object(map)
    }

    /// Check if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Check if the value is a boolean
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// Check if the value is a number (integer or float)
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Integer(_) | Self::Float(_))
    }

    /// Check if the value is a string
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Check if the value is an array
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Check if the value is an object
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Check if the value is empty (null, empty string, empty array, empty object)
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Null => true,
            Self::String(s) => s.is_empty(),
            Self::Array(a) => a.is_empty(),
            Self::Object(o) => o.is_empty(),
            _ => false,
        }
    }

    /// Check if the value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Null => false,
            Self::Bool(b) => *b,
            Self::Integer(i) => *i != 0,
            Self::Float(f) => *f != 0.0 && !f.is_nan(),
            Self::String(s) => !s.is_empty(),
            Self::Array(a) => !a.is_empty(),
            Self::Object(o) => !o.is_empty(),
        }
    }

    /// Get the type name of the value
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Bool(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Float(_) => "float",
            Self::String(_) => "string",
            Self::Array(_) => "array",
            Self::Object(_) => "object",
        }
    }

    /// Convert to boolean
    pub fn as_bool(&self) -> Result<bool> {
        match self {
            Self::Bool(b) => Ok(*b),
            Self::Null => Ok(false),
            Self::Integer(i) => Ok(*i != 0),
            Self::Float(f) => Ok(*f != 0.0 && !f.is_nan()),
            Self::String(s) => match s.to_lowercase().as_str() {
                "true" | "yes" | "1" | "on" => Ok(true),
                "false" | "no" | "0" | "off" | "" => Ok(false),
                _ => Err(Error::type_error(self.type_name(), "boolean")),
            },
            _ => Ok(self.is_truthy()),
        }
    }

    /// Convert to integer
    pub fn as_integer(&self) -> Result<i64> {
        match self {
            Self::Integer(i) => Ok(*i),
            Self::Float(f) => Ok(*f as i64),
            Self::Bool(b) => Ok(if *b { 1 } else { 0 }),
            Self::String(s) => s.parse().map_err(|_| Error::type_error(self.type_name(), "integer")),
            _ => Err(Error::type_error(self.type_name(), "integer")),
        }
    }

    /// Convert to float
    pub fn as_float(&self) -> Result<f64> {
        match self {
            Self::Float(f) => Ok(*f),
            Self::Integer(i) => Ok(*i as f64),
            Self::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Self::String(s) => s.parse().map_err(|_| Error::type_error(self.type_name(), "float")),
            _ => Err(Error::type_error(self.type_name(), "float")),
        }
    }

    /// Convert to string
    pub fn as_string(&self) -> Result<String> {
        match self {
            Self::String(s) => Ok(s.clone()),
            Self::Null => Ok("null".to_string()),
            Self::Bool(b) => Ok(b.to_string()),
            Self::Integer(i) => Ok(i.to_string()),
            Self::Float(f) => Ok(f.to_string()),
            Self::Array(_) | Self::Object(_) => {
                Err(Error::type_error(self.type_name(), "string"))
            }
        }
    }

    /// Get string reference (only for string values)
    pub fn as_str(&self) -> Result<&str> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(Error::type_error(self.type_name(), "string")),
        }
    }

    /// Convert to array
    pub fn as_array(&self) -> Result<&Vec<Value>> {
        match self {
            Self::Array(a) => Ok(a),
            _ => Err(Error::type_error(self.type_name(), "array")),
        }
    }

    /// Convert to mutable array
    pub fn as_array_mut(&mut self) -> Result<&mut Vec<Value>> {
        match self {
            Self::Array(a) => Ok(a),
            _ => Err(Error::type_error(self.type_name(), "array")),
        }
    }

    /// Convert to object
    pub fn as_object(&self) -> Result<&HashMap<String, Value>> {
        match self {
            Self::Object(o) => Ok(o),
            _ => Err(Error::type_error(self.type_name(), "object")),
        }
    }

    /// Convert to mutable object
    pub fn as_object_mut(&mut self) -> Result<&mut HashMap<String, Value>> {
        match self {
            Self::Object(o) => Ok(o),
            _ => Err(Error::type_error(self.type_name(), "object")),
        }
    }

    /// Get a value by key (for objects) or index (for arrays)
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Self::Object(o) => o.get(key),
            Self::Array(a) => {
                if let Ok(index) = key.parse::<usize>() {
                    a.get(index)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get a value by index (for arrays)
    pub fn get_index(&self, index: usize) -> Option<&Value> {
        match self {
            Self::Array(a) => a.get(index),
            _ => None,
        }
    }

    /// Set a value by key (for objects) or index (for arrays)
    pub fn set(&mut self, key: &str, value: Value) -> Result<()> {
        match self {
            Self::Object(o) => {
                o.insert(key.to_string(), value);
                Ok(())
            }
            Self::Array(a) => {
                if let Ok(index) = key.parse::<usize>() {
                    if index < a.len() {
                        a[index] = value;
                        Ok(())
                    } else {
                        Err(Error::index(index as isize, a.len()))
                    }
                } else {
                    Err(Error::type_error("string", "array index"))
                }
            }
            _ => Err(Error::type_error(self.type_name(), "object or array")),
        }
    }

    /// Get the length of the value (for strings, arrays, objects)
    pub fn len(&self) -> Result<usize> {
        match self {
            Self::String(s) => Ok(s.len()),
            Self::Array(a) => Ok(a.len()),
            Self::Object(o) => Ok(o.len()),
            _ => Err(Error::type_error(self.type_name(), "string, array, or object")),
        }
    }

    /// Check if the value has a length of zero
    pub fn is_len_zero(&self) -> bool {
        self.len().map_or(false, |len| len == 0)
    }

    /// Navigate a path in the value (e.g., "user.profile.name")
    pub fn navigate(&self, path: &str) -> Option<&Value> {
        if path.is_empty() {
            return Some(self);
        }

        let parts: Vec<&str> = path.split('.').collect();
        let mut current = self;

        for part in parts {
            current = current.get(part)?;
        }

        Some(current)
    }

    /// Compare values for equality
    pub fn equals(&self, other: &Value) -> bool {
        self == other
    }

    /// Deep clone the value
    pub fn deep_clone(&self) -> Value {
        self.clone()
    }
}

// Conversion implementations
impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Self::Array(value)
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(value: HashMap<String, Value>) -> Self {
        Self::Object(value)
    }
}

// Optional serde integration
#[cfg(feature = "serde")]
impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Self::Null,
            serde_json::Value::Bool(b) => Self::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Self::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    Self::Float(f)
                } else {
                    Self::Null
                }
            }
            serde_json::Value::String(s) => Self::String(s),
            serde_json::Value::Array(a) => {
                Self::Array(a.into_iter().map(Value::from).collect())
            }
            serde_json::Value::Object(o) => {
                Self::Object(o.into_iter().map(|(k, v)| (k, Value::from(v))).collect())
            }
        }
    }
}

#[cfg(feature = "serde")]
impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => serde_json::Value::Null,
            Value::Bool(b) => serde_json::Value::Bool(b),
            Value::Integer(i) => serde_json::Value::Number(i.into()),
            Value::Float(f) => {
                serde_json::Number::from_f64(f)
                    .map_or(serde_json::Value::Null, serde_json::Value::Number)
            }
            Value::String(s) => serde_json::Value::String(s),
            Value::Array(a) => {
                serde_json::Value::Array(a.into_iter().map(serde_json::Value::from).collect())
            }
            Value::Object(o) => serde_json::Value::Object(
                o.into_iter()
                    .map(|(k, v)| (k, serde_json::Value::from(v)))
                    .collect(),
            ),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(b) => write!(f, "{}", b),
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::String(s) => write!(f, "{}", s),
            Self::Array(a) => {
                write!(f, "[")?;
                for (i, item) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Self::Object(o) => {
                write!(f, "{{")?;
                for (i, (key, value)) in o.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_creation() {
        assert_eq!(Value::null(), Value::Null);
        assert_eq!(Value::bool(true), Value::Bool(true));
        assert_eq!(Value::integer(42), Value::Integer(42));
        assert_eq!(Value::float(3.14), Value::Float(3.14));
        assert_eq!(Value::string("hello"), Value::String("hello".to_string()));
    }

    #[test]
    fn test_type_checks() {
        assert!(Value::null().is_null());
        assert!(Value::bool(true).is_bool());
        assert!(Value::integer(42).is_number());
        assert!(Value::float(3.14).is_number());
        assert!(Value::string("hello").is_string());
        assert!(Value::array(vec![]).is_array());
        assert!(Value::object(HashMap::new()).is_object());
    }

    #[test]
    fn test_truthiness() {
        assert!(!Value::null().is_truthy());
        assert!(!Value::bool(false).is_truthy());
        assert!(Value::bool(true).is_truthy());
        assert!(!Value::integer(0).is_truthy());
        assert!(Value::integer(1).is_truthy());
        assert!(!Value::string("").is_truthy());
        assert!(Value::string("hello").is_truthy());
    }

    #[test]
    fn test_conversions() -> Result<()> {
        assert_eq!(Value::bool(true).as_integer()?, 1);
        assert_eq!(Value::integer(42).as_float()?, 42.0);
        assert_eq!(Value::string("hello").as_str()?, "hello");

        let arr = Value::array(vec![Value::integer(1), Value::integer(2)]);
        assert_eq!(arr.as_array()?.len(), 2);

        Ok(())
    }

    #[test]
    fn test_navigation() {
        let mut obj = HashMap::new();
        obj.insert("name".to_string(), Value::string("Alice"));

        let value = Value::object(obj);
        assert_eq!(value.get("name"), Some(&Value::string("Alice")));
        assert_eq!(value.navigate("name"), Some(&Value::string("Alice")));
    }

    #[test]
    fn test_display() {
        assert_eq!(Value::null().to_string(), "null");
        assert_eq!(Value::bool(true).to_string(), "true");
        assert_eq!(Value::integer(42).to_string(), "42");
        assert_eq!(Value::string("hello").to_string(), "hello");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_conversion() {
        use serde_json::json;

        let json_val = json!({"name": "Alice", "age": 30});
        let value = Value::from(json_val.clone());
        let back_to_json = serde_json::Value::from(value);

        assert_eq!(json_val, back_to_json);
    }
}