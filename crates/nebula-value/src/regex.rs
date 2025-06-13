use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

use regex::{Captures, Regex};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{ValueError, ValueResult};

/// Regular expression value with rich pattern matching functionality
#[derive(Debug, Clone)]
pub struct RegexValue {
    pattern: String,
    compiled: Regex,
}

// Custom serialization implementation (feature-gated)
#[cfg(feature = "serde")]
impl Serialize for RegexValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        self.pattern.serialize(serializer)
    }
}

// Custom deserialization implementation (feature-gated)
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for RegexValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let pattern = String::deserialize(deserializer)?;
        RegexValue::new(pattern).map_err(serde::de::Error::custom)
    }
}

impl RegexValue {
    // === Construction ===

    /// Creates a new regex from a pattern string
    pub fn new(pattern: impl AsRef<str>) -> ValueResult<Self> {
        let pattern_str = pattern.as_ref().to_string();
        let compiled = Regex::new(&pattern_str)
            .map_err(|e| ValueError::custom(format!("Invalid regex '{pattern_str}': {e}")))?;

        Ok(Self { pattern: pattern_str, compiled })
    }

    /// Creates a regex with case-insensitive matching
    pub fn new_case_insensitive(pattern: impl AsRef<str>) -> ValueResult<Self> {
        let pattern_str = pattern.as_ref();
        let case_insensitive_pattern = format!("(?i){pattern_str}");
        Self::new(case_insensitive_pattern)
    }

    /// Creates a regex with multiline mode (^ and $ match line boundaries)
    pub fn new_multiline(pattern: impl AsRef<str>) -> ValueResult<Self> {
        let pattern_str = pattern.as_ref();
        let multiline_pattern = format!("(?m){pattern_str}");
        Self::new(multiline_pattern)
    }

    /// Creates a regex with dot-all mode (. matches newlines)
    pub fn new_dotall(pattern: impl AsRef<str>) -> ValueResult<Self> {
        let pattern_str = pattern.as_ref();
        let dotall_pattern = format!("(?s){pattern_str}");
        Self::new(dotall_pattern)
    }

    /// Creates a regex with extended mode (ignore whitespace and allow
    /// comments)
    pub fn new_extended(pattern: impl AsRef<str>) -> ValueResult<Self> {
        let pattern_str = pattern.as_ref();
        let extended_pattern = format!("(?x){pattern_str}");
        Self::new(extended_pattern)
    }

    /// Creates a regex with combined flags
    pub fn new_with_flags(
        pattern: impl AsRef<str>,
        case_insensitive: bool,
        multiline: bool,
        dotall: bool,
        extended: bool,
    ) -> ValueResult<Self> {
        let pattern_str = pattern.as_ref();
        let mut flags = String::new();

        if case_insensitive {
            flags.push('i');
        }
        if multiline {
            flags.push('m');
        }
        if dotall {
            flags.push('s');
        }
        if extended {
            flags.push('x');
        }

        let full_pattern = if flags.is_empty() {
            pattern_str.to_string()
        } else {
            format!("(?{flags}){pattern_str}")
        };

        Self::new(full_pattern)
    }

    // === Common Patterns ===

    /// Creates a regex for matching email addresses
    #[must_use]
    pub fn email() -> Self {
        // Simplified but practical email regex
        Self::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    }

    /// Creates a regex for matching URLs
    #[must_use]
    pub fn url() -> Self {
        Self::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap()
    }

    /// Creates a regex for matching IPv4 addresses
    #[must_use]
    pub fn ipv4() -> Self {
        Self::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap()
    }

    /// Creates a regex for matching UUID v4
    #[must_use]
    pub fn uuid_v4() -> Self {
        Self::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap()
    }

    /// Creates a regex for matching phone numbers (US format)
    #[must_use]
    pub fn phone_us() -> Self {
        Self::new(r"^\+?1?[-.\s]?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})$").unwrap()
    }

    /// Creates a regex for matching hexadecimal colors
    #[must_use]
    pub fn hex_color() -> Self {
        Self::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap()
    }

    /// Creates a regex for matching integers
    #[must_use]
    pub fn integer() -> Self {
        Self::new(r"^-?\d+$").unwrap()
    }

