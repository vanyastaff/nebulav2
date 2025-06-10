use derive_more::{
    AsRef, Deref, DerefMut, Display,
    From, FromStr, Into
};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash,
    Serialize, Deserialize, From, Into, AsRef, Deref, DerefMut,
    Display, FromStr
)]
pub struct StringValue(String);

impl StringValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Into<serde_json::Value> for StringValue {
    fn into(self) -> serde_json::Value {
        serde_json::Value::String(self.0)
    }
}