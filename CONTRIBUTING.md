# Contributing to Nebula

Thank you for your interest in contributing to Nebula! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/nebula.git
   cd nebula
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/your-org/nebula.git
   ```
4. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust 1.85+ (for Rust 2024 edition)
- Docker and Docker Compose
- PostgreSQL 14+ (or use Docker)
- MinIO or S3-compatible storage (or use Docker)

### Environment Setup

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

2. Install development tools:
   ```bash
   make install-tools
   ```

3. Start development services:
   ```bash
   make dev
   ```

4. Run migrations:
   ```bash
   make migrate
   ```

## Development Workflow

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build specific crate
cargo build -p nebula-core
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p nebula-runtime

# Run tests with coverage
make test-coverage

# Run integration tests
make test-integration
```

### Code Quality

Before submitting a PR, ensure:

1. **Format your code:**
   ```bash
   cargo fmt
   ```

2. **Run lints:**
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Check for security issues:**
   ```bash
   cargo audit
   cargo deny check
   ```

4. **Update documentation:**
   ```bash
   cargo doc --no-deps
   ```

### Commit Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `test:` Test additions or changes
- `chore:` Build process or auxiliary tool changes

Examples:
```
feat(runtime): add support for async actions
fix(storage): handle connection timeouts properly
docs(readme): update installation instructions
```

## Pull Request Process

1. **Update your branch:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run pre-commit checks:**
   ```bash
   make pre-commit
   ```

3. **Push your changes:**
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Create a Pull Request** with:
    - Clear title and description
    - Link to related issues
    - Screenshots/examples if applicable

### PR Requirements

- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings
- [ ] Documentation is updated
- [ ] Commit messages follow conventions
- [ ] PR description explains the changes

## Adding New Features

### Adding a New Node Type

1. Create the node in `nebula-nodes/src/`:
   ```rust
   // my_node.rs
   use nebula_core::action::{Action, ActionResult};
   
   pub struct MyNode;
   
   impl Action for MyNode {
       // Implementation
   }
   ```

2. Export it in `lib.rs`:
   ```rust
   pub mod my_node;
   pub use my_node::MyNode;
   ```

3. Add tests in `nebula-nodes/tests/`

4. Document usage in `examples/`

### Adding Storage Backends

1. Create new crate `nebula-storage-{backend}`
2. Implement `Database` and/or `BlobStorage` traits
3. Add integration tests
4. Update documentation

## Testing Guidelines

### Unit Tests

- Test individual functions and methods
- Use mocks for external dependencies
- Keep tests focused and fast

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_node_execution() {
        // Test implementation
    }
}
```

### Integration Tests

- Test interactions between components
- Use real services when possible
- Place in `tests/` directory

```rust
// tests/workflow_execution.rs
#[tokio::test]
async fn test_full_workflow() {
    // Test implementation
}
```

## Documentation

- Add rustdoc comments to all public APIs
- Include examples in doc comments
- Update README when adding features
- Add entries to CHANGELOG.md

Example:
```rust
/// Executes the workflow with the given ID.
/// 
/// # Arguments
/// 
/// * `workflow_id` - The unique identifier of the workflow
/// 
/// # Returns
/// 
/// Returns `Ok(())` if execution completes successfully.
/// 
/// # Example
/// 
/// ```rust
/// let runtime = Runtime::new(config);
/// runtime.execute_workflow(workflow_id).await?;
/// ```
pub async fn execute_workflow(&self, workflow_id: Uuid) -> Result<()> {
    // Implementation
}
```

## Performance Considerations

- Profile before optimizing
- Add benchmarks for critical paths
- Consider memory allocations
- Use `cargo flamegraph` for profiling

## Release Process

1. Update version in `Cargo.toml` files
2. Update CHANGELOG.md
3. Create release PR
4. After merge, tag release:
   ```bash
   git tag -a v0.1.0 -m "Release version 0.1.0"
   git push upstream v0.1.0
   ```

## Getting Help

- Check existing issues and PRs
- Ask in Discord: [https://discord.gg/nebula](https://discord.gg/nebula)
- Read the documentation
- Contact maintainers

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation

Thank you for contributing to Nebula!