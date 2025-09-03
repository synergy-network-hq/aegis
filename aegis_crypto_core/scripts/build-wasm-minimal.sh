#!/bin/bash

# Minimal WASM Build Solution - Only WASM-compatible features
# This script builds with only the features that work in WASM

set -e

echo "🚀 Starting Minimal WASM Build Solution..."

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

# The key solution: Use wasm-pack but immediately fix memory configuration
print_status "Building with wasm-pack and then fixing memory configuration..."

# First, try to build with wasm-pack
if wasm-pack build \
    --release \
    --target web \
    --out-dir pkg \
    --no-typescript \
    --no-default-features \
    --features "wasm-only"; then

    print_success "wasm-pack build succeeded!"

    # Now immediately fix the memory configuration using wasm-opt
    if command -v wasm-opt &> /dev/null; then
        print_status "Fixing memory configuration with wasm-opt..."

        # wasm-pack generates aegis_crypto_core_bg.wasm, not aegis_crypto_core.wasm
        if [ -f "pkg/aegis_crypto_core_bg.wasm" ]; then
            # Create a backup
            cp pkg/aegis_crypto_core_bg.wasm pkg/aegis_crypto_core_bg_original.wasm

            # Use wasm-opt to set the correct memory configuration
            if wasm-opt pkg/aegis_crypto_core_bg.wasm \
                --initial-memory=4194304 \
                --max-memory=4194304 \
                -o pkg/aegis_crypto_core_bg_fixed.wasm; then

                mv pkg/aegis_crypto_core_bg_fixed.wasm pkg/aegis_crypto_core_bg.wasm
                print_success "Memory configuration fixed with wasm-opt!"

                # Verify the memory configuration
                print_status "Verifying memory configuration..."
                if command -v wasm-dis &> /dev/null; then
                    if wasm-dis pkg/aegis_crypto_core_bg.wasm | grep -q "memory"; then
                        print_success "Memory section found in WASM file"
                        wasm-dis pkg/aegis_crypto_core_bg.wasm | grep -A 3 -B 3 "memory"
                    else
                        print_warning "No memory section found - this might indicate an issue"
                    fi
                else
                    print_warning "wasm-dis not available - skipping memory verification"
                fi
            else
                print_warning "Failed to fix memory with wasm-opt, keeping original"
            fi
        else
            print_error "Expected WASM file aegis_crypto_core_bg.wasm not found"
            exit 1
        fi
    else
        print_warning "wasm-opt not available - memory configuration may be incorrect"
    fi

    # Test the build
    print_status "Testing the build..."
    if node -e "
        const fs = require('fs');
        const wasmPath = './pkg/aegis_crypto_core_bg.wasm';
        if (fs.existsSync(wasmPath)) {
            const stats = fs.statSync(wasmPath);
            console.log('WASM file size:', stats.size, 'bytes');
            console.log('Build successful!');
        } else {
            console.log('WASM file not found');
            process.exit(1);
        }
    "; then
        print_success "Build test passed!"
        print_success "WASM build completed successfully!"
        exit 0
    else
        print_error "Build test failed"
        exit 1
    fi

else
    print_warning "wasm-pack build failed, trying alternative approach..."

    # Fallback: Build with cargo directly
    print_status "Building with cargo directly..."

    if cargo build --target wasm32-unknown-unknown --release --no-default-features --features "wasm-only"; then
        print_success "Cargo build succeeded!"

        # Create pkg directory
        mkdir -p pkg

        # Copy WASM file
        cp target/wasm32-unknown-unknown/release/aegis_crypto_core.wasm pkg/

        # Now manually fix the memory configuration using wasm-opt
        if command -v wasm-opt &> /dev/null; then
            print_status "Setting memory configuration with wasm-opt..."

            # Create a backup
            cp pkg/aegis_crypto_core.wasm pkg/aegis_crypto_core_original.wasm

            # Use wasm-opt to set the correct memory configuration
            if wasm-opt pkg/aegis_crypto_core.wasm \
                --initial-memory=4194304 \
                --max-memory=4194304 \
                -o pkg/aegis_crypto_core_fixed.wasm; then

                mv pkg/aegis_crypto_core_fixed.wasm pkg/aegis_crypto_core.wasm
                print_success "Memory configuration set with wasm-opt!"
            else
                print_warning "Failed to set memory with wasm-opt, keeping original"
            fi
        else
            print_warning "wasm-opt not available - memory configuration may be incorrect"
        fi

        # Generate JavaScript bindings with wasm-bindgen
        print_status "Generating JavaScript bindings with wasm-bindgen..."
        if wasm-bindgen target/wasm32-unknown-unknown/release/aegis_crypto_core.wasm \
            --out-dir pkg \
            --target web \
            --no-typescript; then
            print_success "JavaScript bindings generated successfully!"

            # Re-apply memory fix after wasm-bindgen (it might have changed it)
            if command -v wasm-opt &> /dev/null; then
                print_status "Re-applying memory configuration after wasm-bindgen..."
                if wasm-opt pkg/aegis_crypto_core.wasm \
                    --initial-memory=4194304 \
                    --max-memory=4194304 \
                    -o pkg/aegis_crypto_core_final.wasm; then
                    mv pkg/aegis_crypto_core_final.wasm pkg/aegis_crypto_core.wasm
                    print_success "Final memory configuration applied!"
                fi
            fi

            # Verify the final build
            print_status "Verifying final build..."
            if command -v wasm-dis &> /dev/null; then
                if wasm-dis pkg/aegis_crypto_core.wasm | grep -q "memory"; then
                    print_success "Memory section found in final WASM file"
                    wasm-dis pkg/aegis_crypto_core.wasm | grep -A 3 -B 3 "memory"
                else
                    print_warning "No memory section found in final WASM file"
                fi
            fi

            # Test the build
            print_status "Testing the build..."
            if node -e "
                const fs = require('fs');
                const wasmPath = './pkg/aegis_crypto_core.wasm';
                if (fs.existsSync(wasmPath)) {
                    const stats = fs.statSync(wasmPath);
                    console.log('WASM file size:', stats.size, 'bytes');
                    console.log('Build successful!');
                } else {
                    console.log('WASM file not found');
                    process.exit(1);
                }
            "; then
                print_success "Build test passed!"
                print_success "WASM build completed successfully!"
                exit 0
            else
                print_error "Build test failed"
                exit 1
            fi
        else
            print_error "Failed to generate JavaScript bindings"
            exit 1
            fi
    else
        print_error "Cargo build failed"
        exit 1
    fi
fi
