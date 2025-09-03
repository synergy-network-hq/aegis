# RustPQC Integration

This document describes the integration of pure Rust implementations of ML-KEM-768 and ML-DSA-65 from the `rustpqc` folder into Aegis Crypto Core.

## Overview

The `rustpqc` folder contains pure Rust implementations of:
* **ML-KEM-768**: Post-quantum key encapsulation mechanism
* **ML-DSA-65**: Post-quantum digital signature algorithm

These implementations are cryptographically correct and fully functional, though not yet KAT-validated. They are integrated into Aegis to provide WASM/Browser/Node.js compatibility for demonstration purposes.

## Features

### Available Features

* `rustpqc-kyber`: Enables the pure Rust ML-KEM-768 implementation
* `rustpqc-dilithium`: Enables the pure Rust ML-DSA-65 implementation

### Feature Usage

To enable the rustpqc implementations, add the features to your `Cargo.toml` :

```toml
[dependencies]
aegis_crypto_core = { path = ".", features = ["rustpqc-kyber", "rustpqc-dilithium"] }
```

Or when building:

```bash
cargo build --features rustpqc-kyber,rustpqc-dilithium
```

## API Reference

### ML-KEM-768 (RustPQC)

#### Rust API

```rust
use aegis_crypto_core::rustpqc_kyber::*;

// Key generation
let (public_key, secret_key) = rustpqc_kyber768_keygen_rust();

// Encapsulation
let (ciphertext, shared_secret) = rustpqc_kyber768_encapsulate_rust(&public_key)?;

// Decapsulation
let decapsulated_secret = rustpqc_kyber768_decapsulate_rust(&secret_key, &ciphertext)?;
```

#### JavaScript/WebAssembly API

```javascript
// Key generation
const keypair = rustpqcKyber768Keygen();
const publicKey = keypair.public_key();
const secretKey = keypair.secret_key();

// Encapsulation
const encapsulated = rustpqcKyber768Encapsulate(publicKey);
const ciphertext = encapsulated.ciphertext();
const sharedSecret = encapsulated.shared_secret();

// Decapsulation
const decapsulatedSecret = rustpqcKyber768Decapsulate(secretKey, ciphertext);
```

### ML-DSA-65 (RustPQC)

#### Rust API

```rust
use aegis_crypto_core::rustpqc_dilithium::*;

// Key generation
let (public_key, secret_key) = rustpqc_dilithium65_keygen_rust();

// Signing
let message = b"Hello, world!";
let signature = rustpqc_dilithium65_sign_rust(&secret_key, message)?;

// Verification
let is_valid = rustpqc_dilithium65_verify_rust(&public_key, &signature, message)?;
```

#### JavaScript/WebAssembly API

```javascript
// Key generation
const keypair = rustpqcDilithium65Keygen();
const publicKey = keypair.public_key();
const secretKey = keypair.secret_key();

// Signing
const message = new TextEncoder().encode("Hello, world!");
const signature = rustpqcDilithium65Sign(secretKey, message);

// Verification
const isValid = rustpqcDilithium65Verify(publicKey, signature, message);
```

## Parameter Sizes

### ML-KEM-768

* Public Key: 1184 bytes
* Secret Key: 2400 bytes
* Ciphertext: 1088 bytes
* Shared Secret: 32 bytes

### ML-DSA-65

* Public Key: 1952 bytes
* Secret Key: 4016 bytes
* Signature: 3293 bytes

## Testing

Run the integration tests:

```bash
cargo test --features rustpqc-kyber,rustpqc-dilithium
```

## Compatibility

### Non-Interference

The rustpqc implementations are designed to not interfere with:
* Existing Aegis functionality
* Other PQC algorithms (Kyber, Dilithium, Falcon, SPHINCS+, HQC, Classic McEliece)
* Non-WASM/browser/Node.js features

### Feature Isolation

* The rustpqc implementations are only available when explicitly enabled via features
* They use separate module names (`rustpqc_kyber`,     `rustpqc_dilithium`) to avoid conflicts
* The original implementations remain available under their original names

## Building for WebAssembly

To build with rustpqc support for WebAssembly:

```bash
wasm-pack build --features rustpqc-kyber,rustpqc-dilithium
```

## Use Cases

### Demonstration and Research

These implementations are particularly useful for:
* Demonstrating Aegis capabilities to investors
* Research grant applications
* Educational purposes
* Prototyping and development

### Browser/Node.js Applications

The pure Rust implementations provide:
* Full WASM compatibility
* No native dependencies
* Cross-platform support
* Consistent performance across environments

## Limitations

### Current Status

* **Not KAT-validated**: These implementations have not yet been validated against NIST Known Answer Tests
* **For demonstration purposes**: Recommended for research and demonstration, not production use
* **Performance**: May have different performance characteristics compared to optimized implementations

### Recommendations

* Use the original Aegis implementations for production applications
* Use rustpqc implementations for demonstration, research, and development
* Consider KAT validation before production deployment

## Future Work

* KAT validation of the rustpqc implementations
* Performance optimization
* Additional parameter sets (ML-KEM-512, ML-KEM-1024, ML-DSA-44, ML-DSA-87)
* Integration with Aegis's unified trait system
