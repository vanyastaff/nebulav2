# nebula-core

[![Crates.io](https://img.shields.io/crates/v/nebula-core.svg)](https://crates.io/crates/nebula-core)
[![Documentation](https://docs.rs/nebula-core/badge.svg)](https://docs.rs/nebula-core)
[![License](https://img.shields.io/crates/l/nebula-core.svg)](LICENSE)

Core types, traits, and interfaces for the Nebula workflow engine.

## Overview

This crate provides the foundational types and traits used throughout the Nebula ecosystem:

- **Action Traits** - Base traits for workflow nodes (`Action`, `TriggerAction`, etc.)
- **Data Types** - Core structures like `WorkflowDefinition`, `ExecutionState`, `WorkflowDataItem`
- **Context** - `ProcessContext` for node execution
- **Resources & Credentials** - Traits for shared resources and secure credentials
- **Error Types** - Common error definitions

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nebula-core = "0.1"

# With tracing support
nebula-core = { version = "0.1", features = ["tracing"] }
```

### Defining a Custom Action

```rust
use nebula_core::action::{Action, ActionResult};
use nebula_core::context::ProcessContext;
use nebula_core::error::EngineError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyInput {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyOutput {
    pub result: String,
}

pub struct MyAction;

impl Action for MyAction {
    type Input = MyInput;
    type Output = MyOutput;
    
    fn id(&self) -> &'static str { "MY_ACTION" }
    fn name(&self) -> &'static str { "My Action" }
    fn description(&self) -> &'static str { "Does something useful" }
    
    async fn execute(
        &self,
        input: Self::Input,
        context: &mut ProcessContext,
    ) -> Result<ActionResult<Self::Output>, EngineError> {
        // Your logic here
        Ok(ActionResult::Value(MyOutput {
            result: format!("Processed: {}", input.message),
        }))
    }
}
```

### Working with Workflow Definitions

```rust
use nebula_core::definition::{WorkflowDefinition, NodeDefinition};
use uuid::Uuid;

let workflow = WorkflowDefinition {
    id: Uuid::new_v4(),
    name: "My Workflow".to_string(),
    version: 1,
    description: Some("Example workflow".to_string()),
    nodes: vec![
        NodeDefinition {
            id: "start".to_string(),
            action_type_id: "HTTP_TRIGGER".to_string(),
            name: "Start Trigger".to_string(),
            parameters: Default::default(),
            position: None,
        },
    ],
    connections: vec![],
    metadata: Default::default(),
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
};
```

## Features

- `tracing` - Enable tracing instrumentation (disabled by default)

## Type Overview

### Action System
- `Action` - Base trait for all workflow nodes
- `TriggerAction` - Nodes that start workflows
- `ProcessAction` - Standard processing nodes
- `SupplyDataAction` - Nodes that provide instances to other nodes

### Data Types
- `WorkflowDefinition` - Complete workflow structure
- `NodeDefinition` - Individual node configuration
- `ExecutionState` - Runtime state of a workflow execution
- `WorkflowDataItem` - Data passed between nodes
- `Connection` - Describes links between nodes

### Support Types
- `ProcessContext` - Execution context for nodes
- `Resource` - Shared, reusable components
- `Credential` - Secure credential storage
- `EngineError` - Error types

## Minimum Supported Rust Version

This crate requires Rust 1.85 or later (2024 edition).

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.