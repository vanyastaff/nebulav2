// crates/nebula_core/src/value/string.rs

use crate::value::{ValueError, ValueResult};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};
use std::fmt;
use std::ops::{Add, Deref, DerefMut};
use std::str::FromStr;

/// String value type with efficient operations and conversions
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StringValue(String);

impl StringValue {
    pub const fn new(value: String) -> Self {
        Self(value)
    }

    pub fn from(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Creates an empty string value (const)
    pub const fn empty() -> Self {
        Self(String::new())
    }

    /// Creates a string value with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity))
    }

    /// Returns true if the string is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the length of the string in bytes
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the length of the string in characters
    pub fn char_count(&self) -> usize {
        self.0.chars().count()
    }

    /// Returns the string as a &str
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts to owned String
    pub fn into_string(self) -> String {
        self.0
    }

    /// Appends a string slice to this string
    pub fn push_str(&mut self, string: &str) {
        self.0.push_str(string);
    }

    /// Appends a character to this string
    pub fn push(&mut self, ch: char) {
        self.0.push(ch);
    }

    /// Truncates this string to the specified length
    pub fn truncate(&mut self, new_len: usize) {
        self.0.truncate(new_len);
    }

    /// Returns a string slice with leading and trailing whitespace removed
    pub fn trim(&self) -> &str {
        self.0.trim()
    }

    /// Creates a StringValue with trimmed content
    pub fn trimmed(&self) -> StringValue {
        StringValue::new(self.0.trim().to_string())
    }

    /// Converts to lowercase
    pub fn to_lowercase(&self) -> StringValue {
        StringValue::new(self.0.to_lowercase())
    }

    /// Converts to uppercase
    pub fn to_uppercase(&self) -> StringValue {
        StringValue::new(self.0.to_uppercase())
    }

    /// Checks if string contains a pattern
    pub fn contains(&self, pattern: &str) -> bool {
        self.0.contains(pattern)
    }

    /// Checks if string starts with a pattern
    pub fn starts_with(&self, pattern: &str) -> bool {
        self.0.starts_with(pattern)
    }

    /// Checks if string ends with a pattern
    pub fn ends_with(&self, pattern: &str) -> bool {
        self.0.ends_with(pattern)
    }

    /// Removes prefix if present
    pub fn strip_prefix(&self, prefix: &str) -> Option<StringValue> {
        self.0.strip_prefix(prefix).map(|s| StringValue::new(s.to_string()))
    }

    /// Removes suffix if present
    pub fn strip_suffix(&self, suffix: &str) -> Option<StringValue> {
        self.0.strip_suffix(suffix).map(|s| StringValue::new(s.to_string()))
    }

    /// Replaces all matches of a pattern with another string
    pub fn replace(&self, from: &str, to: &str) -> StringValue {
        StringValue::new(self.0.replace(from, to))
    }

    /// Splits the string by a pattern and returns a vector of StringValues
    pub fn split(&self, pattern: &str) -> Vec<StringValue> {
        self.0
            .split(pattern)
            .map(|s| StringValue::new(s.to_string()))
            .collect()
    }

    /// Splits by whitespace
    pub fn split_whitespace(&self) -> impl Iterator<Item = StringValue> + '_ {
        self.0.split_whitespace().map(|s| StringValue::new(s.to_string()))
    }

    /// Returns a substring from start to end (character indices)
    ///
    /// Returns an error if indices are out of bounds
    pub fn substring(&self, start: usize, end: usize) -> ValueResult<StringValue> {
        if start > end {
            return Err(ValueError::custom(format!(
                "Invalid substring range: start ({}) > end ({})", start, end
            )));
        }

        let mut chars = self.0.char_indices();
        let start_byte = chars.nth(start).map(|(i, _)| i).ok_or_else(|| {
            ValueError::index_out_of_bounds(start, self.char_count())
        })?;

        let end_byte = if end == self.char_count() {
            self.0.len()
        } else {
            chars.nth(end.saturating_sub(start + 1))
                .map(|(i, _)| i)
                .ok_or_else(|| ValueError::index_out_of_bounds(end, self.char_count()))?
        };

        Ok(StringValue::new(self.0[start_byte..end_byte].to_string()))
    }

    /// Returns the first n characters
    pub fn take(&self, n: usize) -> ValueResult<StringValue> {
        let mut chars = self.0.char_indices();
        let end_byte = chars.nth(n).map(|(i, _)| i).unwrap_or(self.0.len());
        Ok(StringValue::new(self.0[..end_byte].to_string()))
    }

    /// Returns all characters after skipping n
    pub fn skip(&self, n: usize) -> ValueResult<StringValue> {
        let mut chars = self.0.char_indices();
        let start_byte = chars.nth(n).map(|(i, _)| i).ok_or_else(|| {
            ValueError::index_out_of_bounds(n, self.char_count())
        })?;
        Ok(StringValue::new(self.0[start_byte..].to_string()))
    }

    /// Safe character access by index
    pub fn char_at(&self, index: usize) -> ValueResult<char> {
        self.0.chars().nth(index)
            .ok_or_else(|| ValueError::index_out_of_bounds(index, self.char_count()))
    }

    /// Attempts to parse the string as a specific type
    pub fn parse<T>(&self) -> ValueResult<T>
    where
        T: FromStr,
        T::Err: Into<ValueError>,
    {
        self.0.parse::<T>().map_err(Into::into)
    }

    /// Explicit parse with type annotation
    pub fn parse_with<T>(&self) -> ValueResult<T>
    where
        T: FromStr,
        T::Err: Into<ValueError>,
    {
        self.parse()
    }

    /// Validates that the string is valid UTF-8
    pub fn validate_utf8(&self) -> ValueResult<()> {
        Ok(())
    }
}

