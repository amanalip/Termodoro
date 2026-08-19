# ==============================================================================
# Termodoro Project Automation Makefile
# ==============================================================================

.PHONY: test test-clean check build run clean fmt clippy help

# Default target
all: test-clean

# Run 192 tests and automatically clean build cache to prevent disk bloat
test:
	./scripts/test_and_clean.sh

# Explicit alias for automated test + clean
test-clean:
	./scripts/test_and_clean.sh

# Run all formatting, clippy static analysis, tests, and clean up
check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	./scripts/test_and_clean.sh

# Build optimized production release binary
build:
	cargo build --release

# Run application in release mode
run:
	cargo run --release

# Reclaim disk space by deleting local target/ directory
clean:
	cargo clean

# Format all Rust source files
fmt:
	cargo fmt --all

# Run strict compiler linter
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run Playwright E2E responsive test suite for project website
test-e2e:
	node scripts/e2e-website-test.mjs

# Run full sanity & fact-checking audit against Rust source code & Markdown links
check-facts:
	node scripts/sanity_and_fact_check.mjs
	node scripts/verify_markdown_links.mjs

# Validate all local Markdown links and anchor slugs across all documentation
check-links:
	node scripts/verify_markdown_links.mjs

# Display available commands
help:
	@echo "Termodoro Makefile Commands:"
	@echo "  make test        - Run full 192 tests and automatically clean target/ cache"
	@echo "  make test-clean  - Run tests and auto-clean (reclaims ~1.8GB disk space)"
	@echo "  make test-e2e    - Run Playwright E2E test suite (desktop + mobile viewports)"
	@echo "  make check-facts - Run full sanity, fact-check, and Markdown link audit"
	@echo "  make check-links - Verify 100% of all Markdown internal links and anchors"
	@echo "  make check       - Run fmt, clippy, 192 tests, and auto-clean"
	@echo "  make build       - Compile optimized release binary"
	@echo "  make run         - Run Termodoro in release mode"
	@echo "  make clean       - Reclaim disk space immediately via cargo clean"
	@echo "  make fmt         - Auto-format code using rustfmt"
	@echo "  make clippy      - Run compiler linter with fatal warnings"
