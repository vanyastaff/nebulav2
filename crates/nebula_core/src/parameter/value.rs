use crate::value::Value;
use derive_more::{Deref, DerefMut, From};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Deref, DerefMut, From)]
pub struct ParameterValue(pub Value);

impl ParameterValue {
    pub fn new(value: impl Into<Value>) -> Self {
        Self(value.into())
    }
}