// Trait implementations

impl Deref for StringValue {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StringValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<str> for StringValue {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for StringValue {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl fmt::Display for StringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for StringValue {
    type Err = ValueError;

    fn from_str(s: &str) -> ValueResult<Self> {
        Ok(StringValue::new(s.to_string()))
    }
}

// From implementations
impl From<String> for StringValue {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for StringValue {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

impl From<char> for StringValue {
    fn from(value: char) -> Self {
        Self::new(value.to_string())
    }
}

impl From<Cow<'_, str>> for StringValue {
    fn from(value: Cow<'_, str>) -> Self {
        match value {
            Cow::Borrowed(s) => Self::new(s.to_string()),
            Cow::Owned(s) => Self(s),
        }
    }
}

// Into implementations
impl From<StringValue> for String {
    fn from(value: StringValue) -> Self {
        value.0
    }
}

impl From<StringValue> for Cow<'_, str> {
    fn from(value: StringValue) -> Self {
        Cow::Owned(value.0)
    }
}

impl Default for StringValue {
    fn default() -> Self {
        Self::empty()
    }
}

impl Borrow<str> for StringValue {
    fn borrow(&self) -> &str {
        &self.0
    }
}

// Comparison traits
impl PartialEq<str> for StringValue {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for StringValue {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for StringValue {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

// Add implementations (generic)
impl<T: Into<StringValue>> Add<T> for StringValue {
    type Output = StringValue;

    fn add(mut self, rhs: T) -> Self::Output {
        self.0.push_str(&rhs.into().0);
        self
    }
}

// JSON conversion
impl From<StringValue> for serde_json::Value {
    fn from(value: StringValue) -> Self {
        serde_json::Value::String(value.0)
    }
}

impl TryFrom<serde_json::Value> for StringValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::String(s) => Ok(StringValue::new(s)),
            other => Err(ValueError::type_conversion_with_value(
                other.to_string(),
                "StringValue",
                format!("{:?}", other)
            )),
        }
    }
}

// Iterator support
impl FromIterator<char> for StringValue {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut s = String::new();
        s.extend(iter);
        StringValue::new(s)
    }
}

impl FromIterator<String> for StringValue {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        StringValue::new(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<&'a str> for StringValue {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        StringValue::new(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_creation() {
        const EMPTY: StringValue = StringValue::empty();
        assert!(EMPTY.is_empty());
    }

    #[test]
    fn test_creation() {
        let s1 = StringValue::new("hello".to_string());
        let s2 = StringValue::from("world");
        let s3: StringValue = "test".into();

        assert_eq!(s1.as_str(), "hello");
        assert_eq!(s2.as_str(), "world");
        assert_eq!(s3.as_str(), "test");
    }

    #[test]
    fn test_strip_prefix_suffix() {
        let s = StringValue::from("Hello, world!");
        assert_eq!(s.strip_prefix("Hello").unwrap(), ", world!");
        assert_eq!(s.strip_suffix("world!").unwrap(), "Hello, ");
        assert!(s.strip_prefix("foo").is_none());
    }

