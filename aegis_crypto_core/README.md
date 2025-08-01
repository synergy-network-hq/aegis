# Aegis PQ Cryptography Suite

Aegis is a modular collection of Rust crates that implement a suite of post‑quantum cryptographic algorithms and expose them via WebAssembly and native bindings. The project’s goal is to make quantum‑safe cryptography as easy to integrate as possible across web, mobile, and backend environments. Each algorithm lives in its own crate, while the `aegis_crypto_core` crate provides a unified API and high‑level helpers for building real applications such as secure messaging, document signing, and wallet integration.

> **Note on SPHINCS+**: The SPHINCS+ (FIPS 205) implementation has been temporarily disabled. The upstream `pqc_sphincsplus` crate exhibits critical verification bugs that prevent it from producing valid signatures. Until a functional implementation is available, SPHINCS+ modules and examples are omitted from the build and documentation.

---

## Repository Layout

| Path                         | Description |
|-----------------------------|-------------|
| `aegis_crypto_core/`        | Main library exposing a unified API and WebAssembly bindings for Kyber, Dilithium, Falcon, HQC, and Classic McEliece. Contains the `bindings/` subfolder with JavaScript (`js_bindings.rs`) and Python (`python_bindings.rs`) modules, `examples/` with Node and browser demos, and `docs/` with use‑case guides. |
| `quantum_fips203/`          | Rust crate implementing the Kyber KEM (ML‑KEM‑768) per NIST FIPS 203. |
| `quantum_fips204/`          | Rust crate implementing the Dilithium signature scheme (ML‑DSA‑65) per NIST FIPS 204. |
| `quantum_fips205/`          | Rust crate implementing the SPHINCS+ signature scheme (FIPS 205). **Disabled** due to upstream bugs. |
| `quantum_fips206/`          | Rust crate implementing the Falcon signature scheme (FN‑DSA) per NIST FIPS 206. |
| `sphincsplus_test_isolated/`| Standalone crate used to isolate and diagnose SPHINCS+ issues without impacting the main build. |
| `todo.md`                   | Production readiness checklist summarising tasks, progress and outstanding work. |
| `update_document.md`        | Historical progress report describing the work completed and issues encountered. |
| `Aegis_Technical_Specification.md` | In‑depth technical specification of the Aegis algorithms and design considerations. |

---

## Features

- **Kyber (ML‑KEM‑768, FIPS 203)** – Key encapsulation mechanism for secure key exchange.
- **Dilithium (ML‑DSA‑65, FIPS 204)** – Lattice‑based digital signature scheme for authenticity and integrity.
- **Falcon (FN‑DSA, FIPS 206)** – Compact lattice‑based signature scheme.
- **HQC (HQC‑KEM, FIPS 207)** – Code-based KEM for quantum-safe key exchange.
- **Classic McEliece (FIPS 208, proposed)** – Code-based KEM for specialized applications.
- **Hashing** – SHA3‑256, SHA3‑512 and BLAKE3 implementations.
- **Utilities** – Hex encoding/decoding and secure random generation helpers.
- **JavaScript bindings** – Re‑exported functions for ergonomic use in Node.js and browser environments.
- **Python bindings** – Native Python module exposing key generation, encryption, signing and verification via PyO3.
- **Secure messaging API** – High‑level JS class demonstrating Kyber+AES-GCM+Dilithium/Falcon.

---

## Building and Installation

### Prerequisites

