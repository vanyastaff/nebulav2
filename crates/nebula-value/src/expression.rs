use std::collections::HashSet;
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{ValueError, ValueResult};

/// Expression value for dynamic evaluation in workflows
/// Supports template syntax: "Hello {{ $node('user_data').json.name }}!"
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        Self { template: template.into() }
    }

    /// Creates an expression from a static value (no evaluation needed)
    #[must_use]
    pub fn static_value(value: impl Into<String>) -> Self {
        Self::new(value.into())
    }

    /// Creates an expression that references a node output
    #[must_use]
    pub fn node_ref(node_id: &str, path: &str) -> Self {
        Self::new(format!("{{{{ $node('{node_id}').json.{path} }}}}"))
    }

    /// Creates an expression that references trigger data
    #[must_use]
    pub fn trigger_ref(path: &str) -> Self {
        Self::new(format!("{{{{ $trigger.json.{path} }}}}"))
    }

    /// Creates an expression that references execution metadata
    #[must_use]
    pub fn execution_ref(field: &str) -> Self {
        Self::new(format!("{{{{ $execution.{field} }}}}"))
    }

    /// Creates an expression that references environment variable
    #[must_use]
    pub fn env_ref(var_name: &str) -> Self {
        Self::new(format!("{{{{ $env.{var_name} }}}}"))
    }

    /// Checks if the expression is empty (contains no template)
    pub fn is_empty(&self) -> bool {
        self.template().is_empty()
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
                },
                '}' if chars.peek() == Some(&'}') => {
                    chars.next(); // Consume second '}'
                    brace_depth -= 1;
                    if brace_depth < 0 {
                        return false; // Unmatched closing braces
                    }
                },
                _ => {},
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
        if let Some(var_name) = trimmed.strip_prefix("$env.") {
            // Extract until first non-alphanumeric character (except underscore)
            let end_pos =
                var_name.find(|c: char| !c.is_alphanumeric() && c != '_').unwrap_or(var_name.len());
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
        } else if trimmed.starts_with("$json.")
            || trimmed.starts_with("$date.")
            || trimmed.starts_with("$string.")
        {
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

    // === Static Helpers ===

    /// Creates a simple variable reference expression
    #[must_use]
    pub fn var(name: &str) -> Self {
        Self::new(format!("{{{{ {name} }}}}"))
    }

    /// Creates a function call expression
    #[must_use]
    pub fn function(func_name: &str, args: &[&str]) -> Self {
        let args_str = args.join(", ");
        Self::new(format!("{{{{ {func_name}({args_str}) }}}}"))
    }

    /// Combines multiple expressions with a separator
    #[must_use]
    pub fn join(expressions: &[&ExpressionValue], separator: &str) -> Self {
        let combined = expressions.iter().map(|e| e.template()).collect::<Vec<_>>().join(separator);
        Self::new(combined)
    }
}

impl Display for ExpressionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.template)
    }
}
