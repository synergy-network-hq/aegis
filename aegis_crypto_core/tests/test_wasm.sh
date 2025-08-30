#!/bin/sh
echo "Running WASM PQC tests..."

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    cargo install wasm-pack
fi

# Run WASM tests using wasm-pack
wasm-pack test --node
