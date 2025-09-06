#!/bin/bash

# Comprehensive Test Script for Aegis Crypto Core
# Tests both native (non-WASM) and WASM functionality

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print functions
print_header() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"

    print_info "Running: $test_name"

    if eval "$test_command"; then
        print_success "$test_name passed"
        ((TESTS_PASSED++))
    else
        print_error "$test_name failed"
        ((TESTS_FAILED++))
    fi
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Main test execution
main() {
    echo -e "${BLUE}🧪 Aegis Crypto Core - Comprehensive Test Suite${NC}"
    echo "=================================================="

    # Check prerequisites
    print_header "Checking Prerequisites"

    if command_exists cargo; then
        print_success "Cargo (Rust) is available"
    else
        print_error "Cargo (Rust) is not available"
        exit 1
    fi

    if command_exists node; then
        print_success "Node.js is available"
    else
        print_warning "Node.js is not available - WASM tests may be limited"
    fi

    if command_exists wasm-pack; then
        print_success "wasm-pack is available"
    else
        print_warning "wasm-pack is not available - WASM build tests may fail"
    fi

    # Test 1: Core Native Functionality
    print_header "Testing Core Native Functionality"

    run_test "Core hash and utility functions" \
        "cargo run --bin test_core_functionality > /dev/null 2>&1"

    # Test 2: Native Cryptographic Algorithms (if available)
    print_header "Testing Native Cryptographic Algorithms"

    # Test with different feature combinations
    if [ -d "target/debug" ]; then
        print_info "Testing native build with default features"
        run_test "Default features build" \
            "cargo build --release > /dev/null 2>&1"
    fi

    # Test 3: WASM Build
    print_header "Testing WASM Build"

    if [ -f "pkg/aegis_crypto_core_bg.wasm" ]; then
        print_success "WASM build exists"

        # Check WASM file size
        WASM_SIZE=$(stat -c%s "pkg/aegis_crypto_core_bg.wasm" 2>/dev/null || stat -f%z "pkg/aegis_crypto_core_bg.wasm" 2>/dev/null || echo "unknown")
        print_info "WASM file size: $WASM_SIZE bytes"

        # Check if WASM file is valid
        if command_exists wasm-objdump; then
            if wasm-objdump -h "pkg/aegis_crypto_core_bg.wasm" > /dev/null 2>&1; then
                print_success "WASM file is valid (wasm-objdump check passed)"
            else
                print_error "WASM file is invalid (wasm-objdump check failed)"
            fi
        fi

        # Check memory configuration
        if command_exists wasm-dis; then
            MEMORY_INFO=$(wasm-dis "pkg/aegis_crypto_core_bg.wasm" 2>/dev/null | grep -A 2 -B 2 "(memory" || echo "No memory info found")
            if echo "$MEMORY_INFO" | grep -q "64 64"; then
                print_success "WASM memory configuration: 4MB (64 pages)"
            else
                print_warning "WASM memory configuration may not be 4MB"
            fi
        fi

        # Test JavaScript bindings
        if [ -f "pkg/aegis_crypto_core.js" ]; then
            print_success "JavaScript bindings exist"

            # Basic syntax check
            if command_exists node; then
                if node -c "pkg/aegis_crypto_core.js" 2>/dev/null; then
                    print_success "JavaScript bindings syntax is valid"
                else
                    print_error "JavaScript bindings syntax is invalid"
                fi
            fi
        else
            print_error "JavaScript bindings missing"
        fi

        # Test package.json
        if [ -f "pkg/package.json" ]; then
            print_success "package.json exists"

            # Check if it's valid JSON
            if command_exists node; then
                if node -e "JSON.parse(require('fs').readFileSync('pkg/package.json'))" 2>/dev/null; then
                    print_success "package.json is valid JSON"
                else
                    print_error "package.json is invalid JSON"
                fi
            fi
        else
            print_error "package.json missing"
        fi

    else
        print_error "WASM build does not exist"
        print_info "Attempting to build WASM..."

        if command_exists wasm-pack; then
            run_test "WASM build with wasm-pack" \
                "wasm-pack build --release --target web --out-dir pkg --no-typescript --no-default-features --features 'wasm-only' > /dev/null 2>&1"
        else
            print_warning "Cannot build WASM - wasm-pack not available"
        fi
    fi

    # Test 4: Feature-specific builds
    print_header "Testing Feature-specific Builds"

    # Test WASM-only features
    run_test "WASM-only features build" \
        "cargo build --target wasm32-unknown-unknown --release --no-default-features --features 'wasm-only' > /dev/null 2>&1"

    # Test native features (if not on WASM target)
    if [[ "$(rustc --print target-list)" != *"wasm32"* ]] || [[ "$(rustc --print target-list)" == *"x86_64"* ]]; then
        run_test "Native cryptographic features build" \
            "cargo build --release --no-default-features --features 'kyber,dilithium,falcon,sphincsplus,hqc,classicmceliece,rustpqc-kyber,rustpqc-dilithium' > /dev/null 2>&1"
    fi

    # Test 5: Library Tests
    print_header "Testing Library Tests"

    # Test core library tests
    run_test "Core library tests" \
        "cargo test --lib --no-default-features --features 'wasm-only' > /dev/null 2>&1"

    # Test 6: File Integrity
    print_header "Testing File Integrity"

    # Check if all expected files exist
    EXPECTED_FILES=(
        "pkg/aegis_crypto_core_bg.wasm"
        "pkg/aegis_crypto_core.js"
        "pkg/package.json"
        "pkg/README.md"
    )

    for file in "${EXPECTED_FILES[@]}"; do
        if [ -f "$file" ]; then
            print_success "$file exists"
        else
            print_error "$file missing"
        fi
    done

    # Test 7: WASM Functionality Test
    print_header "Testing WASM Functionality"

    if [ -f "test_wasm_functionality.html" ]; then
        print_success "WASM test HTML file exists"

        # Check if we can serve the files
        if command_exists python3 || command_exists python; then
            print_info "Python HTTP server available for WASM testing"
        else
            print_warning "No HTTP server available for WASM testing"
        fi
    else
        print_warning "WASM test HTML file not found"
    fi

    # Summary
    print_header "Test Summary"
    echo "Tests Passed: $TESTS_PASSED"
    echo "Tests Failed: $TESTS_FAILED"
    echo "Total Tests: $((TESTS_PASSED + TESTS_FAILED))"

    if [ $TESTS_FAILED -eq 0 ]; then
        print_success "All tests passed! 🎉"
        exit 0
    else
        print_error "Some tests failed. Please review the output above."
        exit 1
    fi
}

# Run main function
main "$@"
