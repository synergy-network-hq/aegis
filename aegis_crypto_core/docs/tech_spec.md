# Aegis Quantum Cryptography Core: Technical Specification

## 1. Introduction
Aegis is a next-generation, production-ready quantum cryptography core for the post-quantum era.
It provides a modular, high-assurance, and pluggable foundation for blockchains, web and backend applications, secure messaging, digital identity, and critical infrastructure.
Aegis enables deterministic, auditable, FIPS-compliant cryptography with Rust, WASM, JavaScript/TypeScript, and Python bindings, making it easy to add quantum-safe cryptography to virtually any system—web, embedded, or enterprise.

---

## 2. Core Features & Architecture
- **Modular architecture:** Pluggable algorithm modules for all NIST PQC standards and selected classic algorithms.
- **Rust, WASM, Node.js, and Python bindings:** Native Rust API, WebAssembly/JS/TS APIs, npm installable, and optional Python bindings.
- **Deterministic, auditable signing and verification** for cryptographic reproducibility and compliance.
- **no_std compatible** (embedded, WASM, blockchain, IoT).
- **Constant-time, side-channel resistant execution** (no secret-dependent branching).
- **Secure zeroization** of all secret keys and material (zeroize everywhere).
- **Full support for npm, Node, and browser environments**—plug-n-play PQC for web, blockchain, and app developers.
- **Automated TypeScript type definitions** for WASM/JS APIs.
- **Robust documentation and quickstart/cookbook guides** for every environment.

---

## 3. Supported Standards

Aegis natively supports all finalized NIST PQC standards, as well as those that have been selected for standardization but not yet finalized:

- **FIPS 203 – Kyber (ML-KEM, all parameter sets)**
- **FIPS 204 – Dilithium (ML-DSA, all parameter sets)**
- **FIPS 205 – SPHINCS+ (SLH-DSA, all parameter sets)**
- **FIPS 206 – Falcon (FN-DSA, all parameter sets, NIST-selected)**
- **FIPS 207* – HQC (HQC-KEM, candidate, optional, feature-gated)**
- **FIPS 208* – Classic McEliece (proposed/alternate, optional, feature-gated)**

> **All standards are implemented via the actively maintained [pqcrypto](https://github.com/rustpq/pqcrypto) crate suite.**
>
>**Note:** *Classic McEleice has not been designated as FIPS 208 by the NIST as of yet - the "FIPS 208" label is being tentatively used for this crate, and may be removed in the future.
>
>*Additionally, Classic McEliece has not been officially selected for standardization by NIST. Therefore, it should be implemented with caution and is not recommended for production use.
>
> *Classic McEliece and HQC are optional; they can be enabled or disabled at build time, and are excluded from WASM/npm bundles if they cause compatibility issues.**

---

## 4. Technical Stack & Dependencies

- **Language:** Rust (with no_std, WASM, and native targets)
- **Core cryptographic modules:** pqcrypto-mlkem, pqcrypto-mldsa, pqcrypto-sphincsplus, pqcrypto-falcon, pqcrypto-hqc, pqcrypto-classicmceliece
- **WASM Bindings:** wasm-bindgen, wasm-pack, TypeScript auto-generation
- **NPM/JS Tooling:** npm, package.json, bundler config for Node/browser
- **Dependencies:** `rand_core`, `sha2`, `sha3`, `zeroize`, `criterion`, `pqcrypto` suite, etc.
- **Testing:** Rust test suite, browser and Node.js demo apps, C reference vectors for validation
- **Fuzzing:** [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz)
- **CI/CD:** GitHub Actions for Rust, WASM, and npm builds/tests

---

## 5. Use Cases

- Blockchain transactions (post-quantum wallets, validators, smart contracts)
- Secure messaging, chat, and file transfer (end-to-end PQC)
- Authentication, digital identity, and access control
- Supply chain attestation and device authentication
- Post-quantum VPN/TLS, secure web, and IoT
- Any web, cloud, or on-chain application needing future-proof signatures/encryption

---

## 6. API Overview & Sample Code

**Rust Example:**
```rust
use aegis_crypto_core::{keypair, sign, verify, Algorithm};
let (pk, sk) = keypair(Algorithm::SphincsPlus);
let msg = b"post-quantum security!";
let sig = sign(&sk, msg).unwrap();
assert!(verify(&pk, msg, &sig));
JavaScript (Browser or Node via npm):

```js
import init, { sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify } from "aegis-crypto-core";
await init();
const { pk, sk } = sphincsplus_keygen();
const msg = new TextEncoder().encode("post-quantum security!");
const sig = sphincsplus_sign(sk, msg);
console.log(sphincsplus_verify(pk, msg, sig)); // true

//TypeScript type definitions (.d.ts) are included for all exported APIs.
```

---

## 7. Security & Compliance

- All cryptographic operations are constant-time and side-channel resistant.

- FIPS-deterministic RNGs (optional) for compliance/auditing.

- Explicit error handling—no panics or silent failures.

- Secret key material is securely zeroized after use.

- All algorithms validated against NIST Known Answer Tests (KATs).

- WASM/npm bundles ship with only audited, officially sanctioned pqcrypto code.

---

## 8. Comparison to Other Solutions

- Fully modular, extensible, and pluggable PQC library—Rust, WASM, npm, and Python.

- **npm installable:** usable in web apps, DApps, blockchains, servers.

- no_std and WASM ready for true cross-platform use (from blockchain to browser).

- Deterministic, FIPS-compliant cryptography is the default.

- Extensive docs, quickstarts, and demo apps for every environment.

---

## 9. Glossary of Terms

- **FIPS:** Federal Information Processing Standards

- **no_std:** Rust code not dependent on stdlib (embedded/IOT support)

- **WASM:** WebAssembly, browser/server/embedded targets

- **npm:** Node package manager for JS/TS/Web

- **NIST:** US National Institute of Standards and Technology

- **pqcrypto:** Rust PQC crate suite, official bindings for NIST algorithms

---

## 10. Contact & Integration

- Github Repository: https://github.com/synergy-network-hq/aegis

- Email: security@synergy-network.io

Open to contributions, security audits, and real-world use case feedback!


