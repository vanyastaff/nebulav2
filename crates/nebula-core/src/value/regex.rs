// nebula_core/src/value/regex.rs

use super::ValueError;
use derive_more::{Deref, DerefMut};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deref, DerefMut, Serialize, Deserialize)]
pub struct RegexValue(#[serde(with = "serde_regex")] Regex);

impl RegexValue {
    pub fn new(pattern: impl AsRef<str>) -> Result<Self, ValueError> {
        let pattern_str = pattern.as_ref();
        let regex = Regex::new(pattern_str)
            .map_err(|e| ValueError::invalid_regex(pattern_str, e.to_string()))?;

        Ok(Self(regex))
    }

    pub fn pattern(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq for RegexValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for RegexValue {}

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
