//! Template parsing and rendering

use crate::{
    context::{Context, DataSource},
    error::{Error, Result},
    functions::FunctionRegistry,
    value::Value,
};
use std::{
    collections::HashSet,
    fmt,
    sync::Arc,
};

/// A parsed template that can be rendered with different contexts
#[derive(Debug, Clone)]
pub struct Template {
    /// Original template string
    source: String,
    /// Parsed template elements
    elements: Vec<TemplateElement>,
    /// Dependencies found during parsing
    dependencies: TemplateDependencies,
    /// Function registry to use for evaluation
    functions: Arc<FunctionRegistry>,
}

/// Elements that make up a template
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateElement {
    /// Static text content
    Text(String),
    /// Expression to be evaluated
    Expression(Expression),
}

/// An expression within a template
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    /// Original expression source
    source: String,
    /// Parsed expression tree
    ast: ExpressionAst,
}

/// Abstract syntax tree for expressions
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionAst {
    /// Literal values
    Literal(Value),

    /// Data source access: $input.path, $node('id').path, etc.
    DataAccess {
        source: DataSource,
        path: String,
    },

    /// Function call: uppercase(), default('fallback'), etc.
    FunctionCall {
        name: String,
        args: Vec<ExpressionAst>,
    },

    /// Pipeline: value | func1 | func2
    Pipeline {
        input: Box<ExpressionAst>,
        functions: Vec<PipelineFunction>,
    },

    /// Binary operations: ==, !=, +, -, etc.
    BinaryOp {
        left: Box<ExpressionAst>,
        operator: BinaryOperator,
        right: Box<ExpressionAst>,
    },

    /// Unary operations: !, -
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<ExpressionAst>,
    },

    /// Ternary conditional: condition ? then : else
    Ternary {
        condition: Box<ExpressionAst>,
        then_expr: Box<ExpressionAst>,
        else_expr: Box<ExpressionAst>,
    },

    /// If function: if(condition, then, else?)
    IfFunction {
        condition: Box<ExpressionAst>,
        then_expr: Box<ExpressionAst>,
        else_expr: Option<Box<ExpressionAst>>,
    },
}

/// Function call in a pipeline
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineFunction {
    pub name: String,
    pub args: Vec<ExpressionAst>,
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // Arithmetic
    Add, Subtract, Multiply, Divide, Modulo,
    // Comparison
    Equal, NotEqual, LessThan, LessEqual, GreaterThan, GreaterEqual,
    // Logical
    And, Or,
    // String
    Contains, StartsWith, EndsWith,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,      // !value
    Minus,    // -value
}

/// Dependencies that a template requires
#[derive(Debug, Clone, Default)]
pub struct TemplateDependencies {
    /// Input data paths used
    pub input_paths: HashSet<String>,
    /// Node IDs referenced
    pub node_ids: HashSet<String>,
    /// Environment variables used
    pub env_vars: HashSet<String>,
    /// Whether system data is used
    pub uses_system: bool,
    /// Whether execution data is used
    pub uses_execution: bool,
    /// Whether workflow data is used
    pub uses_workflow: bool,
    /// Functions used in the template
    pub functions: HashSet<String>,
}

impl Template {
    /// Parse a template string
    pub fn parse(source: &str) -> Result<Self> {
        Self::parse_with_functions(source, Arc::new(FunctionRegistry::with_builtins()))
    }

    /// Parse a template with custom functions
    pub fn parse_with_functions(source: &str, functions: Arc<FunctionRegistry>) -> Result<Self> {
        let mut parser = TemplateParser::new(source, functions.clone());
        let elements = parser.parse()?;
        let dependencies = parser.extract_dependencies(&elements);

        Ok(Self {
            source: source.to_string(),
            elements,
            dependencies,
            functions,
        })
    }

    /// Render the template with the given context
    pub fn render(&self, context: &Context) -> Result<String> {
        let mut output = String::new();

        for element in &self.elements {
            match element {
                TemplateElement::Text(text) => {
                    output.push_str(text);
                }
                TemplateElement::Expression(expr) => {
                    let value = expr.evaluate(context, &self.functions)?;
                    output.push_str(&value.as_string()?);
                }
            }
        }

        Ok(output)
    }

