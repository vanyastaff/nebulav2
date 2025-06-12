// nebula_core/src/value/mod.rs

pub mod array;
pub mod binary;
pub mod boolean;
mod comparison;
pub mod datetime;
pub mod duration;
pub mod error;
pub mod expression;
pub mod group;
pub mod mode;
pub mod number;
pub mod object;
pub mod regex;
pub mod string;
mod value;

pub use value::Value;
pub use array::ArrayValue;
pub use binary::BinaryValue;
pub use boolean::BooleanValue;
pub use comparison::{ComparisonResult, ValueComparison};
pub use datetime::DateTimeValue;
pub use duration::DurationValue;
pub use error::{ValueError, ValueResult};
pub use expression::ExpressionValue;
pub use group::GroupValue;
pub use mode::ModeValue;
pub use number::NumberValue;
pub use object::ObjectValue;
pub use regex::RegexValue;
pub use string::StringValue;
