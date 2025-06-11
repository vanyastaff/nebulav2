// src/validation/display.rs

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::value::{Value, ValueComparison};
use crate::parameter::ParameterValue;
use crate::types::ParameterKey;

/// Conditions for showing/hiding fields in the UI
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DisplayCondition {
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
    /// All conditions must be met (logical AND)
    And(Vec<DisplayCondition>),
    /// At least one condition must be met (logical OR)
    Or(Vec<DisplayCondition>),
    /// Condition must NOT be met (logical NOT)
    Not(Box<DisplayCondition>),
}

impl DisplayCondition {
    /// Checks the display condition for the specified value
    ///
    /// Returns `true` if the field should be displayed according to the condition,
    /// `false` otherwise or on error (fail-safe approach for UI)
    pub fn check(&self, value: &Value) -> bool {
        match self {
            Self::Eq(expected) => ValueComparison::equals(value, expected),
            Self::NotEq(expected) => ValueComparison::not_equals(value, expected),
            Self::Gt(expected) => ValueComparison::gt_simple(value, expected),
            Self::Lt(expected) => ValueComparison::lt_simple(value, expected),
            Self::Gte(expected) => ValueComparison::gte_simple(value, expected),
            Self::Lte(expected) => ValueComparison::lte_simple(value, expected),
            Self::In(list) => ValueComparison::in_list(value, list),
            Self::NotIn(list) => ValueComparison::not_in_list(value, list),
            Self::IsEmpty => ValueComparison::is_empty(value),
            Self::IsNotEmpty => ValueComparison::is_not_empty(value),

            Self::And(conditions) => {
                conditions.iter().all(|condition| condition.check(value))
            }
            Self::Or(conditions) => {
                conditions.iter().any(|condition| condition.check(value))
            }
            Self::Not(condition) => {
                !condition.check(value)
            }
        }
    }

    /// Checks the condition with ParameterValue
    ///
    /// Thanks to Deref, we can directly use the inner Value
    pub fn check_parameter_value(&self, param_value: &ParameterValue) -> bool {
        self.check(param_value) // ← Deref автоматически работает!
    }
}

// Builder methods for convenient condition creation
impl DisplayCondition {
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
    pub fn is_in<T: Into<Value>>(values: Vec<T>) -> Self {
        Self::In(values.into_iter().map(Into::into).collect())
    }

    /// Creates a "NOT in list" condition
    pub fn is_not_in<T: Into<Value>>(values: Vec<T>) -> Self {
        Self::NotIn(values.into_iter().map(Into::into).collect())
    }

    /// Creates an "empty value" condition
    pub fn is_empty() -> Self {
        Self::IsEmpty
    }

    /// Creates a "non-empty value" condition
    pub fn is_not_empty() -> Self {
        Self::IsNotEmpty
    }

    /// Creates a logical AND from multiple conditions
    pub fn and(conditions: Vec<Self>) -> Self {
        Self::And(conditions)
    }

    /// Creates a logical OR from multiple conditions
    pub fn or(conditions: Vec<Self>) -> Self {
        Self::Or(conditions)
    }

    /// Creates a logical NOT for a condition
    pub fn not(condition: Self) -> Self {
        Self::Not(Box::new(condition))
    }
}

/// Parameter display rules for UI
///
/// Defines when a parameter should be shown or hidden
/// based on values of other parameters (cross-field logic)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ParameterDisplay {
    /// Conditions for hiding the field
    ///
    /// Key - name of the field whose values are checked
    /// Value - list of conditions, if any of them is met, the current field is hidden
    pub hide: Option<HashMap<ParameterKey, Vec<DisplayCondition>>>,

    /// Conditions for showing the field
    ///
    /// Key - name of the field whose values are checked
    /// Value - list of conditions, if any of them is met, the current field is shown
    pub show: Option<HashMap<ParameterKey, Vec<DisplayCondition>>>,
}

impl ParameterDisplay {
    /// Creates new empty display rules
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if the field should be displayed based on other field values
    ///
    /// # Logic:
    /// 1. If there are `show` rules and none are met - hide the field
    /// 2. If there are `hide` rules and any are met - hide the field
    /// 3. Otherwise - show the field (fields are visible by default)
    pub fn should_display(&self, all_values: &HashMap<ParameterKey, ParameterValue>) -> bool {
        // Check show conditions
        if let Some(show_rules) = &self.show {
            let should_show = show_rules.iter().any(|(field_key, conditions)| {
                if let Some(param_value) = all_values.get(field_key) {
                    // Thanks to Deref, param_value can be used directly as &Value
                    conditions.iter().any(|condition| condition.check(param_value))
                } else {
                    false // Field is missing - condition not met
                }
            });

            if !should_show {
                return false; // No show condition is met
            }
        }

        // Check hide conditions
        if let Some(hide_rules) = &self.hide {
            let should_hide = hide_rules.iter().any(|(field_key, conditions)| {
                if let Some(param_value) = all_values.get(field_key) {
                    // Thanks to Deref, param_value can be used directly as &Value
                    conditions.iter().any(|condition| condition.check(param_value))
                } else {
                    false // Field is missing - condition not met
                }
            });

            if should_hide {
                return false; // At least one hide condition is met
            }
        }

        true // Show field by default
    }

