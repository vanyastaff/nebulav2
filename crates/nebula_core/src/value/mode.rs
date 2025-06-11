use super::StringValue;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use typetag::serde;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
pub struct ModeValue {
    pub mode: String,
    pub value: StringValue,
}

impl ModeValue {
    pub fn new(mode: impl Into<String>, value: impl Into<StringValue>) -> Self {
        Self {
            mode: mode.into(),
            value: value.into(),
        }
    }

    pub fn mode(&self) -> &str {
        &self.mode
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Deref for ModeValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Into<serde_json::Value> for ModeValue {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Object(serde_json::Map::from_iter([
            ("mode".to_string(), self.mode.into()),
            ("value".to_string(), self.value.into()),
        ]))
    }
}