    #[test]
    fn test_operations() {
        let s = StringValue::from("Hello World");

        assert_eq!(s.len(), 11);
        assert_eq!(s.char_count(), 11);
        assert!(s.contains("World"));
        assert!(s.starts_with("Hello"));
        assert!(s.ends_with("World"));
    }

    #[test]
    fn test_mutations() {
        let mut s = StringValue::from("Hello");
        s.push_str(" World");
        s.push('!');

        assert_eq!(s.as_str(), "Hello World!");
    }

    #[test]
    fn test_transformations() {
        let s = StringValue::from("  Hello World  ");

        assert_eq!(s.trimmed().as_str(), "Hello World");
        assert_eq!(s.to_lowercase().as_str(), "  hello world  ");
        assert_eq!(s.to_uppercase().as_str(), "  HELLO WORLD  ");
    }

    #[test]
    fn test_substring() {
        let s = StringValue::from("Hello");

        assert_eq!(s.substring(0, 5).unwrap().as_str(), "Hello");
        assert_eq!(s.substring(1, 4).unwrap().as_str(), "ell");
        assert_eq!(s.substring(0, 3).unwrap().as_str(), "Hel");

        // Test error cases
        assert!(s.substring(5, 10).is_err()); // end > length
        assert!(s.substring(3, 1).is_err());  // start > end
    }

    #[test]
    fn test_take_skip() {
        let s = StringValue::from("Hello World");

        assert_eq!(s.take(5).unwrap().as_str(), "Hello");
        assert_eq!(s.skip(6).unwrap().as_str(), "World");

        // Test error cases
        assert!(s.take(20).is_err()); // n > length
        assert!(s.skip(20).is_err()); // n > length
    }

    #[test]
    fn test_split() {
        let s = StringValue::from("a,b,c");
        let parts = s.split(",");

        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].as_str(), "a");
        assert_eq!(parts[1].as_str(), "b");
        assert_eq!(parts[2].as_str(), "c");
    }

    #[test]
    fn test_add() {
        let s1 = StringValue::from("Hello");
        let s2 = StringValue::from(" World");

        let result = s1 + s2;
        assert_eq!(result.as_str(), "Hello World");

        let s3 = StringValue::from("Hello");
        let result2 = s3 + " World";
        assert_eq!(result2.as_str(), "Hello World");
    }

    #[test]
    fn test_comparisons() {
        let s = StringValue::from("hello");

        assert_eq!(s, "hello");
        assert_eq!(s, String::from("hello"));
        assert_ne!(s, "world");
    }

    #[test]
    fn test_json_conversion() {
        let s = StringValue::from("test");
        let json_val: serde_json::Value = s.into();

        assert_eq!(json_val, serde_json::Value::String("test".to_string()));

        let back = StringValue::try_from(json_val).unwrap();
        assert_eq!(back.as_str(), "test");

        // Test error case
        let number_val = serde_json::Value::Number(serde_json::Number::from(42));
        assert!(StringValue::try_from(number_val).is_err());
    }

    #[test]
    fn test_char_at() {
        let s = StringValue::from("Hello");

        assert_eq!(s.char_at(0).unwrap(), 'H');
        assert_eq!(s.char_at(4).unwrap(), 'o');
        assert!(s.char_at(10).is_err());
    }

    #[test]
    fn test_parse() {
        let s = StringValue::from("42");
        let parsed: i32 = s.parse().unwrap();
        assert_eq!(parsed, 42);

        let s_invalid = StringValue::from("not_a_number");
        let result: ValueResult<i32> = s_invalid.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_error_handling() {
        let s = StringValue::from("test");

        // Index out of bounds
        match s.char_at(10) {
            Err(ValueError::IndexOutOfBounds { index, length }) => {
                assert_eq!(index, 10);
                assert_eq!(length, 4);
            }
            _ => panic!("Expected IndexOutOfBounds error"),
        }

        // Invalid substring range
        match s.substring(3, 1) {
            Err(ValueError::Custom { message }) => {
                assert!(message.contains("Invalid substring range"));
            }
            _ => panic!("Expected Custom error"),
        }
    }

    #[test]
    fn test_from_iterator() {
        let chars = vec!['h', 'e', 'l', 'l', 'o'];
        let s: StringValue = chars.into_iter().collect();
        assert_eq!(s.as_str(), "hello");

        let strings = vec!["hello", " ", "world"];
        let s2: StringValue = strings.into_iter().collect();
        assert_eq!(s2.as_str(), "hello world");
    }
}