    /// Get the original template source
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Get template dependencies
    pub fn dependencies(&self) -> &TemplateDependencies {
        &self.dependencies
    }

    /// Check if the template is static (no expressions)
    pub fn is_static(&self) -> bool {
        self.elements.iter().all(|e| matches!(e, TemplateElement::Text(_)))
    }

    /// Validate that all dependencies are available in the context
    pub fn validate_context(&self, context: &Context) -> Result<()> {
        // Check input dependencies
        if !self.dependencies.input_paths.is_empty() && context.get_input().is_none() {
            return Err(Error::data_not_found(
                "$input".to_string(),
                vec!["Input data required but not provided".to_string()],
            ));
        }

        // Check node dependencies
        for node_id in &self.dependencies.node_ids {
            if context.get_node_output(node_id).is_none() {
                return Err(Error::data_not_found(
                    format!("$node('{}')", node_id),
                    context.available_data_sources(),
                ));
            }
        }

        // Check environment dependencies
        for env_var in &self.dependencies.env_vars {
            if context.get_env(env_var).is_none() {
                return Err(Error::data_not_found(
                    format!("$env.{}", env_var),
                    vec!["Environment variable not set".to_string()],
                ));
            }
        }

        Ok(())
    }

    /// Get all expressions in the template
    pub fn expressions(&self) -> Vec<&Expression> {
        self.elements
            .iter()
            .filter_map(|e| match e {
                TemplateElement::Expression(expr) => Some(expr),
                _ => None,
            })
            .collect()
    }

    /// Check if template uses a specific function
    pub fn uses_function(&self, function_name: &str) -> bool {
        self.dependencies.functions.contains(function_name)
    }

    /// Get the number of expressions in the template
    pub fn expression_count(&self) -> usize {
        self.elements
            .iter()
            .filter(|e| matches!(e, TemplateElement::Expression(_)))
            .count()
    }
}

impl Expression {
    /// Create a new expression
    pub fn new(source: String, ast: ExpressionAst) -> Self {
        Self { source, ast }
    }

    /// Get the original expression source
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Get the AST
    pub fn ast(&self) -> &ExpressionAst {
        &self.ast
    }

    /// Evaluate the expression in the given context
    pub fn evaluate(&self, context: &Context, functions: &FunctionRegistry) -> Result<Value> {
        self.ast.evaluate(context, functions)
    }

    /// Check if this is a simple data access expression
    pub fn is_simple_access(&self) -> bool {
        matches!(self.ast, ExpressionAst::DataAccess { .. })
    }

    /// Check if this is a literal value
    pub fn is_literal(&self) -> bool {
        matches!(self.ast, ExpressionAst::Literal(_))
    }

    /// Extract dependencies from this expression
    pub fn dependencies(&self) -> TemplateDependencies {
        let mut deps = TemplateDependencies::default();
        self.ast.collect_dependencies(&mut deps);
        deps
    }
}

