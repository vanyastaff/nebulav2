#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ValueError, ValueResult, Value};
use std::collections::HashSet;

/// Expression value for dynamic evaluation in workflows
/// Supports template syntax: "Hello {{ $node('user_data').json.name }}!"
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ExpressionValue {
    /// The expression template string
    template: String,
}

impl ExpressionValue {
    // === Construction ===

    /// Creates a new expression from a template string
    #[must_use]
    pub fn new(template: impl Into<String>) -> Self {
        Self {
            template: template.into(),
        }
    }

    /// Creates an expression from a static value (no evaluation needed)
    #[must_use]
    pub fn static_value(value: impl Into<String>) -> Self {
        Self::new(value.into())
    }

    /// Creates an expression that references a node output
    #[must_use]
    pub fn node_ref(node_id: &str, path: &str) -> Self {
        Self::new(format!("{{{{ $node('{}').json.{} }}}}", node_id, path))
    }

    /// Creates an expression that references trigger data
    #[must_use]
    pub fn trigger_ref(path: &str) -> Self {
        Self::new(format!("{{{{ $trigger.json.{} }}}}", path))
    }

    /// Creates an expression that references execution metadata
    #[must_use]
    pub fn execution_ref(field: &str) -> Self {
        Self::new(format!("{{{{ $execution.{} }}}}", field))
    }

    /// Creates an expression that references environment variable
    #[must_use]
    pub fn env_ref(var_name: &str) -> Self {
        Self::new(format!("{{{{ $env.{} }}}}", var_name))
    }

    // === Accessors ===

    /// Gets the raw template string
    #[must_use]
    pub fn template(&self) -> &str {
        &self.template
    }

    /// Gets the template as a mutable reference
    #[must_use]
    pub fn template_mut(&mut self) -> &mut String {
        &mut self.template
    }

    /// Consumes the expression and returns the template string
    #[must_use]
    pub fn into_template(self) -> String {
        self.template
    }

    // === Analysis ===

    /// Checks if this is a static value (contains no expressions)
    #[must_use]
    pub fn is_static(&self) -> bool {
        !self.has_expressions()
    }

    /// Checks if this expression contains dynamic parts
    #[must_use]
    pub fn has_expressions(&self) -> bool {
        self.template.contains("{{") && self.template.contains("}}")
    }

