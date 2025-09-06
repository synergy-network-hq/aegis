#!/bin/bash

# Install WASM development tools
# This script installs the necessary tools for WASM development and analysis

set -e

echo "🔧 Installing WASM development tools..."

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

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo not found. Please install Rust first: https://rustup.rs/"
    exit 1
fi

# Install wasm-pack
print_status "Installing wasm-pack..."
if ! command -v wasm-pack &> /dev/null; then
    cargo install wasm-pack
    print_success "wasm-pack installed"
else
    print_status "wasm-pack already installed"
fi

# Install wasm-bindgen-cli
print_status "Installing wasm-bindgen-cli..."
if ! command -v wasm-bindgen &> /dev/null; then
    cargo install wasm-bindgen-cli
    print_success "wasm-bindgen-cli installed"
else
    print_status "wasm-bindgen-cli already installed"
fi

# Install wasm-objdump for WASM analysis
print_status "Installing wasm-objdump..."
if ! command -v wasm-objdump &> /dev/null; then
    cargo install wasm-objdump
    print_success "wasm-objdump installed"
else
    print_status "wasm-objdump already installed"
fi

# Install wasm-opt for WASM optimization (optional)
print_status "Installing wasm-opt..."
if ! command -v wasm-opt &> /dev/null; then
    # Try to install from binaryen
    if command -v apt-get &> /dev/null; then
        sudo apt-get update && sudo apt-get install -y binaryen
        print_success "wasm-opt installed via apt"
    elif command -v brew &> /dev/null; then
        brew install binaryen
        print_success "wasm-opt installed via brew"
    else
        print_warning "Could not install wasm-opt automatically. Please install binaryen manually."
    fi
else
    print_status "wasm-opt already installed"
fi

# Verify installations
print_status "Verifying installations..."
echo "wasm-pack version: $(wasm-pack --version)"
echo "wasm-bindgen version: $(wasm-bindgen --version)"
echo "wasm-objdump version: $(wasm-objdump --version)"

if command -v wasm-opt &> /dev/null; then
    echo "wasm-opt version: $(wasm-opt --version)"
else
    print_warning "wasm-opt not available"
fi

print_success "🎉 WASM development tools installation completed!"
print_status "You can now use the build scripts to create WASM builds with proper memory configuration."
