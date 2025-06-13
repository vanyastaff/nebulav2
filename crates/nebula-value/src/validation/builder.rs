//! Builder pattern for constructing complex validation rules

use crate::validation::ValidationOperator;
use crate::value::Value;

/// Builder for constructing validation rules
#[derive(Debug, Default)]
pub struct ValidationBuilder {
    operators: Vec<ValidationOperator>,
}

impl ValidationBuilder {
    /// Creates a new validation builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a validation operator
    pub fn add(mut self, operator: ValidationOperator) -> Self {
        self.operators.push(operator);
        self
    }

    /// Adds required validation (not null and not empty)
    pub fn required(self) -> Self {
        self.add(ValidationOperator::required())
    }

    /// Adds optional validation wrapper
    pub fn optional(self, validator: ValidationOperator) -> Self {
        self.add(ValidationOperator::optional(validator))
    }

    /// Adds minimum length validation
    pub fn min_length(self, min: usize) -> Self {
        self.add(ValidationOperator::MinLength(min))
    }

    /// Adds maximum length validation
    pub fn max_length(self, max: usize) -> Self {
        self.add(ValidationOperator::MaxLength(max))
    }

    /// Adds exact length validation
    pub fn exact_length(self, length: usize) -> Self {
        self.add(ValidationOperator::ExactLength(length))
    }

    /// Adds length range validation
    pub fn length_between(self, min: usize, max: usize) -> Self {
        self.add(ValidationOperator::length_between(min, max))
    }

    /// Adds regex validation
    pub fn matches(self, pattern: impl Into<String>) -> Self {
        self.add(ValidationOperator::Matches(pattern.into()))
    }

    /// Adds negative regex validation
    pub fn not_matches(self, pattern: impl Into<String>) -> Self {
        self.add(ValidationOperator::NotMatches(pattern.into()))
    }

    /// Adds string contains validation
    pub fn contains(self, substring: impl Into<String>) -> Self {
        self.add(ValidationOperator::Contains(substring.into()))
    }

    /// Adds string not contains validation
    pub fn not_contains(self, substring: impl Into<String>) -> Self {
        self.add(ValidationOperator::NotContains(substring.into()))
    }

    /// Adds string starts with validation
    pub fn starts_with(self, prefix: impl Into<String>) -> Self {
        self.add(ValidationOperator::StartsWith(prefix.into()))
    }

    /// Adds string ends with validation
    pub fn ends_with(self, suffix: impl Into<String>) -> Self {
        self.add(ValidationOperator::EndsWith(suffix.into()))
    }

    /// Adds equality validation
    pub fn equals(self, value: impl Into<Value>) -> Self {
        self.add(ValidationOperator::Eq(value.into()))
    }

    /// Adds inequality validation
    pub fn not_equals(self, value: impl Into<Value>) -> Self {
        self.add(ValidationOperator::NotEq(value.into()))
    }

    /// Adds greater than validation
    pub fn greater_than(self, value: impl Into<Value>) -> Self {
        self.add(ValidationOperator::Gt(value.into()))
    }

    /// Adds greater than or equal validation
    pub fn greater_than_or_equal(self, value: impl Into<Value>) -> Self {
        self.add(ValidationOperator::Gte(value.into()))
    }

    /// Adds less than validation
    pub fn less_than(self, value: impl Into<Value>) -> Self {
        self.add(ValidationOperator::Lt(value.into()))
    }

    /// Adds less than or equal validation
    pub fn less_than_or_equal(self, value: impl Into<Value>) -> Self {
        self.add(ValidationOperator::Lte(value.into()))
    }

    /// Adds range validation
    pub fn between(self, min: impl Into<Value>, max: impl Into<Value>) -> Self {
        self.add(ValidationOperator::range(min, max))
    }

    /// Adds not between validation
    pub fn not_between(self, min: impl Into<Value>, max: impl Into<Value>) -> Self {
        self.add(ValidationOperator::NotBetween {
            min: min.into(),
            max: max.into(),
        })
    }

    /// Adds in list validation
    pub fn in_list(self, values: Vec<impl Into<Value>>) -> Self {
        let values: Vec<Value> = values.into_iter().map(|v| v.into()).collect();
        self.add(ValidationOperator::In(values))
    }

    /// Adds not in list validation
    pub fn not_in_list(self, values: Vec<impl Into<Value>>) -> Self {
        let values: Vec<Value> = values.into_iter().map(|v| v.into()).collect();
        self.add(ValidationOperator::NotIn(values))
    }

    /// Adds positive number validation
    pub fn positive(self) -> Self {
        self.add(ValidationOperator::Positive)
    }

    /// Adds negative number validation
    pub fn negative(self) -> Self {
        self.add(ValidationOperator::Negative)
    }

    /// Adds zero validation
    pub fn zero(self) -> Self {
        self.add(ValidationOperator::Zero)
    }

    /// Adds non-zero validation
    pub fn non_zero(self) -> Self {
        self.add(ValidationOperator::NonZero)
    }

    /// Adds empty validation
    pub fn empty(self) -> Self {
        self.add(ValidationOperator::Empty)
    }

    /// Adds not empty validation
    pub fn not_empty(self) -> Self {
        self.add(ValidationOperator::NotEmpty)
    }

    /// Adds null validation
    pub fn null(self) -> Self {
        self.add(ValidationOperator::Null)
    }

    /// Adds not null validation
    pub fn not_null(self) -> Self {
        self.add(ValidationOperator::NotNull)
    }

    /// Adds equals field validation
    pub fn equals_field(self, field: impl Into<String>) -> Self {
        self.add(ValidationOperator::EqualsField(field.into()))
    }

    /// Adds not equals field validation
    pub fn not_equals_field(self, field: impl Into<String>) -> Self {
        self.add(ValidationOperator::NotEqualsField(field.into()))
    }

    /// Adds greater than field validation
    pub fn greater_than_field(self, field: impl Into<String>) -> Self {
        self.add(ValidationOperator::GreaterThanField(field.into()))
    }

    /// Adds less than field validation
    pub fn less_than_field(self, field: impl Into<String>) -> Self {
        self.add(ValidationOperator::LessThanField(field.into()))
    }

    /// Adds conditional requirement
    pub fn required_if(self, field: impl Into<String>, condition: ValidationOperator) -> Self {
        self.add(ValidationOperator::RequiredIf(field.into(), Box::new(condition)))
    }

    /// Adds conditional forbiddance
    pub fn forbidden_if(self, field: impl Into<String>, condition: ValidationOperator) -> Self {
        self.add(ValidationOperator::ForbiddenIf(field.into(), Box::new(condition)))
    }
    

    /// Builds the final validation operator using AND logic
    pub fn build(self) -> ValidationOperator {
        match self.operators.len() {
            0 => ValidationOperator::NotNull, // Default: just not null
            1 => self.operators.into_iter().next().unwrap(),
            _ => ValidationOperator::And(self.operators),
        }
    }

    /// Builds the final validation operator using OR logic
    pub fn build_or(self) -> ValidationOperator {
        match self.operators.len() {
            0 => ValidationOperator::NotNull,
            1 => self.operators.into_iter().next().unwrap(),
            _ => ValidationOperator::Or(self.operators),
        }
    }

    /// Builds the final validation operator as optional (can be null)
    pub fn build_optional(self) -> ValidationOperator {
        let validator = self.build();
        ValidationOperator::optional(validator)
    }
}