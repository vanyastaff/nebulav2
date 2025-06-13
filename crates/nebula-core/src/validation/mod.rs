//! # Validation System
//!
//! Comprehensive validation system with type-safe operators and specific error types.
//!
//! ## Features
//!
//! - **Type-safe operators**: Compile-time validation of operator usage
//! - **Specific error types**: Detailed error information for each validation type
//! - **Composable validators**: Combine multiple validation rules
//! - **Cross-field validation**: Validate relationships between fields
//! - **Conditional validation**: Validate based on other field values
//! - **Preset validators**: Common validation patterns (email, URL, etc.)
//!
//! ## Quick Start
//!
//! ```rust
//! use nebula_core::validation::{ValidationOperator, ValidationError};
//!
//! // Basic validation
//! let validator = ValidationOperator::and(vec![
//!     ValidationOperator::MinLength(3),
//!     ValidationOperator::MaxLength(50),
//!     ValidationOperator::Matches("^[a-zA-Z0-9_]+$".to_string()),
//! ]);
//!
//! // Preset validators
//! let email_validator = ValidationOperator::IsEmail;
//! let url_validator = ValidationOperator::IsUrl;
//! ```

// Re-exports
pub mod error;
pub mod operator;
pub mod builder;
pub mod preset;
pub mod impls;
pub mod integration;

// Public API exports
pub use operator::ValidationOperator;
pub use error::{
    ValidationError,
    ComparisonError,
    StringError,
    RegexError,
    SetError,
    RangeError,
    EmptinessError,
    CrossFieldError,
    ConditionalError,
    LogicalError,
};
pub use builder::ValidationBuilder;
pub use preset::Presets;
pub use integration::{ValidatorContext, ValidationResult};

// Convenience re-exports
pub use crate::parameter::{ParameterValue, ParameterKey};
pub use crate::value::Value;