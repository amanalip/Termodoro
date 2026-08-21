#!/usr/bin/env bash
# ==============================================================================
# Termodoro Automated Test & Build Cache Cleanup Utility
# 
# Purpose:
# Executes the full test suite and automatically runs `cargo clean` upon completion
# to reclaim ~1.8GB of disk space while keeping local configuration and user data intact.
# ==============================================================================

set -o pipefail

echo "🧪 Running Termodoro full automated test suite..."
cargo test "$@"
TEST_STATUS=$?

if [ $TEST_STATUS -eq 0 ]; then
    echo "✅ All tests passed successfully!"
    echo "🧹 Automatically cleaning 'target/' directory to reclaim disk space..."
    cargo clean
    echo "✨ Workspace cleaned! Disk space reclaimed while keeping data.json and source files intact."
else
    echo "❌ Test suite failed with exit code $TEST_STATUS."
    echo "🧹 Cleaning 'target/' directory..."
    cargo clean
fi

exit $TEST_STATUS
