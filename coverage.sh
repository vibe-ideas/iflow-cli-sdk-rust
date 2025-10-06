#!/bin/bash
# Script to run test coverage analysis

echo "======================================"
echo "  iFlow CLI SDK Rust - Coverage Report"
echo "======================================"
echo ""

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "⚠️  cargo-llvm-cov not found. Installing..."
    cargo install cargo-llvm-cov
fi

echo "📊 Running coverage analysis..."
echo ""

# Run coverage on unit tests (excluding integration tests that require iFlow)
cargo llvm-cov --all-features \
    --ignore-filename-regex '(examples|tests)' \
    --test error_tests \
    --test logger_tests \
    --test types_additional_tests \
    --test message_additional_tests \
    --test iflow_options_tests \
    --test file_access_and_mcp_tests \
    --test process_manager_additional_tests \
    --test message_tests \
    --test query_tests \
    --test query_additional_tests \
    --test query_comprehensive_tests \
    --test process_manager_tests \
    --test websocket_config_tests \
    --test websocket_integration_tests \
    --test client_exception_additional_tests \
    --test client_exception_tests \
    --test plan_message_test \
    --test process_cleanup_tests \
    --test timeout_test

echo ""
echo "======================================"
echo "✅ Coverage analysis complete!"
echo ""
echo "📝 See COVERAGE.md for detailed analysis"
echo ""
echo "To run integration tests (requires iFlow with API config):"
echo "  cargo test --test integration_with_iflow -- --ignored"
echo "======================================"