    /// Checks if the expression syntax is valid (balanced braces)
    #[must_use]
    pub fn is_valid(&self) -> bool {
        let mut brace_depth = 0;
        let mut chars = self.template.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '{' if chars.peek() == Some(&'{') => {
                    chars.next(); // Consume second '{'
                    brace_depth += 1;
                }
                '}' if chars.peek() == Some(&'}') => {
                    chars.next(); // Consume second '}'
                    brace_depth -= 1;
                    if brace_depth < 0 {
                        return false; // Unmatched closing braces
                    }
                }
                _ => {}
            }
        }

        brace_depth == 0 // All braces should be matched
    }

    /// Validates the expression and returns detailed errors if invalid
    pub fn validate(&self) -> ValueResult<()> {
        if !self.is_valid() {
            return Err(ValueError::InvalidExpression {
                input: self.template.clone(),
                reason: "Unbalanced braces in expression".to_string(),
            });
        }

        // Basic syntax validation for each expression segment
        for expr in self.extract_expressions() {
            self.validate_expression_syntax(&expr)?;
        }

        Ok(())
    }

    // === Variable Extraction ===

    /// Extracts all expression segments from the template
    /// Example: "Hello {{ name }} from {{ location }}" -> ["name", "location"]
    #[must_use]
    pub fn extract_expressions(&self) -> Vec<String> {
        let mut expressions = Vec::new();
        let mut in_expression = false;
        let mut current_expr = String::new();
        let mut chars = self.template.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' && chars.peek() == Some(&'{') {
                chars.next(); // Skip second '{'
                in_expression = true;
                current_expr.clear();
            } else if ch == '}' && chars.peek() == Some(&'}') && in_expression {
                chars.next(); // Skip second '}'
                in_expression = false;
                let trimmed = current_expr.trim();
                if !trimmed.is_empty() {
                    expressions.push(trimmed.to_string());
                }
                current_expr.clear();
            } else if in_expression {
                current_expr.push(ch);
            }
        }

        expressions
    }

    /// Extracts all variable references from the expression
    #[must_use]
    pub fn extract_variables(&self) -> Vec<String> {
        self.extract_expressions()
    }

    /// Extracts all variable references as a set (no duplicates)
    #[must_use]
    pub fn extract_variable_set(&self) -> HashSet<String> {
        self.extract_expressions().into_iter().collect()
    }

    /// Extracts all node references from the expression
    /// Example: "$node('user_data')" -> ["user_data"]
    #[must_use]
    pub fn extract_node_references(&self) -> Vec<String> {
        let mut node_refs = Vec::new();
        let expressions = self.extract_expressions();

        for expr in expressions {
            if let Some(node_id) = self.parse_node_reference(&expr) {
                node_refs.push(node_id);
            }
        }

        node_refs
    }

    /// Extracts all environment variable references
    /// Example: "$env.API_KEY" -> ["API_KEY"]
    #[must_use]
    pub fn extract_env_references(&self) -> Vec<String> {
        let mut env_refs = Vec::new();
        let expressions = self.extract_expressions();

        for expr in expressions {
            if let Some(env_var) = self.parse_env_reference(&expr) {
                env_refs.push(env_var);
            }
        }

        env_refs
    }

    /// Checks if the expression contains a specific variable
    #[must_use]
    pub fn contains_variable(&self, var: &str) -> bool {
        self.extract_variable_set().contains(var)
    }

    /// Checks if the expression references a specific node
    #[must_use]
    pub fn references_node(&self, node_id: &str) -> bool {
        self.extract_node_references().contains(&node_id.to_string())
    }

    // === Expression Parsing Helpers ===

    fn parse_node_reference(&self, expr: &str) -> Option<String> {
        // Parse $node('node_id') pattern
        if expr.trim().starts_with("$node(") {
            if let Some(start) = expr.find('\'') {
                if let Some(end) = expr[start + 1..].find('\'') {
                    return Some(expr[start + 1..start + 1 + end].to_string());
                }
            }
            // Also support double quotes
            if let Some(start) = expr.find('"') {
                if let Some(end) = expr[start + 1..].find('"') {
                    return Some(expr[start + 1..start + 1 + end].to_string());
                }
            }
        }
        None
    }

    fn parse_env_reference(&self, expr: &str) -> Option<String> {
        // Parse $env.VAR_NAME pattern
        let trimmed = expr.trim();
        if trimmed.starts_with("$env.") {
            let var_name = &trimmed[5..]; // Skip "$env."
            // Extract until first non-alphanumeric character (except underscore)
            let end_pos = var_name
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(var_name.len());
            return Some(var_name[..end_pos].to_string());
        }
        None
    }

    fn validate_expression_syntax(&self, expr: &str) -> ValueResult<()> {
        let trimmed = expr.trim();

        // Check for valid expression patterns
        if trimmed.starts_with("$node(") {
            if !trimmed.contains(')') {
                return Err(ValueError::InvalidExpression {
                    input: expr.to_string(),
                    reason: "Missing closing parenthesis in $node() reference".to_string(),
                });
            }
        } else if trimmed.starts_with("$trigger.") {
            // Valid trigger reference
        } else if trimmed.starts_with("$execution.") {
            // Valid execution reference
        } else if trimmed.starts_with("$env.") {
            // Valid environment reference
        } else if trimmed.starts_with("$json.") ||
            trimmed.starts_with("$date.") ||
            trimmed.starts_with("$string.") {
            // Valid function call
        } else if trimmed.chars().all(|c| c.is_alphanumeric() || "_.-".contains(c)) {
            // Simple variable name
        } else {
            return Err(ValueError::InvalidExpression {
                input: expr.to_string(),
                reason: "Unknown expression syntax".to_string(),
            });
        }

        Ok(())
    }

    // === Evaluation Integration ===

    /// Render/evaluate the expression using nebula-template
    /// Returns the rendered string result
    #[cfg(feature = "template-engine")]
    pub fn render(&self, context: &ExpressionContext) -> ValueResult<String> {
        use nebula_template::{Template, Context};

        // Parse template using nebula-template
        let template = Template::parse(&self.template)
            .map_err(|e| ValueError::ExpressionEvaluationFailed {
                reason: format!("Template parsing failed: {}", e),
            })?;

        // Convert our context to nebula-template context
        let mut template_context = Context::new();

        // Set input data if available
        if let Some(input) = &context.input_data {
            template_context.set_input(input.clone().into());
        }

        // Add node outputs
        for (node_id, output) in &context.node_outputs {
            template_context.add_node_output(node_id, output.clone().into());
        }

        // Add environment variables
        for (key, value) in &context.env_vars {
            template_context.set_env(key, value);
        }

        // Add execution metadata
        for (key, value) in &context.execution_metadata {
            template_context.set_execution_data(key, value.clone().into());
        }

        // Render the template
        template.render(&template_context)
            .map_err(|e| ValueError::ExpressionEvaluationFailed {
                reason: format!("Template rendering failed: {}", e),
            })
    }

    /// Evaluate the expression and return a typed Value
    #[cfg(feature = "template-engine")]
    pub fn evaluate(&self, context: &ExpressionContext) -> ValueResult<Value> {
        // If it's a static expression, return as-is
        if self.is_static() {
            return Ok(Value::String(crate::value::StringValue::new(self.template.clone())));
        }

        // Render the template
        let rendered = self.render(context)?;

        // Try to parse the result as different types
        self.parse_rendered_value(&rendered)
    }

    fn parse_rendered_value(&self, rendered: &str) -> ValueResult<Value> {
        // Try to parse as JSON first (for complex objects/arrays)
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(rendered) {
            return Ok(self.json_to_value(json_val)?);
        }

        // Try to parse as different primitive types
        if rendered == "null" {
            return Ok(Value::String(crate::value::StringValue::new("null".to_string())));
        }

        if let Ok(bool_val) = rendered.parse::<bool>() {
            return Ok(Value::Boolean(crate::value::BooleanValue::new(bool_val)));
        }

        if let Ok(int_val) = rendered.parse::<i64>() {
            return Ok(Value::Number(crate::value::NumberValue::new(int_val as f64)?));
        }

        if let Ok(float_val) = rendered.parse::<f64>() {
            return Ok(Value::Number(crate::value::NumberValue::new(float_val)?));
        }

        // Default to string
        Ok(Value::String(crate::value::StringValue::new(rendered.to_string())))
    }

    #[cfg(feature = "template-engine")]
    fn json_to_value(&self, json_val: serde_json::Value) -> ValueResult<Value> {
        match json_val {
            serde_json::Value::Null => Ok(Value::String(crate::value::StringValue::new("null".to_string()))),
            serde_json::Value::Bool(b) => Ok(Value::Boolean(crate::value::BooleanValue::new(b))),
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    Ok(Value::Number(crate::value::NumberValue::new(f)?))
                } else {
                    Err(ValueError::TypeConversion {
                        from_type: "JSON Number".to_string(),
                        to_type: "NumberValue".to_string(),
                    })
                }
            },
            serde_json::Value::String(s) => Ok(Value::String(crate::value::StringValue::new(s))),
            serde_json::Value::Array(arr) => {
                let values: Result<Vec<_>, _> = arr.into_iter()
                    .map(|v| self.json_to_value(v))
                    .collect();
                Ok(Value::Array(crate::value::ArrayValue::new(values?)))
            },
            serde_json::Value::Object(obj) => {
                let mut object_vals = std::collections::HashMap::new();
                for (k, v) in obj {
                    object_vals.insert(k, self.json_to_value(v)?);
                }
                Ok(Value::Object(crate::value::ObjectValue::new(object_vals)))
            },
        }
    }

    // === Transformation ===

    /// Replaces parts of the template with new values
    #[must_use]
    pub fn replace(&self, pattern: &str, replacement: &str) -> Self {
        Self::new(self.template.replace(pattern, replacement))
    }

    /// Appends text to the template
    #[must_use]
    pub fn append(&self, suffix: &str) -> Self {
        Self::new(format!("{}{}", self.template, suffix))
    }

    /// Prepends text to the template
    #[must_use]
    pub fn prepend(&self, prefix: &str) -> Self {
        Self::new(format!("{}{}", prefix, self.template))
    }

    /// Wraps the template in additional text
    #[must_use]
    pub fn wrap(&self, prefix: &str, suffix: &str) -> Self {
        Self::new(format!("{}{}{}", prefix, self.template, suffix))
    }

    // === Dependencies Analysis ===

    /// Gets all dependencies (nodes, env vars) that this expression needs
    #[must_use]
    pub fn get_dependencies(&self) -> ExpressionDependencies {
        ExpressionDependencies {
            nodes: self.extract_node_references(),
            env_vars: self.extract_env_references(),
            uses_trigger: self.template.contains("$trigger"),
            uses_execution: self.template.contains("$execution"),
            uses_input: self.template.contains("$input"),
            uses_system: self.template.contains("$system"),
        }
    }

    // === Static Helpers ===

    /// Creates a simple variable reference expression
    #[must_use]
    pub fn var(name: &str) -> Self {
        Self::new(format!("{{{{ {} }}}}", name))
    }

    /// Creates a function call expression
    #[must_use]
    pub fn function(func_name: &str, args: &[&str]) -> Self {
        let args_str = args.join(", ");
        Self::new(format!("{{{{ {}({}) }}}}", func_name, args_str))
    }

    /// Combines multiple expressions with a separator
    #[must_use]
    pub fn join(expressions: &[&ExpressionValue], separator: &str) -> Self {
        let combined = expressions
            .iter()
            .map(|e| e.template())
            .collect::<Vec<_>>()
            .join(separator);
        Self::new(combined)
    }
}

