//! Context for template evaluation

use crate::{
    error::{Error, Result},
    value::Value,
};
use std::collections::HashMap;

/// Data source types for template expressions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataSource {
    /// Current input data - `$input`
    Input,
    /// Output from another node - `$node('node_id')`
    Node(String),
    /// System data (time, date, etc.) - `$system`
    System,
    /// Execution metadata - `$execution`
    Execution,
    /// Environment variables - `$env`
    Environment,
    /// Workflow information - `$workflow`
    Workflow,
}

impl DataSource {
    /// Get the string representation of the data source
    pub fn as_str(&self) -> &str {
        match self {
            Self::Input => "$input",
            Self::Node(_) => "$node",
            Self::System => "$system",
            Self::Execution => "$execution",
            Self::Environment => "$env",
            Self::Workflow => "$workflow",
        }
    }

    /// Create a node data source
    pub fn node(id: impl Into<String>) -> Self {
        Self::Node(id.into())
    }
}

/// System data that's always available
#[derive(Debug, Clone)]
pub struct SystemData {
    data: HashMap<String, Value>,
}

impl SystemData {
    /// Create new system data
    pub fn new() -> Self {
        let mut data = HashMap::new();

        // Add current timestamp
        #[cfg(feature = "chrono")]
        {
            use chrono::Utc;
            let now = Utc::now();

            let mut datetime = HashMap::new();
            datetime.insert("now".to_string(), Value::string(now.to_rfc3339()));
            datetime.insert("timestamp".to_string(), Value::integer(now.timestamp()));
            datetime.insert("iso".to_string(), Value::string(now.format("%Y-%m-%dT%H:%M:%SZ").to_string()));
            datetime.insert("date".to_string(), Value::string(now.format("%Y-%m-%d").to_string()));
            datetime.insert("time".to_string(), Value::string(now.format("%H:%M:%S").to_string()));

            data.insert("datetime".to_string(), Value::object(datetime));
        }

        #[cfg(not(feature = "chrono"))]
        {
            // Basic timestamp using std
            use std::time::{SystemTime, UNIX_EPOCH};
            if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
                let mut datetime = HashMap::new();
                datetime.insert("timestamp".to_string(), Value::integer(duration.as_secs() as i64));
                data.insert("datetime".to_string(), Value::object(datetime));
            }
        }

        Self { data }
    }

    /// Get a system value by path
    pub fn get(&self, path: &str) -> Option<&Value> {
        if path.is_empty() {
            return Some(&Value::object(self.data.clone()));
        }

        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &Value::object(self.data.clone());

        for part in parts {
            current = current.get(part)?;
        }

        Some(current)
    }

    /// Add or update a system value
    pub fn set(&mut self, key: String, value: Value) {
        self.data.insert(key, value);
    }
}

impl Default for SystemData {
    fn default() -> Self {
        Self::new()
    }
}

/// Execution metadata
#[derive(Debug, Clone, Default)]
pub struct ExecutionData {
    data: HashMap<String, Value>,
}

impl ExecutionData {
    /// Create new execution data
    pub fn new() -> Self {
        Self::default()
    }

    /// Set execution metadata
    pub fn set(&mut self, key: impl Into<String>, value: Value) {
        self.data.insert(key.into(), value);
    }

    /// Get execution metadata
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    /// Get all execution data as a value
    pub fn as_value(&self) -> Value {
        Value::object(self.data.clone())
    }
}

/// Workflow metadata
#[derive(Debug, Clone, Default)]
pub struct WorkflowData {
    data: HashMap<String, Value>,
}

impl WorkflowData {
    /// Create new workflow data
    pub fn new() -> Self {
        Self::default()
    }

    /// Set workflow metadata
    pub fn set(&mut self, key: impl Into<String>, value: Value) {
        self.data.insert(key.into(), value);
    }

    /// Get workflow metadata
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    /// Get all workflow data as a value
    pub fn as_value(&self) -> Value {
        Value::object(self.data.clone())
    }
}

/// Context for template evaluation containing all available data sources
#[derive(Debug, Clone)]
pub struct Context {
    /// Current input data
    input_data: Option<Value>,
    /// Node outputs by node ID
    node_outputs: HashMap<String, Value>,
    /// System data (dates, times, etc.)
    system_data: SystemData,
    /// Execution metadata
    execution_data: ExecutionData,
    /// Environment variables
    env_vars: HashMap<String, String>,
    /// Workflow metadata
    workflow_data: WorkflowData,
}

