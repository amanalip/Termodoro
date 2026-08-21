# ==============================================================================
# Termodoro Project Automation Makefile
# ==============================================================================

.PHONY: all test test-clean check build run clean fmt clippy help test-e2e check-facts check-links

# Default target
all: test-clean

# Run the full Rust test suite and automatically clean the build cache to
# prevent disk bloat
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

# Run Playwright E2E responsive test suite for the project website.
# Installs the pinned Node toolchain first so a fresh clone works out of the
# box (npm ci when a lockfile exists, npm install otherwise).
test-e2e:
	@if [ -f package-lock.json ]; then npm ci || npm install; else npm install; fi
	node scripts/e2e-website-test.mjs

# Run full sanity & fact-checking audit against Rust source code & Markdown links
check-facts:
	node scripts/sanity_and_fact_check.mjs
	node scripts/verify_markdown_links.mjs

# Validate all local Markdown links and anchor slugs across all documentation.
# Set SKIP_EXTERNAL_LINK_CHECK=1 to skip network-dependent external URL checks.
check-links:
	node scripts/verify_markdown_links.mjs

# Display available commands
help:
	@echo "Termodoro Makefile Commands:"
	@echo "  make test        - Run the full Rust test suite and auto-clean target/ cache"
	@echo "  make test-clean  - Run tests and auto-clean (reclaims ~1.8GB disk space)"
	@echo "  make test-e2e    - Install Node deps, then run Playwright E2E suite (desktop + mobile)"
	@echo "  make check-facts - Run full sanity, fact-check, and Markdown link audit"
	@echo "  make check-links - Verify Markdown internal links, anchors, and external URLs"
	@echo "  make check       - Run fmt, clippy, tests, and auto-clean"
	@echo "  make build       - Compile optimized release binary"
	@echo "  make run         - Run Termodoro in release mode"
	@echo "  make clean       - Reclaim disk space immediately via cargo clean"
	@echo "  make fmt         - Auto-format code using rustfmt"
	@echo "  make clippy      - Run compiler linter with fatal warnings"