    /// Creates a regex for matching floating-point numbers
    #[must_use]
    pub fn float() -> Self {
        Self::new(r"^-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?$").unwrap()
    }

    // === Pattern Information ===

    /// Returns the pattern string
    #[inline]
    #[must_use]
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Returns the number of capture groups in the pattern
    #[inline]
    #[must_use]
    pub fn capture_groups(&self) -> usize {
        self.compiled.captures_len()
    }

    /// Returns the capture group names
    #[must_use]
    pub fn capture_names(&self) -> Vec<Option<&str>> {
        self.compiled.capture_names().collect()
    }

    /// Returns whether the pattern has named groups
    #[must_use]
    pub fn has_named_groups(&self) -> bool {
        self.compiled.capture_names().any(|name| name.is_some())
    }

    // === Matching Operations ===

    /// Tests if the regex matches the given text
    #[inline]
    #[must_use]
    pub fn is_match(&self, text: &str) -> bool {
        self.compiled.is_match(text)
    }

    /// Finds the first match in the text
    #[must_use]
    pub fn find(&self, text: &str) -> Option<RegexMatch> {
        self.compiled.find(text).map(|m| RegexMatch {
            text: m.as_str().to_string(),
            start: m.start(),
            end: m.end(),
        })
    }

    /// Finds all matches in the text
    #[must_use]
    pub fn find_all(&self, text: &str) -> Vec<RegexMatch> {
        self.compiled
            .find_iter(text)
            .map(|m| RegexMatch { text: m.as_str().to_string(), start: m.start(), end: m.end() })
            .collect()
    }

    /// Captures the first match with groups
    #[must_use]
    pub fn captures(&self, text: &str) -> Option<RegexCaptures> {
        self.compiled.captures(text).map(|caps| self.build_regex_captures(&caps))
    }

    /// Captures all matches with groups
    #[must_use]
    pub fn captures_all(&self, text: &str) -> Vec<RegexCaptures> {
        self.compiled.captures_iter(text).map(|caps| self.build_regex_captures(&caps)).collect()
    }

    // === Helper Methods ===

    /// Build RegexCaptures from Captures (reduces code duplication)
    fn build_regex_captures(&self, caps: &Captures) -> RegexCaptures {
        let groups: Vec<Option<String>> =
            caps.iter().map(|m| m.map(|m| m.as_str().to_string())).collect();

        let named_groups: std::collections::HashMap<String, String> = caps
            .iter()
            .enumerate()
            .filter_map(|(i, m)| {
                if let (Some(name), Some(mat)) = (self.compiled.capture_names().nth(i).flatten(), m)
                {
                    Some((name.to_string(), mat.as_str().to_string()))
                } else {
                    None
                }
            })
            .collect();

        RegexCaptures {
            full_match: caps.get(0).map(|m| m.as_str().to_string()).unwrap_or_default(),
            groups,
            named_groups,
        }
    }

    // === Replacement Operations ===

    /// Replace the first match with the given replacement
    #[must_use]
    pub fn replace(&self, text: &str, replacement: &str) -> String {
        self.compiled.replace(text, replacement).to_string()
    }

    /// Replace all matches with the given replacement
    #[must_use]
    pub fn replace_all(&self, text: &str, replacement: &str) -> String {
        self.compiled.replace_all(text, replacement).to_string()
    }

    /// Replace matches using a closure
    pub fn replace_all_with<F>(&self, text: &str, replacer: F) -> String
    where F: Fn(&RegexCaptures) -> String {
        let mut result = String::new();
        let mut last_end = 0;

        for caps in self.captures_all(text) {
            if let Some(m) = self.compiled.find(&text[last_end..]) {
                let actual_start = last_end + m.start();
                let actual_end = last_end + m.end();

                result.push_str(&text[last_end..actual_start]);
                result.push_str(&replacer(&caps));
                last_end = actual_end;
            }
        }

        result.push_str(&text[last_end..]);
        result
    }

    // === Text Splitting ===

    /// Split text by the regex pattern
    #[must_use]
    pub fn split(&self, text: &str) -> Vec<String> {
        self.compiled.split(text).map(|s| s.to_string()).collect()
    }