impl ExpressionAst {
    /// Evaluate the AST node
    pub fn evaluate(&self, context: &Context, functions: &FunctionRegistry) -> Result<Value> {
        match self {
            Self::Literal(value) => Ok(value.clone()),

            Self::DataAccess { source, path } => {
                context.resolve_data_source(source, path)
            }

            Self::FunctionCall { name, args } => {
                let function = functions.get(name)
                    .ok_or_else(|| Error::function(name.clone(), "Function not found".to_string(), vec![]))?;

                let arg_values: Result<Vec<Value>> = args
                    .iter()
                    .map(|arg| arg.evaluate(context, functions))
                    .collect();

                function.execute(arg_values?)
            }

            Self::Pipeline { input, functions: pipeline_functions } => {
                let mut value = input.evaluate(context, functions)?;

                for pipeline_func in pipeline_functions {
                    let function = functions.get(&pipeline_func.name)
                        .ok_or_else(|| Error::function(
                            pipeline_func.name.clone(),
                            "Function not found".to_string(),
                            vec![]
                        ))?;

                    let mut args = vec![value];
                    for arg in &pipeline_func.args {
                        args.push(arg.evaluate(context, functions)?);
                    }

                    value = function.execute(args)?;
                }

                Ok(value)
            }

            Self::BinaryOp { left, operator, right } => {
                let left_val = left.evaluate(context, functions)?;
                let right_val = right.evaluate(context, functions)?;

                match operator {
                    BinaryOperator::Add => {
                        if left_val.is_number() && right_val.is_number() {
                            let left_f = left_val.as_float()?;
                            let right_f = right_val.as_float()?;
                            Ok(Value::float(left_f + right_f))
                        } else {
                            // String concatenation
                            let left_s = left_val.as_string()?;
                            let right_s = right_val.as_string()?;
                            Ok(Value::string(format!("{}{}", left_s, right_s)))
                        }
                    }
                    BinaryOperator::Subtract => {
                        let left_f = left_val.as_float()?;
                        let right_f = right_val.as_float()?;
                        Ok(Value::float(left_f - right_f))
                    }
                    BinaryOperator::Multiply => {
                        let left_f = left_val.as_float()?;
                        let right_f = right_val.as_float()?;
                        Ok(Value::float(left_f * right_f))
                    }
                    BinaryOperator::Divide => {
                        let left_f = left_val.as_float()?;
                        let right_f = right_val.as_float()?;
                        if right_f == 0.0 {
                            Err(Error::math("Division by zero"))
                        } else {
                            Ok(Value::float(left_f / right_f))
                        }
                    }
                    BinaryOperator::Equal => {
                        Ok(Value::bool(left_val.equals(&right_val)))
                    }
                    BinaryOperator::NotEqual => {
                        Ok(Value::bool(!left_val.equals(&right_val)))
                    }
                    BinaryOperator::LessThan => {
                        let left_f = left_val.as_float()?;
                        let right_f = right_val.as_float()?;
                        Ok(Value::bool(left_f < right_f))
                    }
                    BinaryOperator::And => {
                        Ok(Value::bool(left_val.is_truthy() && right_val.is_truthy()))
                    }
                    BinaryOperator::Or => {
                        Ok(Value::bool(left_val.is_truthy() || right_val.is_truthy()))
                    }
                    _ => Err(Error::evaluation(format!("Operator {:?} not implemented", operator))),
                }
            }

            Self::UnaryOp { operator, operand } => {
                let value = operand.evaluate(context, functions)?;

                match operator {
                    UnaryOperator::Not => Ok(Value::bool(!value.is_truthy())),
                    UnaryOperator::Minus => {
                        let f = value.as_float()?;
                        Ok(Value::float(-f))
                    }
                }
            }

            Self::Ternary { condition, then_expr, else_expr } => {
                let condition_val = condition.evaluate(context, functions)?;

                if condition_val.is_truthy() {
                    then_expr.evaluate(context, functions)
                } else {
                    else_expr.evaluate(context, functions)
                }
            }

            Self::IfFunction { condition, then_expr, else_expr } => {
                let condition_val = condition.evaluate(context, functions)?;

                if condition_val.is_truthy() {
                    then_expr.evaluate(context, functions)
                } else if let Some(else_expr) = else_expr {
                    else_expr.evaluate(context, functions)
                } else {
                    Ok(Value::null())
                }
            }
        }
    }

