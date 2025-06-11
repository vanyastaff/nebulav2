use crate::parameter::ParameterValue;
use crate::types::ParameterKey;
use crate::value::{ComparisonResult, Value, ValueComparison, ValueError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

/// Errors that can occur during validation
#[derive(Debug, Error, PartialEq, Clone)]
pub enum ValidationError {
    /// Field validation failed with specific comparison error
    #[error("Field '{field}' validation failed: {operator} comparison failed - expected {expected}, got {actual}")]
    ComparisonFailed {
        field: ParameterKey,
        operator: String,
        expected: String,
        actual: String,
    },

    /// String length validation failed
    #[error("Field '{field}' string length validation failed: expected length {constraint}, got {actual}")]
    StringLengthFailed {
        field: ParameterKey,
        constraint: String, // e.g., "at least 5", "at most 100", "between 5 and 100"
        actual: usize,
    },

    /// Cross-field validation failed
    #[error("Field '{field}' must {relationship} field '{target_field}'")]
    CrossFieldFailed {
        field: ParameterKey,
        target_field: ParameterKey,
        relationship: String, // e.g., "equal", "be greater than"
    },

    /// Conditional validation failed
    #[error("Field '{field}' is required when field '{condition_field}' {condition_description}")]
    ConditionalRequired {
        field: ParameterKey,
        condition_field: ParameterKey,
        condition_description: String,
    },

    /// Group validation failed (one of, all or none, etc.)
    #[error("Group validation failed: {rule_description}. Fields involved: {fields:?}")]
    GroupValidationFailed {
        rule_description: String,
        fields: Vec<ParameterKey>,
    },

    /// Value type error from underlying Value system
    #[error("Field '{field}' type validation failed: {source}")]
    ValueError {
        field: ParameterKey,
        #[source]
        source: ValueError,
    },

    /// Custom validation error
    #[error("Field '{field}' custom validation failed: {message}")]
    Custom {
        field: ParameterKey,
        message: String,
    },

    /// Missing required field
    #[error("Required field '{field}' is missing")]
    MissingField { field: ParameterKey },
}

/// Custom validator function type
pub type ValidatorFn = Arc<
    dyn Fn(
            &ParameterValue,
            &ParameterKey,
            &HashMap<ParameterKey, ParameterValue>,
        ) -> Result<(), ValidationError>
        + Send
        + Sync,
>;

/// Validation conditions with full cross-field support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationCondition {
    // Basic value comparisons
    /// Value equals the specified value
    Eq(Value),
    /// Value does not equal the specified value
    NotEq(Value),
    /// Value is greater than the specified value (for numbers)
    Gt(Value),
    /// Value is less than the specified value (for numbers)
    Lt(Value),
    /// Value is greater than or equal to the specified value (for numbers)
    Gte(Value),
    /// Value is less than or equal to the specified value (for numbers)
    Lte(Value),
    /// Value is in the list of specified values
    In(Vec<Value>),
    /// Value is NOT in the list of specified values
    NotIn(Vec<Value>),
    /// Value is empty (empty string, empty array, null)
    IsEmpty,
    /// Value is NOT empty
    IsNotEmpty,
    /// String contains the specified substring
    Contains(Value),
    /// String starts with the specified prefix
    StartsWith(Value),
    /// String ends with the specified suffix
    EndsWith(Value),
    /// String matches the specified regex pattern
    Regex(String),

    // String length constraints
    /// String has minimum length
    MinLength(usize),
    /// String has maximum length
    MaxLength(usize),
    /// String length is between min and max (inclusive)
    LengthBetween { min: usize, max: usize },

    // Numeric range constraints
    /// Numeric value is between min and max (inclusive)
    Between { min: Value, max: Value },

    // Cross-field validations
    /// Field value equals another field's value
    EqualsField(ParameterKey),
    /// Field value does not equal another field's value
    NotEqualsField(ParameterKey),
    /// Field value is greater than another field's value
    GreaterThanField(ParameterKey),
    /// Field value is less than another field's value
    LessThanField(ParameterKey),
    /// Field value is greater than or equal to another field's value
    GreaterThanOrEqualField(ParameterKey),
    /// Field value is less than or equal to another field's value
    LessThanOrEqualField(ParameterKey),

    // Conditional validations
    /// Field is required if another field meets a condition
    RequiredIf {
        field: ParameterKey,
        condition: Box<ValidationCondition>,
    },
    /// Field is required unless another field meets a condition
    RequiredUnless {
        field: ParameterKey,
        condition: Box<ValidationCondition>,
    },
    /// Field is forbidden if another field meets a condition
    ForbiddenIf {
        field: ParameterKey,
        condition: Box<ValidationCondition>,
    },

    // Group validations
    /// At least one of the specified fields must be non-empty
    OneOf(Vec<ParameterKey>),
    /// Either all fields are filled or none are filled
    AllOrNone(Vec<ParameterKey>),
    /// Only one of the specified fields can be non-empty
    MutuallyExclusive(Vec<ParameterKey>),
    /// All specified fields must be non-empty
    AllRequired(Vec<ParameterKey>),

    // Logical operators
    /// All conditions must be met (logical AND)
    And(Vec<ValidationCondition>),
    /// At least one condition must be met (logical OR)
    Or(Vec<ValidationCondition>),
    /// Condition must NOT be met (logical NOT)
    Not(Box<ValidationCondition>),

    // Custom validation
    /// Custom validator function (not serializable)
    #[serde(skip)]
    Custom(ValidatorFn),
}

