# nebula-storage-postgres

[![Crates.io](https://img.shields.io/crates/v/nebula-storage-postgres.svg)](https://crates.io/crates/nebula-storage-postgres)
[![Documentation](https://docs.rs/nebula-storage-postgres/badge.svg)](https://docs.rs/nebula-storage-postgres)
[![License](https://img.shields.io/crates/l/nebula-storage-postgres.svg)](LICENSE)

PostgreSQL and S3 storage implementation for Nebula workflow engine.

## Overview

This crate provides production-ready storage backends:

- **PostgreSQL** - For workflow definitions, execution state, and job queue
- **S3/MinIO** - For large binary data and workflow artifacts
- **Connection pooling** - Using deadpool-postgres
- **Migrations** - Using refinery
- **Optimized queries** - With proper indexing and batch operations

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nebula-storage-postgres = "0.1"
```

### Basic Setup

```rust
use nebula_storage_postgres::{PostgresStorage, StorageConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // From environment
    let storage = PostgresStorage::from_env().await?;
    
    // Or with config
    let config = StorageConfig {
        database_url: "postgres://user:pass@localhost/nebula".to_string(),
        max_connections: 100,
        s3_endpoint: Some("http://localhost:9000".to_string()),
        s3_bucket: "nebula-artifacts".to_string(),
        s3_access_key: "minioadmin".to_string(),
        s3_secret_key: "minioadmin".to_string(),
    };
    
    let storage = PostgresStorage::new(config).await?;
    
    // Run migrations
    storage.run_migrations().await?;
    
    Ok(())
}
```

### Using as Storage Backend

```rust
use nebula_storage::{Database, BlobStorage};
use nebula_runtime::Runtime;

let storage = PostgresStorage::from_env().await?;

// Use as Database trait
let db: Arc<dyn Database> = Arc::new(storage.clone());

// Use as BlobStorage trait  
let blob: Arc<dyn BlobStorage> = Arc::new(storage);

// Create runtime with storage
let runtime = Runtime::new(config, db);
```

## Database Schema

### Core Tables

```sql
-- Workflows table
CREATE TABLE workflows (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    version INTEGER NOT NULL,
    definition JSONB NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    deployed_at TIMESTAMPTZ,
    UNIQUE(name, version)
);

-- Executions table
CREATE TABLE executions (
    id UUID PRIMARY KEY,
    workflow_id UUID NOT NULL REFERENCES workflows(id),
    status VARCHAR(50) NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    state JSONB NOT NULL,
    error TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- Job queue table
CREATE TABLE workflow_jobs (
    id UUID PRIMARY KEY,
    workflow_id UUID NOT NULL,
    execution_id UUID NOT NULL,
    priority INTEGER DEFAULT 0,
    scheduled_at TIMESTAMPTZ NOT NULL,
    locked_by UUID,
    locked_at TIMESTAMPTZ,
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,
    status VARCHAR(50) NOT NULL,
    payload JSONB,
    error TEXT,
    created_at TIMESTAMPTZ NOT NULL
);

-- Indexes for performance
CREATE INDEX idx_workflows_status ON workflows(status);
CREATE INDEX idx_executions_workflow_status ON executions(workflow_id, status);
CREATE INDEX idx_jobs_queue ON workflow_jobs(status, priority DESC, scheduled_at) 
    WHERE status = 'pending';
```

## Configuration

### Environment Variables

```bash
# PostgreSQL
DATABASE_URL=postgres://user:password@localhost:5432/nebula
DATABASE_MAX_CONNECTIONS=100
DATABASE_MIN_CONNECTIONS=10

# S3/MinIO
S3_ENDPOINT=http://localhost:9000
S3_REGION=us-east-1
S3_BUCKET=nebula-artifacts
S3_ACCESS_KEY_ID=minioadmin
S3_SECRET_ACCESS_KEY=minioadmin
S3_USE_PATH_STYLE=true  # For MinIO

# Optional
ENABLE_STATEMENT_LOGGING=false
MIGRATION_PATH=./migrations
```

### Connection Pool Configuration

```rust
use nebula_storage_postgres::{PostgresStorage, PoolConfig};

let pool_config = PoolConfig {
    max_size: 100,
    min_idle: 10,
    max_lifetime: Some(Duration::from_secs(30 * 60)),
    idle_timeout: Some(Duration::from_secs(10 * 60)),
    connection_timeout: Duration::from_secs(30),
};

let storage = PostgresStorage::with_pool_config(database_url, pool_config).await?;
```

## Migrations

Migrations are managed using refinery and located in `migrations/` directory:

```sql
-- V1__initial_schema.sql
CREATE TABLE workflows (...);

-- V2__add_executions.sql
CREATE TABLE executions (...);

-- V3__add_job_queue.sql
CREATE TABLE workflow_jobs (...);
```

Run migrations:

```rust
// Programmatically
storage.run_migrations().await?;

// Or using CLI
refinery migrate -e DATABASE_URL -p migrations
```

## Performance Optimizations

### Query Optimization

- Proper indexes on frequently queried columns
- Partial indexes for queue queries
- JSONB GIN indexes for workflow definitions
- Prepared statements via tokio-postgres

### Batch Operations

```rust
// Batch insert jobs
let jobs = vec![job1, job2, job3];
storage.push_jobs(&jobs).await?;

// Batch update executions
storage.update_executions(&executions).await?;
```

### Connection Pooling

- Automatic connection recycling
- Health checks on idle connections
- Configurable pool sizes based on workload

## S3 Integration

### Binary Data Storage

```rust
use nebula_storage::BlobStorage;

// Upload workflow artifact
let url = storage.upload(
    "workflows/123/output.json",
    output_data.as_bytes()
).await?;

// Download artifact
let data = storage.download("workflows/123/output.json").await?;

// Check existence
if storage.exists("workflows/123/output.json").await? {
    // Process file
}
```

### Multipart Uploads

For large files, the storage automatically uses multipart uploads:

```rust
// Files > 5MB automatically use multipart
let large_data = vec![0u8; 10 * 1024 * 1024]; // 10MB
storage.upload("large-file.bin", &large_data).await?;
```

## Error Handling

```rust
use nebula_storage_postgres::StorageError;

match storage.get_workflow(id).await {
    Ok(Some(workflow)) => process_workflow(workflow),
    Ok(None) => println!("Workflow not found"),
    Err(StorageError::Database(e)) => {
        eprintln!("Database error: {}", e);
    }
    Err(StorageError::S3(e)) => {
        eprintln!("S3 error: {}", e);
    }
    Err(e) => eprintln!("Storage error: {}", e),
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use testcontainers::{clients, images};
    
    #[tokio::test]
    async fn test_storage() {
        // Start PostgreSQL container
        let docker = clients::Cli::default();
        let postgres = docker.run(images::postgres::Postgres::default());
        
        // Start MinIO container
        let minio = docker.run(images::minio::MinIO::default());
        
        // Create storage with test containers
        let storage = PostgresStorage::new(test_config).await.unwrap();
        
        // Run tests
        test_workflow_operations(&storage).await;
    }
}
```

## Features

- `tracing` - Enable query tracing (disabled by default)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.