# Nebula Template

A powerful, type-safe template engine for Rust designed for workflow automation and data transformation. Nebula Template provides a rich expression language with pipeline operations, built-in functions, and multiple data sources.

## Features

- 🚀 **High Performance** - Zero-copy parsing and efficient evaluation
- 🔒 **Type Safety** - Compile-time function signature validation
- 🔄 **Pipeline Operations** - Chain functions with `|` operator
- 📊 **Multiple Data Sources** - Access various data contexts
- 🎯 **Rich Function Library** - String, array, math, date operations
- 🔀 **Control Flow** - Conditionals and loops for complex templates
- 📝 **Document Generation** - Perfect for generating reports and documents
- 🛡️ **Memory Safe** - Built with Rust's safety guarantees

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
nebula-template = "0.1.0"
```

### Basic Usage

```rust
use nebula_template::{Template, Context};
use serde_json::json;

// Simple variable substitution
let template = Template::parse("Hello {{ $input.name }}!")?;
let mut context = Context::new();
context.set_input(json!({"name": "Alice"}));
let result = template.render(&context)?;
assert_eq!(result, "Hello Alice!");

// Pipeline operations
let template = Template::parse("{{ $input.name | uppercase | default('Anonymous') }}")?;
let result = template.render(&context)?;
assert_eq!(result, "ALICE");
```

## Expression Syntax

### Data Sources

Access different data contexts in your templates:

```rust
"{{ $input.user.name }}"              // Current input data
"{{ $node('user_data').json.email }}" // Output from another node
"{{ $env.API_KEY }}"                  // Environment variables
"{{ $system.datetime.now }}"          // System data (time, date)
"{{ $execution.id }}"                 // Execution metadata
"{{ $workflow.version }}"             // Workflow information
```

### Pipeline Operations

Chain functions together with the pipe operator:

```rust
"{{ $input.text | trim | uppercase | default('N/A') }}"
"{{ $input.items | pluck('name') | join(', ') }}"
"{{ $input.price | multiply(1.2) | round(2) | currency }}"
```

### Conditional Logic

```rust
// Ternary operator
"{{ $input.age >= 18 ? 'Adult' : 'Minor' }}"

// Function-style conditionals
"{{ if($input.active, 'Enabled', 'Disabled') }}"

// Block conditionals
"{{ if $input.vip }}
  VIP Customer: {{ $input.name }}
{{ else }}
  Regular Customer: {{ $input.name }}
{{ endif }}"
```

### Loops and Iteration

Generate repeated content with loops:

```rust
// Simple foreach
"{{ foreach item in $input.products }}
- {{ item.name }}: {{ item.price | currency }}
{{ endforeach }}"

