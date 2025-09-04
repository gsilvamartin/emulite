# Makefile for Emulite

.PHONY: all build test bench clean fmt clippy docs install uninstall

# Default target
all: build

# Build the project
build:
	cargo build --release

# Build debug version
build-debug:
	cargo build

# Run tests
test:
	cargo test

# Run tests with output
test-verbose:
	cargo test -- --nocapture

# Run benchmarks
bench:
	cargo bench

# Run examples
examples:
	cargo run --example basic_usage
	cargo run --example debug_example
	cargo run --example cpu_test

# Format code
fmt:
	cargo fmt

# Check formatting
fmt-check:
	cargo fmt -- --check

# Run clippy
clippy:
	cargo clippy -- -D warnings

# Run clippy with all lints
clippy-all:
	cargo clippy --all-targets --all-features -- -D warnings

# Generate documentation
docs:
	cargo doc --open

# Generate documentation for all features
docs-all:
	cargo doc --all-features --open

# Clean build artifacts
clean:
	cargo clean

# Install the binary
install: build
	cargo install --path .

# Uninstall the binary
uninstall:
	cargo uninstall emulite

# Run with a ROM file
run:
	cargo run --release -- $(ROM)

# Run with debug
run-debug:
	cargo run -- $(ROM)

# Run with specific platform
run-platform:
	cargo run --release -- $(ROM) $(PLATFORM)

# Check everything
check: fmt-check clippy test

# Full CI pipeline
ci: fmt-check clippy test bench

# Development setup
dev-setup:
	rustup component add rustfmt clippy
	cargo install cargo-watch

# Watch for changes and run tests
watch-test:
	cargo watch -x test

# Watch for changes and run clippy
watch-clippy:
	cargo watch -x clippy

# Watch for changes and run build
watch-build:
	cargo watch -x build

# Run with specific features
run-features:
	cargo run --release --features $(FEATURES) -- $(ROM)

# Build with specific features
build-features:
	cargo build --release --features $(FEATURES)

# Test with specific features
test-features:
	cargo test --features $(FEATURES)

# Cross-compile for different targets
cross-build:
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-apple-darwin

# Package for distribution
package:
	cargo package

# Publish to crates.io
publish:
	cargo publish

# Help
help:
	@echo "Available targets:"
	@echo "  build          - Build the project in release mode"
	@echo "  build-debug    - Build the project in debug mode"
	@echo "  test           - Run tests"
	@echo "  test-verbose   - Run tests with output"
	@echo "  bench          - Run benchmarks"
	@echo "  examples       - Run examples"
	@echo "  fmt            - Format code"
	@echo "  fmt-check      - Check formatting"
	@echo "  clippy         - Run clippy"
	@echo "  clippy-all     - Run clippy with all lints"
	@echo "  docs           - Generate documentation"
	@echo "  docs-all       - Generate documentation for all features"
	@echo "  clean          - Clean build artifacts"
	@echo "  install        - Install the binary"
	@echo "  uninstall      - Uninstall the binary"
	@echo "  run            - Run with ROM file (set ROM=path/to/rom)"
	@echo "  run-debug      - Run in debug mode"
	@echo "  run-platform   - Run with specific platform (set ROM=path PLATFORM=nes)"
	@echo "  check          - Check formatting, clippy, and tests"
	@echo "  ci             - Full CI pipeline"
	@echo "  dev-setup      - Setup development environment"
	@echo "  watch-test     - Watch for changes and run tests"
	@echo "  watch-clippy   - Watch for changes and run clippy"
	@echo "  watch-build    - Watch for changes and run build"
	@echo "  run-features   - Run with specific features (set FEATURES=feature1,feature2)"
	@echo "  build-features - Build with specific features"
	@echo "  test-features  - Test with specific features"
	@echo "  cross-build    - Cross-compile for different targets"
	@echo "  package        - Package for distribution"
	@echo "  publish        - Publish to crates.io"
	@echo "  help           - Show this help"