    /// Collect dependencies from this AST node
    pub fn collect_dependencies(&self, deps: &mut TemplateDependencies) {
        match self {
            Self::DataAccess { source, path } => {
                match source {
                    DataSource::Input => {
                        deps.input_paths.insert(path.clone());
                    }
                    DataSource::Node(id) => {
                        deps.node_ids.insert(id.clone());
                    }
                    DataSource::Environment => {
                        deps.env_vars.insert(path.clone());
                    }
                    DataSource::System => {
                        deps.uses_system = true;
                    }
                    DataSource::Execution => {
                        deps.uses_execution = true;
                    }
                    DataSource::Workflow => {
                        deps.uses_workflow = true;
                    }
                }
            }
            Self::FunctionCall { name, args } => {
                deps.functions.insert(name.clone());
                for arg in args {
                    arg.collect_dependencies(deps);
                }
            }
            Self::Pipeline { input, functions } => {
                input.collect_dependencies(deps);
                for func in functions {
                    deps.functions.insert(func.name.clone());
                    for arg in &func.args {
                        arg.collect_dependencies(deps);
                    }
                }
            }
            Self::BinaryOp { left, right, .. } => {
                left.collect_dependencies(deps);
                right.collect_dependencies(deps);
            }
            Self::UnaryOp { operand, .. } => {
                operand.collect_dependencies(deps);
            }
            Self::Ternary { condition, then_expr, else_expr } => {
                condition.collect_dependencies(deps);
                then_expr.collect_dependencies(deps);
                else_expr.collect_dependencies(deps);
            }
            Self::IfFunction { condition, then_expr, else_expr } => {
                condition.collect_dependencies(deps);
                then_expr.collect_dependencies(deps);
                if let Some(else_expr) = else_expr {
                    else_expr.collect_dependencies(deps);
                }
            }
            Self::Literal(_) => {
                // No dependencies for literals
            }
        }
    }
}

/// Basic template parser (placeholder implementation)
struct TemplateParser {
    source: String,
    functions: Arc<FunctionRegistry>,
}

impl TemplateParser {
    fn new(source: &str, functions: Arc<FunctionRegistry>) -> Self {
        Self {
            source: source.to_string(),
            functions,
        }
    }

    fn parse(&mut self) -> Result<Vec<TemplateElement>> {
        let mut elements = Vec::new();
        let mut current_pos = 0;

        while current_pos < self.source.len() {
            // Look for expression start
            if let Some(expr_start) = self.source[current_pos..].find("{{") {
                let absolute_start = current_pos + expr_start;

                // Add any text before the expression
                if expr_start > 0 {
                    elements.push(TemplateElement::Text(
                        self.source[current_pos..absolute_start].to_string()
                    ));
                }

                // Find expression end
                if let Some(expr_end) = self.source[absolute_start + 2..].find("}}") {
                    let absolute_end = absolute_start + 2 + expr_end;
                    let expr_content = &self.source[absolute_start + 2..absolute_end].trim();

                    // Parse the expression
                    let expression = self.parse_expression(expr_content)?;
                    elements.push(TemplateElement::Expression(expression));

                    current_pos = absolute_end + 2;
                } else {
                    return Err(Error::parse(
                        "Unclosed expression".to_string(),
                        absolute_start,
                        self.source.clone(),
                    ));
                }
            } else {
                // No more expressions, add remaining text
                if current_pos < self.source.len() {
                    elements.push(TemplateElement::Text(
                        self.source[current_pos..].to_string()
                    ));
                }
                break;
            }
        }

        Ok(elements)
    }

    fn parse_expression(&self, content: &str) -> Result<Expression> {
        // This is a very basic parser - a full implementation would use a proper lexer/parser
        let ast = if content.starts_with('$') {
            // Data access
            self.parse_data_access(content)?
        } else if content.contains('|') {
            // Pipeline
            self.parse_pipeline(content)?
        } else if content.contains('?') && content.contains(':') {
            // Ternary
            self.parse_ternary(content)?
        } else if content.starts_with("if(") {
            // If function
            self.parse_if_function(content)?
        } else {
            // Try to parse as literal
            self.parse_literal(content)?
        };

        Ok(Expression::new(content.to_string(), ast))
    }

