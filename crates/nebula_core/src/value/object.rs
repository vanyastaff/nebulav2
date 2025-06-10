use crate::value::Value;
use derive_more::{Deref, DerefMut, From, Into};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Object value type for key-value collections
#[derive(
    Debug, Clone, PartialEq,
    Serialize, Deserialize, From, Into, Deref, DerefMut
)]
pub struct ObjectValue(IndexMap<String, Value>);

impl ObjectValue {
    /// Creates a new object value
    pub fn new(map: IndexMap<String, Value>) -> Self {
        Self(map)
    }
}

impl Default for ObjectValue {
    fn default() -> Self {
        Self(IndexMap::new())
    }
}
impl FromIterator<(String, Value)> for ObjectValue {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Extend<(String, Value)> for ObjectValue {
    fn extend<T: IntoIterator<Item = (String, Value)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl Into<serde_json::Value> for ObjectValue {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Object(
            self.0.into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect()
        )
    }
}