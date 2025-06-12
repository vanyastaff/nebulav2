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



#[derive(Resouce]
pub struct TelegramResource {...}

#[derive(Parameters)]
pub struct TelegramSendMessage {
    #[text(requred)]
    pub chat_id: String,
}

#[derive(Actioon)]
#[action(name = "Send Telegram Message", key = "send_telegram_message", decscription = "Sends a message to a Telegram chat")]
pub struct SendTelegramMessageAction;

impl Plugin for SendTelegramMessageAction {
    build() {
        ctx.add_rescources(TelegramResource)
    }
}

impl Action for SendTelegramMessageAction {
    type Input = TelegramSendMessage;
    type Output = ();

    async fn execute<C>(
        &self,
        context: &C,
        input: Self::Input,
    ) -> Result<engine::ActionResult<Self::Output>, engine::ActionError>
    where
        C: engine::ProcessContext + Send + Sync,
    {
        let telegram = context.get_resource::<TelegramResource>()?;
        telegram.send_message(input.chat_id, "Hello from Nebula!").await?;
        Ok(engine::ActionResult::Value(()))
    }

    fn supports_rollback(&self) -> bool {
        false
    }
}

#[derive(Node)]
pub struct SendTelegramMessageNode;


impl Node for SendTelegramMessageNode {
    actions() -> Vec<Box<dyn Action>> {
        vec![Box::new(SendTelegramMessageAction)]
    }
}


impl Plugin for SendTelegramMessageNode {
    fn build(&self, ctx: &mut PluginContext) {
        ctx.add_plugins(TelegramPlugin);
    }
}

#[derive(Plugin)]
pub struct TelegramPlugin;
impl Plugin for TelegramPlugin {
    fn build(&self, ctx: &mut PluginContext) {
        ctx.add_plugin(LoginPlugin);
        ctx.add_systems(Fatal, send_telemetry);
    }
}