#!/bin/bash
# -----------------------------------------------------------------------------
# WASM Testing Script
# -----------------------------------------------------------------------------
# Runs WASM checks and reactivity tests for the yew-datatable crate.
# -----------------------------------------------------------------------------

set -e

echo "🌐 Running WASM checks and tests (yew-datatable)..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check if wasm-pack is installed.
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed!"
    echo "Install it with: cargo install wasm-pack"
    exit 1
fi

# Ensure the WASM target is installed.
echo "🎯 Ensuring wasm32-unknown-unknown target is installed..."
rustup target add wasm32-unknown-unknown

# Run clippy on the yew-datatable package for wasm.
echo ""
echo "📋 Running Clippy for yew-datatable (wasm32 target)..."
cargo clippy --package yew-datatable --all-targets --target wasm32-unknown-unknown -- -D warnings

# Run the wasm integration tests (reactivity coverage).
echo ""
echo "🧪 Running reactivity tests in headless Chrome..."
(
    cd yew-datatable
    wasm-pack test --headless --chrome
)

echo ""
echo "✅ WASM checks and tests passed!"