impl ValidationCondition {
    /// Validates a value against this condition
    pub fn validate(
        &self,
        value: &ParameterValue,
        field: &ParameterKey,
        all_values: &HashMap<ParameterKey, ParameterValue>,
    ) -> Result<(), ValidationError> {
        match self {
            // Basic comparisons
            Self::Eq(expected) => {
                if ValueComparison::equals(value, expected) {
                    Ok(())
                } else {
                    Err(ValidationError::ComparisonFailed {
                        field: field.clone(),
                        operator: "equals".to_string(),
                        expected: ValueComparison::format_for_display(expected),
                        actual: ValueComparison::format_for_display(value),
                    })
                }
            }

            Self::NotEq(expected) => {
                if ValueComparison::not_equals(value, expected) {
                    Ok(())
                } else {
                    Err(ValidationError::ComparisonFailed {
                        field: field.clone(),
                        operator: "not_equals".to_string(),
                        expected: format!("not {}", ValueComparison::format_for_display(expected)),
                        actual: ValueComparison::format_for_display(value),
                    })
                }
            }

            Self::Gt(expected) => match ValueComparison::greater_than(value, expected) {
                ComparisonResult::True => Ok(()),
                ComparisonResult::False => Err(ValidationError::ComparisonFailed {
                    field: field.clone(),
                    operator: "greater_than".to_string(),
                    expected: ValueComparison::format_for_display(expected),
                    actual: ValueComparison::format_for_display(value),
                }),
                ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                    field: field.clone(),
                    source: e,
                }),
            },

