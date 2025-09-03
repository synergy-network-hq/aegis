# Aegis, the Quantum-Safe Cryptography Core

**Project Build Status Report**

**Date:** 8/2/2025

---

## 1. Project Overview and Status

Aegis is a modular, production-ready post-quantum cryptography (PQC) core designed for secure applications in Rust, WebAssembly (WASM), JavaScript/TypeScript (npm), and Python.
It delivers plug-and-play, NIST-standard PQC for wallet integration, messaging, document signing, blockchains, and more—supporting both native Rust and npm/JS/WASM users.

**Latest Status:**
* Migrated all primitives to use the [pqcrypto](https://github.com/rustpq/pqcrypto) crate suite for best-in-class PQC implementations.
* Unified Rust/WASM/JS APIs for all algorithms.
* Browser and Node.js demos, cookbook, and quickstart docs are available.
* **npm packaging and full plug-n-play support in progress.**

---

## 2. Supported NIST PQC Algorithms

| Algorithm          | PQC Standard | pqcrypto Module          | Rust | WASM | npm | Status/Notes              |
|--------------------|--------------|--------------------------|------|------|-----|---------------------------|
| Kyber (ML-KEM)     | FIPS 203     | pqcrypto-mlkem           | ✅    | ✅    | WIP | All parameter sets        |
| Dilithium (ML-DSA) | FIPS 204     | pqcrypto-mldsa           | ✅    | ✅    | WIP | All parameter sets        |
| SPHINCS+ (SLH-DSA) | FIPS 205     | pqcrypto-sphincsplus     | ✅    | ✅    | WIP | All parameter sets        |
| Falcon (FN-DSA)    | FIPS 206     | pqcrypto-falcon          | ✅    | ✅    | WIP | All parameter sets        |
| HQC                | FIPS 207     | pqcrypto-hqc             | ✅    | ✅    | WIP | Optional/feature-gated    |
| Classic McEliece   | FIPS 208?    | pqcrypto-classicmceliece | ✅    | ✅*   | WIP | **DISABLED BY DEFAULT** - Experimental, not NIST standardized |

> **⚠️ IMPORTANT NOTICE: Classic McEliece Disclaimer**
>
> **Classic McEliece has not been officially selected by NIST for standardization** and is considered experimental. This algorithm is **disabled by default** in Aegis and is **not recommended for production use**. Classic McEliece should only be used for research, testing, or educational purposes.
>
> **Note:** npm/WASM support is in progress for all algorithms; *Classic McEliece* and *HQC* are feature-gated and may be excluded from web builds for compatibility.

---

## 3. API & Integration Summary

* **Rust Native:** Fully implemented; all pqcrypto APIs available with unified wrappers.
* **WebAssembly/JS:** WASM bindings exported for all primitives; TypeScript definitions to be completed.
* **npm (Node & Browser):** Packaging work in progress—will ship with WASM, JS glue, and `.d.ts` types.
* **Python:** Optional PyO3 bindings under consideration; not yet part of plug-n-play release.
* **FFI/C:** Not currently exported, but possible via cbindgen if needed for other languages.

---

## 4. Documentation & Examples

* **Rust & WASM APIs:** Documented and covered in `docs/` (quickstart, cookbook, wallet integration, etc.).
* **npm/JS/TS:** Docs being extended for npm install, usage, and integration in modern frontend apps.
* **Demos:** Browser and Node.js demos included in `examples/`; show real keygen, sign/verify, and KEM use for each PQC algorithm.

---

## 5. Testing & CI

* **Rust native tests:** High coverage for all primitives.
* **WASM/JS tests:** In place for major algorithms; npm packaging will add test targets for Node/browser.
* **Continuous Integration:** GitHub Actions pipeline for Rust; npm/WASM/TS CI pipeline planned.

---

## 6. To-Do List & Project Checklist

> **Phase 1: Core Migration and WASM APIs**
> - [x] Migrate all PQC code to use pqcrypto exclusively (done)
> - [x] WASM-bindgen wrappers for all algorithms
> - [x] Browser and Node.js demo scripts

> **Phase 2: npm Packaging and WASM Distribution**
> - [ ] Add a `package.json` in project root with correct metadata.
> - [ ] Add npm build scripts (using `wasm-pack` , `rollup` / `vite` as needed).
> - [ ] Ensure WASM, JS glue, and typings are bundled for npm ( `npm pack` test).
> - [ ] Export all major API functions (keygen, sign, verify, encaps, decaps) to JS with developer-friendly names.
> - [ ] Auto-generate or hand-write TypeScript `.d.ts` for all JS APIs.
> - [ ] Document optional/feature-gated algorithms (Classic McEliece, HQC) in npm docs.
> - [ ] Add browser and Node.js usage examples to README and `/examples` .

> **Phase 3: Docs, Testing, and CI**
> - [ ] Overhaul main README with Rust, npm, WASM, and integration >quickstarts.
> - [ ] Add a full feature table for all algorithms (status: Rust, WASM, npm).
> - [ ] Expand cookbook and wallet/messaging guides for web/npm users.
> - [ ] Add npm/WASM/TS test scripts (run in CI: Node, browser).
> - [ ] Add badges: test, WASM, npm version, docs, etc.
> - [ ] Document known issues or limitations for each environment.
> - [ ] Ensure high coverage for npm APIs via browser/Node test suites.

> **Phase 4: Release & Maintenance**
> - [ ] Test `npm publish` workflow, document any gotchas for devs.
> - [ ] Finalize CHANGELOG/release notes for public npm & Rust releases.
> - [ ] Tag production-ready release (v1.0.0).
> - [ ] Announce and solicit real-world feedback (Rust, npm, blockchain, web).
