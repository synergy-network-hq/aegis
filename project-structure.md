# AEGIS Project Structure

## Overview
This document provides a complete file tree of the AEGIS post-quantum cryptography library with descriptions of each file and folder's purpose.

## Project Root Structure

```
aegis/
├── 📁 .cargo/                          # Cargo configuration directory
│   └── config.toml                     # Global Cargo configuration for Rust builds
├── 📁 .git/                            # Git repository metadata
├── 📁 .github/                         # GitHub-specific configuration
│   └── workflows/
│       └── ci.yml                      # GitHub Actions CI/CD pipeline configuration
├── 📁 aegis_crypto_core/               # Main Rust crate - core cryptography library
│   ├── 📁 benches/                     # Performance benchmark tests
│   │   ├── classicmceliece_benchmarks.rs  # Classic McEliece performance benchmarks
│   │   ├── performance_benchmarks.rs      # General performance benchmarks
│   │   └── simple_bench.rs               # Simple benchmark utilities
│   ├── 📁 config/                      # Build configuration files
│   │   ├── package.json                # npm package configuration for WASM builds
│   │   ├── pyproject.toml              # Python package configuration
│   │   └── wasm-pack.toml              # wasm-pack build configuration
│   ├── 📁 demos/                       # Interactive demonstration applications
│   │   ├── 📁 advanced-messaging/      # Advanced messaging demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 blockchain-wallet/       # Blockchain wallet demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 digital-identity/        # Digital identity demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 document-signing/        # Document signing demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 financial-security/      # Financial security demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 government-communications/ # Government communications demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 healthcare-data-protection/ # Healthcare data protection demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 interactive-learning/    # Interactive learning demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 iot-security/            # IoT security demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 ml-model-protection/     # ML model protection demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 portal/                  # Main demo portal
│   │   │   ├── index.html              # Portal web page
│   │   │   ├── style.css               # Portal styling
│   │   │   └── portal.js               # Portal JavaScript
│   │   ├── 📁 post-quantum-blockchain/ # Post-quantum blockchain demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 post-quantum-cloud-storage/ # Post-quantum cloud storage demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 post-quantum-database/   # Post-quantum database demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 quantum-key-distribution/ # Quantum key distribution demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 quantum-resistant-iot/   # Quantum-resistant IoT demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 quantum-resistant-vpn/   # Quantum-resistant VPN demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 real-time-crypto/        # Real-time cryptography demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 secure-messaging/        # Secure messaging demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 secure-voting-system/    # Secure voting system demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 smart-contract-security/ # Smart contract security demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 supply-chain-security/   # Supply chain security demo
│   │   │   ├── index.html              # Demo web page
│   │   │   └── demo.js                 # Demo JavaScript code
│   │   ├── 📁 demo-scripts/            # Demo utility scripts
│   │   │   ├── README.md               # Demo scripts documentation
│   │   │   └── run_all_demos.sh        # Script to run all demos
│   │   ├── blockchain_wallet_example.md # Blockchain wallet example documentation
│   │   ├── FINAL_REORGANIZATION_SUMMARY.md # Demo reorganization summary
│   │   ├── README.md                   # Demo documentation
│   │   ├── REORGANIZATION_SUMMARY.md   # Demo reorganization notes
│   │   ├── sample_programs_guide.md    # Sample programs guide
│   │   └── secure_messaging_example.md # Secure messaging example documentation
│   ├── 📁 fuzz/                        # Fuzzing test configuration
│   │   ├── Cargo.toml                  # Fuzzing dependencies
│   │   └── fuzz_targets/               # Fuzzing test targets
│   │       ├── kyber_fuzz.rs           # ML-KEM fuzzing tests
│   │       └── dilithium_fuzz.rs       # ML-DSA fuzzing tests
│   ├── 📁 pkg/                         # Generated WASM package files
│   │   ├── aegis_crypto_core_bg.wasm   # Compiled WASM binary
│   │   ├── aegis_crypto_core_bg.wasm.d.ts # TypeScript definitions for WASM
│   │   ├── aegis_crypto_core.d.ts      # TypeScript definitions
│   │   ├── aegis_crypto_core.js        # JavaScript bindings
│   │   ├── LICENSE                     # License file for WASM package
│   │   ├── package.json                # npm package configuration
│   │   └── README.md                   # Package documentation
│   ├── 📁 src/                         # Rust source code
│   │   ├── 📁 bin/                     # Binary executables
│   │   │   ├── blockchain_wallet.rs    # Blockchain wallet binary
│   │   │   ├── digital_identity.rs     # Digital identity binary
│   │   │   ├── document_signing.rs     # Document signing binary
│   │   │   ├── financial_security.rs   # Financial security binary
│   │   │   ├── iot_security.rs         # IoT security binary
│   │   │   ├── secure_messaging.rs     # Secure messaging binary
│   │   │   └── web_api_server.rs       # Web API server binary
│   │   ├── 📁 classicmceliece/         # Classic McEliece implementation
│   │   │   ├── mod.rs                  # Module definition
│   │   │   ├── keygen.rs               # Key generation
│   │   │   ├── encap.rs                # Encapsulation
│   │   │   └── decap.rs                # Decapsulation
│   │   ├── 📁 dilithium/               # ML-DSA (Dilithium) implementation
│   │   │   ├── mod.rs                  # Module definition
│   │   │   ├── keygen.rs               # Key generation
│   │   │   ├── sign.rs                 # Signing
│   │   │   └── verify.rs               # Verification
│   │   ├── 📁 falcon/                  # FN-DSA (Falcon) implementation
│   │   │   ├── mod.rs                  # Module definition
│   │   │   ├── keygen.rs               # Key generation
│   │   │   ├── sign.rs                 # Signing
│   │   │   └── verify.rs               # Verification
│   │   ├── 📁 hqc/                     # HQC-KEM implementation
│   │   │   ├── mod.rs                  # Module definition
│   │   │   ├── keygen.rs               # Key generation
│   │   │   ├── encap.rs                # Encapsulation
│   │   │   └── decap.rs                # Decapsulation
│   │   ├── 📁 kyber/                   # ML-KEM (Kyber) implementation
│   │   │   ├── mod.rs                  # Module definition
│   │   │   ├── keygen.rs               # Key generation
│   │   │   ├── encap.rs                # Encapsulation
│   │   │   ├── decap.rs                # Decapsulation
│   │   │   └── utils.rs                # Utility functions
│   │   ├── 📁 rustpqc_dilithium/       # RustPQC ML-DSA bindings
│   │   │   └── mod.rs                  # RustPQC ML-DSA module
│   │   ├── 📁 rustpqc_kyber/           # RustPQC ML-KEM bindings
│   │   │   └── mod.rs                  # RustPQC ML-KEM module
│   │   ├── 📁 sphincsplus/             # SLH-DSA (SPHINCS+) implementation
│   │   │   ├── mod.rs                  # Module definition
│   │   │   ├── keygen.rs               # Key generation
│   │   │   ├── sign.rs                 # Signing
│   │   │   └── verify.rs               # Verification
│   │   ├── blockchain.rs               # Blockchain integration utilities
│   │   ├── hash.rs                     # Hash function implementations
│   │   ├── js_bindings.rs              # JavaScript/WASM bindings
│   │   ├── lib.rs                      # Main library entry point
│   │   ├── nist_wasm_mldsa.rs          # NIST WASM ML-DSA bindings
│   │   ├── nist_wasm_mlkem.rs          # NIST WASM ML-KEM bindings
│   │   ├── performance.rs              # Performance utilities
│   │   ├── traits.rs                   # Common trait definitions
│   │   ├── utils.rs                    # General utility functions
│   │   └── wasm_loader.rs              # WASM module loader
│   ├── 📁 tests/                       # Test suite
│   │   ├── 📁 integration/             # Integration tests
│   │   ├── 📁 unit/                    # Unit tests
│   │   ├── test_*.rs                   # Individual test files
│   │   ├── test_*.sh                   # Shell test scripts
│   │   └── README.md                   # Test documentation
│   ├── 📁 tools/                       # Build tools and utilities
│   │   └── wasi-sdk-20.0/              # WASI SDK for WASM compilation
│   ├── Cargo.lock                      # Rust dependency lock file
│   ├── Cargo.toml                      # Rust package configuration
│   └── README.md                       # Core library documentation
├── 📁 archive/                         # Archived files (not needed for runtime)
│   ├── 📁 documentation/               # Archived documentation
│   ├── 📁 NIST/                        # Archived NIST source files
│   ├── 📁 pqclean/                     # Archived pqclean source files
│   ├── 📁 rustpqc_backup/              # Archived RustPQC backup
│   ├── 📁 temp_files/                  # Archived temporary files
│   ├── 📁 target/                      # Archived build artifacts
│   └── *.sh, *.js, *.md                # Archived build scripts and docs
├── 📁 docs/                            # Project documentation
│   ├── AEGIS_USER_MANUAL.md            # Comprehensive user manual
│   ├── cookbook.md                     # Code examples and recipes
│   ├── document_signing_identity.md    # Document signing guide
│   ├── general_purpose_pq_crypto.md    # General PQC usage guide
│   ├── hardware_embedded_integration.md # Hardware integration guide
│   ├── on_chain_smart_contract_use.md  # Smart contract integration
│   ├── overview.md                     # Project overview
│   ├── quickstart.md                   # Quick start guide
│   ├── RUSTPQC_INTEGRATION.md          # RustPQC integration guide
│   ├── secure_messaging.md             # Secure messaging guide
│   ├── tech_spec.md                    # Technical specifications
│   ├── update_document.md              # Update documentation
│   └── wallet_integration.md           # Wallet integration guide
├── 📁 examples/                        # Usage examples
│   ├── basic-usage.js                  # Basic usage examples
│   └── performance-benchmark.js        # Performance benchmarking examples
├── 📁 pqcrypto/                        # Rust PQC implementations
│   ├── 📁 pqcrypto/                    # Main pqcrypto crate
│   ├── 📁 pqcrypto-classicmceliece/    # Classic McEliece Rust implementation
│   ├── 📁 pqcrypto-falcon/             # Falcon Rust implementation
│   ├── 📁 pqcrypto-hqc/                # HQC-KEM Rust implementation
│   ├── 📁 pqcrypto-internals/          # Internal PQC utilities
│   ├── 📁 pqcrypto-mldsa/              # ML-DSA Rust implementation
│   ├── 📁 pqcrypto-mlkem/              # ML-KEM Rust implementation
│   ├── 📁 pqcrypto-sphincsplus/        # SPHINCS+ Rust implementation
│   ├── 📁 pqcrypto-template/           # PQC implementation template
│   ├── 📁 pqcrypto-traits/             # PQC trait definitions
│   ├── 📁 pqclean/                     # C implementations (for compilation)
│   ├── Cargo.toml.bak                  # Backup Cargo configuration
│   ├── CHANGELOG.md                    # Change log
│   ├── generate-implementations.py     # Implementation generator
│   ├── implementations.yaml            # Implementation configuration
│   ├── Pipfile                         # Python dependencies
│   ├── Pipfile.lock                    # Python dependency lock
│   ├── README.md                       # pqcrypto documentation
│   ├── release.sh                      # Release script
│   ├── SECURITY.md                     # Security documentation
│   └── WASM.md                         # WASM build documentation
├── 📁 pqkat/                           # Known Answer Test vectors
│   ├── 📁 NIST-fn-dsa/                 # FN-DSA KAT vectors
│   ├── 📁 NIST-hqc-kem/                # HQC-KEM KAT vectors
│   ├── 📁 NIST-ml-dsa/                 # ML-DSA KAT vectors
│   ├── 📁 NIST-ml-kem/                 # ML-KEM KAT vectors
│   └── 📁 NIST-slh-dsa/                # SLH-DSA KAT vectors
├── 📁 pqwasm/                          # Pre-compiled WASM implementations
│   ├── 📁 addimp/                      # Additional implementations
│   ├── 📁 avx2/                        # AVX2 optimized implementations
│   ├── 📁 clean/                       # Clean implementations
│   ├── 📁 m4/                          # M4 optimized implementations
│   ├── 📁 optimp/                      # Optimized implementations
│   └── 📁 refimp/                      # Reference implementations
├── 📁 target/                          # Rust build artifacts (regeneratable)
├── Cargo.lock                          # Root Rust dependency lock file
├── Cargo.toml                          # Root Rust workspace configuration
├── DEMO_QUICK_START.md                 # Demo quick start guide
├── LICENSE                             # Project license (MIT OR Apache-2.0)
├── M2_MAC_MINI_BENCHMARKS.md          # Performance benchmarks on M2 Mac Mini
├── package.json                        # npm package configuration
├── README.md                           # Main project documentation
├── to-do.md                            # Project status and task tracking
├── tree.md                             # Legacy project tree (to be archived)
└── WASM_USAGE_GUIDE.md                 # Comprehensive WASM usage guide
```