            Self::Lt(expected) => match ValueComparison::less_than(value, expected) {
                ComparisonResult::True => Ok(()),
                ComparisonResult::False => Err(ValidationError::ComparisonFailed {
                    field: field.clone(),
                    operator: "less_than".to_string(),
                    expected: ValueComparison::format_for_display(expected),
                    actual: ValueComparison::format_for_display(value),
                }),
                ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                    field: field.clone(),
                    source: e,
                }),
            },

            Self::Gte(expected) => match ValueComparison::greater_than_or_equal(value, expected) {
                ComparisonResult::True => Ok(()),
                ComparisonResult::False => Err(ValidationError::ComparisonFailed {
                    field: field.clone(),
                    operator: "greater_than_or_equal".to_string(),
                    expected: ValueComparison::format_for_display(expected),
                    actual: ValueComparison::format_for_display(value),
                }),
                ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                    field: field.clone(),
                    source: e,
                }),
            },

            Self::Lte(expected) => match ValueComparison::less_than_or_equal(value, expected) {
                ComparisonResult::True => Ok(()),
                ComparisonResult::False => Err(ValidationError::ComparisonFailed {
                    field: field.clone(),
                    operator: "less_than_or_equal".to_string(),
                    expected: ValueComparison::format_for_display(expected),
                    actual: ValueComparison::format_for_display(value),
                }),
                ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                    field: field.clone(),
                    source: e,
                }),
            },

            Self::In(list) => {
                if ValueComparison::in_list(value, list) {
                    Ok(())
                } else {
                    Err(ValidationError::ComparisonFailed {
                        field: field.clone(),
                        operator: "in_list".to_string(),
                        expected: format!(
                            "one of {:?}",
                            list.iter()
                                .map(ValueComparison::format_for_display)
                                .collect::<Vec<_>>()
                        ),
                        actual: ValueComparison::format_for_display(value),
                    })
                }
            }

            Self::NotIn(list) => {
                if ValueComparison::not_in_list(value, list) {
                    Ok(())
                } else {
                    Err(ValidationError::ComparisonFailed {
                        field: field.clone(),
                        operator: "not_in_list".to_string(),
                        expected: format!(
                            "not one of {:?}",
                            list.iter()
                                .map(ValueComparison::format_for_display)
                                .collect::<Vec<_>>()
                        ),
                        actual: ValueComparison::format_for_display(value),
                    })
                }
            }

            Self::IsEmpty => {
                if ValueComparison::is_empty(value) {
                    Ok(())
                } else {
                    Err(ValidationError::ComparisonFailed {
                        field: field.clone(),
                        operator: "is_empty".to_string(),
                        expected: "empty value".to_string(),
                        actual: "non-empty value".to_string(),
                    })
                }
            }

            Self::IsNotEmpty => {
                if ValueComparison::is_not_empty(value) {
                    Ok(())
                } else {
                    Err(ValidationError::ComparisonFailed {
                        field: field.clone(),
                        operator: "is_not_empty".to_string(),
                        expected: "non-empty value".to_string(),
                        actual: "empty value".to_string(),
                    })
                }
            }

            Self::Contains(substring) => match ValueComparison::contains(value, substring) {
                ComparisonResult::True => Ok(()),
                ComparisonResult::False => Err(ValidationError::ComparisonFailed {
                    field: field.clone(),
                    operator: "contains".to_string(),
                    expected: format!(
                        "string containing {}",
                        ValueComparison::format_for_display(substring)
                    ),
                    actual: ValueComparison::format_for_display(value),
                }),
                ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                    field: field.clone(),
                    source: e,
                }),
            },

            Self::StartsWith(prefix) => match ValueComparison::starts_with(value, prefix) {
                ComparisonResult::True => Ok(()),
                ComparisonResult::False => Err(ValidationError::ComparisonFailed {
                    field: field.clone(),
                    operator: "starts_with".to_string(),
                    expected: format!(
                        "string starting with {}",
                        ValueComparison::format_for_display(prefix)
                    ),
                    actual: ValueComparison::format_for_display(value),
                }),
                ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                    field: field.clone(),
                    source: e,
                }),
            },

            Self::EndsWith(suffix) => match ValueComparison::ends_with(value, suffix) {
                ComparisonResult::True => Ok(()),
                ComparisonResult::False => Err(ValidationError::ComparisonFailed {
                    field: field.clone(),
                    operator: "ends_with".to_string(),
                    expected: format!(
                        "string ending with {}",
                        ValueComparison::format_for_display(suffix)
                    ),
                    actual: ValueComparison::format_for_display(value),
                }),
                ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                    field: field.clone(),
                    source: e,
                }),
            },

            Self::Regex(pattern) => match ValueComparison::matches_regex(value, pattern) {
                ComparisonResult::True => Ok(()),
                ComparisonResult::False => Err(ValidationError::ComparisonFailed {
                    field: field.clone(),
                    operator: "regex_match".to_string(),
                    expected: format!("string matching pattern '{}'", pattern),
                    actual: ValueComparison::format_for_display(value),
                }),
                ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                    field: field.clone(),
                    source: e,
                }),
            },

            // String length validations
            Self::MinLength(min) => match ValueComparison::min_length(value, *min) {
                ComparisonResult::True => Ok(()),
                ComparisonResult::False => {
                    let actual_len = value.as_string().map(|s| s.len()).unwrap_or(0);
                    Err(ValidationError::StringLengthFailed {
                        field: field.clone(),
                        constraint: format!("at least {}", min),
                        actual: actual_len,
                    })
                }
                ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                    field: field.clone(),
                    source: e,
                }),
            },

            Self::MaxLength(max) => match ValueComparison::max_length(value, *max) {
                ComparisonResult::True => Ok(()),
                ComparisonResult::False => {
                    let actual_len = value.as_string().map(|s| s.len()).unwrap_or(0);
                    Err(ValidationError::StringLengthFailed {
                        field: field.clone(),
                        constraint: format!("at most {}", max),
                        actual: actual_len,
                    })
                }
                ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                    field: field.clone(),
                    source: e,
                }),
            },

            Self::LengthBetween { min, max } => {
                if let Some(text) = value.as_string() {
                    let len = text.len();
                    if len >= *min && len <= *max {
                        Ok(())
                    } else {
                        Err(ValidationError::StringLengthFailed {
                            field: field.clone(),
                            constraint: format!("between {} and {}", min, max),
                            actual: len,
                        })
                    }
                } else {
                    Err(ValidationError::ValueError {
                        field: field.clone(),
                        source: ValueError::type_conversion(value.type_name(), "string"),
                    })
                }
            }

            Self::Between { min, max } => match ValueComparison::between(value, min, max) {
                ComparisonResult::True => Ok(()),
                ComparisonResult::False => Err(ValidationError::ComparisonFailed {
                    field: field.clone(),
                    operator: "between".to_string(),
                    expected: format!(
                        "between {} and {}",
                        ValueComparison::format_for_display(min),
                        ValueComparison::format_for_display(max)
                    ),
                    actual: ValueComparison::format_for_display(value),
                }),
                ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                    field: field.clone(),
                    source: e,
                }),
            },

            // Cross-field validations
            Self::EqualsField(target_field) => {
                if let Some(target_value) = all_values.get(target_field) {
                    if ValueComparison::equals(value, target_value) {
                        Ok(())
                    } else {
                        Err(ValidationError::CrossFieldFailed {
                            field: field.clone(),
                            target_field: target_field.clone(),
                            relationship: "equal".to_string(),
                        })
                    }
                } else {
                    Err(ValidationError::MissingField {
                        field: target_field.clone(),
                    })
                }
            }

            Self::NotEqualsField(target_field) => {
                if let Some(target_value) = all_values.get(target_field) {
                    if ValueComparison::not_equals(value, target_value) {
                        Ok(())
                    } else {
                        Err(ValidationError::CrossFieldFailed {
                            field: field.clone(),
                            target_field: target_field.clone(),
                            relationship: "not equal".to_string(),
                        })
                    }
                } else {
                    Err(ValidationError::MissingField {
                        field: target_field.clone(),
                    })
                }
            }

            Self::GreaterThanField(target_field) => {
                if let Some(target_value) = all_values.get(target_field) {
                    match ValueComparison::greater_than(value, target_value) {
                        ComparisonResult::True => Ok(()),
                        ComparisonResult::False => Err(ValidationError::CrossFieldFailed {
                            field: field.clone(),
                            target_field: target_field.clone(),
                            relationship: "be greater than".to_string(),
                        }),
                        ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                            field: field.clone(),
                            source: e,
                        }),
                    }
                } else {
                    Err(ValidationError::MissingField {
                        field: target_field.clone(),
                    })
                }
            }

            Self::LessThanField(target_field) => {
                if let Some(target_value) = all_values.get(target_field) {
                    match ValueComparison::less_than(value, target_value) {
                        ComparisonResult::True => Ok(()),
                        ComparisonResult::False => Err(ValidationError::CrossFieldFailed {
                            field: field.clone(),
                            target_field: target_field.clone(),
                            relationship: "be less than".to_string(),
                        }),
                        ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                            field: field.clone(),
                            source: e,
                        }),
                    }
                } else {
                    Err(ValidationError::MissingField {
                        field: target_field.clone(),
                    })
                }
            }

            Self::GreaterThanOrEqualField(target_field) => {
                if let Some(target_value) = all_values.get(target_field) {
                    match ValueComparison::greater_than_or_equal(value, target_value) {
                        ComparisonResult::True => Ok(()),
                        ComparisonResult::False => Err(ValidationError::CrossFieldFailed {
                            field: field.clone(),
                            target_field: target_field.clone(),
                            relationship: "be greater than or equal to".to_string(),
                        }),
                        ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                            field: field.clone(),
                            source: e,
                        }),
                    }
                } else {
                    Err(ValidationError::MissingField {
                        field: target_field.clone(),
                    })
                }
            }

            Self::LessThanOrEqualField(target_field) => {
                if let Some(target_value) = all_values.get(target_field) {
                    match ValueComparison::less_than_or_equal(value, target_value) {
                        ComparisonResult::True => Ok(()),
                        ComparisonResult::False => Err(ValidationError::CrossFieldFailed {
                            field: field.clone(),
                            target_field: target_field.clone(),
                            relationship: "be less than or equal to".to_string(),
                        }),
                        ComparisonResult::Error(e) => Err(ValidationError::ValueError {
                            field: field.clone(),
                            source: e,
                        }),
                    }
                } else {
                    Err(ValidationError::MissingField {
                        field: target_field.clone(),
                    })
                }
            }

            // Conditional validations
            Self::RequiredIf {
                field: condition_field,
                condition,
            } => {
                if let Some(condition_value) = all_values.get(condition_field) {
                    // Check if the condition is met
                    if condition
                        .validate(condition_value, condition_field, all_values)
                        .is_ok()
                    {
                        // Condition is met, current field is required
                        if ValueComparison::is_empty(value) {
                            Err(ValidationError::ConditionalRequired {
                                field: field.clone(),
                                condition_field: condition_field.clone(),
                                condition_description: "meets the specified condition".to_string(),
                            })
                        } else {
                            Ok(())
                        }
                    } else {
                        Ok(()) // Condition not met, field not required
                    }
                } else {
                    Ok(()) // Condition field missing, rule doesn't apply
                }
            }

            Self::RequiredUnless {
                field: condition_field,
                condition,
            } => {
                if let Some(condition_value) = all_values.get(condition_field) {
                    // Check if the condition is NOT met
                    if condition
                        .validate(condition_value, condition_field, all_values)
                        .is_err()
                    {
                        // Condition is NOT met, current field is required
                        if ValueComparison::is_empty(value) {
                            Err(ValidationError::ConditionalRequired {
                                field: field.clone(),
                                condition_field: condition_field.clone(),
                                condition_description: "does not meet the specified condition"
                                    .to_string(),
                            })
                        } else {
                            Ok(())
                        }
                    } else {
                        Ok(()) // Condition met, field not required
                    }
                } else {
                    // Condition field missing, treat as condition not met, so field is required
                    if ValueComparison::is_empty(value) {
                        Err(ValidationError::ConditionalRequired {
                            field: field.clone(),
                            condition_field: condition_field.clone(),
                            condition_description: "is missing".to_string(),
                        })
                    } else {
                        Ok(())
                    }
                }
            }

            Self::ForbiddenIf {
                field: condition_field,
                condition,
            } => {
                if let Some(condition_value) = all_values.get(condition_field) {
                    // Check if the condition is met
                    if condition
                        .validate(condition_value, condition_field, all_values)
                        .is_ok()
                    {
                        // Condition is met, current field must be empty
                        if ValueComparison::is_not_empty(value) {
                            Err(ValidationError::ConditionalRequired {
                                field: field.clone(),
                                condition_field: condition_field.clone(),
                                condition_description: "is forbidden when condition is met"
                                    .to_string(),
                            })
                        } else {
                            Ok(())
                        }
                    } else {
                        Ok(()) // Condition not met, field allowed
                    }
                } else {
                    Ok(()) // Condition field missing, rule doesn't apply
                }
            }

            // Group validations
            Self::OneOf(fields) => {
                let non_empty_count = fields
                    .iter()
                    .filter_map(|f| all_values.get(f))
                    .filter(|v| ValueComparison::is_not_empty(v))
                    .count();

                if non_empty_count >= 1 {
                    Ok(())
                } else {
                    Err(ValidationError::GroupValidationFailed {
                        rule_description: "At least one field must be filled".to_string(),
                        fields: fields.clone(),
                    })
                }
            }

            Self::AllOrNone(fields) => {
                let non_empty_count = fields
                    .iter()
                    .filter_map(|f| all_values.get(f))
                    .filter(|v| ValueComparison::is_not_empty(v))
                    .count();

                if non_empty_count == 0 || non_empty_count == fields.len() {
                    Ok(())
                } else {
                    Err(ValidationError::GroupValidationFailed {
                        rule_description: "Either all fields must be filled or none".to_string(),
                        fields: fields.clone(),
                    })
                }
            }

            Self::MutuallyExclusive(fields) => {
                let non_empty_count = fields
                    .iter()
                    .filter_map(|f| all_values.get(f))
                    .filter(|v| ValueComparison::is_not_empty(v))
                    .count();

                if non_empty_count <= 1 {
                    Ok(())
                } else {
                    Err(ValidationError::GroupValidationFailed {
                        rule_description: "Only one field can be filled".to_string(),
                        fields: fields.clone(),
                    })
                }
            }

            Self::AllRequired(fields) => {
                let missing_fields: Vec<ParameterKey> = fields
                    .iter()
                    .filter(|f| {
                        all_values
                            .get(f)
                            .map(|v| ValueComparison::is_empty(v))
                            .unwrap_or(true) // Missing field counts as empty
                    })
                    .cloned()
                    .collect();

                if missing_fields.is_empty() {
                    Ok(())
                } else {
                    Err(ValidationError::GroupValidationFailed {
                        rule_description: format!(
                            "All fields must be filled. Missing: {:?}",
                            missing_fields
                        ),
                        fields: fields.clone(),
                    })
                }
            }

            // Logical operators
            Self::And(conditions) => {
                for condition in conditions {
                    condition.validate(value, field, all_values)?;
                }
                Ok(())
            }

            Self::Or(conditions) => {
                let mut errors = Vec::new();
                for condition in conditions {
                    match condition.validate(value, field, all_values) {
                        Ok(()) => return Ok(()), // At least one condition passed
                        Err(e) => errors.push(e),
                    }
                }
                // All conditions failed
                Err(ValidationError::Custom {
                    field: field.clone(),
                    message: format!("All OR conditions failed: {:?}", errors),
                })
            }

            Self::Not(condition) => {
                match condition.validate(value, field, all_values) {
                    Ok(()) => Err(ValidationError::Custom {
                        field: field.clone(),
                        message: "NOT condition failed: inner condition passed".to_string(),
                    }),
                    Err(_) => Ok(()), // Inner condition failed, so NOT passes
                }
            }

            // Custom validation
            Self::Custom(validator) => validator(value, field, all_values),
        }
    }
}

