# nebula-worker

[![Crates.io](https://img.shields.io/crates/v/nebula-worker.svg)](https://crates.io/crates/nebula-worker)
[![Documentation](https://docs.rs/nebula-worker/badge.svg)](https://docs.rs/nebula-worker)
[![License](https://img.shields.io/crates/l/nebula-worker.svg)](LICENSE)

Queue worker for processing Nebula workflow executions.

## Overview

This crate provides a scalable worker implementation for processing workflow executions:

- Poll-based or push-based queue consumption
- Concurrent execution with configurable worker pools
- Automatic retries and error handling
- Health checks and metrics
- Graceful shutdown
- Horizontal scaling support

## Usage

### Running the Worker

```bash
# Start with default settings
nebula-worker

# With custom concurrency
nebula-worker --concurrency 8

# With custom worker ID
nebula-worker --worker-id worker-prod-01

# With health check endpoint
nebula-worker --health-port 9090
```

### Command Line Options

```
nebula-worker 0.1.0
Queue worker for Nebula workflow engine

USAGE:
    nebula-worker [OPTIONS]

OPTIONS:
    -c, --concurrency <N>      Number of concurrent workers [default: 4]
    -w, --worker-id <ID>       Worker pool identifier [env: WORKER_ID]
    -h, --health-port <PORT>   Health check port [default: 9090]
    -p, --poll-interval <MS>   Queue poll interval in ms [default: 100]
        --config <FILE>        Configuration file path
    -v, --verbose             Enable verbose logging
        --help                Display help information
```

### Docker

```dockerfile
FROM rust:1.85 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin nebula-worker

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/nebula-worker /usr/local/bin/
CMD ["nebula-worker"]
```

## Architecture

```
Worker Pool
├── Queue Manager
│   ├── Job Claiming
│   ├── Heartbeat
│   └── Result Publishing
├── Worker Threads
│   ├── Job Processing
│   ├── Error Handling
│   └── State Updates
└── Health Monitor
    ├── Liveness Check
    ├── Readiness Check
    └── Metrics Collection
```

## Configuration

### Environment Variables

```bash
# Worker settings
WORKER_CONCURRENCY=8
WORKER_ID=worker-prod-01
QUEUE_POLL_INTERVAL_MS=100
WORKER_SHUTDOWN_TIMEOUT_SECONDS=30

# Database
DATABASE_URL=postgres://user:pass@localhost/nebula

# Health checks
HEALTH_PORT=9090
ENABLE_METRICS=true

# Logging
RUST_LOG=nebula_worker=info
```

### Configuration File

```toml
# config/worker.toml
[worker]
concurrency = 8
poll_interval_ms = 100
heartbeat_interval_seconds = 30
shutdown_timeout_seconds = 30

[queue]
batch_size = 10
visibility_timeout_seconds = 300
max_retries = 3

[database]
url = "postgres://localhost/nebula"
max_connections = 20

[health]
port = 9090
check_interval_seconds = 10
```

## Scaling Strategies

### Horizontal Scaling

Deploy multiple worker instances:

```yaml
# docker-compose.yml
services:
  worker:
    image: nebula-worker:latest
    command: ["nebula-worker", "--concurrency", "4"]
    environment:
      - DATABASE_URL=postgres://db/nebula
    deploy:
      replicas: 10
```

### Kubernetes HPA

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: nebula-worker-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: nebula-worker
  minReplicas: 2
  maxReplicas: 50
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: External
    external:
      metric:
        name: queue_depth
        selector:
          matchLabels:
            queue: nebula_jobs
      target:
        type: AverageValue
        averageValue: "30"
```

## Queue Behavior

### Job Processing Flow

1. **Claim**: Worker claims job from queue with visibility timeout
2. **Process**: Execute workflow using runtime
3. **Heartbeat**: Extend visibility timeout for long-running jobs
4. **Complete**: Mark job as complete or failed

### Error Handling

```rust
// Automatic retry for transient errors
match process_job(job).await {
    Ok(_) => complete_job(job.id),
    Err(e) if e.is_retryable() => {
        if job.retry_count < max_retries {
            release_job(job.id, retry_delay);
        } else {
            fail_job(job.id, "Max retries exceeded");
        }
    }
    Err(e) => fail_job(job.id, e.to_string()),
}
```

## Health Checks

The worker exposes health endpoints:

```http
# Liveness - worker process is running
GET /health
200 OK

# Readiness - worker can process jobs
GET /ready
200 OK
{
  "status": "ready",
  "workers": 8,
  "active_jobs": 3,
  "queue_connected": true
}

# Metrics (Prometheus format)
GET /metrics
```

## Monitoring

Key metrics exposed:

- `worker_jobs_processed_total` - Total jobs processed
- `worker_jobs_failed_total` - Total failed jobs
- `worker_job_duration_seconds` - Job processing time
- `worker_active_jobs` - Currently processing jobs
- `worker_queue_depth` - Pending jobs in queue

## Graceful Shutdown

The worker handles shutdown signals gracefully:

1. Stop claiming new jobs
2. Wait for active jobs to complete (with timeout)
3. Release any incomplete jobs back to queue
4. Close database connections

```bash
# Send shutdown signal
kill -TERM <worker-pid>

# Force shutdown after timeout
kill -KILL <worker-pid>
```

## Features

- `tracing` (default) - Enable execution tracing
- `health` (default) - Enable health check endpoint

## Performance Tuning

- **Concurrency**: Set based on CPU cores and job types
- **Poll Interval**: Lower for responsive processing, higher to reduce DB load
- **Batch Size**: Larger batches reduce overhead but increase memory usage
- **Connection Pool**: Size based on concurrency + overhead

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.