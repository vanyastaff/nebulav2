// nebula_core/src/value/expression.rs

use derive_more::{Deref, DerefMut, Display, Into};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Deref, DerefMut, Into, Display)]
pub struct ExpressionValue(String);

impl ExpressionValue {
    pub fn new(template: impl Into<String>) -> Self {
        Self(template.into())
    }

    pub fn template(&self) -> &str {
        &self.0
    }

    pub fn is_static(&self) -> bool {
        !self.0.contains("{{") || !self.0.contains("}}")
    }

    pub fn extract_variables(&self) -> Vec<String> {
        let mut variables = Vec::new();
        let mut in_expression = false;
        let mut current_var = String::new();
        let mut chars = self.0.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' && chars.peek() == Some(&'{') {
                chars.next(); // Skip second '{'
                in_expression = true;
                current_var.clear();
            } else if ch == '}' && chars.peek() == Some(&'}') && in_expression {
                chars.next(); // Skip second '}'
                in_expression = false;
                if !current_var.trim().is_empty() {
                    variables.push(current_var.trim().to_string());
                }
                current_var.clear();
            } else if in_expression {
                current_var.push(ch);
            }
        }

        variables
    }

    pub fn is_valid(&self) -> bool {
        let mut brace_count = 0;
        let mut chars = self.0.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '{' if chars.peek() == Some(&'{') => {
                    chars.next();
                    brace_count += 1;
                }
                '}' if chars.peek() == Some(&'}') => {
                    chars.next();
                    brace_count -= 1;
                    if brace_count < 0 {
                        return false;
                    }
                }
                _ => {}
            }
        }

        brace_count == 0
    }

    /// Проверяет, содержит ли expression указанную переменную
    pub fn contains_variable(&self, var: &str) -> bool {
        self.extract_variables().contains(&var.to_string())
    }
}

impl Default for ExpressionValue {
    fn default() -> Self {
        Self::new("")
    }
}

impl From<&str> for ExpressionValue {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ExpressionValue {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Into<serde_json::Value> for ExpressionValue {
    fn into(self) -> serde_json::Value {
        serde_json::Value::String(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_variables() {
        let expr = ExpressionValue::new("Hello {{ user.name }}, today is {{ date }}!");
        let vars = expr.extract_variables();
        assert_eq!(vars, vec!["user.name", "date"]);
    }

    #[test]
    fn test_is_static() {
        assert!(ExpressionValue::new("static text").is_static());
        assert!(!ExpressionValue::new("{{ dynamic }}").is_static());
    }

    #[test]
    fn test_is_valid() {
        assert!(ExpressionValue::new("{{ valid }}").is_valid());
        assert!(ExpressionValue::new("text {{ var }} text").is_valid());
        assert!(!ExpressionValue::new("{{ invalid }").is_valid());
        assert!(!ExpressionValue::new("invalid }}").is_valid());
    }
}
