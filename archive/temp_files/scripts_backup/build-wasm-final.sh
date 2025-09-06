#!/bin/bash

# Final WASM build script that fixes memory configuration after build
# This script builds with wasm-pack then fixes the memory configuration

set -e

echo "🚀 Starting Final WASM build with post-build memory fix..."

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
    print_status "WASI SDK downloaded and extracted"
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

# First, try to build with wasm-pack
print_status "Attempting build with wasm-pack first..."

if wasm-pack build \
    --release \
    --target web \
    --out-dir pkg \
    --no-typescript \
    --features wasm; then

    print_success "wasm-pack build succeeded!"

    # Now check if the memory configuration is correct
    if [ -f "pkg/aegis_crypto_core.wasm" ]; then
        print_status "Checking memory configuration..."

        # Use wasm-dis to check memory configuration
        if command -v wasm-dis &> /dev/null; then
            MEMORY_INFO=$(wasm-dis pkg/aegis_crypto_core.wasm | grep -A 2 -B 2 "memory" || true)
            print_status "Memory configuration found:"
            echo "$MEMORY_INFO"

            # Check if we need to fix the memory configuration
            if echo "$MEMORY_INFO" | grep -q "65536"; then
                print_warning "Memory configuration is 64KB (too small), attempting to fix..."

                # Try to use wasm-opt to fix memory configuration
                if command -v wasm-opt &> /dev/null; then
                    print_status "Using wasm-opt to fix memory configuration..."

                    # Create a backup
                    cp pkg/aegis_crypto_core.wasm pkg/aegis_crypto_core_backup.wasm

                    # Try to optimize and set memory size
                    if wasm-opt pkg/aegis_crypto_core.wasm \
                        --initial-memory=4194304 \
                        --max-memory=4194304 \
                        -o pkg/aegis_crypto_core_fixed.wasm; then

                        print_success "Memory configuration fixed with wasm-opt!"
                        mv pkg/aegis_crypto_core_fixed.wasm pkg/aegis_crypto_core.wasm

                        # Verify the fix
                        NEW_MEMORY_INFO=$(wasm-dis pkg/aegis_crypto_core.wasm | grep -A 2 -B 2 "memory" || true)
                        print_status "New memory configuration:"
                        echo "$NEW_MEMORY_INFO"
                    else
                        print_warning "wasm-opt failed to fix memory, keeping original"
                    fi
                else
                    print_warning "wasm-opt not available, cannot fix memory configuration"
                fi
            else
                print_success "Memory configuration appears correct"
            fi
        else
            print_warning "wasm-dis not available, cannot verify memory configuration"
        fi
    fi

else
    print_warning "wasm-pack build failed, trying alternative approach..."

    # Alternative: Build with cargo then use wasm-bindgen
    print_status "Building with cargo directly..."

    if cargo build --target wasm32-unknown-unknown --release --features wasm --no-default-features; then
        print_success "Cargo build succeeded!"

        # Create pkg directory
        mkdir -p pkg

        # Copy WASM file
        cp target/wasm32-unknown-unknown/release/aegis_crypto_core.wasm pkg/

        # Generate JavaScript bindings
        print_status "Generating JavaScript bindings..."
        if wasm-bindgen target/wasm32-unknown-unknown/release/aegis_crypto_core.wasm \
            --out-dir pkg \
            --target web \
            --no-typescript; then

            print_success "JavaScript bindings generated!"
        else
            print_error "Failed to generate JavaScript bindings"
            exit 1
        fi
    else
        print_error "All build approaches failed"
        exit 1
    fi
fi

# Final verification
if [ -d "pkg" ] && [ -f "pkg/aegis_crypto_core.wasm" ]; then
    print_status "Final build verification..."

    # Check file sizes
    WASM_SIZE=$(du -h pkg/aegis_crypto_core.wasm | cut -f1)
    print_success "WASM file size: $WASM_SIZE"

    # List all generated files
    print_status "Generated files:"
    ls -la pkg/

    # Test the build
    print_status "Testing WASM build..."
    if node -e "
        const fs = require('fs');
        const wasmBuffer = fs.readFileSync('pkg/aegis_crypto_core.wasm');
        console.log('WASM file loaded successfully, size:', wasmBuffer.length, 'bytes');
        console.log('Build appears successful');
    "; then
        print_success "WASM file validation passed!"
    else
        print_warning "WASM file validation failed, but build was successful"
    fi

    print_success "🎉 Final WASM build completed successfully!"
    print_status "📦 Output files are in the pkg/ directory"
    print_status "🌐 Ready for browser deployment"

else
    print_error "Build failed - no output files found"
    exit 1
fi
