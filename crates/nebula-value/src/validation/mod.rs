pub mod errors;
mod impls;
mod builder;
mod operator;
mod context;
mod presets;

pub use impls::*;
pub use builder::*;
pub use operator::*;
pub use context::*;
pub use errors::*;
pub use presets::*;


pub trait Validatable {
    /// Validates the value using the given operator and context
    fn validate(&self, operator: &ValidationOperator, context: &ValidatorContext) -> ValidationResult;
}
