use crate::parameter::{Parameter, ParameterError, ParameterValue};
use crate::types::ParameterKey;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Collection of parameters with type-safe access and management
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ParameterCollection {
    /// Ordered map of parameters (preserves declaration order)
    parameters: IndexMap<ParameterKey, Box<dyn Parameter>>,
}

impl ParameterCollection {
    /// Creates a new empty parameter collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a parameter collection with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parameters: IndexMap::with_capacity(capacity),
        }
    }

    /// Adds a parameter to the collection
    pub fn add_parameter(&mut self, parameter: Box<dyn Parameter>) -> Result<(), ParameterError> {
        let key = parameter.key().clone();

        if self.parameters.contains_key(&key) {
            return Err(ParameterError::AlreadyExists(key));
        }

        self.parameters.insert(key, parameter);
        Ok(())
    }

    /// Adds a parameter with builder pattern
    pub fn with_parameter(mut self, parameter: Box<dyn Parameter>) -> Result<Self, ParameterError> {
        self.add_parameter(parameter)?;
        Ok(self)
    }

    /// Gets a parameter by key
    pub fn get_parameter(&self, key: &ParameterKey) -> Option<&dyn Parameter> {
        self.parameters.get(key).map(|p| p.as_ref())
    }

    /// Gets a mutable parameter by key
    pub fn get_parameter_mut(&mut self, key: &ParameterKey) -> Option<&mut dyn Parameter> {
        self.parameters.get_mut(key).map(|p| p.as_mut())
    }

    /// Gets a parameter as a specific type
    pub fn get_parameter_as<T: Parameter + 'static>(&self, key: &ParameterKey) -> Option<&T> {
        self.parameters.get(key)?.as_any().downcast_ref::<T>()
    }

    /// Gets a mutable parameter as a specific type
    pub fn get_parameter_as_mut<T: Parameter + 'static>(
        &mut self,
        key: &ParameterKey,
    ) -> Option<&mut T> {
        self.parameters
            .get_mut(key)?
            .as_any_mut()
            .downcast_mut::<T>()
    }

    /// Checks if a parameter exists
    pub fn contains_parameter(&self, key: &ParameterKey) -> bool {
        self.parameters.contains_key(key)
    }

    /// Returns the number of parameters
    pub fn len(&self) -> usize {
        self.parameters.len()
    }

    /// Returns true if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.parameters.is_empty()
    }

    /// Returns an iterator over parameter keys
    pub fn keys(&self) -> impl Iterator<Item = &ParameterKey> {
        self.parameters.keys()
    }

    /// Returns an iterator over parameters
    pub fn parameters(&self) -> impl Iterator<Item = &dyn Parameter> {
        self.parameters.values().map(|p| p.as_ref())
    }

    /// Returns an iterator over key-parameter pairs
    pub fn iter(&self) -> impl Iterator<Item = (&ParameterKey, &dyn Parameter)> {
        self.parameters.iter().map(|(k, p)| (k, p.as_ref()))
    }

    /// Returns a mutable iterator over parameters
    pub fn parameters_mut(&mut self) -> impl Iterator<Item = &mut dyn Parameter> {
        self.parameters.values_mut().map(|p| p.as_mut())
    }

    /// Returns a mutable iterator over key-parameter pairs
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ParameterKey, &mut dyn Parameter)> {
        self.parameters.iter_mut().map(|(k, p)| (k, p.as_mut()))
    }

    /// Creates parameter values from input, applying defaults and validation
    pub fn create_values(
        &self,
        input: HashMap<ParameterKey, ParameterValue>,
    ) -> Result<HashMap<ParameterKey, ParameterValue>, ParameterError> {
        let mut result = HashMap::new();

        // Process each parameter
        for (key, parameter) in &self.parameters {
            let value = if let Some(input_value) = input.get(key) {
                // Use provided value
                input_value.clone()
            } else if let Some(default) = parameter.get_value() {
                // Use default value
                default.clone()
            } else if parameter.metadata().required {
                // Required parameter missing
                return Err(ParameterError::NotFound(key.clone()));
            } else {
                // Optional parameter, skip
                continue;
            };

            result.insert(key.clone(), value);
        }

        Ok(result)
    }

    /// Validates a complete set of parameter values
    pub fn validate_values(
        &self,
        values: &HashMap<ParameterKey, ParameterValue>,
    ) -> Result<(), Vec<ParameterError>> {
        let mut errors = Vec::new();

        // Validate each parameter
        for (key, parameter) in &self.parameters {
            if let Some(value) = values.get(key) {
                // Individual parameter validation
                unimplemented!();
                // if let Err(validation_error) = parameter.validate_value(value, values) {
                //     errors.push(ParameterError::ValidationError {
                //         key: key.clone(),
                //         reason: validation_error.to_string(),
                //     });
                // }
            } else if parameter.metadata().required {
                errors.push(ParameterError::NotFound(key.clone()));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Checks which parameters should be displayed based on current values
    pub fn evaluate_display(
        &self,
        values: &HashMap<ParameterKey, ParameterValue>,
    ) -> HashMap<ParameterKey, bool> {
        let mut display_state = HashMap::new();

        for (key, parameter) in &self.parameters {
            let should_display = parameter.should_display(values);
            display_state.insert(key.clone(), should_display);
        }

        display_state
    }

    /// Returns parameters that have display conditions
    pub fn get_conditional_parameters(&self) -> Vec<&ParameterKey> {
        self.parameters
            .iter()
            .filter(|(_, param)| param.display_conditions().is_some())
            .map(|(key, _)| key)
            .collect()
    }

    /// Returns parameters that are required
    pub fn get_required_parameters(&self) -> Vec<&ParameterKey> {
        self.parameters
            .iter()
            .filter(|(_, param)| param.metadata().required)
            .map(|(key, _)| key)
            .collect()
    }

    /// Returns parameters of a specific kind
    pub fn get_parameters_by_kind(
        &self,
        kind: crate::parameter::ParameterKind,
    ) -> Vec<&ParameterKey> {
        self.parameters
            .iter()
            .filter(|(_, param)| param.kind() == kind)
            .map(|(key, _)| key)
            .collect()
    }

    /// Merges another collection into this one
    pub fn merge(&mut self, other: ParameterCollection) -> Result<(), ParameterError> {
        for (key, parameter) in other.parameters {
            if self.parameters.contains_key(&key) {
                return Err(ParameterError::AlreadyExists(key));
            }
            self.parameters.insert(key, parameter);
        }

        Ok(())
    }

    /// Creates a subset collection with only specified parameters
    pub fn subset(&self, keys: &[ParameterKey]) -> Result<ParameterCollection, ParameterError> {
        let mut result = ParameterCollection::new();

        for key in keys {
            if let Some(parameter) = self.parameters.get(key) {
                // Clone the parameter (requires Clone on Parameter trait)
                result
                    .parameters
                    .insert(key.clone(), parameter.clone());
            } else {
                return Err(ParameterError::NotFound(key.clone()));
            }
        }

        Ok(result)
    }
}

/// Builder for constructing ParameterCollection with fluent API
#[derive(Debug, Default)]
pub struct ParameterCollectionBuilder {
    collection: ParameterCollection,
}

impl ParameterCollectionBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a parameter
    pub fn parameter(mut self, parameter: Box<dyn Parameter>) -> Result<Self, ParameterError> {
        self.collection.add_parameter(parameter)?;
        Ok(self)
    }

    /// Builds the final collection
    pub fn build(self) -> ParameterCollection {
        self.collection
    }
}
