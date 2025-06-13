use super::Value;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupValue(IndexMap<String, Value>);

impl From<HashMap<String, Value>> for GroupValue {
    fn from(map: HashMap<String, Value>) -> Self {
        Self(IndexMap::from(map))
    }
}

impl Display for GroupValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[group][{} items]", self.keys().len())
    }
}

impl From<IndexMap<String, Value>> for GroupValue {
    fn from(map: IndexMap<String, Value>) -> Self {
        Self(map)
    }
}

impl FromIterator<(String, Value)> for GroupValue {
    fn from_iter<T: IntoIterator<Item = (String, Value)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Into<serde_json::Value> for GroupValue {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Object(self.0.into_iter().map(|(k, v)| (k, v.into())).collect())
    }
}
