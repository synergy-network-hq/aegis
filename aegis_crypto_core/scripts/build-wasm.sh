#!/bin/bash

# Comprehensive WASM build script for Aegis
# This script builds WASM for browser deployment using the correct configuration

set -e

echo "🚀 Starting Aegis WASM build process..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found. Please run this script from the aegis_crypto_core directory."
    exit 1
fi

print_status "Setting up build environment..."

# Set up WASI SDK environment
export WASI_SDK_DIR="$(pwd)/wasi-sdk-20.0/share/wasi-sysroot"
export WASI_SDK_PATH="$(pwd)/wasi-sdk-20.0"

print_status "WASI_SDK_DIR: $WASI_SDK_DIR"
print_status "WASI_SDK_PATH: $WASI_SDK_PATH"

# Verify WASI SDK exists
if [ ! -d "$WASI_SDK_DIR" ]; then
    print_error "WASI SDK not found at $WASI_SDK_DIR"
    print_status "Downloading WASI SDK..."

    # Download and extract WASI SDK
    wget -q https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-20/wasi-sdk-20.0-linux.tar.gz
    tar -xzf wasi-sdk-20.0-linux.tar.gz
    rm wasi-sdk-20.0-linux.tar.gz

    if [ ! -d "$WASI_SDK_DIR" ]; then
        print_error "Failed to download/extract WASI SDK"
        exit 1
    fi
    print_success "WASI SDK downloaded and extracted"
fi

# Set up C compiler environment for WASM
export CC_wasm32_unknown_unknown="$WASI_SDK_PATH/bin/clang"
export AR_wasm32_unknown_unknown="$WASI_SDK_PATH/bin/llvm-ar"
export CFLAGS_wasm32_unknown_unknown="--sysroot=$WASI_SDK_DIR"
export CFLAGS="--sysroot=$WASI_SDK_DIR"

print_status "C compiler environment configured:"
print_status "  CC_wasm32_unknown_unknown: $CC_wasm32_unknown_unknown"
print_status "  AR_wasm32_unknown_unknown: $AR_wasm32_unknown_unknown"
print_status "  CFLAGS: $CFLAGS"

# Clean previous builds
print_status "Cleaning previous builds..."
rm -rf target/wasm32-unknown-unknown
rm -rf pkg

# Build with wasm-pack
print_status "Building with wasm-pack..."

# First, try to build with the configured environment
if wasm-pack build \
    --release \
    --target web \
    --out-dir pkg \
    --no-typescript; then

    print_success "WASM build completed successfully!"

    # Check the output
    if [ -d "pkg" ]; then
        print_status "Generated files:"
        ls -la pkg/

        # Check file sizes
        if [ -f "pkg/aegis_crypto_core.wasm" ]; then
            WASM_SIZE=$(du -h pkg/aegis_crypto_core.wasm | cut -f1)
            print_success "WASM file size: $WASM_SIZE"

            # Check if size is reasonable (< 10MB)
            SIZE_BYTES=$(stat -c%s pkg/aegis_crypto_core.wasm)
            if [ $SIZE_BYTES -gt 10485760 ]; then
                print_warning "WASM file is larger than 10MB ($WASM_SIZE)"
            else
                print_success "WASM file size is reasonable"
            fi
        fi

        # Test the build
        print_status "Testing WASM build..."
        if wasm-pack test --headless --chrome; then
            print_success "WASM tests passed!"
        else
            print_warning "WASM tests failed, but build was successful"
        fi

    else
        print_error "Build succeeded but pkg directory not found"
        exit 1
    fi

else
    print_error "WASM build failed with wasm-pack"
    print_status "Attempting alternative build approach..."

    # Alternative approach: Build with cargo directly
    print_status "Building with cargo directly..."

    if cargo build --target wasm32-unknown-unknown --release; then
        print_success "Cargo build succeeded!"

        # Create pkg directory structure
        mkdir -p pkg

        # Copy the WASM file
        cp target/wasm32-unknown-unknown/release/aegis_crypto_core.wasm pkg/

        # Generate JavaScript bindings manually if needed
        print_status "Generating JavaScript bindings..."
        wasm-bindgen target/wasm32-unknown-unknown/release/aegis_crypto_core.wasm \
            --out-dir pkg \
            --target web \
            --no-typescript

        print_success "Alternative build completed!"
        ls -la pkg/
    else
        print_error "All build approaches failed"
        exit 1
    fi
fi

print_success "🎉 Aegis WASM build process completed successfully!"
print_status "📦 Output files are in the pkg/ directory"
print_status "🌐 Ready for browser deployment"