## File Descriptions

### Core Files
- **`Cargo.toml`** - Root Rust workspace configuration defining all crates and dependencies
- **`package.json`** - npm package configuration for publishing to npm registry
- **`LICENSE`** - Dual license (MIT OR Apache-2.0) for maximum compatibility
- **`README.md`** - Main project documentation with installation and usage instructions

### Documentation
- **`WASM_USAGE_GUIDE.md`** - Comprehensive guide for using AEGIS in WASM environments
- **`M2_MAC_MINI_BENCHMARKS.md`** - Detailed performance benchmarks and test results
- **`to-do.md`** - Project status tracking and completed tasks
- **`DEMO_QUICK_START.md`** - Quick start guide for running demonstrations

### Source Code
- **`aegis_crypto_core/`** - Main Rust crate containing all PQC algorithm implementations
- **`pqcrypto/`** - Rust PQC implementations and C bindings for native compilation
- **`pqwasm/`** - Pre-compiled WASM files for all PQC algorithms and variants
- **`pqkat/`** - Known Answer Test vectors for cryptographic validation

### Examples and Demos
- **`examples/`** - Basic usage examples and performance benchmarks
- **`aegis_crypto_core/demos/`** - Interactive web-based demonstrations

### Configuration
- **`.github/workflows/ci.yml`** - GitHub Actions CI/CD pipeline
- **`.cargo/config.toml`** - Global Cargo configuration
- **`aegis_crypto_core/config/`** - Build configuration files

### Archived Files
- **`archive/`** - Contains source files, build scripts, and temporary files not needed for runtime

## Algorithm Support

AEGIS supports all 5 NIST-standardized post-quantum cryptographic algorithms:

1. **ML-KEM / FIPS203** (formerly CRYSTALS-Kyber) - Key Encapsulation Mechanism
2. **ML-DSA / FIPS204** (formerly CRYSTALS-Dilithium) - Digital Signature Algorithm
3. **SLH-DSA / FIPS205** (formerly SPHINCS+) - Stateless Hash-based Signatures
4. **FN-DSA / FIPS206*** (formerly Falcon) - Fast-Fourier Lattice-based Signatures
5. **HQC-KEM / FIPS207*** - Hamming Quasi-Cyclic Key Encapsulation Mechanism

*FIPS designations not officially assigned by NIST yet

## Build Targets

- **Native Rust** - Full performance with all optimizations
- **WASM** - WebAssembly for browser and Node.js environments
- **Python** - Python bindings for data science and scripting
- **JavaScript/TypeScript** - Full type support for web development

## Project Status

✅ **Production Ready** - All components tested, documented, and ready for npm publishing

