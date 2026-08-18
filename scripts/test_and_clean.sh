#!/usr/bin/env bash
# ==============================================================================
# Termodoro Automated Test & Build Cache Cleanup Utility
# 
# Purpose:
# Executes the 154-test suite and automatically runs `cargo clean` upon completion
# to reclaim ~1.8 GB of compiler cache and prevent repository directory bloat.
# ==============================================================================

set -o pipefail

echo "🧪 Running Termodoro 154-test automated suite..."
cargo test "$@"
TEST_STATUS=$?

if [ $TEST_STATUS -eq 0 ]; then
    echo "✅ All 154 tests passed successfully!"
    echo "🧹 Automatically cleaning 'target/' directory to reclaim disk space..."
    cargo clean
    echo "✨ Workspace cleaned! Disk space reclaimed while keeping data.json and source files intact."
else
    echo "❌ Test suite failed with exit code $TEST_STATUS."
    echo "🧹 Cleaning 'target/' directory..."
    cargo clean
fi

exit $TEST_STATUS