// Builder methods for ValidationCondition
impl ValidationCondition {
    /// Creates an equality condition
    pub fn equals<T: Into<Value>>(value: T) -> Self {
        Self::Eq(value.into())
    }

    /// Creates an inequality condition
    pub fn not_equals<T: Into<Value>>(value: T) -> Self {
        Self::NotEq(value.into())
    }

    /// Creates a "greater than" condition
    pub fn greater_than<T: Into<Value>>(value: T) -> Self {
        Self::Gt(value.into())
    }

    /// Creates a "less than" condition
    pub fn less_than<T: Into<Value>>(value: T) -> Self {
        Self::Lt(value.into())
    }

    /// Creates a "greater than or equal" condition
    pub fn greater_than_or_equal<T: Into<Value>>(value: T) -> Self {
        Self::Gte(value.into())
    }

    /// Creates a "less than or equal" condition
    pub fn less_than_or_equal<T: Into<Value>>(value: T) -> Self {
        Self::Lte(value.into())
    }

    /// Creates an "in list" condition
    pub fn in_list<T: Into<Value>>(values: Vec<T>) -> Self {
        Self::In(values.into_iter().map(Into::into).collect())
    }

    /// Creates a "not in list" condition
    pub fn not_in_list<T: Into<Value>>(values: Vec<T>) -> Self {
        Self::NotIn(values.into_iter().map(Into::into).collect())
    }