* **Rust toolchain** – Install via [rustup](https://rustup.rs/).
* **wasm‑pack** – Install with `cargo install wasm-pack`.
* **Node.js** – For running Node examples.
* **Python 3.8+** – For Python bindings (optional).

### Building WebAssembly package

From within the `aegis_crypto_core` directory:

```bash
wasm-pack build --target web    # for browser
wasm-pack build --target nodejs # for Node.js
```

### Building Python extension

```bash
pip install maturin
cd aegis_crypto_core
maturin develop --release
```

---

## Usage

### Basic Example

```javascript
import init, * as wasm from "./pkg/aegis_crypto_core.js";

async function example() {
  await init();

  // Dilithium (ML-DSA) digital signatures
  const dilithiumKeypair = wasm.dilithium_keygen();
  const message = new TextEncoder().encode("Hello, quantum world!");
  const dilithiumSignature = wasm.dilithium_sign(
    dilithiumKeypair.secret_key,
    message
  );
  const dilithiumValid = wasm.dilithium_verify(
    dilithiumKeypair.public_key,
    message,
    dilithiumSignature
  );
  console.log("Dilithium signature valid:", dilithiumValid);

  // Kyber (ML-KEM) key encapsulation
  const kyberKeypair = wasm.kyber_keygen();
  const kyberEncap = wasm.kyber_encapsulate(kyberKeypair.public_key);
  const kyberShared = wasm.kyber_decapsulate(
    kyberKeypair.secret_key,
    kyberEncap.ciphertext
  );
  console.log("Kyber shared secret length:", kyberShared.length);

  // Falcon (FN-DSA) digital signatures
  const falconKeypair = wasm.falcon_keygen();
  const falconSignature = wasm.falcon_sign(falconKeypair.secret_key, message);
  const falconValid = wasm.falcon_verify(
    falconKeypair.public_key,
    message,
    falconSignature
  );
  console.log("Falcon signature valid:", falconValid);

  // HQC (HQC-KEM)
  const hqcKeypair = wasm.hqc_keygen();
  const hqcEncap = wasm.hqc_encapsulate(hqcKeypair.public_key);
  const hqcShared = wasm.hqc_decapsulate(
    hqcKeypair.secret_key,
    hqcEncap.ciphertext
  );
  console.log("HQC shared secret length:", hqcShared.length);

  // Classic McEliece
  const mcelieceKeypair = wasm.classicmceliece_keygen();
  const mcelieceEncap = wasm.classicmceliece_encapsulate(mcelieceKeypair.public_key);
  const mcelieceShared = wasm.classicmceliece_decapsulate(
    mcelieceKeypair.secret_key,
    mcelieceEncap.ciphertext
  );
  console.log("Classic McEliece shared secret length:", mcelieceShared.length);

  // Hash functions
  const data = new TextEncoder().encode("test data");
  const sha3Hash = wasm.sha3_256_hash(data);
  const blake3Hash = wasm.blake3_hash(data);
  console.log("SHA3-256:", wasm.bytes_to_hex(sha3Hash));
  console.log("BLAKE3:", wasm.bytes_to_hex(blake3Hash));
}

example();
```

---

## API Reference

### Dilithium (ML-DSA-65)
- `dilithium_keygen(): DilithiumKeyPair`
- `dilithium_sign(secret_key: Uint8Array, message: Uint8Array): Uint8Array`
- `dilithium_verify(public_key: Uint8Array, message: Uint8Array, signature: Uint8Array): boolean`

### Kyber (ML-KEM-768)
- `kyber_keygen(): KyberKeyPair`
- `kyber_encapsulate(public_key: Uint8Array): {ciphertext: Uint8Array, shared_secret: Uint8Array}`
- `kyber_decapsulate(secret_key: Uint8Array, ciphertext: Uint8Array): Uint8Array`

### Falcon (FN-DSA)
- `falcon_keygen(): FalconKeyPair`
- `falcon_sign(secret_key: Uint8Array, message: Uint8Array): Uint8Array`
- `falcon_verify(public_key: Uint8Array, message: Uint8Array, signature: Uint8Array): boolean`

### HQC (HQC-KEM, FIPS 207)
- `hqc_keygen(): HQCKeyPair`
- `hqc_encapsulate(public_key: Uint8Array): {ciphertext: Uint8Array, shared_secret: Uint8Array}`
- `hqc_decapsulate(secret_key: Uint8Array, ciphertext: Uint8Array): Uint8Array`

### Classic McEliece (FIPS 208, proposed)
- `classicmceliece_keygen(): ClassicMcElieceKeyPair`
- `classicmceliece_encapsulate(public_key: Uint8Array): {ciphertext: Uint8Array, shared_secret: Uint8Array}`
- `classicmceliece_decapsulate(secret_key: Uint8Array, ciphertext: Uint8Array): Uint8Array`

### Hash Functions
- `sha3_256_hash(data: Uint8Array): Uint8Array`
- `sha3_512_hash(data: Uint8Array): Uint8Array`
- `blake3_hash(data: Uint8Array): Uint8Array`

### Utilities
- `hex_to_bytes(hex_string: string): Uint8Array`
- `bytes_to_hex(bytes: Uint8Array): string`

---

## Security Considerations

- This library implements post-quantum cryptographic algorithms that are designed to be secure against both classical and quantum computers.
- All implementations use constant-time operations where possible.
- Private keys are automatically zeroized from memory after use.
- Input validation is performed to reject malformed keys and signatures.

---

## Known Limitations

- SPHINCS+ is currently disabled due to upstream bugs.
- Falcon is included but is not yet a finalized NIST PQC standard (as of this release); Dilithium, Kyber, HQC are finalized.
- Classic McEliece is included (proposed for standardization, not yet finalized).
- SPHINCS+ signature sizes (when enabled) are much larger than Dilithium or Falcon.
- The library is primarily designed for web and JavaScript environments. Other runtimes may require adaptation.
- Experimental software: thorough security review and testing is recommended before production use.

---

## Contributing and Future Work

Development is ongoing. The `todo.md` file tracks outstanding tasks, including restoring SPHINCS+ once upstream bugs are resolved, improving Rust documentation/test coverage, and polishing the documentation. Contributions are welcome via pull requests.

---

## License

This project is distributed under the MIT License. See the `LICENSE` file for details.

---

## Acknowledgments

- Built using the pqcrypto Rust crate suite, which implements Kyber, Dilithium, Falcon, HQC, and Classic McEliece.
- Hash functions provided by sha3 and blake3 crates.
- Thanks to the NIST PQC project and the open-source cryptography community.

---

## Wallet Integration

See [docs/wallet_integration.md](./docs/wallet_integration.md) for guidance on Web3 wallet integration.

---

## Messaging/Chat

See [docs/secure_messaging.md](./docs/secure_messaging.md) for secure messaging and chat implementation.

---

## Document Signing

See [docs/document_signing_identity.md](./docs/document_signing_identity.md) for document signing and identity verification.

---

## Release Notes

**July 29, 2025:**  
- Major upgrade: Integrated HQC and Classic McEliece alongside Kyber, Dilithium, and Falcon.
- Upgraded documentation, examples, and API references for all five algorithms.
- Removed all legacy quantum_fips20x modules and code from the project.

**July 20, 2025:**  
- Added SPHINCS+ (temporarily disabled) with correct FIPS number and API docs.
- Clarified known limitations and expanded acknowledgments.

---