/// Context for expression evaluation
#[derive(Debug, Clone, Default)]
pub struct ExpressionContext {
    /// Input data (from current node or trigger)
    pub input_data: Option<serde_json::Value>,
    /// Node outputs by node ID
    pub node_outputs: std::collections::HashMap<String, serde_json::Value>,
    /// Environment variables
    pub env_vars: std::collections::HashMap<String, String>,
    /// Execution metadata
    pub execution_metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl ExpressionContext {
    /// Creates a new empty context
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets input data
    pub fn with_input(mut self, data: serde_json::Value) -> Self {
        self.input_data = Some(data);
        self
    }

    /// Adds node output data
    pub fn with_node(mut self, node_id: &str, data: serde_json::Value) -> Self {
        self.node_outputs.insert(node_id.to_string(), data);
        self
    }

    /// Adds environment variable
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env_vars.insert(key.to_string(), value.to_string());
        self
    }

    /// Adds execution metadata
    pub fn with_execution_data(mut self, key: &str, value: serde_json::Value) -> Self {
        self.execution_metadata.insert(key.to_string(), value);
        self
    }

    /// Checks if all required dependencies are available
    pub fn has_dependencies(&self, deps: &ExpressionDependencies) -> bool {
        // Check node dependencies
        for node_id in &deps.nodes {
            if !self.node_outputs.contains_key(node_id) {
                return false;
            }
        }

        // Check environment dependencies
        for env_var in &deps.env_vars {
            if !self.env_vars.contains_key(env_var) {
                return false;
            }
        }

        // Check input dependency
        if deps.uses_input && self.input_data.is_none() {
            return false;
        }

        true
    }
}