    /// Creates an "empty" condition
    pub fn is_empty() -> Self {
        Self::IsEmpty
    }

    /// Creates a "not empty" condition
    pub fn is_not_empty() -> Self {
        Self::IsNotEmpty
    }

    /// Creates a "contains" condition
    pub fn contains<T: Into<Value>>(substring: T) -> Self {
        Self::Contains(substring.into())
    }

    /// Creates a "starts with" condition
    pub fn starts_with<T: Into<Value>>(prefix: T) -> Self {
        Self::StartsWith(prefix.into())
    }

    /// Creates an "ends with" condition
    pub fn ends_with<T: Into<Value>>(suffix: T) -> Self {
        Self::EndsWith(suffix.into())
    }

    /// Creates a regex matching condition
    pub fn regex<T: Into<String>>(pattern: T) -> Self {
        Self::Regex(pattern.into())
    }

    /// Creates a minimum length condition
    pub fn min_length(length: usize) -> Self {
        Self::MinLength(length)
    }

    /// Creates a maximum length condition
    pub fn max_length(length: usize) -> Self {
        Self::MaxLength(length)
    }

    /// Creates a length range condition
    pub fn length_between(min: usize, max: usize) -> Self {
        Self::LengthBetween { min, max }
    }

    /// Creates a numeric range condition
    pub fn between<T: Into<Value>, U: Into<Value>>(min: T, max: U) -> Self {
        Self::Between {
            min: min.into(),
            max: max.into(),
        }
    }

    /// Creates an "equals field" condition
    pub fn equals_field(field: ParameterKey) -> Self {
        Self::EqualsField(field)
    }

    /// Creates a "not equals field" condition
    pub fn not_equals_field(field: ParameterKey) -> Self {
        Self::NotEqualsField(field)
    }

    /// Creates a "greater than field" condition
    pub fn greater_than_field(field: ParameterKey) -> Self {
        Self::GreaterThanField(field)
    }

    /// Creates a "less than field" condition
    pub fn less_than_field(field: ParameterKey) -> Self {
        Self::LessThanField(field)
    }

    /// Creates a "greater than or equal field" condition
    pub fn greater_than_or_equal_field(field: ParameterKey) -> Self {
        Self::GreaterThanOrEqualField(field)
    }

    /// Creates a "less than or equal field" condition
    pub fn less_than_or_equal_field(field: ParameterKey) -> Self {
        Self::LessThanOrEqualField(field)
    }

    /// Creates a "required if" condition
    pub fn required_if(field: ParameterKey, condition: ValidationCondition) -> Self {
        Self::RequiredIf {
            field,
            condition: Box::new(condition),
        }
    }

    /// Creates a "required unless" condition
    pub fn required_unless(field: ParameterKey, condition: ValidationCondition) -> Self {
        Self::RequiredUnless {
            field,
            condition: Box::new(condition),
        }
    }

    /// Creates a "forbidden if" condition
    pub fn forbidden_if(field: ParameterKey, condition: ValidationCondition) -> Self {
        Self::ForbiddenIf {
            field,
            condition: Box::new(condition),
        }
    }

    /// Creates a "one of" group condition
    pub fn one_of(fields: Vec<ParameterKey>) -> Self {
        Self::OneOf(fields)
    }

    /// Creates an "all or none" group condition
    pub fn all_or_none(fields: Vec<ParameterKey>) -> Self {
        Self::AllOrNone(fields)
    }

    /// Creates a "mutually exclusive" group condition
    pub fn mutually_exclusive(fields: Vec<ParameterKey>) -> Self {
        Self::MutuallyExclusive(fields)
    }

    /// Creates an "all required" group condition
    pub fn all_required(fields: Vec<ParameterKey>) -> Self {
        Self::AllRequired(fields)
    }

    /// Creates a logical AND condition
    pub fn and(conditions: Vec<ValidationCondition>) -> Self {
        Self::And(conditions)
    }

    /// Creates a logical OR condition
    pub fn or(conditions: Vec<ValidationCondition>) -> Self {
        Self::Or(conditions)
    }

    /// Creates a logical NOT condition
    pub fn not(condition: ValidationCondition) -> Self {
        Self::Not(Box::new(condition))
    }

