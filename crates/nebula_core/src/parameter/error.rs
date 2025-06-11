use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParameterError {
    /// Build error (e.g., during configuration struct building).
    #[error("Build error: {0}")]
    BuildError(#[from] derive_builder::UninitializedFieldError),
}