    /// Split text by the regex pattern with a limit
    #[must_use]
    pub fn splitn(&self, text: &str, limit: usize) -> Vec<String> {
        self.compiled.splitn(text, limit).map(|s| s.to_string()).collect()
    }

    // === Validation Methods ===

    /// Check if the pattern is simple (no special regex characters)
    #[must_use]
    pub fn is_simple_pattern(&self) -> bool {
        let special_chars = r"\.^$*+?{}[]|()\";
        !self.pattern().chars().any(|c| special_chars.contains(c))
    }

    /// Check if the pattern contains anchors (^ or $)
    #[must_use]
    pub fn has_anchors(&self) -> bool {
        let pattern = self.pattern();
        pattern.contains('^') || pattern.contains('$')
    }

    /// Check if the pattern is likely to be expensive to evaluate
    #[must_use]
    pub fn is_potentially_expensive(&self) -> bool {
        let pattern = self.pattern();
        // Look for patterns that might cause exponential backtracking
        pattern.contains(".*.*")
            || pattern.contains(".+.+")
            || pattern.contains("(.*)+")
            || pattern.contains("(.+)+")
            || pattern.contains("(a|a)*")
            || pattern.matches('*').count() > 3
            || pattern.matches('+').count() > 3
    }

    // === Utility Methods ===

    /// Escape special regex characters in a string
    #[must_use]
    pub fn escape(text: &str) -> String {
        regex::escape(text)
    }

    /// Convert to a case-insensitive version
    pub fn to_case_insensitive(&self) -> ValueResult<Self> {
        if self.pattern().starts_with("(?i)") {
            Ok(self.clone())
        } else {
            Self::new_case_insensitive(self.pattern())
        }
    }

    /// Test the regex against multiple test strings
    #[must_use]
    pub fn test_against(&self, test_strings: &[&str]) -> Vec<(String, bool)> {
        test_strings.iter().map(|&s| (s.to_string(), self.is_match(s))).collect()
    }

    /// Get the underlying Regex object
    #[inline]
    #[must_use]
    pub fn as_regex(&self) -> &Regex {
        &self.compiled
    }
}

// === Helper Structures ===

/// Represents a regex match with position information
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegexMatch {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

impl RegexMatch {
    #[must_use]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// Represents captured groups from a regex match
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegexCaptures {
    pub full_match: String,
    pub groups: Vec<Option<String>>,
    pub named_groups: std::collections::HashMap<String, String>,
}

impl RegexCaptures {
    /// Get a capture group by index
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&String> {
        self.groups.get(index)?.as_ref()
    }

    /// Get a capture group by name
    #[must_use]
    pub fn get_named(&self, name: &str) -> Option<&String> {
        self.named_groups.get(name)
    }

    /// Get the number of capture groups
    #[must_use]
    pub fn len(&self) -> usize {
        self.groups.len()
    }

    /// Check if there are no capture groups
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }
}

// === Trait Implementations ===

impl PartialEq for RegexValue {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}

impl Eq for RegexValue {}

impl fmt::Display for RegexValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Pretty format with additional info
            write!(f, "/{}/", self.pattern())
        } else {
            // Just the pattern
            write!(f, "{}", self.pattern())
        }
    }
}

impl FromStr for RegexValue {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<&str> for RegexValue {
    type Error = ValueError;

    fn try_from(pattern: &str) -> Result<Self, Self::Error> {
        Self::new(pattern)
    }
}

impl TryFrom<String> for RegexValue {
    type Error = ValueError;

    fn try_from(pattern: String) -> Result<Self, Self::Error> {
        Self::new(pattern)
    }
}

impl Hash for RegexValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pattern.hash(state);
    }
}

// === JSON Conversion (feature-gated) ===

#[cfg(feature = "json")]
impl From<RegexValue> for serde_json::Value {
    fn from(regex: RegexValue) -> Self {
        serde_json::Value::String(regex.pattern().to_string())
    }
}