    /// Creates a custom validation condition
    pub fn custom<F>(validator: F) -> Self
    where
        F: Fn(
                &ParameterValue,
                &ParameterKey,
                &HashMap<ParameterKey, ParameterValue>,
            ) -> Result<(), ValidationError>
            + Send
            + Sync
            + 'static,
    {
        Self::Custom(std::sync::Arc::new(validator))
    }
}

/// Parameter validation container
///
/// Holds a collection of validation rules that will be applied to a parameter
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParameterValidation {
    /// List of validation rules to apply
    rules: Vec<ValidationCondition>,
}

impl ParameterValidation {
    /// Creates a new empty validation container
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a validation container from a list of rules
    pub fn from_rules(rules: Vec<ValidationCondition>) -> Self {
        Self { rules }
    }

    /// Creates a new validation builder
    pub fn builder() -> ParameterValidationBuilder {
        ParameterValidationBuilder::new()
    }

    /// Adds a validation rule
    pub fn add_rule(&mut self, rule: ValidationCondition) -> &mut Self {
        self.rules.push(rule);
        self
    }

    /// Adds multiple validation rules
    pub fn add_rules(&mut self, rules: Vec<ValidationCondition>) -> &mut Self {
        self.rules.extend(rules);
        self
    }

    /// Returns the validation rules
    pub fn rules(&self) -> &[ValidationCondition] {
        &self.rules
    }

    /// Returns the number of validation rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Checks if there are no validation rules
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Validates a parameter value against all rules
    ///
    /// # Arguments
    ///
    /// * `value` - The parameter value to validate
    /// * `field` - The field key for error reporting
    /// * `all_values` - All parameter values for cross-field validation
    ///
    /// # Returns
    ///
    /// * `Ok(())` if all rules pass
    /// * `Err(ValidationError)` if any rule fails
    pub fn validate(
        &self,
        value: &ParameterValue,
        field: &ParameterKey,
        all_values: &HashMap<ParameterKey, ParameterValue>,
    ) -> Result<(), ValidationError> {
        for rule in &self.rules {
            rule.validate(value, field, all_values)?;
        }
        Ok(())
    }