    fn parse_data_access(&self, content: &str) -> Result<ExpressionAst> {
        // Parse $input.path, $node('id').path, etc.
        if content.starts_with("$input") {
            let path = if content.len() > 6 && content.chars().nth(6) == Some('.') {
                content[7..].to_string()
            } else {
                String::new()
            };
            Ok(ExpressionAst::DataAccess {
                source: DataSource::Input,
                path,
            })
        } else if content.starts_with("$node(") {
            // Extract node ID from $node('id')
            if let Some(start) = content.find('\'') {
                if let Some(end) = content[start + 1..].find('\'') {
                    let node_id = content[start + 1..start + 1 + end].to_string();
                    let remaining = &content[start + 1 + end + 1..];
                    let path = if remaining.starts_with(".json.") {
                        remaining[6..].to_string()
                    } else if remaining.starts_with('.') {
                        remaining[1..].to_string()
                    } else {
                        String::new()
                    };

                    Ok(ExpressionAst::DataAccess {
                        source: DataSource::Node(node_id),
                        path,
                    })
                } else {
                    Err(Error::parse("Invalid node reference".to_string(), 0, content.to_string()))
                }
            } else {
                Err(Error::parse("Invalid node reference".to_string(), 0, content.to_string()))
            }
        } else if content.starts_with("$env.") {
            Ok(ExpressionAst::DataAccess {
                source: DataSource::Environment,
                path: content[5..].to_string(),
            })
        } else if content.starts_with("$system.") {
            Ok(ExpressionAst::DataAccess {
                source: DataSource::System,
                path: content[8..].to_string(),
            })
        } else if content.starts_with("$execution.") {
            Ok(ExpressionAst::DataAccess {
                source: DataSource::Execution,
                path: content[11..].to_string(),
            })
        } else if content.starts_with("$workflow.") {
            Ok(ExpressionAst::DataAccess {
                source: DataSource::Workflow,
                path: content[10..].to_string(),
            })
        } else {
            Err(Error::parse("Unknown data source".to_string(), 0, content.to_string()))
        }
    }

    fn parse_pipeline(&self, content: &str) -> Result<ExpressionAst> {
        let parts: Vec<&str> = content.split('|').map(|s| s.trim()).collect();
        if parts.len() < 2 {
            return Err(Error::parse("Invalid pipeline".to_string(), 0, content.to_string()));
        }

        let input = Box::new(self.parse_expression(parts[0])?.ast);
        let mut functions = Vec::new();

        for part in &parts[1..] {
            if let Some(paren_pos) = part.find('(') {
                let name = part[..paren_pos].trim().to_string();
                // For now, assume no arguments in pipeline functions
                functions.push(PipelineFunction {
                    name,
                    args: vec![],
                });
            } else {
                functions.push(PipelineFunction {
                    name: part.to_string(),
                    args: vec![],
                });
            }
        }

        Ok(ExpressionAst::Pipeline { input, functions })
    }

    fn parse_ternary(&self, content: &str) -> Result<ExpressionAst> {
        // This is a simplified ternary parser
        if let Some(question_pos) = content.find('?') {
            if let Some(colon_pos) = content[question_pos..].find(':') {
                let colon_pos = question_pos + colon_pos;

                let condition = self.parse_expression(content[..question_pos].trim())?.ast;
                let then_expr = self.parse_expression(content[question_pos + 1..colon_pos].trim())?.ast;
                let else_expr = self.parse_expression(content[colon_pos + 1..].trim())?.ast;

                Ok(ExpressionAst::Ternary {
                    condition: Box::new(condition),
                    then_expr: Box::new(then_expr),
                    else_expr: Box::new(else_expr),
                })
            } else {
                Err(Error::parse("Missing ':' in ternary expression".to_string(), 0, content.to_string()))
            }
        } else {
            Err(Error::parse("Missing '?' in ternary expression".to_string(), 0, content.to_string()))
        }
    }

