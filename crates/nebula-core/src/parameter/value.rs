use serde::{Deserialize, Serialize};
use nebula_value::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterValue(pub Value);

impl ParameterValue {
    pub fn new(value: impl Into<Value>) -> Self {
        Self(value.into())
    }
}
