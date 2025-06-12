.PHONY: all build test check fmt lint clean doc release install-tools

# Default target
all: check test

# Build all crates
build:
	cargo build --all-features --workspace

# Build in release mode
release:
	cargo build --release --all-features --workspace

# Run all tests
test:
	cargo test --all-features --workspace

# Run tests with coverage (requires cargo-llvm-cov)
test-coverage:
	cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Run integration tests only
test-integration:
	cargo test --all-features --workspace --test '*' -- --test-threads=1

# Run benchmarks
bench:
	cargo bench --all-features --workspace

# Format code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all -- --check

# Run clippy
lint:
	cargo clippy --all-features --workspace --all-targets -- -D warnings

# Run all checks (format, lint, build, test)
check: fmt-check lint
	cargo check --all-features --workspace --all-targets

# Run cargo-deny checks
check-deps:
	cargo deny check

# Run security audit
audit:
	cargo audit

# Generate documentation
doc:
	cargo doc --all-features --workspace --no-deps --open

# Clean build artifacts
clean:
	cargo clean

# Install development tools
install-tools:
	@echo "Installing development tools..."
	cargo install cargo-watch
	cargo install cargo-edit
	cargo install cargo-outdated
	cargo install cargo-audit
	cargo install cargo-deny
	cargo install cargo-llvm-cov
	cargo install cargo-nextest
	cargo install cargo-machete
	cargo install cargo-expand
	cargo install sqlx-cli --features postgres
	cargo install refinery_cli

# Watch for changes and run tests
watch:
	cargo watch -x check -x test

# Watch for changes and run specific crate tests
watch-crate:
	@echo "Usage: make watch-crate CRATE=nebula-core"
	cargo watch -x "test -p $(CRATE)"

# Update dependencies
update:
	cargo update
	cargo outdated

# Remove unused dependencies
cleanup-deps:
	cargo machete

# Run database migrations (requires DATABASE_URL)
migrate:
	refinery migrate -e DATABASE_URL -p crates/nebula-storage-postgres/migrations

# Create new migration
migration-new:
	@echo "Usage: make migration-new NAME=create_workflows_table"
	refinery migrate generate $(NAME) -p crates/nebula-storage-postgres/migrations

# Run local development environment
dev:
	docker-compose up -d postgres minio
	@echo "PostgreSQL: localhost:5432"
	@echo "MinIO: localhost:9000"
	@echo "MinIO Console: localhost:9001"

# Stop development environment
dev-stop:
	docker-compose down

# Run example workflow
example:
	cargo run --example basic_workflow

# Package all crates
package:
	@for crate in nebula-*; do \
		echo "Packaging $$crate..."; \
		(cd crates/$$crate && cargo package --allow-dirty); \
	done

# Lint Dockerfiles
lint-docker:
	hadolint Dockerfile

# Run pre-commit checks
pre-commit: fmt check test

# CI/CD pipeline simulation
ci: fmt-check lint check-deps audit test doc

# Show help
help:
	@echo "Available targets:"
	@echo "  all           - Run checks and tests (default)"
	@echo "  build         - Build all crates"
	@echo "  release       - Build in release mode"
	@echo "  test          - Run all tests"
	@echo "  test-coverage - Run tests with coverage"
	@echo "  bench         - Run benchmarks"
	@echo "  fmt           - Format code"
	@echo "  lint          - Run clippy lints"
	@echo "  check         - Run all checks"
	@echo "  check-deps    - Check dependencies with cargo-deny"
	@echo "  audit         - Run security audit"
	@echo "  doc           - Generate documentation"
	@echo "  clean         - Clean build artifacts"
	@echo "  install-tools - Install development tools"
	@echo "  watch         - Watch for changes and run tests"
	@echo "  update        - Update dependencies"
	@echo "  dev           - Run local development environment"
	@echo "  pre-commit    - Run pre-commit checks"
	@echo "  ci            - Run CI pipeline"