    fn parse_if_function(&self, content: &str) -> Result<ExpressionAst> {
        // Simplified if function parser: if(condition, then, else?)
        if !content.starts_with("if(") || !content.ends_with(')') {
            return Err(Error::parse("Invalid if function".to_string(), 0, content.to_string()));
        }

        let inner = &content[3..content.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

        if parts.len() < 2 || parts.len() > 3 {
            return Err(Error::parse("If function requires 2 or 3 arguments".to_string(), 0, content.to_string()));
        }

        let condition = Box::new(self.parse_expression(parts[0])?.ast);
        let then_expr = Box::new(self.parse_expression(parts[1])?.ast);
        let else_expr = if parts.len() == 3 {
            Some(Box::new(self.parse_expression(parts[2])?.ast))
        } else {
            None
        };

        Ok(ExpressionAst::IfFunction {
            condition,
            then_expr,
            else_expr,
        })
    }

    fn parse_literal(&self, content: &str) -> Result<ExpressionAst> {
        // Try to parse as different literal types
        if content == "null" {
            Ok(ExpressionAst::Literal(Value::null()))
        } else if content == "true" {
            Ok(ExpressionAst::Literal(Value::bool(true)))
        } else if content == "false" {
            Ok(ExpressionAst::Literal(Value::bool(false)))
        } else if let Ok(int_val) = content.parse::<i64>() {
            Ok(ExpressionAst::Literal(Value::integer(int_val)))
        } else if let Ok(float_val) = content.parse::<f64>() {
            Ok(ExpressionAst::Literal(Value::float(float_val)))
        } else if (content.starts_with('"') && content.ends_with('"')) ||
            (content.starts_with('\'') && content.ends_with('\'')) {
            let string_content = &content[1..content.len() - 1];
            Ok(ExpressionAst::Literal(Value::string(string_content)))
        } else {
            Err(Error::parse("Unknown literal type".to_string(), 0, content.to_string()))
        }
    }

    fn extract_dependencies(&self, elements: &[TemplateElement]) -> TemplateDependencies {
        let mut deps = TemplateDependencies::default();

        for element in elements {
            if let TemplateElement::Expression(expr) = element {
                expr.ast.collect_dependencies(&mut deps);
            }
        }

        deps
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

// Placeholder for functions module - will be implemented later
mod temp_functions {
    use crate::{error::Result, value::Value};
    use std::collections::HashMap;

    pub struct FunctionRegistry {
        _functions: HashMap<String, Box<dyn Fn(Vec<Value>) -> Result<Value>>>,
    }

    impl FunctionRegistry {
        pub fn with_builtins() -> Self {
            Self {
                _functions: HashMap::new(),
            }
        }

        pub fn get(&self, _name: &str) -> Option<&dyn Fn(Vec<Value>) -> Result<Value>> {
            None
        }
    }

    pub trait Function {
        fn execute(&self, args: Vec<Value>) -> Result<Value>;
    }
}

// Temporary re-export until we implement the real functions module
pub use temp_functions::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_static_template() -> Result<()> {
        let template = Template::parse("Hello World!")?;
        let context = Context::new();

        assert!(template.is_static());
        assert_eq!(template.render(&context)?, "Hello World!");

        Ok(())
    }

    #[test]
    fn test_empty_template() -> Result<()> {
        let template = Template::parse("")?;
        let context = Context::new();

        assert!(template.is_static());
        assert_eq!(template.render(&context)?, "");

        Ok(())
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_simple_expression() -> Result<()> {
        use serde_json::json;

        let template = Template::parse("Hello {{ $input.name }}!")?;
        let mut context = Context::new();
        context.set_input(json!({"name": "Alice"}).into());

        assert!(!template.is_static());
        assert_eq!(template.expression_count(), 1);

        let result = template.render(&context)?;
        assert_eq!(result, "Hello Alice!");

        Ok(())
    }

    #[test]
    fn test_multiple_expressions() -> Result<()> {
        let template = Template::parse("{{ $input.greeting }} {{ $input.name }}!")?;
        assert_eq!(template.expression_count(), 2);

        Ok(())
    }

    #[test]
    fn test_template_dependencies() -> Result<()> {
        let template = Template::parse("{{ $input.name }} {{ $node('test').value }} {{ $env.KEY }}")?;
        let deps = template.dependencies();

        assert!(deps.input_paths.contains("name"));
        assert!(deps.node_ids.contains("test"));
        assert!(deps.env_vars.contains("KEY"));

        Ok(())
    }

    #[test]
    fn test_invalid_template() {
        let result = Template::parse("{{ unclosed expression");
        assert!(result.is_err());
    }

    #[test]
    fn test_expression_types() -> Result<()> {
        let expr = Expression::new(
            "$input.name".to_string(),
            ExpressionAst::DataAccess {
                source: DataSource::Input,
                path: "name".to_string(),
            }
        );

        assert!(expr.is_simple_access());
        assert!(!expr.is_literal());

        let literal_expr = Expression::new(
            "42".to_string(),
            ExpressionAst::Literal(Value::integer(42))
        );

        assert!(literal_expr.is_literal());
        assert!(!literal_expr.is_simple_access());

        Ok(())
    }
}