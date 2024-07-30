.PHONY: helm-docs run watch-run docker-build

# Generate Helm chart documentation
helm-docs:
	cd charts/tembo-metrics && helm-docs --output-file CHART_README.md

# Run the application
run:
	cargo run

# Watch for changes and run the application
watch-run:
	cargo watch -x run

# Build Docker image
docker-build:
	docker buildx build -f Dockerfile -t tembo-metrics:latest .

# Run all checks (add more as needed)
check: helm-docs
	cargo fmt -- --check
	cargo clippy -- -D warnings

# Build the project
build:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean

# Run tests
test:
	cargo test

# All-in-one command for checks, tests, and build
all: check test build

# Help command to display available make targets
help:
	@echo "Available targets:"
	@echo "  helm-docs     - Generate Helm chart documentation"
	@echo "  run           - Run the application"
	@echo "  watch-run     - Watch for changes and run the application"
	@echo "  docker-build  - Build Docker image"
	@echo "  check         - Run all checks"
	@echo "  build         - Build the project"
	@echo "  clean         - Clean build artifacts"
	@echo "  test          - Run tests"
	@echo "  all           - Run checks, tests, and build"
