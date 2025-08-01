# Aegis, the Quantum-Safe Cryptography Core :: Project Build Status Report

**Date:** 7/22/2025

---

## 1. Project Overview and Status

Aegis is a modular, production-ready post-quantum cryptography (PQC) core designed for secure applications in Rust, WebAssembly (WASM), and JavaScript/TypeScript environments. Aegis aims to deliver plug-and-play, standards-compliant PQC for wallet integration, messaging, document signing, and more.

The core cryptographic primitives in Aegis will be implemented using the [pqcrypto](https://github.com/rustpq/pqcrypto) Rust library suite, which includes official, actively maintained bindings for all major NIST PQC algorithms.

---

## 2. Supported NIST PQC Algorithms (via pqcrypto)

Aegis is being upgraded to use **pqcrypto** for all core algorithms. Each will be supported in Rust and WASM, with tested and documented APIs.

### 2.1 Kyber (ML-KEM-768, FIPS 203)
- **Type:** Key Encapsulation Mechanism (KEM), lattice-based.
- **Status:** Standardized (FIPS 203).
- **Role in Aegis:** Used for quantum-safe key exchange, hybrid encryption, and establishing secure channels.
- **pqcrypto Module:** pqcrypto-mlkem (e.g., pqcrypto_mlkem::kyber768)
- **Variants Supported:** kyber512, kyber768 (default), kyber1024.

### 2.2 Dilithium (ML-DSA-65, FIPS 204)
- **Type:** Digital Signature Algorithm, lattice-based.
- **Status:** Standardized (FIPS 204).
- **Role in Aegis:** Used for signing messages, transactions, and documents with quantum-safe digital signatures.
- **pqcrypto Module:** pqcrypto-mldsa (e.g., pqcrypto_mldsa::dilithium3)
- **Variants Supported:** dilithium2, dilithium3 (default), dilithium5.

### 2.3 SPHINCS+ (SLH-DSA, FIPS 205)
- **Type:** Stateless Hash-Based Digital Signature Algorithm.
- **Status:** Standardized (FIPS 205).
- **Role in Aegis:** Used for stateless, hash-based signatures where maximum long-term security and hash-function trust is desired.
- **pqcrypto Module:** pqcrypto-sphincsplus
- **Variants Supported:** sphincsharaka128frobust, sphincssha256128frobust, sphincsshake256128frobust, etc.

### 2.4 Falcon (FN-DSA, FIPS 206)
- **Type:** Digital Signature Algorithm, lattice-based (NTRU lattice).
- **Status:** Selected by NIST, standard pending (FIPS 206).
- **Role in Aegis:** Used for very short signatures, high-performance cryptography, and post-quantum authentication.
- **pqcrypto Module:** pqcrypto-falcon (e.g., pqcrypto_falcon::falcon512, falcon1024)
- **Variants Supported:** falcon512, falcon1024.

### 2.5 HQC (HQC-KEM, FIPS 207)
- **Type:** Code-Based Key Encapsulation Mechanism.
- **Status:** NIST selected, standard in progress (FIPS 207).
- **Role in Aegis:** Provides a code-based alternative for quantum-safe key encapsulation, with distinct performance and security properties.
- **pqcrypto Module:** pqcrypto-hqc
- **Variants Supported:** All NIST-sanctioned parameter sets (e.g., hqc128, hqc192, hqc256).

### 2.6 Classic McEliece (FIPS 208 - proposed)
- **Type:** Code-Based Key Encapsulation Mechanism.
- **Status:** NIST alternate, not yet standardized but widely studied and implemented.
- **Role in Aegis:** Large-key, high-trust KEM for specialized applications and crypto-agility.
- **pqcrypto Module:** pqcrypto-classicmceliece
- **Variants Supported:** classicmceliece348864, classicmceliece460896, classicmceliece6688128, classicmceliece6960119, classicmceliece8192128, etc.

---

## 3. Implementation Plan

All legacy, in-project implementations of the NIST PQC standards (quantum_fips203, quantum_fips204, quantum_fips205, quantum_fips206) will be fully replaced with pqcrypto crates. All cryptographic operations—key generation, encapsulation, signature creation, and verification—will use pqcrypto as the sole backend.

**Key Implementation Points:**
- Only officially supported pqcrypto APIs and parameter sets will be used.
- The codebase will support switching between all six algorithms for cryptographic operations.
- WASM, TypeScript, and Node.js APIs will be generated/bound for each algorithm, with auto-generated TypeScript definitions and documentation.
- All integration demos, quickstarts, and plug-and-play guides will use pqcrypto.

---

## 4. To-Do List & Project Checklist

**Phase 1: Set up development environment and initial project review**
- [x] Install Rust and wasm-pack.
- [x] Build the project using `wasm-pack build --target web`.
- [x] Review the project structure and existing files.

**Phase 2: Migrate to pqcrypto and Remove Legacy Code**
- [x] Remove quantum_fips203, quantum_fips204, quantum_fips205, quantum_fips206 folders and related code.
- [x] Integrate all six pqcrypto modules: mlkem (Kyber), mldsa (Dilithium), sphincsplus (SPHINCS+), falcon, hqc, classicmceliece.
- [ ] Refactor all Rust code to use pqcrypto APIs for keygen, sign, verify, encaps, and decaps.
- [ ] Update Cargo.toml for new dependencies, removing all local/legacy entries.

**Phase 3: Ensure WASM bindings parity and TypeScript definitions**
- [x] Ensure all algorithms have matching, complete WASM/JS/TS APIs.
- [x] Double-check WASM exports for browser and Node compatibility.
- [x] Auto-generate or hand-write .d.ts files for all WASM/JS APIs.
- [x] Ensure types are developer-friendly and well-documented.

**Phase 4: Develop browser and Node.js demo / example apps**
- [x] Create a /examples folder at the root.
- [x] Include a working browser demo (single-page app, showing keygen, sign, verify, encaps, decaps for all algos).
- [x] Include a Node.js demo (using the WASM package).
- [x] Add “quickstart” and “cookbook” guides for both environments in the docs.

**Phase 5: Enhance integration and plug-and-play documentation**
- [x] Expand README and docs with “Wallet Integration,” “Messaging/Chat,” and “Document Signing” sections.
- [x] Include step-by-step plug-in instructions and real code snippets.
- [x] Provide usage notes for key management, signature/ciphertext formats, and security caveats.

**Phase 6: Improve Rust API reference and test coverage**
- [ ] Ensure all Rust APIs are thoroughly documented (with Rustdoc) and every public function is explained.
- [ ] Achieve high test coverage across all crates, including all pqcrypto-backed operations.
- [ ] Include “how to test” instructions for both Rust and WASM (Node and browser).

**Phase 7: Apply market-ready polish and final documentation**
- [ ] Ensure all documentation is professional, complete, and free of “TODO” or placeholder notes.
- [ ] Check for code duplication, dead files, or unclear patterns.
- [ ] Provide a summary “What’s Inside” and “Who Should Use This” in the root README.
- [ ] Add a “Changelog” or “Release Notes” file describing major improvements.

**Phase 8: Prepare final project package**
- [ ] Ensure all code is clean, well-commented, and follows Rust best practices.
- [ ] Create an “update document” detailing what’s complete, what’s partial, what’s left, and any issues or decisions.

---

## 5. Next Steps and Recommendations

1. **Priority: Replace all legacy quantum_fips20x code with pqcrypto.**
   - Remove legacy implementations from the codebase.
   - Integrate pqcrypto modules for all 6 algorithms, update all APIs and tests.
2. **Refactor and Document:** Update all usage, documentation, and code samples to match pqcrypto’s API.
3. **Comprehensive Testing:** Run and expand unit/integration tests across Rust, WASM, Node, and browser targets.
4. **Finalize Documentation and Release Notes:** Ensure all guides, references, and changelogs are up to date.
5. **Monitor for New NIST Standards:** Track NIST progress on new PQC standards (e.g., FIPS 206–208) and update dependencies as pqcrypto evolves.

---

By completing the migration to pqcrypto and methodically finishing each checklist item, Aegis will deliver a world-class, production-grade quantum-safe cryptography core—ready for real-world adoption and developer integration.

---
