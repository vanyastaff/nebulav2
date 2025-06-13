use crate::parameter::value::ParameterValue;
use crate::parameter::{ParameterDisplay, ParameterError, ParameterKind, ParameterMetadata, ParameterValidation, ValidationError};
use crate::types::ParameterKey;
use downcast_rs::{impl_downcast, Downcast};
use dyn_clone::{clone_trait_object, DynClone};
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

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

    fn set_value(&mut self, _value: ParameterValue) -> Result<(), ParameterError>;

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
        

        Ok(())
    }

    fn should_display(&self, values: &HashMap<ParameterKey, ParameterValue>) -> bool {
        if !self.kind().is_displayable() {
            return false;
        }



        true // По умолчанию показываем
    }
}

impl_downcast!(Parameter);
clone_trait_object!(Parameter);


#[derive(Parameters)]
struct User {
    #[text(name = "First_name", key = "fisrtname", required = true)]
    #[validation(min_length = 1, max_length = 255)]
    fisrt_name: String,
    
    
    #[text(name = "Lastname", key = "lastname", required = true)]
    #[validation(min_length = 1, max_length = 255)]
    last_name: String,
    
    #[text(name = "Fullname", key = "fullname", required = false)]
    #[validation(min_length = 1, max_length = 255)]
    full_name: Option<String>,
}

impl User {
    pub fn new(fisrt_name: String, last_name: String) -> Self {} // Dont alllow any constructors
    
    fn full_name(&self) -> String { // ALlOOW
        format!("{} {}", self.fisrt_name, self.last_name)
    }
    
    fn reverse_name(&self) -> String {
        format!("{} {}", self.last_name, self.fisrt_name)
    }
}


async fn execute(&self, context: &C, input: Self::Input) -> Result<...> {
    input.full_name = "dsfsdfs"; // dont allow chenge input parameter directly
    let mut request = input.clone(); // IS OK
}


