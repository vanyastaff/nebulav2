//! # nebula-value
//!
//! Value type system for the Nebula workflow engine.
//!
//! This crate provides a rich type system for representing and manipulating
//! values in workflow contexts, with support for validation, conversion,
//! and serialization.
//!
//! ## Core Types
//!
//! The main [`Value`] enum represents all possible value types:
//!
//! - **Primitives**: [`String`](StringValue), [`Number`](NumberValue),
//!   [`Boolean`](BooleanValue)
//! - **Collections**: [`Array`](ArrayValue), [`Object`](ObjectValue)
//! - **Time**: [`DateTime`](DateTimeValue), [`Duration`](DurationValue),
//!   [`Cron`](CronValue)
//! - **Special**: [`Binary`](BinaryValue), [`File`](FileValue),
//!   [`Color`](ColorValue)
//! - **Advanced**: [`Expression`](ExpressionValue), [`Regex`](RegexValue),
//!   [`Group`](GroupValue), [`Mode`](ModeValue)
//!
//! ## Features
//!
//! - `std` (default): Standard library support
//! - `json`: JSON serialization via serde_json
//! - `collections`: Enhanced collection operations via indexmap
//! - `full`: All features enabled
//!
//! ## Examples
//!
//! ```rust
//! use nebula_value::{NumberValue, Value};
//!
//! // Create values
//! let text = Value::string("hello");
//! let number = Value::number(42);
//! let boolean = Value::boolean(true);
//!
//! // Type checking
//! assert!(text.is_string());
//! assert_eq!(text.as_string(), Some("hello"));
//!
//! // Conversion
//! let from_str: Value = "hello".into();
//! let from_int: Value = 42.into();
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(missing_docs)]
#![warn(clippy::all)]
#![deny(unsafe_code)]

// Core modules - always available
pub mod array;
pub mod binary;
pub mod boolean;
pub mod color;
mod comparison;
pub mod cron;
pub mod datetime;
pub mod duration;
pub mod error;
pub mod expression;
pub mod file;
pub mod mode;
pub mod number;
pub mod object;
pub mod regex;
pub mod string;
pub mod value;

// Re-exports - Main API
// Value types
pub use array::ArrayValue;
pub use binary::BinaryValue;
pub use boolean::BooleanValue;
pub use color::ColorValue;
pub use comparison::ValueComparison;
pub use cron::CronValue;
pub use datetime::DateTimeValue;
pub use duration::DurationValue;
// Error types
pub use error::{ValueError, ValueResult};
pub use expression::ExpressionValue;
pub use file::FileValue;
pub use mode::ModeValue;
pub use number::NumberValue;
pub use object::ObjectValue;
pub use regex::RegexValue;
pub use string::StringValue;
pub use value::Value;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        ArrayValue, BinaryValue, BooleanValue, ColorValue, CronValue, DateTimeValue, DurationValue,
        ExpressionValue, FileValue, ModeValue, NumberValue, ObjectValue, RegexValue, StringValue,
        Value, ValueError, ValueResult,
    };
}

// Feature-gated re-exports
#[cfg(all(feature = "json", feature = "serde"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "json", feature = "serde"))))]
pub mod json {
    //! JSON utilities for Value types
    //!
    //! Available when both `json` and `serde` features are enabled.

    pub use serde_json::{Map, from_str, from_value, to_string, to_value, to_vec};

    /// Convert a Value to a serde_json::Value
    pub fn value_to_json(value: &crate::Value) -> serde_json::Result<serde_json::Value> {
        to_value(value)
    }

    /// Convert a serde_json::Value to a Value
    pub fn json_to_value(json: serde_json::Value) -> serde_json::Result<crate::Value> {
        from_value(json)
    }
}

#[cfg(feature = "collections")]
#[cfg_attr(docsrs, doc(cfg(feature = "collections")))]
pub mod collections {
    //! Enhanced collection utilities
    //!
    //! Available when the `collections` feature is enabled.

    pub use indexmap::{IndexMap, IndexSet};

    /// Type alias for ordered object maps
    pub type OrderedMap<K, V> = IndexMap<K, V>;

    /// Type alias for ordered sets
    pub type OrderedSet<T> = IndexSet<T>;
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_creation() {
        let text = Value::string("hello");
        assert!(text.is_string());
        assert_eq!(text.as_string(), Some("hello"));

        let number = Value::number(42);
        assert!(number.is_number());

        let boolean = Value::boolean(true);
        assert!(boolean.is_boolean());
        assert_eq!(boolean.as_boolean(), Some(true));
    }

    #[test]
    fn test_value_conversions() {
        let from_str: Value = "hello".into();
        assert!(from_str.is_string());

        let from_int: Value = 42.into();
        assert!(from_int.is_number());

        let from_bool: Value = true.into();
        assert!(from_bool.is_boolean());
    }

    #[test]
    fn test_null_value() {
        let null = Value::default();
        assert!(null.is_null());
        assert_eq!(null.type_name(), "null");
    }

    #[cfg(all(feature = "json", feature = "serde"))]
    #[test]
    fn test_json_integration() {
        use crate::json::*;

        let value = Value::string("test");
        let json = value_to_json(&value).unwrap();
        let back = json_to_value(json).unwrap();

        assert_eq!(value, back);
    }
}
