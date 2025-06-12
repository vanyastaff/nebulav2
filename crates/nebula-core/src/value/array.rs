use crate::value::Value;
use serde::{Deserialize, Serialize};

/// Array value type for collections of values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayValue(Vec<Value>);

impl ArrayValue {
    /// Creates a new array value
    pub fn new(values: impl Into<Vec<Value>>) -> Self {
        Self(values.into())
    }
}

impl Default for ArrayValue {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> From<Vec<T>> for ArrayValue
where
    T: Into<Value>,
{
    fn from(values: Vec<T>) -> Self {
        values.into_iter().map(Into::into).collect()
    }
}
impl FromIterator<Value> for ArrayValue {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Extend<Value> for ArrayValue {
    fn extend<T: IntoIterator<Item = Value>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl std::ops::Index<usize> for ArrayValue {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for ArrayValue {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Into<serde_json::Value> for ArrayValue {
    fn into(self) -> serde_json::Value {
        serde_json::Value::Array(self.0.into_iter().map(|value| value.into()).collect())
    }
}
