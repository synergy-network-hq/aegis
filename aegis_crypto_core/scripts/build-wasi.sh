#!/bin/bash

# Custom WASI build script for Aegis
# This script sets up the WASI SDK environment and builds with wasm-pack

set -e

echo "🔧 Setting up WASI SDK environment..."

# Set WASI SDK directory
export WASI_SDK_DIR="$(pwd)/wasi-sdk-20.0/share/wasi-sysroot"
export WASI_SDK_PATH="$(pwd)/wasi-sdk-20.0"

echo "📁 WASI_SDK_DIR: $WASI_SDK_DIR"
echo "📁 WASI_SDK_PATH: $WASI_SDK_PATH"

# Verify WASI SDK exists
if [ ! -d "$WASI_SDK_DIR" ]; then
    echo "❌ WASI SDK not found at $WASI_SDK_DIR"
    echo "Please ensure wasi-sdk-20.0 is extracted in the project root"
    exit 1
fi

# Set up C compiler environment for WASI
export CC_wasm32_unknown_unknown="$WASI_SDK_PATH/bin/clang"
export AR_wasm32_unknown_unknown="$WASI_SDK_PATH/bin/llvm-ar"
export CFLAGS_wasm32_unknown_unknown="--sysroot=$WASI_SDK_DIR"
export CFLAGS="--sysroot=$WASI_SDK_DIR"

echo "🔨 Building with wasm-pack..."

# Build with wasm-pack using the configured environment
wasm-pack build \
    --release \
    --target web \
    --out-dir pkg \
    --no-typescript

echo "✅ WASI build completed successfully!"
echo "📦 Output files in pkg/ directory:"
ls -la pkg/