    /// Validates and collects all validation errors
    ///
    /// Unlike `validate()`, this method doesn't stop at the first error
    /// and returns all validation failures
    pub fn validate_all(
        &self,
        value: &ParameterValue,
        field: &ParameterKey,
        all_values: &HashMap<ParameterKey, ParameterValue>,
    ) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        for rule in &self.rules {
            if let Err(error) = rule.validate(value, field, all_values) {
                errors.push(error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Checks if the validation contains any cross-field rules
    pub fn has_cross_field_rules(&self) -> bool {
        self.rules.iter().any(|rule| rule.is_cross_field())
    }

    /// Returns all fields that this validation depends on
    pub fn dependent_fields(&self) -> Vec<ParameterKey> {
        let mut fields = Vec::new();
        for rule in &self.rules {
            rule.collect_dependent_fields(&mut fields);
        }
        fields.sort();
        fields.dedup();
        fields
    }
}

/// Helper methods for ValidationCondition
impl ValidationCondition {
    /// Checks if this condition involves cross-field validation
    pub fn is_cross_field(&self) -> bool {
        match self {
            Self::EqualsField(_)
            | Self::NotEqualsField(_)
            | Self::GreaterThanField(_)
            | Self::LessThanField(_)
            | Self::GreaterThanOrEqualField(_)
            | Self::LessThanOrEqualField(_)
            | Self::RequiredIf { .. }
            | Self::RequiredUnless { .. }
            | Self::ForbiddenIf { .. }
            | Self::OneOf(_)
            | Self::AllOrNone(_)
            | Self::MutuallyExclusive(_)
            | Self::AllRequired(_) => true,

            Self::And(conditions) | Self::Or(conditions) => {
                conditions.iter().any(|c| c.is_cross_field())
            }
            Self::Not(condition) => condition.is_cross_field(),

            _ => false,
        }
    }

    /// Collects all fields that this condition depends on
    pub fn collect_dependent_fields(&self, fields: &mut Vec<ParameterKey>) {
        match self {
            Self::EqualsField(f)
            | Self::NotEqualsField(f)
            | Self::GreaterThanField(f)
            | Self::LessThanField(f)
            | Self::GreaterThanOrEqualField(f)
            | Self::LessThanOrEqualField(f) => {
                fields.push(f.clone());
            }

            Self::RequiredIf { field, condition }
            | Self::RequiredUnless { field, condition }
            | Self::ForbiddenIf { field, condition } => {
                fields.push(field.clone());
                condition.collect_dependent_fields(fields);
            }

            Self::OneOf(fs)
            | Self::AllOrNone(fs)
            | Self::MutuallyExclusive(fs)
            | Self::AllRequired(fs) => {
                fields.extend(fs.iter().cloned());
            }

            Self::And(conditions) | Self::Or(conditions) => {
                for condition in conditions {
                    condition.collect_dependent_fields(fields);
                }
            }

            Self::Not(condition) => {
                condition.collect_dependent_fields(fields);
            }

            _ => {} // No dependencies for basic conditions
        }
    }
}

/// Builder for creating parameter validation
#[derive(Debug, Default)]
pub struct ParameterValidationBuilder {
    validation: ParameterValidation,
}

impl ParameterValidationBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a validation rule
    pub fn with_rule(mut self, rule: ValidationCondition) -> Self {
        self.validation.add_rule(rule);
        self
    }

    /// Adds multiple validation rules
    pub fn with_rules(mut self, rules: Vec<ValidationCondition>) -> Self {
        self.validation.add_rules(rules);
        self
    }

    // Convenience methods for common validations

    /// Adds a required validation (not empty)
    pub fn required(self) -> Self {
        self.with_rule(ValidationCondition::is_not_empty())
    }

    /// Adds an equality validation
    pub fn equals<T: Into<Value>>(self, value: T) -> Self {
        self.with_rule(ValidationCondition::equals(value))
    }

    /// Adds a minimum length validation
    pub fn min_length(self, length: usize) -> Self {
        self.with_rule(ValidationCondition::min_length(length))
    }

    /// Adds a maximum length validation
    pub fn max_length(self, length: usize) -> Self {
        self.with_rule(ValidationCondition::max_length(length))
    }

    /// Adds a length range validation
    pub fn length_between(self, min: usize, max: usize) -> Self {
        self.with_rule(ValidationCondition::length_between(min, max))
    }

    /// Adds a regex validation
    pub fn regex<T: Into<String>>(self, pattern: T) -> Self {
        self.with_rule(ValidationCondition::regex(pattern))
    }

    /// Adds a numeric range validation
    pub fn between<T: Into<Value>, U: Into<Value>>(self, min: T, max: U) -> Self {
        self.with_rule(ValidationCondition::between(min, max))
    }

    /// Adds a "greater than" validation
    pub fn greater_than<T: Into<Value>>(self, value: T) -> Self {
        self.with_rule(ValidationCondition::greater_than(value))
    }

    /// Adds a "less than" validation
    pub fn less_than<T: Into<Value>>(self, value: T) -> Self {
        self.with_rule(ValidationCondition::less_than(value))
    }

    /// Adds a "greater than or equal" validation
    pub fn greater_than_or_equal<T: Into<Value>>(self, value: T) -> Self {
        self.with_rule(ValidationCondition::greater_than_or_equal(value))
    }

    /// Adds a "less than or equal" validation
    pub fn less_than_or_equal<T: Into<Value>>(self, value: T) -> Self {
        self.with_rule(ValidationCondition::less_than_or_equal(value))
    }

    /// Adds a "not equals" validation
    pub fn not_equals<T: Into<Value>>(self, value: T) -> Self {
        self.with_rule(ValidationCondition::not_equals(value))
    }

    /// Adds a "not in list" validation
    pub fn not_in_list<T: Into<Value>>(self, values: Vec<T>) -> Self {
        self.with_rule(ValidationCondition::not_in_list(values))
    }

    /// Adds an "in list" validation
    pub fn in_list<T: Into<Value>>(self, values: Vec<T>) -> Self {
        self.with_rule(ValidationCondition::in_list(values))
    }

    /// Adds a "contains" validation
    pub fn contains<T: Into<Value>>(self, substring: T) -> Self {
        self.with_rule(ValidationCondition::contains(substring))
    }

    /// Adds a "starts with" validation
    pub fn starts_with<T: Into<Value>>(self, prefix: T) -> Self {
        self.with_rule(ValidationCondition::starts_with(prefix))
    }

    /// Adds an "ends with" validation
    pub fn ends_with<T: Into<Value>>(self, suffix: T) -> Self {
        self.with_rule(ValidationCondition::ends_with(suffix))
    }

    // Cross-field validations

    /// Adds an "equals field" validation
    pub fn equals_field(self, field: ParameterKey) -> Self {
        self.with_rule(ValidationCondition::equals_field(field))
    }

    /// Adds a "greater than field" validation
    pub fn greater_than_field(self, field: ParameterKey) -> Self {
        self.with_rule(ValidationCondition::greater_than_field(field))
    }

    /// Adds a "required if" validation
    pub fn required_if(self, field: ParameterKey, condition: ValidationCondition) -> Self {
        self.with_rule(ValidationCondition::required_if(field, condition))
    }

    /// Adds a "one of" group validation
    pub fn one_of(self, fields: Vec<ParameterKey>) -> Self {
        self.with_rule(ValidationCondition::one_of(fields))
    }

    /// Adds an "all or none" group validation
    pub fn all_or_none(self, fields: Vec<ParameterKey>) -> Self {
        self.with_rule(ValidationCondition::all_or_none(fields))
    }

    /// Adds a "mutually exclusive" group validation
    pub fn mutually_exclusive(self, fields: Vec<ParameterKey>) -> Self {
        self.with_rule(ValidationCondition::mutually_exclusive(fields))
    }

    /// Adds a custom validation
    pub fn custom<F>(self, validator: F) -> Self
    where
        F: Fn(
                &ParameterValue,
                &ParameterKey,
                &HashMap<ParameterKey, ParameterValue>,
            ) -> Result<(), ValidationError>
            + Send
            + Sync
            + 'static,
    {
        self.with_rule(ValidationCondition::custom(validator))
    }

    /// Builds the final ParameterValidation
    pub fn build(self) -> ParameterValidation {
        self.validation
    }
}

/// Common validation patterns
pub mod validators {
    use super::*;

    /// Email validation
    pub fn email() -> ParameterValidation {
        ParameterValidation::builder()
            .required()
            .regex(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .build()
    }

    /// URL validation
    pub fn url() -> ParameterValidation {
        ParameterValidation::builder()
            .required()
            .regex(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$")
            .build()
    }

    /// UUID validation
    pub fn uuid() -> ParameterValidation {
        ParameterValidation::builder()
            .required()
            .regex(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")
            .build()
    }

    /// API key validation (starts with prefix, minimum length)
    pub fn api_key(prefix: &str, min_length: usize) -> ParameterValidation {
        ParameterValidation::builder()
            .required()
            .starts_with(Value::string(prefix))
            .min_length(min_length)
            .build()
    }

    /// Password validation
    pub fn password(min_length: usize, require_special: bool) -> ParameterValidation {
        let mut builder = ParameterValidation::builder()
            .required()
            .min_length(min_length);

        if require_special {
            builder =
                builder.regex(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]");
        }

        builder.build()
    }

    /// Phone number validation
    pub fn phone() -> ParameterValidation {
        ParameterValidation::builder()
            .required()
            .regex(r"^\+?[1-9]\d{1,14}$")
            .build()
    }

    /// Required string with length constraints
    pub fn required_string(
        min_length: Option<usize>,
        max_length: Option<usize>,
    ) -> ParameterValidation {
        let mut builder = ParameterValidation::builder().required();

        if let Some(min) = min_length {
            builder = builder.min_length(min);
        }

        if let Some(max) = max_length {
            builder = builder.max_length(max);
        }

        builder.build()
    }

    /// Numeric range validation
    pub fn number_range(min: Option<f64>, max: Option<f64>) -> ParameterValidation {
        let mut builder = ParameterValidation::builder().required();

        if let (Some(min_val), Some(max_val)) = (min, max) {
            builder = builder.between(min_val, max_val);
        } else {
            if let Some(min_val) = min {
                builder = builder.greater_than_or_equal(min_val);
            }
            if let Some(max_val) = max {
                builder = builder.less_than_or_equal(max_val);
            }
        }

        builder.build()
    }

    /// Password confirmation validation
    pub fn password_confirmation(password_field: ParameterKey) -> ParameterValidation {
        ParameterValidation::builder()
            .required()
            .equals_field(password_field)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameter::ParameterValue;
    use crate::value::Value;

    #[test]
    fn test_basic_validation() {
        let validation = ParameterValidation::builder()
            .required()
            .min_length(5)
            .build();

        let field = ParameterKey::new("test_field").unwrap();
        let values = HashMap::new();

        // Valid value
        let valid_value = ParameterValue::new(Value::string("hello world"));
        assert!(validation.validate(&valid_value, &field, &values).is_ok());

        // Too short
        let short_value = ParameterValue::new(Value::string("hi"));
        assert!(validation.validate(&short_value, &field, &values).is_err());

        // Empty value
        let empty_value = ParameterValue::new(Value::string(""));
        assert!(validation.validate(&empty_value, &field, &values).is_err());
    }

    #[test]
    fn test_cross_field_validation() {
        let validation = ParameterValidation::builder()
            .equals_field(ParameterKey::new("password").unwrap())
            .build();

        let field = ParameterKey::new("confirm_password").unwrap();
        let password_field = ParameterKey::new("password").unwrap();

        let mut values = HashMap::new();
        values.insert(
            password_field,
            ParameterValue::new(Value::string("secret123")),
        );

        // Matching passwords
        let matching_value = ParameterValue::new(Value::string("secret123"));
        assert!(validation
            .validate(&matching_value, &field, &values)
            .is_ok());

        // Non-matching passwords
        let non_matching_value = ParameterValue::new(Value::string("different"));
        assert!(validation
            .validate(&non_matching_value, &field, &values)
            .is_err());
    }

    #[test]
    fn test_conditional_validation() {
        let validation = ParameterValidation::builder()
            .required_if(
                ParameterKey::new("notification_type").unwrap(),
                ValidationCondition::equals("email"),
            )
            .build();

        let field = ParameterKey::new("email_address").unwrap();
        let notification_field = ParameterKey::new("notification_type").unwrap();

        let mut values = HashMap::new();

        // Email notification - email required
        values.insert(
            notification_field.clone(),
            ParameterValue::new(Value::string("email")),
        );
        let empty_email = ParameterValue::new(Value::string(""));
        assert!(validation.validate(&empty_email, &field, &values).is_err());

        let valid_email = ParameterValue::new(Value::string("test@example.com"));
        assert!(validation.validate(&valid_email, &field, &values).is_ok());

        // SMS notification - email not required
        values.insert(
            notification_field,
            ParameterValue::new(Value::string("sms")),
        );
        assert!(validation.validate(&empty_email, &field, &values).is_ok());
    }

    #[test]
    fn test_group_validation() {
        let validation = ParameterValidation::builder()
            .one_of(vec![
                ParameterKey::new("email").unwrap(),
                ParameterKey::new("phone").unwrap(),
            ])
            .build();

        let field = ParameterKey::new("contact").unwrap();
        let email_field = ParameterKey::new("email").unwrap();
        let phone_field = ParameterKey::new("phone").unwrap();

        let mut values = HashMap::new();

        // No contact info - should fail
        let empty_value = ParameterValue::new(Value::string(""));
        assert!(validation.validate(&empty_value, &field, &values).is_err());

        // Email provided - should pass
        values.insert(
            email_field,
            ParameterValue::new(Value::string("test@example.com")),
        );
        assert!(validation.validate(&empty_value, &field, &values).is_ok());

        // Both email and phone - should pass (one_of means "at least one")
        values.insert(
            phone_field,
            ParameterValue::new(Value::string("+1234567890")),
        );
        assert!(validation.validate(&empty_value, &field, &values).is_ok());
    }

    #[test]
    fn test_validators_module() {
        // Test email validator
        let email_validation = validators::email();
        let field = ParameterKey::new("email").unwrap();
        let values = HashMap::new();

        let valid_email = ParameterValue::new(Value::string("test@example.com"));
        assert!(email_validation
            .validate(&valid_email, &field, &values)
            .is_ok());

        let invalid_email = ParameterValue::new(Value::string("invalid-email"));
        assert!(email_validation
            .validate(&invalid_email, &field, &values)
            .is_err());

        // Test password confirmation
        let password_field = ParameterKey::new("password").unwrap();
        let confirm_validation = validators::password_confirmation(password_field.clone());
        let confirm_field = ParameterKey::new("confirm_password").unwrap();

        let mut values = HashMap::new();
        values.insert(
            password_field,
            ParameterValue::new(Value::string("secret123")),
        );

        let matching_confirm = ParameterValue::new(Value::string("secret123"));
        assert!(confirm_validation
            .validate(&matching_confirm, &confirm_field, &values)
            .is_ok());

        let non_matching_confirm = ParameterValue::new(Value::string("different"));
        assert!(confirm_validation
            .validate(&non_matching_confirm, &confirm_field, &values)
            .is_err());
    }
}