impl Context {
    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            input_data: None,
            node_outputs: HashMap::new(),
            system_data: SystemData::new(),
            execution_data: ExecutionData::new(),
            env_vars: HashMap::new(),
            workflow_data: WorkflowData::new(),
        }
    }

    /// Set the input data
    pub fn set_input(&mut self, data: Value) {
        self.input_data = Some(data);
    }

    /// Get the input data
    pub fn get_input(&self) -> Option<&Value> {
        self.input_data.as_ref()
    }

    /// Add node output data
    pub fn add_node_output(&mut self, node_id: impl Into<String>, data: Value) {
        self.node_outputs.insert(node_id.into(), data);
    }

    /// Get node output data
    pub fn get_node_output(&self, node_id: &str) -> Option<&Value> {
        self.node_outputs.get(node_id)
    }

    /// Set environment variable
    pub fn set_env(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.env_vars.insert(key.into(), value.into());
    }

    /// Get environment variable
    pub fn get_env(&self, key: &str) -> Option<&str> {
        self.env_vars.get(key).map(|s| s.as_str())
    }

    /// Set execution metadata
    pub fn set_execution_data(&mut self, key: impl Into<String>, value: Value) {
        self.execution_data.set(key, value);
    }

    /// Get execution metadata
    pub fn get_execution_data(&self, key: &str) -> Option<&Value> {
        self.execution_data.get(key)
    }

    /// Set workflow metadata
    pub fn set_workflow_data(&mut self, key: impl Into<String>, value: Value) {
        self.workflow_data.set(key, value);
    }

    /// Get workflow metadata
    pub fn get_workflow_data(&self, key: &str) -> Option<&Value> {
        self.workflow_data.get(key)
    }

    /// Get system data
    pub fn get_system_data(&self) -> &SystemData {
        &self.system_data
    }

    /// Get mutable system data
    pub fn get_system_data_mut(&mut self) -> &mut SystemData {
        &mut self.system_data
    }

    /// Resolve a data source to a value
    pub fn resolve_data_source(&self, source: &DataSource, path: &str) -> Result<Value> {
        match source {
            DataSource::Input => {
                if let Some(input) = &self.input_data {
                    if path.is_empty() {
                        Ok(input.clone())
                    } else if let Some(value) = input.navigate(path) {
                        Ok(value.clone())
                    } else {
                        Err(Error::data_not_found(
                            format!("$input.{}", path),
                            vec!["$input".to_string()],
                        ))
                    }
                } else {
                    Err(Error::data_not_found(
                        "$input".to_string(),
                        vec!["No input data available".to_string()],
                    ))
                }
            }
            DataSource::Node(node_id) => {
                if let Some(output) = self.node_outputs.get(node_id) {
                    if path.is_empty() {
                        Ok(output.clone())
                    } else if let Some(value) = output.navigate(path) {
                        Ok(value.clone())
                    } else {
                        Err(Error::data_not_found(
                            format!("$node('{}').{}", node_id, path),
                            vec![format!("$node('{}')", node_id)],
                        ))
                    }
                } else {
                    let available: Vec<String> = self.node_outputs.keys()
                        .map(|k| format!("$node('{}')", k))
                        .collect();
                    Err(Error::data_not_found(
                        format!("$node('{}')", node_id),
                        available,
                    ))
                }
            }
            DataSource::System => {
                if let Some(value) = self.system_data.get(path) {
                    Ok(value.clone())
                } else {
                    Err(Error::data_not_found(
                        format!("$system.{}", path),
                        vec!["$system.datetime".to_string()],
                    ))
                }
            }
            DataSource::Execution => {
                if path.is_empty() {
                    Ok(self.execution_data.as_value())
                } else if let Some(value) = self.execution_data.get(path) {
                    Ok(value.clone())
                } else {
                    let available: Vec<String> = self.execution_data.data.keys()
                        .map(|k| format!("$execution.{}", k))
                        .collect();
                    Err(Error::data_not_found(
                        format!("$execution.{}", path),
                        available,
                    ))
                }
            }
            DataSource::Environment => {
                if let Some(value) = self.env_vars.get(path) {
                    Ok(Value::string(value.clone()))
                } else {
                    let available: Vec<String> = self.env_vars.keys()
                        .map(|k| format!("$env.{}", k))
                        .collect();
                    Err(Error::data_not_found(
                        format!("$env.{}", path),
                        available,
                    ))
                }
            }
            DataSource::Workflow => {
                if path.is_empty() {
                    Ok(self.workflow_data.as_value())
                } else if let Some(value) = self.workflow_data.get(path) {
                    Ok(value.clone())
                } else {
                    let available: Vec<String> = self.workflow_data.data.keys()
                        .map(|k| format!("$workflow.{}", k))
                        .collect();
                    Err(Error::data_not_found(
                        format!("$workflow.{}", path),
                        available,
                    ))
                }
            }
        }
    }

    /// Get all available data sources for debugging
    pub fn available_data_sources(&self) -> Vec<String> {
        let mut sources = Vec::new();

        if self.input_data.is_some() {
            sources.push("$input".to_string());
        }

        for node_id in self.node_outputs.keys() {
            sources.push(format!("$node('{}')", node_id));
        }

        sources.push("$system".to_string());
        sources.push("$execution".to_string());
        sources.push("$workflow".to_string());

        for env_key in self.env_vars.keys() {
            sources.push(format!("$env.{}", env_key));
        }

        sources
    }

    /// Check if a data source exists
    pub fn has_data_source(&self, source: &DataSource) -> bool {
        match source {
            DataSource::Input => self.input_data.is_some(),
            DataSource::Node(node_id) => self.node_outputs.contains_key(node_id),
            DataSource::System => true, // Always available
            DataSource::Execution => true, // Always available (might be empty)
            DataSource::Environment => true, // Always available (might be empty)
            DataSource::Workflow => true, // Always available (might be empty)
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let context = Context::new();
        assert!(context.get_input().is_none());
        assert!(context.available_data_sources().contains(&"$system".to_string()));
    }

    #[test]
    fn test_input_data() {
        let mut context = Context::new();
        let input = Value::string("test");
        context.set_input(input.clone());
        assert_eq!(context.get_input(), Some(&input));
    }

    #[test]
    fn test_node_output() {
        let mut context = Context::new();
        let output = Value::integer(42);
        context.add_node_output("test_node", output.clone());
        assert_eq!(context.get_node_output("test_node"), Some(&output));
    }

    #[test]
    fn test_environment_variables() {
        let mut context = Context::new();
        context.set_env("TEST_VAR", "test_value");
        assert_eq!(context.get_env("TEST_VAR"), Some("test_value"));
    }

    #[test]
    fn test_data_source_resolution() -> Result<()> {
        let mut context = Context::new();

        // Test input resolution
        context.set_input(Value::string("test"));
        let result = context.resolve_data_source(&DataSource::Input, "")?;
        assert_eq!(result, Value::string("test"));

        // Test node resolution
        context.add_node_output("test", Value::integer(42));
        let result = context.resolve_data_source(&DataSource::Node("test".to_string()), "")?;
        assert_eq!(result, Value::integer(42));

        // Test env resolution
        context.set_env("TEST", "value");
        let result = context.resolve_data_source(&DataSource::Environment, "TEST")?;
        assert_eq!(result, Value::string("value"));

        Ok(())
    }

    #[test]
    fn test_data_source_not_found() {
        let context = Context::new();

        let result = context.resolve_data_source(&DataSource::Input, "");
        assert!(result.is_err());

        let result = context.resolve_data_source(&DataSource::Node("missing".to_string()), "");
        assert!(result.is_err());

        let result = context.resolve_data_source(&DataSource::Environment, "MISSING");
        assert!(result.is_err());
    }

    #[test]
    fn test_system_data() {
        let context = Context::new();
        let system = context.get_system_data();

        // Should have datetime information
        assert!(system.get("datetime").is_some());
    }

    #[test]
    fn test_data_source_enum() {
        assert_eq!(DataSource::Input.as_str(), "$input");
        assert_eq!(DataSource::System.as_str(), "$system");
        assert_eq!(DataSource::node("test").as_str(), "$node");

        if let DataSource::Node(id) = DataSource::node("test") {
            assert_eq!(id, "test");
        } else {
            panic!("Expected Node variant");
        }
    }
}