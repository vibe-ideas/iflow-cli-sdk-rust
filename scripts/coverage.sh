#!/bin/bash
# Script to measure code coverage using cargo-llvm-cov

set -e

echo "Installing cargo-llvm-cov if needed..."
if ! command -v cargo-llvm-cov &> /dev/null; then
    cargo install cargo-llvm-cov
fi

echo "Running tests with coverage..."
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

echo ""
echo "Coverage Summary:"
cargo llvm-cov --all-features --workspace

echo ""
echo "Coverage report saved to lcov.info"
echo ""
echo "To generate an HTML report:"
echo "  cargo llvm-cov --all-features --workspace --html"
echo "  open target/llvm-cov/html/index.html"
