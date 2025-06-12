# nebula-storage

[![Crates.io](https://img.shields.io/crates/v/nebula-storage.svg)](https://crates.io/crates/nebula-storage)
[![Documentation](https://docs.rs/nebula-storage/badge.svg)](https://docs.rs/nebula-storage)
[![License](https://img.shields.io/crates/l/nebula-storage.svg)](LICENSE)

Storage trait abstractions for the Nebula workflow engine.

## Overview

This crate defines the storage interfaces used by Nebula, allowing for different storage backend implementations:

- **Database trait** - For workflow definitions, execution state, and queue operations
- **BlobStorage trait** - For large binary data (files, images, etc.)
- **Common types** - Query options, filters, and storage-related errors

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nebula-storage = "0.1"

# With tracing support
nebula-storage = { version = "0.1", features = ["tracing"] }
```

### Implementing a Storage Backend

```rust
use nebula_storage::{Database, BlobStorage, StorageError};
use nebula_core::definition::WorkflowDefinition;
use uuid::Uuid;

pub struct MyDatabase {
    // Your database connection
}

impl Database for MyDatabase {
    async fn save_workflow(&self, workflow: &WorkflowDefinition) -> Result<(), StorageError> {
        // Implementation
        Ok(())
    }
    
    async fn get_workflow(&self, id: Uuid) -> Result<Option<WorkflowDefinition>, StorageError> {
        // Implementation
        Ok(None)
    }
    
    // ... other required methods
}

pub struct MyBlobStorage {
    // Your blob storage client
}

impl BlobStorage for MyBlobStorage {
    async fn upload(&self, key: &str, data: &[u8]) -> Result<String, StorageError> {
        // Upload implementation
        Ok(format!("blob://{}", key))
    }
    
    async fn download(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        // Download implementation
        Ok(vec![])
    }
    
    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        // Delete implementation
        Ok(())
    }
}
```

### Using Storage in Your Application

```rust
use nebula_storage::Database;
use std::sync::Arc;

async fn example(db: Arc<dyn Database>) {
    // List workflows
    let workflows = db.list_workflows(Default::default()).await?;
    
    // Get specific workflow
    if let Some(workflow) = db.get_workflow(workflow_id).await? {
        println!("Found workflow: {}", workflow.name);
    }
    
    // Queue operations
    let job = db.claim_job(worker_id).await?;
}
```

## Traits

### Database

The main trait for persistent storage operations:

```rust
pub trait Database: Send + Sync {
    // Workflow operations
    async fn save_workflow(&self, workflow: &WorkflowDefinition) -> Result<(), StorageError>;
    async fn get_workflow(&self, id: Uuid) -> Result<Option<WorkflowDefinition>, StorageError>;
    async fn list_workflows(&self, options: ListOptions) -> Result<Vec<WorkflowDefinition>, StorageError>;
    
    // Execution operations
    async fn create_execution(&self, execution: &ExecutionState) -> Result<(), StorageError>;
    async fn update_execution(&self, execution: &ExecutionState) -> Result<(), StorageError>;
    async fn get_execution(&self, id: Uuid) -> Result<Option<ExecutionState>, StorageError>;
    
    // Queue operations
    async fn push_job(&self, job: &WorkflowJob) -> Result<(), StorageError>;
    async fn claim_job(&self, worker_id: Uuid) -> Result<Option<WorkflowJob>, StorageError>;
    async fn complete_job(&self, job_id: Uuid, result: JobResult) -> Result<(), StorageError>;
}
```

### BlobStorage

For handling large binary data:

```rust
pub trait BlobStorage: Send + Sync {
    async fn upload(&self, key: &str, data: &[u8]) -> Result<String, StorageError>;
    async fn download(&self, key: &str) -> Result<Vec<u8>, StorageError>;
    async fn delete(&self, key: &str) -> Result<(), StorageError>;
    async fn exists(&self, key: &str) -> Result<bool, StorageError>;
}
```

## Available Implementations

- [`nebula-storage-postgres`](https://crates.io/crates/nebula-storage-postgres) - PostgreSQL + S3
- `nebula-storage-sqlite` (planned) - SQLite + filesystem
- `nebula-storage-memory` (planned) - In-memory for testing

## Features

- `tracing` - Enable tracing instrumentation (disabled by default)

## Creating Your Own Implementation

1. Create a new crate (e.g., `nebula-storage-mysql`)
2. Add `nebula-storage` as a dependency
3. Implement the `Database` and/or `BlobStorage` traits
4. Write integration tests
5. Publish to crates.io

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.