#[cfg(feature = "json")]
impl TryFrom<serde_json::Value> for RegexValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::String(pattern) => Self::new(pattern),
            other => Err(ValueError::custom(format!(
                "Cannot convert {:?} to RegexValue, expected string pattern",
                other
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let regex = RegexValue::new(r"\d+").unwrap();
        assert_eq!(regex.pattern(), r"\d+");

        let email_regex = RegexValue::email();
        assert!(email_regex.is_match("test@example.com"));
    }

    #[test]
    fn test_matching() {
        let regex = RegexValue::new(r"\d+").unwrap();

        assert!(regex.is_match("123"));
        assert!(!regex.is_match("abc"));

        let matches = regex.find_all("abc 123 def 456");
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].text, "123");
        assert_eq!(matches[1].text, "456");
    }

    #[test]
    fn test_captures() {
        let regex = RegexValue::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        let caps = regex.captures("Date: 2024-01-15").unwrap();

        assert_eq!(caps.full_match, "2024-01-15");
        assert_eq!(caps.get(1).unwrap(), "2024");
        assert_eq!(caps.get(2).unwrap(), "01");
        assert_eq!(caps.get(3).unwrap(), "15");
    }

    #[test]
    fn test_replacement() {
        let regex = RegexValue::new(r"\d+").unwrap();
        let result = regex.replace_all("abc 123 def 456", "XXX");
        assert_eq!(result, "abc XXX def XXX");
    }

    #[test]
    fn test_splitting() {
        let regex = RegexValue::new(r"\s+").unwrap();
        let parts = regex.split("hello    world  rust");
        assert_eq!(parts, vec!["hello", "world", "rust"]);
    }

    #[test]
    fn test_validation() {
        let simple = RegexValue::new("hello").unwrap();
        assert!(simple.is_simple_pattern());

        let complex = RegexValue::new(r"\d+.*").unwrap();
        assert!(!complex.is_simple_pattern());

        let expensive = RegexValue::new(r"(.*)+").unwrap();
        assert!(expensive.is_potentially_expensive());
    }

    #[test]
    fn test_common_patterns() {
        let email = RegexValue::email();
        assert!(email.is_match("user@example.com"));
        assert!(!email.is_match("invalid-email"));

        let uuid = RegexValue::uuid_v4();
        assert!(uuid.is_match("550e8400-e29b-41d4-a716-446655440000"));
        assert!(!uuid.is_match("not-a-uuid"));
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_json_conversion() {
        let regex = RegexValue::new(r"\d+").unwrap();
        let json: serde_json::Value = regex.into();
        assert_eq!(json, serde_json::Value::String(r"\d+".to_string()));

        let parsed: RegexValue = serde_json::Value::String(r"\w+".to_string()).try_into().unwrap();
        assert_eq!(parsed.pattern(), r"\w+");
    }

    #[test]
    fn test_flags() {
        let case_insensitive = RegexValue::new_case_insensitive("hello").unwrap();
        assert!(case_insensitive.is_match("HELLO"));
        assert!(case_insensitive.is_match("hello"));

        let multiline = RegexValue::new_multiline("^test").unwrap();
        assert!(multiline.is_match("line1\ntest"));

        let combined = RegexValue::new_with_flags("hello", true, false, false, false).unwrap();
        assert!(combined.is_match("HELLO"));
    }

    #[test]
    fn test_captures_with_names() {
        let regex = RegexValue::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
        let caps = regex.captures("Date: 2024-01-15").unwrap();

        assert_eq!(caps.get_named("year").unwrap(), "2024");
        assert_eq!(caps.get_named("month").unwrap(), "01");
        assert_eq!(caps.get_named("day").unwrap(), "15");
        assert!(regex.has_named_groups());
    }

    #[test]
    fn test_utility_methods() {
        let escaped = RegexValue::escape("Hello. World? (Test)");
        assert_eq!(escaped, r"Hello\. World\? \(Test\)");

        let regex = RegexValue::new("hello").unwrap();
        let case_insensitive = regex.to_case_insensitive().unwrap();
        assert!(case_insensitive.is_match("HELLO"));

        let test_results = regex.test_against(&["hello", "world", "hello world"]);
        assert_eq!(test_results.len(), 3);
        assert!(test_results[0].1); // "hello" matches
        assert!(!test_results[1].1); // "world" doesn't match
        assert!(test_results[2].1); // "hello world" matches
    }
}
