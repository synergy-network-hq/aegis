# Aegis Crypto Core: Complete Command Reference

**Last Updated:** 2024-12-19
**Purpose:** Comprehensive command reference for setting up and using Aegis Crypto Core

---

## 📋 **Table of Contents**

1. [Prerequisites & System Setup](#prerequisites--system-setup)
2. [Repository Setup](#repository-setup)
3. [WASM/Node.js/Browser Features](#wasmnodejsbrowser-features)
4. [Non-WASM Features](#non-wasm-features)
5. [Testing & Validation](#testing--validation)
6. [Performance Benchmarking](#performance-benchmarking)
7. [Deployment & Distribution](#deployment--distribution)
8. [Troubleshooting](#troubleshooting)

---

## 🔧 **Prerequisites & System Setup**

### System Requirements

**Operating Systems:**
* Ubuntu 20.04+ / Debian 11+ / CentOS 8+
* macOS 10.15+ (Catalina)
* Windows 10+ with WSL2 or native Rust support

**Hardware Requirements:**
* CPU: x86_64 or ARM64 (AArch64)
* RAM: 8GB minimum, 16GB recommended
* Storage: 10GB free space minimum

### Essential Tools Installation

**Ubuntu/Debian:**

```bash
# Update package list
sudo apt update

# Install essential build tools
sudo apt install -y build-essential curl git pkg-config libssl-dev

# Install Node.js 18+ and npm
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# Verify installations
node --version  # Should be v18.x.x or higher
npm --version   # Should be 8.x.x or higher
```

**macOS:**

```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install essential tools
brew install curl git pkg-config openssl

# Install Node.js 18+ and npm
brew install node@18
brew link node@18

# Verify installations
node --version  # Should be v18.x.x or higher
npm --version   # Should be 8.x.x or higher
```

**Windows (WSL2):**

```bash
# Follow Ubuntu/Debian instructions above
# Or use Windows native tools below
```

**Windows (Native):**

```powershell
# Install Chocolatey if not already installed
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install essential tools
choco install git nodejs.install -y

# Verify installations
node --version  # Should be v18.x.x or higher
npm --version   # Should be 8.x.x or higher
```

### Rust Installation

**All Platforms:**

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts and select option 1 (default installation)
# Restart your terminal or run:
source ~/.bashrc  # Linux/macOS
# OR
source ~/.zshrc   # macOS with zsh

# Verify Rust installation
rustc --version   # Should be 1.70+ or higher
cargo --version   # Should be 1.70+ or higher
rustup --version  # Should be 1.25+ or higher

# Add required targets
rustup target add wasm32-unknown-unknown
rustup target add x86_64-pc-windows-msvc  # Windows only
rustup target add x86_64-apple-darwin     # macOS only
rustup target add aarch64-unknown-linux-gnu  # ARM64 Linux

# Verify targets
rustup target list --installed
```

### WebAssembly Tools Installation

**All Platforms:**

```bash
# Install wasm-pack for WebAssembly builds
cargo install wasm-pack

# Verify wasm-pack installation
wasm-pack --version  # Should be 0.12+ or higher

# Install wasm-bindgen-cli for advanced WASM operations
cargo install wasm-bindgen-cli

# Verify wasm-bindgen-cli installation
wasm-bindgen --version
```

### Additional Development Tools

**Ubuntu/Debian:**

```bash
# Install additional development tools
sudo apt install -y clang llvm-dev libclang-dev

# Install Python 3.8+ for future Python bindings
sudo apt install -y python3 python3-pip python3-dev

# Install additional utilities
sudo apt install -y jq tree htop
```

**macOS:**

```bash
# Install additional development tools
brew install clang llvm

# Install Python 3.8+ for future Python bindings
brew install python@3.9

# Install additional utilities
brew install jq tree htop
```

---

## 📥 **Repository Setup**

### Clone and Initialize Repository

```bash
# Clone the Aegis repository
git clone https://github.com/synergy-network-hq/aegis.git

# Navigate to the project directory
cd aegis

# Initialize and update submodules
git submodule update --init --recursive

# Navigate to the core library
cd aegis_crypto_core

# Verify the project structure
ls -la
tree -L 2  # If tree is installed
```

### Verify Dependencies

```bash
# Check Rust toolchain
rustup show

# Check available targets
rustup target list --installed

# Check Cargo configuration
cargo --version
cargo config --list

# Check Node.js and npm
node --version
npm --version

# Check wasm-pack
wasm-pack --version

# Check git submodules
git submodule status
```

---

## 🌐 **WASM/Node.js/Browser Features**

### Basic WASM Build

**Standard WASM Build (pqcrypto implementations):**

```bash
# Navigate to the core library
cd aegis_crypto_core

# Build for web target (browser)
wasm-pack build --target web --features kyber,dilithium,falcon,sphincsplus

# Build for bundler target (Node.js, webpack, etc.)
wasm-pack build --target bundler --features kyber,dilithium,falcon,sphincsplus

# Build for Node.js target
wasm-pack build --target nodejs --features kyber,dilithium,falcon,sphincsplus

# Build for all targets
wasm-pack build --target all --features kyber,dilithium,falcon,sphincsplus
```

**Pure Rust WASM Build (rustpqc implementations - KAT validated):**

> **Note:** The rustpqc ML-KEM-768 and ML-DSA-65 implementations are now fully KAT validated and production-ready. These provide pure Rust alternatives to the pqcrypto implementations with identical security guarantees.

```bash
# Build with rustpqc features for web
wasm-pack build --target web --features rustpqc-kyber,rustpqc-dilithium

# Build with rustpqc features for bundler
wasm-pack build --target bundler --features rustpqc-kyber,rustpqc-dilithium

# Build with rustpqc features for Node.js
wasm-pack build --target nodejs --features rustpqc-kyber,rustpqc-dilithium

# Build with all features (both pqcrypto and rustpqc)
wasm-pack build --target web --features kyber,dilithium,falcon,sphincsplus,rustpqc-kyber,rustpqc-dilithium

# Build with rustpqc only (fully KAT validated)
wasm-pack build --target web --release --features rustpqc-kyber,rustpqc-dilithium
```

### Advanced WASM Build Options

**Optimized Production Build:**

```bash
# Build with optimizations for production
wasm-pack build --target web --release --features kyber,dilithium,falcon,sphincsplus

# Build with specific algorithm selection
wasm-pack build --target web --release --features kyber,dilithium

# Build with experimental algorithms (not recommended for production)
wasm-pack build --target web --release --features kyber,dilithium,falcon,sphincsplus,hqc,classicmceliece
```

**Debug Build for Development:**

```bash
# Build with debug information
wasm-pack build --target web --dev --features kyber,dilithium,falcon,sphincsplus

# Build with profiling information
wasm-pack build --target web --profiling --features kyber,dilithium,falcon,sphincsplus
```

### npm Package Creation

**Create npm Package:**

```bash
# Build and create npm package
wasm-pack pack --features kyber,dilithium,falcon,sphincsplus

# Build and publish to npm (requires npm login)
wasm-pack publish --features kyber,dilithium,falcon,sphincsplus
```

**Install npm Package Locally:**

```bash
# Install the built package locally
npm install ./pkg

# Or install from npm registry (when published)
npm install aegis-crypto-core
```

### Browser Integration

**HTML Integration:**

```html
<!DOCTYPE html>
<html>

<head>
    <title>Aegis Crypto Core Demo</title>
</head>

<body>
    <h1>Aegis Crypto Core - Post-Quantum Cryptography</h1>
    <div id="output"></div>

    <script type="module">
        import init, {
            kyber768_keygen,
            kyber768_encapsulate
        } from './pkg/aegis_crypto_core.js';

        async function runDemo() {
            await init();

            // Generate keypair
            const keypair = kyber768_keygen();
            document.getElementById('output').innerHTML = `
                <h2>Generated Keypair</h2>
                <p>Public Key Length: ${keypair.public_key().length} bytes</p>
                <p>Secret Key Length: ${keypair.secret_key().length} bytes</p>
            `;
        }

        runDemo();
    </script>
</body>

</html>
```

**Serve for Testing:**

```bash
# Install a simple HTTP server
npm install -g http-server

# Serve the built files
cd pkg
http-server -p 8080

# Or use Python's built-in server
python3 -m http.server 8080

# Or use Node.js serve package
npx serve -p 8080
```

### Node.js Integration

**Basic Node.js Usage:**

```javascript
// test-node.js
const {
    kyber768_keygen,
    kyber768_encapsulate
} = require('./pkg/aegis_crypto_core.js');

async function testNode() {
    try {
        // Generate keypair
        const keypair = kyber768_keygen();
        console.log('Generated keypair:', {
            publicKeyLength: keypair.public_key().length,
            secretKeyLength: keypair.secret_key().length
        });

        // Test encapsulation
        const encapsulated = kyber768_encapsulate(keypair.public_key());
        console.log('Encapsulation successful:', {
            ciphertextLength: encapsulated.ciphertext().length,
            sharedSecretLength: encapsulated.shared_secret().length
        });

    } catch (error) {
        console.error('Error:', error);
    }
}

testNode();
```

**Run Node.js Test:**

```bash
# Run the Node.js test
node test-node.js

# Or run with specific Node.js flags
node --experimental-wasm-threads test-node.js
```

---

## 🖥️ **Non-WASM Features**

### Rust Library Build

**Basic Build:**

```bash
# Navigate to the core library
cd aegis_crypto_core

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Build with specific features
cargo build --features kyber,dilithium,falcon,sphincsplus

# Build with all features
cargo build --all-features
```

**Feature-Specific Builds:**

```bash
# Build with only Kyber
cargo build --features kyber

# Build with Kyber and Dilithium
cargo build --features kyber,dilithium

# Build with production algorithms only
cargo build --features kyber,dilithium,falcon,sphincsplus

# Build with experimental algorithms
cargo build --features kyber,dilithium,falcon,sphincsplus,hqc,classicmceliece

# Build with rustpqc implementations (fully KAT validated)
cargo build --features rustpqc-kyber,rustpqc-dilithium

# Build with both production and rustpqc
cargo build --features kyber,dilithium,falcon,sphincsplus,rustpqc-kyber,rustpqc-dilithium
```

**Cross-Platform Builds:**

```bash
# Build for Linux x86_64
cargo build --target x86_64-unknown-linux-gnu --release

# Build for Windows x86_64
cargo build --target x86_64-pc-windows-msvc --release

# Build for macOS x86_64
cargo build --target x86_64-apple-darwin --release

# Build for ARM64 Linux
cargo build --target aarch64-unknown-linux-gnu --release

# Build for specific target with features
cargo build --target x86_64-unknown-linux-gnu --release --features kyber,dilithium
```

### Cargo Workspace Operations

**Check Dependencies:**

```bash
# Check if project compiles without building
cargo check

# Check with specific features
cargo check --features kyber,dilithium

# Check with all features
cargo check --all-features

# Check for specific target
cargo check --target wasm32-unknown-unknown --features kyber,dilithium
```

**Clean and Rebuild:**

```bash
# Clean all build artifacts
cargo clean

# Clean and rebuild
cargo clean && cargo build

# Clean and rebuild with specific features
cargo clean && cargo build --features kyber,dilithium --release
```

---

## 🧪 **Testing & Validation**

### Rust Testing

**Basic Testing:**

```bash
# Run all tests
cargo test

# Run tests with specific features
cargo test --features kyber,dilithium

# Run tests with all features
cargo test --all-features

# Run tests with rustpqc features (fully KAT validated)
cargo test --features rustpqc-kyber,rustpqc-dilithium

# Run tests with both production and rustpqc
cargo test --features kyber,dilithium,falcon,sphincsplus,rustpqc-kyber,rustpqc-dilithium
```

**Specific Test Categories:**

```bash
# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run specific test
cargo test test_kyber768_keygen

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel (default)
cargo test --jobs 4

# Run tests sequentially
cargo test --jobs 1
```

**WASM Testing:**

```bash
# Test WASM build
wasm-pack test --headless --firefox

# Test with specific browser
wasm-pack test --headless --chrome

# Test with Node.js
wasm-pack test --node

# Test with all targets
wasm-pack test --headless --firefox --chrome --safari
```

### KAT Validation Testing

**Run KAT Tests:**

```bash
# Run all KAT validation tests
cargo test --features kyber,dilithium,falcon,sphincsplus --test kat_validation

# Run specific algorithm KAT tests
cargo test --features kyber --test kat_validation test_kyber

# Run KAT tests with output
cargo test --features kyber,dilithium,falcon,sphincsplus --test kat_validation -- --nocapture
```

**Integration Testing:**

```bash
# Run integration tests
cargo test --features kyber,dilithium,falcon,sphincsplus --test integration

# Run rustpqc integration tests (fully KAT validated)
cargo test --features rustpqc-kyber,rustpqc-dilithium --test test_rustpqc_integration

# Run blockchain integration tests
cargo test --features kyber,dilithium --test blockchain_integration
```

### Performance Testing

**Benchmarking:**

```bash
# Run all benchmarks
cargo bench

# Run benchmarks with specific features
cargo bench --features kyber,dilithium

# Run specific benchmark
cargo bench kyber768_keygen

# Run benchmarks with output
cargo bench -- --nocapture
```

**Performance Profiling:**

```bash
# Profile with perf (Linux)
cargo build --release --features kyber,dilithium
perf record --call-graph=dwarf target/release/aegis_crypto_core
perf report

# Profile with valgrind (Linux)
cargo build --release --features kyber,dilithium
valgrind --tool=callgrind target/release/aegis_crypto_core
```

---

## 📊 **Performance Benchmarking**

### WASM Performance Testing

**Bundle Size Analysis:**

```bash
# Build optimized WASM
wasm-pack build --target web --release --features kyber,dilithium

# Analyze bundle size
ls -lh pkg/*.wasm
ls -lh pkg/*.js

# Use wasm-opt for further optimization
wasm-opt -O4 -o pkg/aegis_crypto_core_opt.wasm pkg/aegis_crypto_core.wasm

# Compare sizes
ls -lh pkg/*.wasm
```

**Browser Performance Testing:**

```html
<!DOCTYPE html>
<html>

<head>
    <title>Aegis Performance Test</title>
</head>

<body>
    <h1>Performance Benchmarking</h1>
    <div id="results"></div>

    <script type="module">
        import init, {
            kyber768_keygen,
            kyber768_encapsulate
        } from './pkg/aegis_crypto_core.js';

        async function benchmark() {
            await init();

            const results = document.getElementById('results');

            // Benchmark key generation
            const start = performance.now();
            for (let i = 0; i < 100; i++) {
                kyber768_keygen();
            }
            const keygenTime = performance.now() - start;

            // Benchmark encapsulation
            const keypair = kyber768_keygen();
            const start2 = performance.now();
            for (let i = 0; i < 100; i++) {
                kyber768_encapsulate(keypair.public_key());
            }
            const encapsTime = performance.now() - start2;

            results.innerHTML = `
                <h2>Benchmark Results (100 iterations)</h2>
                <p>Key Generation: ${(keygenTime / 100).toFixed(2)}ms per operation</p>
                <p>Encapsulation: ${(encapsTime / 100).toFixed(2)}ms per operation</p>
            `;
        }

        benchmark();
    </script>
</body>

</html>
```

### Native Performance Testing

**Rust Performance Tests:**

```rust
// benches/performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aegis_crypto_core::kyber::*;

fn benchmark_kyber768_keygen(c: &mut Criterion) {
    c.bench_function("kyber768_keygen", |b| {
        b.iter(|| {
            black_box(kyber768_keygen());
        });
    });
}

fn benchmark_kyber768_encapsulate(c: &mut Criterion) {
    let keypair = kyber768_keygen();
    c.bench_function("kyber768_encapsulate", |b| {
        b.iter(|| {
            black_box(kyber768_encapsulate(&keypair.public_key()));
        });
    });
}

criterion_group!(benches, benchmark_kyber768_keygen, benchmark_kyber768_encapsulate);
criterion_main!(benches);
```

**Run Performance Tests:**

```bash
# Run performance benchmarks
cargo bench

# Run specific benchmark
cargo bench kyber768_keygen

# Run benchmarks with detailed output
cargo bench -- --verbose
```

---

## 🚀 **Deployment & Distribution**

### npm Package Distribution

**Prepare for npm:**

```bash
# Update package.json version
npm version patch  # or minor, major

# Build optimized package
wasm-pack build --target bundler --release --features kyber,dilithium

# Pack the npm package
wasm-pack pack

# Test the package locally
npm install ./pkg
```

**Publish to npm:**

```bash
# Login to npm (if not already logged in)
npm login

# Publish the package
wasm-pack publish

# Or publish manually
cd pkg
npm publish
```

### Cargo Crate Distribution

**Prepare for crates.io:**

```bash
# Check if package is ready for publishing
cargo package

# Check package contents
cargo package --list

# Verify package builds
cargo verify-project

# Check for common issues
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

**Publish to crates.io:**

```bash
# Login to crates.io (if not already logged in)
cargo login

# Publish the crate
cargo publish

# Or publish with specific features
cargo publish --features kyber,dilithium
```

### Docker Distribution

**Create Docker Image:**

```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /usr/src/aegis
COPY . .

RUN cargo build --release --features kyber,dilithium

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/aegis/target/release/aegis_crypto_core /usr/local/bin/

CMD ["aegis_crypto_core"]
```

**Build and Run Docker:**

```bash
# Build Docker image
docker build -t aegis-crypto-core .

# Run Docker container
docker run -it aegis-crypto-core

# Push to Docker Hub
docker tag aegis-crypto-core yourusername/aegis-crypto-core
docker push yourusername/aegis-crypto-core
```

---

## 🔍 **Troubleshooting**

### Common Build Issues

**WASM Build Failures:**

```bash
# Check Rust toolchain
rustup show
rustup target list --installed

# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
wasm-pack build --target web --features kyber,dilithium

# Check for missing dependencies
cargo check --target wasm32-unknown-unknown --features kyber,dilithium
```

**Native Build Failures:**

```bash
# Check system dependencies
pkg-config --version
openssl version

# Install missing dependencies (Ubuntu/Debian)
sudo apt install -y pkg-config libssl-dev

# Install missing dependencies (macOS)
brew install pkg-config openssl

# Clean and rebuild
cargo clean
cargo build --features kyber,dilithium
```

**Feature Flag Issues:**

```bash
# Check available features
cargo check --help | grep features

# List all features
cargo check --features | head -20

# Test specific feature combinations
cargo check --features kyber
cargo check --features kyber,dilithium
cargo check --features rustpqc-kyber,rustpqc-dilithium
```

### Testing Issues

**Test Failures:**

```bash
# Run tests with verbose output
cargo test -- --nocapture --verbose

# Run specific failing test
cargo test test_name -- --nocapture

# Check test dependencies
cargo test --no-default-features

# Run tests with specific features
cargo test --features kyber,dilithium -- --nocapture
```

**WASM Test Issues:**

```bash
# Check wasm-pack version
wasm-pack --version

# Update wasm-pack
cargo install wasm-pack --force

# Test with specific browser
wasm-pack test --headless --firefox

# Test with Node.js
wasm-pack test --node
```

### Performance Issues

**Slow Builds:**

```bash
# Use parallel compilation
cargo build --jobs $(nproc)

# Use incremental compilation
CARGO_INCREMENTAL=1 cargo build

# Use specific target
cargo build --target x86_64-unknown-linux-gnu --release
```

**Large WASM Bundles:**

```bash
# Build with optimizations
wasm-pack build --target web --release --features kyber,dilithium

# Use wasm-opt for further optimization
wasm-opt -O4 -o pkg/optimized.wasm pkg/aegis_crypto_core.wasm

# Analyze bundle contents
wasm-objdump -x pkg/aegis_crypto_core.wasm
```

### Dependency Issues

**Update Dependencies:**

```bash
# Update all dependencies
cargo update

# Update specific dependency
cargo update -p dependency-name

# Check outdated dependencies
cargo outdated

# Audit dependencies for security issues
cargo audit
```

**Clean Dependencies:**

```bash
# Clean all build artifacts and dependencies
cargo clean

# Remove Cargo.lock and rebuild
rm Cargo.lock
cargo build

# Clean specific target
cargo clean --target wasm32-unknown-unknown
```

---

## 📚 **Additional Resources**

### Documentation

**Project Documentation:**
* [Technical Specification](docs/tech_spec.md)
* [Progress Documentation](progress-doc.md)
* [API Reference](docs/api_reference.md)

**External Resources:**
* [Rust Book](https://doc.rust-lang.org/book/)
* [WebAssembly Documentation](https://webassembly.org/)
* [wasm-pack Documentation](https://rustwasm.github.io/docs/wasm-pack/)
* [NIST PQC Standards](https://www.nist.gov/programs-projects/post-quantum-cryptography)

### Support Channels

**Community Support:**
* GitHub Issues: [Report bugs and request features](https://github.com/synergy-network-hq/aegis/issues)
* GitHub Discussions: [Community discussions](https://github.com/synergy-network-hq/aegis/discussions)
* Email: security@synergy-network.io

**Commercial Support:**
* Enterprise consulting and custom implementations
* Security audits and compliance assessments
* Training and workshops

---

*This command reference is maintained by the Aegis development team and updated regularly. For the latest information, always refer to the project repository and documentation.*