    /// Checks if the field should be displayed for a specific field
    ///
    /// More specific version that checks only rules
    /// related to the specified field
    pub fn should_display_field(
        &self,
        _field_key: &ParameterKey,
        all_values: &HashMap<ParameterKey, ParameterValue>,
    ) -> bool {
        // Similar to should_display, but can add logic
        // specific to the concrete field
        self.should_display(all_values)
    }

    /// Adds a hide condition
    pub fn add_hide_condition(
        &mut self,
        target_field: ParameterKey,
        condition: DisplayCondition,
    ) -> &mut Self {
        self.hide
            .get_or_insert_with(HashMap::new)
            .entry(target_field)
            .or_insert_with(Vec::new)
            .push(condition);
        self
    }

    /// Adds a show condition
    pub fn add_show_condition(
        &mut self,
        target_field: ParameterKey,
        condition: DisplayCondition,
    ) -> &mut Self {
        self.show
            .get_or_insert_with(HashMap::new)
            .entry(target_field)
            .or_insert_with(Vec::new)
            .push(condition);
        self
    }

    /// Adds multiple hide conditions for one field
    pub fn add_hide_conditions(
        &mut self,
        target_field: ParameterKey,
        conditions: Vec<DisplayCondition>,
    ) -> &mut Self {
        self.hide
            .get_or_insert_with(HashMap::new)
            .entry(target_field)
            .or_insert_with(Vec::new)
            .extend(conditions);
        self
    }

    /// Adds multiple show conditions for one field
    pub fn add_show_conditions(
        &mut self,
        target_field: ParameterKey,
        conditions: Vec<DisplayCondition>,
    ) -> &mut Self {
        self.show
            .get_or_insert_with(HashMap::new)
            .entry(target_field)
            .or_insert_with(Vec::new)
            .extend(conditions);
        self
    }

    /// Checks if there are any display rules
    pub fn has_rules(&self) -> bool {
        self.hide.is_some() || self.show.is_some()
    }

    /// Checks if there are hide rules
    pub fn has_hide_rules(&self) -> bool {
        self.hide.as_ref().map_or(false, |rules| !rules.is_empty())
    }

    /// Checks if there are show rules
    pub fn has_show_rules(&self) -> bool {
        self.show.as_ref().map_or(false, |rules| !rules.is_empty())
    }

    /// Returns all fields that the display depends on
    pub fn dependent_fields(&self) -> Vec<&ParameterKey> {
        let mut fields = Vec::new();

        if let Some(hide_rules) = &self.hide {
            fields.extend(hide_rules.keys());
        }

        if let Some(show_rules) = &self.show {
            fields.extend(show_rules.keys());
        }

        fields.sort();
        fields.dedup();
        fields
    }
}

/// Builder for creating display rules
#[derive(Debug, Default)]
pub struct ParameterDisplayBuilder {
    display: ParameterDisplay,
}