// With separators
"{{ foreach user in $input.users | join('\n') }}
{{ user.name }} ({{ user.email }})
{{ endforeach }}"
```

## Built-in Functions

### String Functions
- `uppercase()`, `lowercase()`, `trim()`
- `replace(search, replace)`, `split(delimiter)`
- `substring(start, end)`, `length()`
- `format(template, ...args)`

### Array Functions
- `pluck(field)` - Extract field from objects
- `filter(condition)` - Filter items
- `map(expression)` - Transform items
- `join(separator)` - Join into string
- `slice(start, end)` - Array slice
- `sum(field?)`, `average(field?)`, `count()`
- `first()`, `last()`, `unique()`

### Object/JSON Functions
- `keys()`, `values()`, `get(path)`
- `merge(other)`, `pick(fields)`, `omit(fields)`
- `has(key)`, `empty()`, `not_empty()`

### Math Functions
- `add(n)`, `subtract(n)`, `multiply(n)`, `divide(n)`
- `round(digits?)`, `ceil()`, `floor()`
- `min()`, `max()`, `abs()`
- `random()`, `random_int(min, max)`

### Date Functions
- `format_date(format)` - Format date string
- `add_days(count)`, `add_hours(count)`
- `diff_days(other)`, `now()`
- `parse_date(format)`, `timestamp()`

### Comparison Functions
- `equals(value)`, `not_equals(value)`
- `greater_than(value)`, `less_than(value)`
- `in(list)`, `contains(value)`
- `starts_with(prefix)`, `ends_with(suffix)`

### Utility Functions
- `default(fallback)` - Provide fallback value
- `coalesce(...values)` - First non-null value
- `type()` - Get value type
- `debug()` - Debug output

## Advanced Examples

### Document Generation

```rust
let invoice_template = Template::parse(r#"
INVOICE #{{ $execution.id }}
Date: {{ $system.datetime.now | format_date('YYYY-MM-DD') }}

Bill To:
{{ $input.customer.name }}
{{ $input.customer.address }}
{{ $input.customer.city }}, {{ $input.customer.zip }}

Items:
{{ foreach item in $input.items }}
{{ item.name | pad_right(30) }} {{ item.quantity }}x {{ item.price | currency }}
{{ endforeach }}

Subtotal: {{ $input.items | sum('total') | currency }}
Tax:      {{ $input.items | sum('total') | multiply(0.08) | currency }}
Total:    {{ $input.items | sum('total') | multiply(1.08) | currency }}

{{ if($input.customer.vip, 'Thank you for being a VIP customer!', '') }}
"#)?;
```

### Data Processing

```rust
let report_template = Template::parse(r#"
Sales Report for {{ $input.month | format_date('MMMM YYYY') }}

Top Products:
{{ foreach product in $input.sales | sort_by('revenue') | reverse | slice(0, 5) }}
{{ loop.index }}. {{ product.name }}: {{ product.revenue | currency }}
{{ endforeach }}

Regional Performance:
{{ foreach region in $input.regions }}
{{ region.name }}: {{ region.sales | sum() | currency }} 
({{ region.sales | sum() | divide($input.total_sales) | multiply(100) | round(1) }}%)
{{ endforeach }}

Summary:
- Total Revenue: {{ $input.total_sales | currency }}
- Average Order: {{ $input.total_sales | divide($input.order_count) | currency }}
- Growth: {{ $input.growth_rate | multiply(100) | round(1) }}%
"#)?;
```

### API Response Formatting

```rust
let api_template = Template::parse(r#"
{
  "status": "{{ $input.success ? 'success' : 'error' }}",
  "data": {
    {{ if $input.users }}
    "users": [
      {{ foreach user in $input.users | join(',\n      ') }}
      {
        "id": {{ user.id }},
        "name": "{{ user.name | escape_json }}",
        "email": "{{ user.email }}",
        "active": {{ user.active | default(false) }}
      }
      {{ endforeach }}
    ],
    {{ endif }}
    "total": {{ $input.users | count }},
    "timestamp": "{{ $system.datetime.now | format_date('ISO') }}"
  }
}
"#)?;
```

## Context Management

```rust
use nebula_template::{Context, Value};

let mut context = Context::new();

// Set input data
context.set_input(json!({
    "user": {"name": "Alice", "age": 30},
    "products": [
        {"name": "Widget", "price": 9.99},
        {"name": "Gadget", "price": 19.99}
    ]
}));

// Add node outputs
context.add_node_output("user_data", json!({
    "profile": {"email": "alice@example.com"}
}));

// Set environment variables
context.set_env("API_KEY", "secret123");
context.set_env("DEBUG_MODE", "true");

// Add execution metadata
context.set_execution_data("id", "exec_123");
context.set_execution_data("started_at", "2024-01-01T12:00:00Z");
```

## Error Handling

```rust
use nebula_template::{Template, TemplateError};

match Template::parse("{{ invalid syntax") {
    Ok(template) => {
        // Template parsed successfully
        match template.render(&context) {
            Ok(result) => println!("Result: {}", result),
            Err(TemplateError::EvaluationError { message, .. }) => {
                println!("Evaluation failed: {}", message);
            }
            Err(e) => println!("Other error: {}", e),
        }
    }
    Err(TemplateError::ParseError { message, position, .. }) => {
        println!("Parse error at position {}: {}", position, message);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Custom Functions

Extend the template engine with your own functions:

```rust
use nebula_template::{Function, FunctionSignature, ValueType, Value};

struct EncodeBase64;

impl Function for EncodeBase64 {
    fn name(&self) -> &str { "base64_encode" }
    
    fn signature(&self) -> FunctionSignature {
        FunctionSignature {
            input_type: ValueType::String,
            parameters: vec![],
            return_type: ValueType::String,
        }
    }
    
    fn execute(&self, input: Value, _args: Vec<Value>) -> Result<Value, FunctionError> {
        let text = input.as_str()?;
        let encoded = base64::encode(text);
        Ok(Value::String(encoded))
    }
}

// Register custom function
let mut registry = FunctionRegistry::with_builtins();
registry.register(EncodeBase64);

let template = Template::with_functions("{{ $input.data | base64_encode }}", registry)?;
```

## Performance

Nebula Template is designed for high performance:

- **Zero-copy parsing** where possible
- **Lazy evaluation** of expressions
- **Efficient memory usage** with minimal allocations
- **Caching** of parsed templates
- **SIMD optimizations** for string operations (where available)

Benchmark results on modern hardware:
- Simple substitution: ~50ns per render
- Complex pipeline: ~200ns per render
- Document generation: ~1-5μs depending on size

## Integration

### With Serde

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

let user = User {
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
};

let mut context = Context::new();
context.set_input(serde_json::to_value(user)?);
```

### With Async

```rust
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let template = Template::parse("Hello {{ $input.name }}!")?;
    let context = Context::new();
    
    // Templates are Send + Sync, safe for concurrent use
    let result = tokio::task::spawn_blocking(move || {
        template.render(&context)
    }).await??;
    
    println!("{}", result);
    Ok(())
}
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
git clone https://github.com/your-org/nebula-template
cd nebula-template
cargo test
cargo bench
```

### Running Examples

```bash
cargo run --example basic_usage
cargo run --example document_generation
cargo run --example data_processing
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history.