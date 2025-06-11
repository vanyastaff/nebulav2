use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::{clone_trait_object, DynClone};
use std::any::Any;
use std::fmt::Debug;
use crate::parameter::ParameterKind;
use crate::types::ParameterKey;

pub trait TypedParameter {
    fn kind(&self) -> ParameterKind;
}

#[typetag::serde(tag = "type")]
pub trait Parameter: TypedParameter + DynClone + Downcast + Any + Debug + Send + Sync {
    fn metadata(&self) -> &ParameterMetadata;

    fn name(&self) -> &str {
        &self.metadata().name
    }

    fn key(&self) -> &ParameterKey {
        &self.metadata().key
    }

    fn get_value(&self) -> Option<&ParameterValue> {
        None
    }

    fn set_value(&mut self, value: ParameterValue) -> Result<(), ParameterError> {
        if !self.type_id().is_editable() {
            return Err(ParameterError::NotEditable(self.key().clone()));
        }
        Ok(())
    }

    fn validation_rules(&self) -> Option<&ParameterValidation> {
        None
    }

    fn display_conditions(&self) -> Option<&ParameterDisplay> {
        None
    }

    fn validate(&self) -> Result<(), ValidationError> {
        if !self.kind().is_validatable() {
            return Ok(());
        }

        if let (Some(value), Some(rules)) = (self.get_value(), self.validation_rules()) {
            rules.validate(value)?;
        }

        Ok(())
    }

    fn should_display(&self, context: &DisplayContext) -> bool {
        if !self.kind().is_displayable() {
            return false;
        }

        if let Some(display) = self.display_conditions() {
            return display.evaluate(context);
        }

        true // По умолчанию показываем
    }
}

impl_downcast!(Parameter);
clone_trait_object!(Parameter);

