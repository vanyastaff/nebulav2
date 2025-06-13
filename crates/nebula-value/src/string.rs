use crate::{ValueError, ValueResult};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};
use std::fmt;
use std::ops::{Add, Deref, DerefMut};
use std::str::FromStr;

/// String value type with efficient operations and conversions
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct StringValue(String);

impl StringValue {
    // --- Constructors ---

    /// Creates a new string value from an owned String
    #[inline]
    #[must_use]
    pub const fn new(value: String) -> Self {
        Self(value)
    }

    /// Creates a new string value from anything convertible to String
    #[inline]
    #[must_use]
    pub fn from_value(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Creates an empty string value (const)
    #[inline]
    #[must_use]
    pub const fn empty() -> Self {
        Self(String::new())
    }

    /// Creates a string value with specified capacity
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity))
    }

    // --- Basic Properties ---

    /// Returns true if the string is empty
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the length of the string in bytes
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the length of the string in characters
    #[inline]
    #[must_use]
    pub fn char_count(&self) -> usize {
        self.0.chars().count()
    }

    /// Returns the string as a &str
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts to owned String, consuming self
    #[inline]
    #[must_use = "consuming self usually means its value is needed"]
    pub fn into_string(self) -> String {
        self.0
    }

    // --- Modification (in-place) ---

    /// Appends a string slice to this string
    #[inline]
    pub fn push_str(&mut self, string: &str) {
        self.0.push_str(string);
    }

    /// Appends a character to this string
    #[inline]
    pub fn push(&mut self, ch: char) {
        self.0.push(ch);
    }

    /// Truncates this string to the specified length
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        self.0.truncate(new_len);
    }

    /// Clears the string, removing all contents
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    // --- Transformations (returning new StringValue) ---

    /// Returns a string slice with leading and trailing whitespace removed
    #[inline]
    #[must_use = "the trimmed string slice should be used"]
    pub fn trim(&self) -> &str {
        self.0.trim()
    }

    /// Creates a StringValue with trimmed content
    #[must_use = "the new trimmed string should be used"]
    pub fn trimmed(&self) -> StringValue {
        StringValue::new(self.0.trim().to_string())
    }

    /// Converts to lowercase
    #[must_use = "the new lowercase string should be used"]
    pub fn to_lowercase(&self) -> StringValue {
        StringValue::new(self.0.to_lowercase())
    }

    /// Converts to uppercase
    #[must_use = "the new uppercase string should be used"]
    pub fn to_uppercase(&self) -> StringValue {
        StringValue::new(self.0.to_uppercase())
    }

    /// Capitalizes the first character
    #[must_use]
    pub fn capitalize(&self) -> StringValue {
        let mut chars = self.0.chars();
        match chars.next() {
            None => StringValue::empty(),
            Some(first) => {
                let capitalized = first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase();
                StringValue::new(capitalized)
            }
        }
    }

    // --- Search and Comparison ---

    /// Checks if string contains a pattern
    #[inline]
    #[must_use = "the boolean result should be used"]
    pub fn contains(&self, pattern: &str) -> bool {
        self.0.contains(pattern)
    }

    /// Checks if string starts with a pattern
    #[inline]
    #[must_use = "the boolean result should be used"]
    pub fn starts_with(&self, pattern: &str) -> bool {
        self.0.starts_with(pattern)
    }

    /// Checks if string ends with a pattern
    #[inline]
    #[must_use = "the boolean result should be used"]
    pub fn ends_with(&self, pattern: &str) -> bool {
        self.0.ends_with(pattern)
    }

    /// Finds the first occurrence of a pattern
    #[inline]
    #[must_use]
    pub fn find(&self, pattern: &str) -> Option<usize> {
        self.0.find(pattern)
    }

    /// Finds the last occurrence of a pattern
    #[inline]
    #[must_use]
    pub fn rfind(&self, pattern: &str) -> Option<usize> {
        self.0.rfind(pattern)
    }

    /// Removes prefix if present
    #[must_use = "the optional new string should be used"]
    pub fn strip_prefix(&self, prefix: &str) -> Option<StringValue> {
        self.0
            .strip_prefix(prefix)
            .map(|s| StringValue::new(s.to_string()))
    }

    /// Removes suffix if present
    #[must_use = "the optional new string should be used"]
    pub fn strip_suffix(&self, suffix: &str) -> Option<StringValue> {
        self.0
            .strip_suffix(suffix)
            .map(|s| StringValue::new(s.to_string()))
    }

    /// Replaces all matches of a pattern with another string
    #[must_use = "the new string with replacements should be used"]
    pub fn replace(&self, from: &str, to: &str) -> StringValue {
        StringValue::new(self.0.replace(from, to))
    }

    /// Replaces the first match of a pattern with another string
    #[must_use]
    pub fn replacen(&self, from: &str, to: &str, count: usize) -> StringValue {
        StringValue::new(self.0.replacen(from, to, count))
    }

    // --- Splitting ---

    /// Splits the string by a pattern and returns a vector of StringValues
    #[must_use = "the vector of split strings should be used"]
    pub fn split(&self, pattern: &str) -> Vec<StringValue> {
        self.0
            .split(pattern)
            .map(|s| StringValue::new(s.to_string()))
            .collect()
    }

    /// Splits by whitespace
    #[must_use = "the vector of split strings should be used"]
    pub fn split_whitespace(&self) -> Vec<StringValue> {
        self.0
            .split_whitespace()
            .map(|s| StringValue::new(s.to_string()))
            .collect()
    }

    /// Splits into lines
    #[must_use]
    pub fn lines(&self) -> Vec<StringValue> {
        self.0
            .lines()
            .map(|s| StringValue::new(s.to_string()))
            .collect()
    }

    // --- Substring and Character Access ---

    /// Returns a substring from start to end (character indices)
    ///
    /// Returns an error if indices are out of bounds
    pub fn substring(&self, start: usize, end: usize) -> ValueResult<StringValue> {
        if start > end {
            return Err(ValueError::custom(format!(
                "Invalid substring range: start ({}) > end ({})",
                start, end
            )));
        }

        let char_count = self.char_count();
        if start > char_count || end > char_count {
            return Err(ValueError::custom(format!(
                "Substring indices out of bounds: start={}, end={}, length={}",
                start, end, char_count
            )));
        }

        let mut chars = self.0.char_indices();
        let start_byte = if start == 0 {
            0
        } else {
            chars
                .nth(start)
                .map(|(i, _)| i)
                .unwrap_or(self.0.len())
        };

        let end_byte = if end == char_count {
            self.0.len()
        } else {
            chars
                .nth(end.saturating_sub(start + 1))
                .map(|(i, _)| i)
                .unwrap_or(self.0.len())
        };

        Ok(StringValue::new(self.0[start_byte..end_byte].to_string()))
    }

    /// Returns the first n characters
    pub fn take(&self, n: usize) -> StringValue {
        if n == 0 {
            return StringValue::empty();
        }

        let char_count = self.char_count();
        if n >= char_count {
            return self.clone();
        }

        let mut chars = self.0.char_indices();
        let end_byte = chars.nth(n).map(|(i, _)| i).unwrap_or(self.0.len());
        StringValue::new(self.0[..end_byte].to_string())
    }

    /// Returns all characters after skipping n
    pub fn skip(&self, n: usize) -> StringValue {
        if n == 0 {
            return self.clone();
        }

        let char_count = self.char_count();
        if n >= char_count {
            return StringValue::empty();
        }

        let mut chars = self.0.char_indices();
        let start_byte = chars.nth(n).map(|(i, _)| i).unwrap_or(self.0.len());
        StringValue::new(self.0[start_byte..].to_string())
    }

    /// Safe character access by index
    pub fn char_at(&self, index: usize) -> ValueResult<char> {
        self.0.chars().nth(index).ok_or_else(|| {
            ValueError::custom(format!(
                "Character index {} out of bounds for string of length {}",
                index,
                self.char_count()
            ))
        })
    }

    // --- Parsing and Validation ---

    /// Attempts to parse the string as a specific type
    pub fn parse<T>(&self) -> ValueResult<T>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        self.0
            .parse::<T>()
            .map_err(|e| ValueError::custom(format!("Parse error: {}", e)))
    }

    /// Explicit parse with type annotation (alias for parse)
    #[inline]
    pub fn parse_with<T>(&self) -> ValueResult<T>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        self.parse()
    }

    /// Validates that the string is valid UTF-8 (always true for String)
    #[inline]
    pub fn validate_utf8(&self) -> ValueResult<()> {
        Ok(())
    }

    /// Checks if the string is numeric (can be parsed as a number)
    #[must_use]
    pub fn is_numeric(&self) -> bool {
        self.parse::<f64>().is_ok()
    }

    /// Checks if the string is alphabetic
    #[must_use]
    pub fn is_alphabetic(&self) -> bool {
        !self.0.is_empty() && self.0.chars().all(|c| c.is_alphabetic())
    }

    /// Checks if the string is alphanumeric
    #[must_use]
    pub fn is_alphanumeric(&self) -> bool {
        !self.0.is_empty() && self.0.chars().all(|c| c.is_alphanumeric())
    }

    // --- Advanced Operations ---

    /// Repeats the string n times
    #[must_use]
    pub fn repeat(&self, n: usize) -> StringValue {
        StringValue::new(self.0.repeat(n))
    }

    /// Reverses the string (character-wise)
    #[must_use]
    pub fn reverse(&self) -> StringValue {
        StringValue::new(self.0.chars().rev().collect())
    }

    /// Pads the string to a specified width with spaces
    #[must_use]
    pub fn pad_left(&self, width: usize) -> StringValue {
        if self.char_count() >= width {
            self.clone()
        } else {
            let padding = " ".repeat(width - self.char_count());
            StringValue::new(format!("{}{}", padding, self.0))
        }
    }

    /// Pads the string to a specified width with spaces on the right
    #[must_use]
    pub fn pad_right(&self, width: usize) -> StringValue {
        if self.char_count() >= width {
            self.clone()
        } else {
            let padding = " ".repeat(width - self.char_count());
            StringValue::new(format!("{}{}", self.0, padding))
        }
    }

    /// Centers the string within a specified width
    #[must_use]
    pub fn center(&self, width: usize) -> StringValue {
        let current_len = self.char_count();
        if current_len >= width {
            return self.clone();
        }

        let total_padding = width - current_len;
        let left_padding = total_padding / 2;
        let right_padding = total_padding - left_padding;

        StringValue::new(format!(
            "{}{}{}",
            " ".repeat(left_padding),
            self.0,
            " ".repeat(right_padding)
        ))
    }
}