/// Dependencies that an expression requires for evaluation
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionDependencies {
    /// Node IDs that this expression references
    pub nodes: Vec<String>,
    /// Environment variables that this expression references
    pub env_vars: Vec<String>,
    /// Whether this expression uses trigger data
    pub uses_trigger: bool,
    /// Whether this expression uses execution metadata
    pub uses_execution: bool,
    /// Whether this expression uses input data
    pub uses_input: bool,
    /// Whether this expression uses system data
    pub uses_system: bool,
}

impl ExpressionDependencies {
    /// Checks if the expression has any dependencies
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
            && self.env_vars.is_empty()
            && !self.uses_trigger
            && !self.uses_execution
            && !self.uses_input
            && !self.uses_system
    }

    /// Gets all dependencies as a set of strings
    #[must_use]
    pub fn all_dependencies(&self) -> HashSet<String> {
        let mut deps = HashSet::new();

        for node in &self.nodes {
            deps.insert(format!("node:{}", node));
        }

        for env_var in &self.env_vars {
            deps.insert(format!("env:{}", env_var));
        }

        if self.uses_trigger {
            deps.insert("trigger".to_string());
        }

        if self.uses_execution {
            deps.insert("execution".to_string());
        }

        if self.uses_input {
            deps.insert("input".to_string());
        }

        if self.uses_system {
            deps.insert("system".to_string());
        }

        deps
    }
}

// === Default and Display implementations ===

impl Default for ExpressionValue {
    fn default() -> Self {
        Self::new("")
    }
}

impl std::fmt::Display for ExpressionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.template)
    }
}

// === From implementations ===

impl From<&str> for ExpressionValue {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ExpressionValue {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl std::str::FromStr for ExpressionValue {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let expr = Self::new(s);
        expr.validate()?;
        Ok(expr)
    }
}

// === JSON conversion ===

impl From<ExpressionValue> for serde_json::Value {
    fn from(expr: ExpressionValue) -> Self {
        serde_json::Value::String(expr.template)
    }
}

impl TryFrom<serde_json::Value> for ExpressionValue {
    type Error = ValueError;

