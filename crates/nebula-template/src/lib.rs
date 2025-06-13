//! # Nebula Template Engine
//!
//! A powerful, type-safe template engine for workflow automation and data transformation.
//!
//! ## Features
//!
//! - 🚀 **High Performance** - Zero-copy parsing and efficient evaluation
//! - 🔒 **Type Safety** - Compile-time function signature validation
//! - 🔄 **Pipeline Operations** - Chain functions with `|` operator
//! - 📊 **Multiple Data Sources** - Access various data contexts
//! - 🎯 **Rich Function Library** - String, array, math, date operations
//! - 🔀 **Control Flow** - Conditionals and loops for complex templates
//!
//! ## Quick Start
//!
//! ```rust
//! use nebula_template::{Template, Context};
//! # #[cfg(feature = "serde")]
//! use serde_json::json;
//!
//! # #[cfg(feature = "serde")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Simple variable substitution
//! let template = Template::parse("Hello {{ $input.name }}!")?;
//! let mut context = Context::new();
//! context.set_input(json!({"name": "Alice"}));
//! let result = template.render(&context)?;
//! assert_eq!(result, "Hello Alice!");
//!
//! // Pipeline operations
//! let template = Template::parse("{{ $input.name | uppercase | default('Anonymous') }}")?;
//! let result = template.render(&context)?;
//! assert_eq!(result, "ALICE");
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "serde"))]
//! # fn main() {}
//! ```
//!
//! ## Expression Syntax
//!
//! ### Data Sources
//!
//! ```text
//! {{ $input.user.name }}              // Current input data
//! {{ $node('user_data').json.email }} // Output from another node
//! {{ $env.API_KEY }}                  // Environment variables
//! {{ $system.datetime.now }}          // System data (time, date)
//! {{ $execution.id }}                 // Execution metadata
//! {{ $workflow.version }}             // Workflow information
//! ```
//!
//! ### Pipeline Operations
//!
//! ```text
//! {{ $input.text | trim | uppercase | default('N/A') }}
//! {{ $input.items | pluck('name') | join(', ') }}
//! {{ $input.price | multiply(1.2) | round(2) | currency }}
//! ```
//!
//! ### Conditional Logic
//!
//! ```text
//! {{ $input.age >= 18 ? 'Adult' : 'Minor' }}
//! {{ if($input.active, 'Enabled', 'Disabled') }}
//! ```
//!
//! ### Loops
//!
//! ```text
//! {{ foreach item in $input.products }}
//! - {{ item.name }}: {{ item.price | currency }}
//! {{ endforeach }}
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]

// Re-export main types for convenience
pub use context::{Context, DataSource};
pub use error::{Error, Result};
pub use template::Template;
pub use value::Value;

// Core modules
pub mod context;
pub mod error;
pub mod template;
pub mod value;

// Internal modules
mod evaluator;
mod functions;
mod parser;
mod error;
mod value;
mod context;
mod template;

// Re-export function-related types
pub use functions::{Function, FunctionRegistry, FunctionSignature};

// Optional feature re-exports
#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub use serde_json;

/// Template engine version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Prelude module for convenient imports
pub mod prelude {
    //! Common imports for nebula-template usage

    pub use crate::{
        Context, DataSource, Error, Result, Template, Value,
        Function, FunctionRegistry, FunctionSignature,
    };

    #[cfg(feature = "serde")]
    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    pub use serde_json::{json, Value as JsonValue};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_basic_template() -> Result<()> {
        use serde_json::json;

        let template = Template::parse("Hello {{ $input.name }}!")?;
        let mut context = Context::new();
        context.set_input(json!({"name": "World"}));

        let result = template.render(&context)?;
        assert_eq!(result, "Hello World!");

        Ok(())
    }

    #[test]
    fn test_static_template() -> Result<()> {
        let template = Template::parse("Hello World!")?;
        let context = Context::new();

        let result = template.render(&context)?;
        assert_eq!(result, "Hello World!");

        Ok(())
    }

    #[test]
    fn test_empty_template() -> Result<()> {
        let template = Template::parse("")?;
        let context = Context::new();

        let result = template.render(&context)?;
        assert_eq!(result, "");

        Ok(())
    }

    #[test]
    fn test_invalid_syntax() {
        let result = Template::parse("{{ invalid syntax");
        assert!(result.is_err());
    }

    #[test]
    fn test_prelude_imports() {
        use crate::prelude::*;

        // Should be able to use all main types
        let _template: Template;
        let _context: Context;
        let _value: Value;
        let _error: Error;

        #[cfg(feature = "serde")]
        {
            let _json_value = json!({"test": "value"});
        }
    }
}