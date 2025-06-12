# nebula-runtime

[![Crates.io](https://img.shields.io/crates/v/nebula-runtime.svg)](https://crates.io/crates/nebula-runtime)
[![Documentation](https://docs.rs/nebula-runtime/badge.svg)](https://docs.rs/nebula-runtime)
[![License](https://img.shields.io/crates/l/nebula-runtime.svg)](LICENSE)

The workflow execution engine for Nebula.

## Overview

This crate provides the core runtime components for executing workflows:

- **Scheduler** - Orchestrates workflow execution and node scheduling
- **State Manager** - Handles persistence and recovery of execution state
- **Expression Parser** - Evaluates dynamic expressions in node parameters
- **Runner** - Main execution loop for processing workflows
- **ProcessContext** - Provides execution context to nodes

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nebula-runtime = "0.1"

# Without tracing (minimal build)
nebula-runtime = { version = "0.1", default-features = false }
```

### Creating a Runtime

```rust
use nebula_runtime::{Runtime, RuntimeConfig};
use nebula_storage_postgres::PostgresStorage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage backend
    let storage = PostgresStorage::new(conn_string).await?;
    
    // Create runtime
    let config = RuntimeConfig::default();
    let runtime = Runtime::new(config, storage);
    
    // Start runtime
    runtime.start().await?;
    
    Ok(())
}
```

### Executing Workflows

```rust
use nebula_runtime::Runtime;
use uuid::Uuid;

async fn execute_workflow(runtime: &Runtime, workflow_id: Uuid) {
    // Create new execution
    let execution_id = runtime.create_execution(workflow_id).await?;
    
    // Start execution
    runtime.start_execution(execution_id).await?;
    
    // Check status
    let status = runtime.get_execution_status(execution_id).await?;
    println!("Execution status: {:?}", status);
}
```

### Expression Syntax

The runtime supports expressions for dynamic parameter values:

```javascript
// Access node output
{{ $node('previous_node').json.field }}

// Access trigger data
{{ $trigger.json.body.message }}

// Access execution metadata
{{ $execution.id }}
{{ $execution.workflowId }}

// Access environment (if enabled)
{{ $env.API_KEY }}

// Functions
{{ $json.parse($node('http').json.response) }}
{{ $date.now() }}
{{ $string.uppercase($input.text) }}
```

### Custom Node Registration

```rust
use nebula_runtime::{Runtime, Registry};
use my_nodes::CustomNode;

let mut registry = Registry::new();
registry.register(Box::new(CustomNode::new()));

let runtime = Runtime::builder()
    .with_storage(storage)
    .with_registry(registry)
    .build();
```

## Architecture

```
Runtime
├── Scheduler
│   ├── DAG Analysis
│   ├── Node Dependencies
│   └── Execution Planning
├── State Manager
│   ├── Persistence
│   ├── Recovery
│   └── Checkpointing
├── Expression Parser
│   ├── Tokenizer
│   ├── Parser
│   └── Evaluator
└── Runner
    ├── Node Execution
    ├── Error Handling
    └── Result Processing
```

## Configuration

```rust
use nebula_runtime::{RuntimeConfig, RetryPolicy};
use std::time::Duration;

let config = RuntimeConfig {
    // Maximum concurrent executions
    max_concurrent_executions: 100,
    
    // Default timeout for node execution
    node_execution_timeout: Duration::from_secs(300),
    
    // Retry policy for failed nodes
    retry_policy: RetryPolicy {
        max_attempts: 3,
        initial_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(60),
        exponential_base: 2.0,
    },
    
    // Enable expression sandboxing
    enable_expression_sandbox: true,
    
    // State persistence interval
    persistence_interval: Duration::from_secs(10),
};
```

## Features

- `tracing` (default) - Enable execution tracing and logging

## Performance Considerations

- Workflows are analyzed once and execution plan is cached
- State is persisted asynchronously to avoid blocking execution
- Expressions are parsed once and cached per node
- Parallel node execution where possible (based on DAG analysis)

## Error Handling

The runtime provides comprehensive error handling:

```rust
use nebula_runtime::{RuntimeError, ExecutionError};

match runtime.execute(workflow_id).await {
    Ok(result) => println!("Success: {:?}", result),
    Err(RuntimeError::WorkflowNotFound(id)) => {
        eprintln!("Workflow {} not found", id);
    }
    Err(RuntimeError::ExecutionFailed(ExecutionError::NodeTimeout(node))) => {
        eprintln!("Node {} timed out", node);
    }
    Err(e) => eprintln!("Execution failed: {}", e),
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.