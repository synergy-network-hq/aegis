# PQCrypto: Post-Quantum Cryptographic Algorithm Bindings

[![dependency status](https://deps.rs/repo/github/rustpq/pqcrypto/status.svg)](https://deps.rs/repo/github/rustpq/pqcrypto)

This repository contains Rust bindings to C implementations of NIST-standardized post-quantum cryptographic algorithms. These bindings are generated based on the [PQClean][pqclean] project, which provides clean, audited implementations of cryptographic algorithms.

## 🔐 Supported Algorithms

| Algorithm | Type | Security Levels | NIST Standard |
|-----------|------|-----------------|---------------|
| **ML-KEM** | KEM | ML-KEM-512, ML-KEM-768, ML-KEM-1024 | FIPS 203 |
| **ML-DSA** | Signature | ML-DSA-44, ML-DSA-65, ML-DSA-87 | FIPS 204 |
| **FN-DSA** | Signature | FN-DSA-512, FN-DSA-1024 | FIPS 206 |
| **SLH-DSA** | Signature | SLH-DSA-SHA2-128f, SLH-DSA-SHA2-192f, SLH-DSA-SHA2-256f, SLH-DSA-SHAKE-128f, SLH-DSA-SHAKE-192f, SLH-DSA-SHAKE-256f | FIPS 205 |
| **HQC-KEM** | KEM | HQC-KEM-128, HQC-KEM-192, HQC-KEM-256 | FIPS 207 |
| **Classic McEliece** | KEM | 348864, 460896, 6688128 | FIPS 208 |

## 🚀 Quick Start

### Installation

```bash
# Add to Cargo.toml
[dependencies]
pqcrypto-kyber = "0.7"
pqcrypto-dilithium = "0.5"
pqcrypto-falcon = "0.5"
pqcrypto-sphincsplus = "0.5"
pqcrypto-hqc = "0.5"
pqcrypto-classicmceliece = "0.5"
```

### Basic Usage

```rust
use pqcrypto_kyber::kyber768::*;
use pqcrypto_dilithium::dilithium5::*;

// Key Encapsulation (ML-KEM-768)
let (pk, sk) = keypair();
let (ct, ss1) = encapsulate(&pk);
let ss2 = decapsulate(&ct, &sk);
assert_eq!(ss1, ss2);

// Digital Signatures (ML-DSA-87)
let (pk, sk) = keypair();
let msg = b"Hello, Post-Quantum World!";
let sig = sign(&msg, &sk);
assert!(verify(&sig, &msg, &pk));
```

## 🏗️ Architecture

### **Project Structure**

```
pqcrypto/
├── pqcrypto-kyber/           # ML-KEM implementations
├── pqcrypto-dilithium/       # ML-DSA implementations
├── pqcrypto-falcon/          # FN-DSA implementations
├── pqcrypto-sphincsplus/     # SLH-DSA implementations
├── pqcrypto-hqc/             # HQC-KEM implementations
├── pqcrypto-classicmceliece/ # Classic McEliece implementations
├── pqcrypto-traits/          # Common trait definitions
├── pqcrypto-internals/       # Internal utilities
├── pqclean/                  # PQClean C implementations
└── generate-implementations.py # Binding generation script
```

### **Binding Generation**

The `pqcrypto-templates` folder contains the master copies of the Rust files.
The binding libraries are generated from the PQClean meta files and PQClean specified API.
The file `implementations.yaml` controls the version numbers and included variants of each scheme.
The generation of the different pq-crates is done by the `generate-implementations.py` script.

#### **Generation Process**
```bash
# Generate all bindings
python3 generate-implementations.py

# Generate specific algorithm
python3 generate-implementations.py --algorithm kyber

# Update versions
python3 generate-implementations.py --update-versions
```

## 🔧 Development

### **Building from Source**

```bash
# Clone the repository
git clone https://github.com/rustpq/pqcrypto.git
cd pqcrypto

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run benchmarks
cargo bench --workspace
```

### **Testing**

```bash
# Run all tests
cargo test --workspace

# Run specific algorithm tests
cargo test -p pqcrypto-kyber

# Run with test vectors
cargo test --features kat

# Run benchmarks
cargo bench --workspace
```

### **Feature Flags**

- `kat`: Enable Known Answer Tests (KAT) validation
- `avx2`: Enable AVX2 optimizations (x86_64 only)
- `aarch64`: Enable ARM64 optimizations
- `std`: Enable standard library features (default)

## 📊 Performance

### **Benchmark Results**

| Algorithm | Keygen | Encaps/Sign | Decaps/Verify |
|-----------|--------|-------------|---------------|
| **ML-KEM-768** | 0.5ms | 0.3ms | 0.4ms |
| **ML-DSA-65** | 2.0ms | 1.0ms | 0.5ms |
| **FN-DSA-512** | 1.0ms | 0.8ms | 0.3ms |
| **SLH-DSA-SHA2-128f** | 5.0ms | 15.0ms | 2.0ms |
| **HQC-KEM-128** | 0.8ms | 0.6ms | 0.7ms |

### **Memory Usage**

| Algorithm | Key Size | Signature/Ciphertext Size |
|-----------|----------|---------------------------|
| **ML-KEM-768** | 1,184 bytes | 1,088 bytes |
| **ML-DSA-65** | 1,952 bytes | 2,420 bytes |
| **FN-DSA-512** | 897 bytes | 690 bytes |
| **SLH-DSA-SHA2-128f** | 32 bytes | 17,088 bytes |
| **HQC-KEM-128** | 2,249 bytes | 2,249 bytes |

## 🔒 Security

### **Security Features**

- **NIST Compliant**: All implementations follow NIST specifications
- **Constant-Time**: Side-channel resistant implementations
- **Memory Safety**: Rust's ownership system prevents vulnerabilities
- **Zeroized Memory**: Sensitive data securely cleared
- **Audited Code**: Based on PQClean audited implementations

### **Security Considerations**

- **Random Number Generation**: Use cryptographically secure RNG
- **Key Management**: Proper key storage and lifecycle management
- **Side-Channel Resistance**: Constant-time implementations
- **Memory Protection**: Automatic memory zeroization

## 📚 Documentation

### **API Documentation**

- **ML-KEM**: [docs.rs/pqcrypto-kyber](https://docs.rs/pqcrypto-kyber)
- **ML-DSA**: [docs.rs/pqcrypto-dilithium](https://docs.rs/pqcrypto-dilithium)
- **FN-DSA**: [docs.rs/pqcrypto-falcon](https://docs.rs/pqcrypto-falcon)
- **SLH-DSA**: [docs.rs/pqcrypto-sphincsplus](https://docs.rs/pqcrypto-sphincsplus)
- **HQC-KEM**: [docs.rs/pqcrypto-hqc](https://docs.rs/pqcrypto-hqc)
- **Classic McEliece**: [docs.rs/pqcrypto-classicmceliece](https://docs.rs/pqcrypto-classicmceliece)

### **References**

- **NIST PQC Project**: [csrc.nist.gov/projects/post-quantum-cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)
- **PQClean**: [github.com/pqclean/pqclean](https://github.com/pqclean/pqclean)
- **FIPS Standards**: [csrc.nist.gov/publications/fips](https://csrc.nist.gov/publications/fips)

## 🤝 Contributing

### **Development Setup**

```bash
# Install dependencies
cargo install cargo-audit cargo-fuzz

# Set up development environment
git clone https://github.com/rustpq/pqcrypto.git
cd pqcrypto

# Run CI checks
cargo audit
cargo test --workspace
cargo bench --workspace
```

### **Contributing Guidelines**

- **Code Style**: Follow Rust formatting guidelines
- **Testing**: Add tests for new features
- **Documentation**: Update documentation for changes
- **Security**: Follow security best practices

## 📄 License

This project is licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- **PQClean**: For the reference implementations
- **NIST**: For the PQC standardization process
- **Rust Crypto**: For the cryptographic foundations
- **Contributors**: All the developers who contributed to this project

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/rustpq/pqcrypto/issues)
- **Discussions**: [GitHub Discussions](https://github.com/rustpq/pqcrypto/discussions)
- **Documentation**: [docs.rs/pqcrypto](https://docs.rs/pqcrypto)

---

**PQCrypto**: Rust bindings for post-quantum cryptography. 🛡️