impl ParameterDisplayBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a hide condition
    pub fn hide_when(
        mut self,
        field: ParameterKey,
        condition: DisplayCondition,
    ) -> Self {
        self.display.add_hide_condition(field, condition);
        self
    }

    /// Adds a show condition
    pub fn show_when(
        mut self,
        field: ParameterKey,
        condition: DisplayCondition,
    ) -> Self {
        self.display.add_show_condition(field, condition);
        self
    }

    /// Adds a hide condition by equality
    pub fn hide_when_equals<T: Into<Value>>(
        self,
        field: ParameterKey,
        value: T,
    ) -> Self {
        self.hide_when(field, DisplayCondition::equals(value))
    }

    /// Adds a show condition by equality
    pub fn show_when_equals<T: Into<Value>>(
        self,
        field: ParameterKey,
        value: T,
    ) -> Self {
        self.show_when(field, DisplayCondition::equals(value))
    }

    /// Adds a hide condition by list inclusion
    pub fn hide_when_in<T: Into<Value>>(
        self,
        field: ParameterKey,
        values: Vec<T>,
    ) -> Self {
        self.hide_when(field, DisplayCondition::is_in(values))
    }

    /// Adds a show condition by list inclusion
    pub fn show_when_in<T: Into<Value>>(
        self,
        field: ParameterKey,
        values: Vec<T>,
    ) -> Self {
        self.show_when(field, DisplayCondition::is_in(values))
    }

    /// Adds a hide condition for empty value
    pub fn hide_when_empty(self, field: ParameterKey) -> Self {
        self.hide_when(field, DisplayCondition::is_empty())
    }

    /// Adds a show condition for non-empty value
    pub fn show_when_not_empty(self, field: ParameterKey) -> Self {
        self.show_when(field, DisplayCondition::is_not_empty())
    }

    /// Builds the final ParameterDisplay object
    pub fn build(self) -> ParameterDisplay {
        self.display
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;
    use crate::parameter::ParameterValue;

    #[test]
    fn test_display_condition_equals() {
        let condition = DisplayCondition::equals("test");
        assert!(condition.check(&Value::string("test")));
        assert!(!condition.check(&Value::string("other")));
    }

    #[test]
    fn test_display_condition_in_list() {
        let condition = DisplayCondition::is_in(vec!["a", "b", "c"]);
        assert!(condition.check(&Value::string("b")));
        assert!(!condition.check(&Value::string("d")));
    }

    #[test]
    fn test_display_condition_and() {
        let condition = DisplayCondition::and(vec![
            DisplayCondition::equals("test"),
            DisplayCondition::is_not_empty(),
        ]);

        assert!(condition.check(&Value::string("test")));
        assert!(!condition.check(&Value::string("other")));
        assert!(!condition.check(&Value::string("")));
    }

    #[test]
    fn test_display_condition_not() {
        let condition = DisplayCondition::not(DisplayCondition::equals("test"));
        assert!(!condition.check(&Value::string("test")));
        assert!(condition.check(&Value::string("other")));
    }

    #[test]
    fn test_display_condition_with_parameter_value() {
        let condition = DisplayCondition::equals("test");
        let param_value = ParameterValue::new(Value::string("test"));

        // Thanks to Deref, we can call check directly
        assert!(condition.check(&param_value));
        // Or use the explicit method
        assert!(condition.check_parameter_value(&param_value));
    }

    #[test]
    fn test_parameter_display_should_display() {
        let mut display = ParameterDisplay::new();
        display.add_show_condition(
            ParameterKey::new("mode").unwrap(),
            DisplayCondition::equals("advanced"),
        );

        let mut values = HashMap::new();
        values.insert(
            ParameterKey::new("mode").unwrap(),
            ParameterValue::new(Value::string("advanced"))
        );

        assert!(display.should_display(&values));

        values.insert(
            ParameterKey::new("mode").unwrap(),
            ParameterValue::new(Value::string("simple"))
        );
        assert!(!display.should_display(&values));
    }

    #[test]
    fn test_parameter_display_builder() {
        let display = ParameterDisplayBuilder::new()
            .show_when_equals(ParameterKey::new("type").unwrap(), "oauth2")
            .hide_when_empty(ParameterKey::new("api_key").unwrap())
            .build();

        assert!(display.has_rules());
        assert!(display.has_show_rules());
        assert!(display.has_hide_rules());
    }

    #[test]
    fn test_cross_field_display_logic() {
        let mut values = HashMap::new();
        values.insert(
            ParameterKey::new("notification_type").unwrap(),
            ParameterValue::new(Value::string("email"))
        );
        values.insert(
            ParameterKey::new("email_address").unwrap(),
            ParameterValue::new(Value::string("test@example.com"))
        );

        // Show email settings only if notification_type = "email" AND email is not empty
        let display = ParameterDisplayBuilder::new()
            .show_when(
                ParameterKey::new("notification_type").unwrap(),
                DisplayCondition::equals("email")
            )
            .show_when(
                ParameterKey::new("email_address").unwrap(),
                DisplayCondition::is_not_empty()
            )
            .build();

        assert!(display.should_display(&values));

        // Change notification type - should hide
        values.insert(
            ParameterKey::new("notification_type").unwrap(),
            ParameterValue::new(Value::string("sms"))
        );
        assert!(!display.should_display(&values));
    }

    #[test]
    fn test_deref_behavior() {
        let param_value = ParameterValue::new(Value::string("test"));
        let condition = DisplayCondition::equals("test");

        // Thanks to Deref, ParameterValue can be used directly as &Value
        assert!(condition.check(&param_value));

        // We can also access Value methods directly
        assert_eq!(param_value.as_string(), Some("test"));
        assert!(param_value.is_string());
    }
}