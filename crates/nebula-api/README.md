# nebula-api

[![Crates.io](https://img.shields.io/crates/v/nebula-server.svg)](https://crates.io/crates/nebula-server)
[![Documentation](https://docs.rs/nebula-server/badge.svg)](https://docs.rs/nebula-server)
[![License](https://img.shields.io/crates/l/nebula-server.svg)](LICENSE)

REST API server for the Nebula workflow engine.

## Overview

This crate provides the HTTP API server for Nebula, offering:

- RESTful API for workflow management
- WebSocket support for real-time updates
- Webhook endpoints for triggers
- Health and metrics endpoints
- Authentication and authorization (optional)
- OpenAPI documentation (optional)

## Usage

### Running the Server

```bash
# Start API server only
nebula-api api --port 8080

# Start with embedded worker
nebula-api all --port 8080 --worker-concurrency 4

# With custom config
nebula-api api --config config/production.toml
```

### Docker

```dockerfile
FROM rust:1.85 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin nebula-server

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/nebula-server /usr/local/bin/
CMD ["nebula-server", "api"]
```

## API Endpoints

### Workflows

```http
# List workflows
GET /api/v1/workflows

# Get workflow
GET /api/v1/workflows/{id}

# Create workflow
POST /api/v1/workflows
Content-Type: application/json

{
  "name": "My Workflow",
  "nodes": [...],
  "connections": [...]
}

# Update workflow
PUT /api/v1/workflows/{id}

# Delete workflow
DELETE /api/v1/workflows/{id}

# Deploy workflow
POST /api/v1/workflows/{id}/deploy

# Execute workflow
POST /api/v1/workflows/{id}/execute
```

### Executions

```http
# List executions
GET /api/v1/executions

# Get execution
GET /api/v1/executions/{id}

# Get execution status
GET /api/v1/executions/{id}/status

# Cancel execution
POST /api/v1/executions/{id}/cancel

# Get execution logs
GET /api/v1/executions/{id}/logs
```

### Triggers

```http
# Webhook trigger
POST /webhook/{workflow_id}/{path}

# Get trigger info
GET /api/v1/triggers/{workflow_id}
```

### System

```http
# Health check
GET /health

# Readiness check
GET /ready

# Metrics (Prometheus format)
GET /metrics

# OpenAPI documentation
GET /api/docs
```

## Configuration

### Environment Variables

```bash
# Server
NEBULA_HOST=0.0.0.0
NEBULA_PORT=8080

# Database
DATABASE_URL=postgres://user:pass@localhost/nebula

# S3/MinIO
S3_ENDPOINT=http://localhost:9000
S3_ACCESS_KEY_ID=minioadmin
S3_SECRET_ACCESS_KEY=minioadmin

# Optional features
ENABLE_AUTH=true
ENABLE_METRICS=true
ENABLE_SWAGGER=true
```

### Configuration File

```toml
# config/server.toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[database]
url = "postgres://localhost/nebula"
max_connections = 100

[auth]
enabled = true
jwt_secret = "your-secret-key"
token_expiry = "24h"

[cors]
allowed_origins = ["http://localhost:3000"]
allowed_methods = ["GET", "POST", "PUT", "DELETE"]
allowed_headers = ["Content-Type", "Authorization"]

[rate_limit]
enabled = true
requests_per_minute = 60
burst_size = 20
```

## Features

- `tracing` (default) - Request/response tracing

## Authentication

When authentication is enabled:

```http
# Login
POST /auth/login
Content-Type: application/json

{
  "username": "admin",
  "password": "password"
}

# Response
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "expires_at": "2024-01-01T00:00:00Z"
}

# Use token
GET /api/v1/workflows
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

## WebSocket Support

For real-time updates:

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onopen = () => {
  // Subscribe to execution updates
  ws.send(JSON.stringify({
    type: 'subscribe',
    execution_id: '550e8400-e29b-41d4-a716-446655440000'
  }));
};

ws.onmessage = (event) => {
  const update = JSON.parse(event.data);
  console.log('Execution update:', update);
};
```

## Health Checks

The server provides health check endpoints for container orchestration:

```bash
# Liveness probe
curl http://localhost:8080/health

# Readiness probe (checks database connection)
curl http://localhost:8080/ready
```

## Deployment

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nebula-api
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: nebula-api
        image: nebula-api:latest
        command: ["nebula-api", "api"]
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: nebula-secrets
              key: database-url
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
```

## Monitoring

When metrics are enabled, the server exposes Prometheus metrics:

- `http_requests_total` - Total HTTP requests
- `http_request_duration_seconds` - Request latency
- `workflow_executions_total` - Total workflow executions
- `active_executions` - Currently active executions

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.