use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::types::ParameterKey;
use super::ParameterError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "owned",
    setter(strip_option, into),
    build_fn(error = "ParameterError")
)]
pub struct ParameterMetadata {
    #[builder(
        setter(strip_option, into),
        field(ty = "String", build = "ParameterKey::new(self.key.clone())?")
    )]
    pub key: ParameterKey,
    pub name: String,
    pub required: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

impl ParameterMetadata {
    pub fn new(key: ParameterKey, name: impl Into<String>) -> Self {
        ParameterMetadata {
            key,
            name: name.into(),
            required: false,
            description: None,
            placeholder: None,
            hint: None,
        }
    }
    pub fn builder() -> ParameterMetadataBuilder {
        ParameterMetadataBuilder::default()
    }
}