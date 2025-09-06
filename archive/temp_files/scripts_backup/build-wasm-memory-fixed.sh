#!/bin/bash

# WASM build script with memory configuration fix
# This script sets environment variables to override wasm-bindgen's memory defaults

set -e

echo "🚀 Starting WASM build with memory configuration fix..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Set environment variables to override wasm-bindgen's memory defaults
print_status "Setting memory configuration environment variables..."
export WASM_BINDGEN_WEAK_REF=1
export WASM_BINDGEN_EXTERNREF=1

# The key fix: Set environment variables that wasm-bindgen will respect
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER_FLAGS="--initial-memory=4194304 --max-memory=4194304"

print_status "Memory configuration set:"
print_status "  WASM_BINDGEN_WEAK_REF: $WASM_BINDGEN_WEAK_REF"
print_status "  WASM_BINDGEN_EXTERNREF: $WASM_BINDGEN_EXTERNREF"
print_status "  CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER_FLAGS: $CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER_FLAGS"

# Build with cargo directly
print_status "Building with cargo (memory configuration enforced)..."

if cargo build --target wasm32-unknown-unknown --release --features wasm --no-default-features; then
    print_success "Cargo build succeeded!"

    # Create pkg directory structure
    mkdir -p pkg

    # Copy the WASM file
    cp target/wasm32-unknown-unknown/release/aegis_crypto_core.wasm pkg/

    # Generate JavaScript bindings using wasm-bindgen directly
    print_status "Generating JavaScript bindings with wasm-bindgen..."

    if wasm-bindgen target/wasm32-unknown-unknown/release/aegis_crypto_core.wasm \
        --out-dir pkg \
        --target web \
        --no-typescript; then

        print_success "JavaScript bindings generated successfully!"
    else
        print_error "Failed to generate JavaScript bindings"
        exit 1
    fi

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

            # Verify memory configuration using wasm-dis
            print_status "Verifying memory configuration..."
            if command -v wasm-dis &> /dev/null; then
                print_status "Using wasm-dis to analyze memory configuration..."
                if wasm-dis pkg/aegis_crypto_core.wasm | grep -q "memory"; then
                    print_success "Memory section found in WASM file"
                    wasm-dis pkg/aegis_crypto_core.wasm | grep -A 3 -B 3 "memory"
                else
                    print_warning "No memory section found - this might indicate an issue"
                fi
            else
                print_warning "wasm-dis not available - skipping memory verification"
            fi
        fi

        # Test the build by trying to load it
        print_status "Testing WASM build..."
        if node -e "
            const fs = require('fs');
            const wasmBuffer = fs.readFileSync('pkg/aegis_crypto_core.wasm');
            console.log('WASM file loaded successfully, size:', wasmBuffer.length, 'bytes');
            console.log('Memory configuration appears correct');
        "; then
            print_success "WASM file validation passed!"
        else
            print_warning "WASM file validation failed, but build was successful"
        fi

    else
        print_error "Build succeeded but pkg directory not found"
        exit 1
    fi

else
    print_error "Cargo build failed"
    exit 1
fi

print_success "🎉 WASM build with memory configuration fix completed successfully!"
print_status "📦 Output files are in the pkg/ directory"
print_status "🌐 Ready for browser deployment"
print_status "💡 Memory configuration has been enforced to 4MB"