    fn try_from(value: serde_json::Value) -> ValueResult<Self> {
        match value {
            serde_json::Value::String(s) => {
                let expr = Self::new(s);
                expr.validate()?;
                Ok(expr)
            }
            other => Err(ValueError::TypeConversion {
                from_type: format!("{:?}", other),
                to_type: "ExpressionValue".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let expr = ExpressionValue::new("Hello {{ name }}!");
        assert_eq!(expr.template(), "Hello {{ name }}!");

        let node_ref = ExpressionValue::node_ref("user_data", "name");
        assert_eq!(node_ref.template(), "{{ $node('user_data').json.name }}");

        let env_ref = ExpressionValue::env_ref("API_KEY");
        assert_eq!(env_ref.template(), "{{ $env.API_KEY }}");
    }

    #[test]
    fn test_static_vs_dynamic() {
        assert!(ExpressionValue::new("static text").is_static());
        assert!(!ExpressionValue::new("Hello {{ name }}!").is_static());

        assert!(!ExpressionValue::new("static text").has_expressions());
        assert!(ExpressionValue::new("Hello {{ name }}!").has_expressions());
    }

    #[test]
    fn test_validation() {
        assert!(ExpressionValue::new("{{ valid }}").is_valid());
        assert!(ExpressionValue::new("text {{ var }} text").is_valid());
        assert!(!ExpressionValue::new("{{ invalid }").is_valid());
        assert!(!ExpressionValue::new("invalid }}").is_valid());
    }

    #[test]
    fn test_extract_expressions() {
        let expr = ExpressionValue::new("Hello {{ user.name }}, today is {{ date }}!");
        let expressions = expr.extract_expressions();
        assert_eq!(expressions, vec!["user.name", "date"]);

        let complex_expr = ExpressionValue::new("{{ $node('user').json.name }} from {{ $env.LOCATION }}");
        let complex_expressions = complex_expr.extract_expressions();
        assert_eq!(complex_expressions, vec!["$node('user').json.name", "$env.LOCATION"]);
    }

    #[test]
    fn test_extract_node_references() {
        let expr = ExpressionValue::new("{{ $node('user_data').json.name }} and {{ $node('settings').json.theme }}");
        let node_refs = expr.extract_node_references();
        assert_eq!(node_refs, vec!["user_data", "settings"]);
    }

    #[test]
    fn test_extract_env_references() {
        let expr = ExpressionValue::new("{{ $env.API_KEY }} and {{ $env.DEBUG_MODE }}");
        let env_refs = expr.extract_env_references();
        assert_eq!(env_refs, vec!["API_KEY", "DEBUG_MODE"]);
    }

    #[test]
    fn test_dependencies() {
        let expr = ExpressionValue::new("Hello {{ $node('user').json.name }}, API: {{ $env.API_KEY }}, trigger: {{ $trigger.json.type }}");
        let deps = expr.get_dependencies();

        assert_eq!(deps.nodes, vec!["user"]);
        assert_eq!(deps.env_vars, vec!["API_KEY"]);
        assert!(deps.uses_trigger);
        assert!(!deps.uses_execution);
    }

    #[test]
    fn test_context_dependencies() {
        let context = ExpressionContext::new()
            .with_node("user", serde_json::json!({"name": "Alice"}))
            .with_env("API_KEY", "secret123");

        let expr = ExpressionValue::new("Hello {{ $node('user').json.name }}!");
        let deps = expr.get_dependencies();
        assert!(context.has_dependencies(&deps));

        let missing_expr = ExpressionValue::new("Hello {{ $node('missing').json.name }}!");
        let missing_deps = missing_expr.get_dependencies();
        assert!(!context.has_dependencies(&missing_deps));
    }

    #[test]
    fn test_helper_constructors() {
        let var_expr = ExpressionValue::var("username");
        assert_eq!(var_expr.template(), "{{ username }}");

        let func_expr = ExpressionValue::function("uppercase", &["name"]);
        assert_eq!(func_expr.template(), "{{ uppercase(name) }}");
    }

    #[test]
    fn test_transformation() {
        let expr = ExpressionValue::new("Hello {{ name }}!");

        let replaced = expr.replace("name", "username");
        assert_eq!(replaced.template(), "Hello {{ username }}!");

        let appended = expr.append(" How are you?");
        assert_eq!(appended.template(), "Hello {{ name }}! How are you?");

        let wrapped = expr.wrap("Greeting: ", " End.");
        assert_eq!(wrapped.template(), "Greeting: Hello {{ name }}! End.");
    }

    #[cfg(feature = "template-engine")]
    #[test]
    fn test_evaluation() -> ValueResult<()> {
        let expr = ExpressionValue::new("Hello {{ $input.name }}!");
        let context = ExpressionContext::new()
            .with_input(serde_json::json!({"name": "Alice"}));

        let result = expr.render(&context)?;
        assert_eq!(result, "Hello Alice!");

        Ok(())
    }

    #[test]
    fn test_serialization() {
        let expr = ExpressionValue::new("Hello {{ $node('user').json.name }}!");

        let json = serde_json::to_string(&expr).unwrap();
        assert_eq!(json, "\"Hello {{ $node('user').json.name }}!\"");

        let deserialized: ExpressionValue = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, expr);
    }
}