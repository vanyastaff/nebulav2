use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::parameter::ParameterError;
use crate::types::ParameterKey;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
#[builder(
    pattern = "owned",
    setter(strip_option, into),
    build_fn(error = "ParameterError")
)]
pub struct ParameterOption {
    #[builder(
        setter(into),
        field(ty = "String", build = "ParameterKey::new(self.key.clone())?")
    )]
    pub key: ParameterKey,

    pub name: String,

    pub value: ParameterOptionValue,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ParameterOptionValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl ParameterOption {
    pub fn builder() -> ParameterOptionBuilder {
        ParameterOptionBuilder::default()
    }

    pub fn simple(
        key: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<ParameterOptionValue>
    ) -> Result<Self, ParameterError> {
        Self::builder()
            .key(key)
            .name(name)
            .value(value)
            .build()
    }

    pub fn with_description(
        key: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<ParameterOptionValue>,
        description: impl Into<String>
    ) -> Result<Self, ParameterError> {
        Self::builder()
            .key(key)
            .name(name)
            .value(value)
            .description(description)
            .build()
    }

    pub fn grouped(
        key: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<ParameterOptionValue>,
        group: impl Into<String>
    ) -> Result<Self, ParameterError> {
        Self::builder()
            .key(key)
            .name(name)
            .value(value)
            .group(group)
            .build()
    }

    pub fn rich(
        key: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<ParameterOptionValue>
    ) -> ParameterOptionBuilder {
        ParameterOptionBuilder::default()
            .key(key)
            .name(name)
            .value(value)
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled.unwrap_or(false)
    }

    pub fn display_value(&self) -> String {
        match &self.value {
            ParameterOptionValue::String(s) => s.clone(),
            ParameterOptionValue::Number(n) => n.to_string(),
            ParameterOptionValue::Boolean(b) => b.to_string(),
        }
    }

    pub fn in_group(&self, group_name: &str) -> bool {
        self.group.as_ref().map_or(false, |g| g == group_name)
    }
}

impl From<&str> for ParameterOptionValue {
    fn from(value: &str) -> Self {
        ParameterOptionValue::String(value.to_string())
    }
}

impl From<String> for ParameterOptionValue {
    fn from(value: String) -> Self {
        ParameterOptionValue::String(value)
    }
}

impl From<f64> for ParameterOptionValue {
    fn from(value: f64) -> Self {
        ParameterOptionValue::Number(value)
    }
}

impl From<i32> for ParameterOptionValue {
    fn from(value: i32) -> Self {
        ParameterOptionValue::Number(value as f64)
    }
}

impl From<i64> for ParameterOptionValue {
    fn from(value: i64) -> Self {
        ParameterOptionValue::Number(value as f64)
    }
}

impl From<bool> for ParameterOptionValue {
    fn from(value: bool) -> Self {
        ParameterOptionValue::Boolean(value)
    }
}

impl PartialEq<str> for ParameterOptionValue {
    fn eq(&self, other: &str) -> bool {
        match self {
            ParameterOptionValue::String(s) => s == other,
            _ => false,
        }
    }
}

impl PartialEq<f64> for ParameterOptionValue {
    fn eq(&self, other: &f64) -> bool {
        match self {
            ParameterOptionValue::Number(n) => n == other,
            _ => false,
        }
    }
}

impl PartialEq<bool> for ParameterOptionValue {
    fn eq(&self, other: &bool) -> bool {
        match self {
            ParameterOptionValue::Boolean(b) => b == other,
            _ => false,
        }
    }
}

#[macro_export]
macro_rules! option {
    ($key:expr, $name:expr, $value:expr) => {
        ParameterOption::simple($key, $name, $value)
    };
    ($key:expr, $name:expr, $value:expr, $desc:expr) => {
        ParameterOption::with_description($key, $name, $value, $desc)
    };
}

#[macro_export]
macro_rules! options {
    [$(($key:expr, $name:expr, $value:expr)),* $(,)?] => {
        vec![
            $(ParameterOption::simple($key, $name, $value).unwrap()),*
        ]
    };
}