// --- Trait Implementations ---

impl Deref for StringValue {
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StringValue {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<str> for StringValue {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for StringValue {
    #[inline]
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

    #[inline]
    fn from_str(s: &str) -> ValueResult<Self> {
        Ok(StringValue::new(s.to_string()))
    }
}

// From implementations
impl From<String> for StringValue {
    #[inline]
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for StringValue {
    #[inline]
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

impl From<char> for StringValue {
    #[inline]
    fn from(value: char) -> Self {
        Self::new(value.to_string())
    }
}

impl From<Cow<'_, str>> for StringValue {
    #[inline]
    fn from(value: Cow<'_, str>) -> Self {
        match value {
            Cow::Borrowed(s) => Self::new(s.to_string()),
            Cow::Owned(s) => Self(s),
        }
    }
}

// Into implementations
impl From<StringValue> for String {
    #[inline]
    fn from(value: StringValue) -> Self {
        value.0
    }
}

impl From<StringValue> for Cow<'_, str> {
    #[inline]
    fn from(value: StringValue) -> Self {
        Cow::Owned(value.0)
    }
}

impl Default for StringValue {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl Borrow<str> for StringValue {
    #[inline]
    fn borrow(&self) -> &str {
        &self.0
    }
}

// Comparison traits
impl PartialEq<str> for StringValue {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for StringValue {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for StringValue {
    #[inline]
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

// JSON conversion (feature-gated)
#[cfg(feature = "json")]
impl From<StringValue> for serde_json::Value {
    #[inline]
    fn from(value: StringValue) -> Self {
        serde_json::Value::String(value.0)
    }
}

#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for StringValue {
    type Error = ValueError;

    #[inline]
    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::String(s) => Ok(StringValue::new(s)),
            other => Err(ValueError::custom(format!(
                "Cannot convert {:?} to StringValue",
                other
            ))),
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
        assert_eq!(s.strip_prefix("Hello").unwrap().as_str(), ", world!");
        assert_eq!(s.strip_suffix("world!").unwrap().as_str(), "Hello, ");
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
        assert!(s.substring(3, 1).is_err()); // start > end
    }

    #[test]
    fn test_take_skip() {
        let s = StringValue::from("Hello World");

        assert_eq!(s.take(5).as_str(), "Hello");
        assert_eq!(s.skip(6).as_str(), "World");

        // Edge cases
        assert_eq!(s.take(0).as_str(), "");
        assert_eq!(s.take(100).as_str(), "Hello World");
        assert_eq!(s.skip(0).as_str(), "Hello World");
        assert_eq!(s.skip(100).as_str(), "");
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

    #[cfg(feature = "json")]
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
    fn test_from_iterator() {
        let chars = vec!['h', 'e', 'l', 'l', 'o'];
        let s: StringValue = chars.into_iter().collect();
        assert_eq!(s.as_str(), "hello");

        let strings = vec!["hello", " ", "world"];
        let s2: StringValue = strings.into_iter().collect();
        assert_eq!(s2.as_str(), "hello world");
    }

    #[test]
    fn test_validation() {
        let numeric = StringValue::from("123.45");
        assert!(numeric.is_numeric());

        let alpha = StringValue::from("hello");
        assert!(alpha.is_alphabetic());

        let alnum = StringValue::from("hello123");
        assert!(alnum.is_alphanumeric());
    }

    #[test]
    fn test_advanced_operations() {
        let s = StringValue::from("hello");

        assert_eq!(s.repeat(3).as_str(), "hellohellohello");
        assert_eq!(s.reverse().as_str(), "olleh");
        assert_eq!(s.capitalize().as_str(), "Hello");

        assert_eq!(s.pad_left(10).as_str(), "     hello");
        assert_eq!(s.pad_right(10).as_str(), "hello     ");
        assert_eq!(s.center(11).as_str(), "   hello   ");
    }

    #[test]
    fn test_find_operations() {
        let s = StringValue::from("hello world hello");

        assert_eq!(s.find("hello"), Some(0));
        assert_eq!(s.find("world"), Some(6));
        assert_eq!(s.rfind("hello"), Some(12));
        assert_eq!(s.find("xyz"), None);
    }
}