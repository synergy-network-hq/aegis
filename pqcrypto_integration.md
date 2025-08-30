# Using PQClean / pqcrypto in Aegis

This document describes how to integrate PQClean and pqcrypto into Aegis for secure, correct, and portable post-quantum cryptography (PQC) support.

---

## 1. Architecture: PQClean ↔ pqcrypto ↔ Aegis

- **PQClean**: curated, constant-time C implementations of PQC algorithms plus KATs.  
- **pqcrypto (Rust)**: Rust crates that wrap PQClean safely, providing idiomatic Rust APIs.  
- **Aegis**: Your facade. Use `pqcrypto-*` crates as path dependencies and expose a stable, feature-gated API surface.

This ensures correctness (KATs), constant-time C code, and safe Rust API exposure.

---

## 2. Rust Track: Wiring pqcrypto into Aegis

(unchanged content above)

---

## 8. Action Checklist

1. Add feature-gated adapters for all 5 algorithms.  
2. Add KAT vector tests.  
3. Add WASM bindings.  
4. Add zeroization for secret buffers.  
5. Provide Python bindings via pqcrypto/liboqs.  
6. Add CI matrix for native + WASM.  
7. Write docs mapping FIPS names → pqcrypto crates.  
8. Preserve licensing and attributions.

---

# 9. Project Status Analysis & Production Readiness Checklist

This section reviews the **current state of Aegis** and what must be addressed before production readiness, based on the provided status report.

## Executive Summary

Aegis is a PQC library providing Rust, WASM, and Python bindings for Kyber, Dilithium, Falcon, HQC, and Classic McEliece. Architecture is strong, but production readiness is blocked by build system failures, incomplete tests, and missing CI/CD.

## ✅ Current Strengths

- Modular architecture with algorithm separation
- Support for 5+ algorithms
- Comprehensive documentation and examples
- Browser + Node demos already working
- Clean API design
- Hash (SHA3, BLAKE3) and utility functions available

## ❌ Critical Issues

1. Broken build system (`rusty-*` crates missing)
2. SPHINCS+ disabled due to upstream bugs
3. WASM packaging incomplete (no `pkg/` dir)
4. Test suite mostly unimplemented
5. No CI/CD pipeline

## ⚠️ Known Limitations

- SPHINCS+ unstable
- Falcon not yet finalized by NIST
- Classic McEliece only proposed
- Security audit pending

## Production Readiness Checklist (High Priority Items)

### Infrastructure & Build System
- Fix workspace `Cargo.toml`
- Automate WASM builds with `wasm-pack`
- Package `pkg/` for distribution
- Add GitHub Actions CI/CD with builds + tests

### Testing & Quality Assurance
- Implement missing tests (Classic McEliece, browser, integration)
- Add fuzzing, side-channel, and timing tests
- Add performance benchmarks
- Ensure cross-platform coverage (browsers, Node, OSes)

### Security & Cryptography
- Repair/replace SPHINCS+
- Validate against NIST vectors
- Ensure constant-time operations + randomness validation
- Zeroize secrets and validate memory handling

### Documentation & Examples
- Expand API documentation
- Add integration guides for web, Node, Python
- Provide secure messaging examples

### Deployment & Distribution
- Publish npm, PyPI, and crates.io packages
- Implement semantic versioning and release process
- Add automated release notes

### Compliance & Standards
- Document NIST/FIPS compliance per algorithm
- Review dependency licenses + patents
- Align with Web Crypto API and FIPS standards

### Performance & Optimization
- Benchmark and optimize WASM bundle size (<2MB target)
- Profile keygen, signing, encaps/decaps performance
- Optimize for mobile/browser performance

## Immediate 2-Week Action Items

**Week 1**: Fix build system, implement tests, set up basic CI/CD, audit crypto.  
**Week 2**: Complete API docs, add integration examples, set up distribution, begin optimization.

## Success Metrics

- All tests pass on all platforms
- WASM bundle size < 2MB
- Keygen < 100ms
- Zero critical vulnerabilities
- >95% documentation coverage
- CI/CD fully automated

## Risk Assessment

- **High Risk**: SPHINCS+ bugs, security vulnerabilities, broken build system
- **Medium Risk**: Performance, browser compatibility, NIST finalization
- **Low Risk**: Docs, minor optimizations, future features

## Conclusion

Aegis is promising but **not yet production-ready**. With focused work on build system, tests, and CI/CD, production readiness is feasible in 4–6 weeks.

---

# Prompt for AI Agent

"""
You are an expert Rust + Python cryptography engineer. Analyze the current Aegis project (code, file tree, and dependencies). Using the guidelines in `pqcrypto_integration.md` and the project status analysis, finish implementing Aegis so that it:

- Fixes workspace and build system issues
- Cleanly integrates pqcrypto/PQClean for ML-KEM, ML-DSA, FN-DSA, SLH-DSA, HQC, and Classic McEliece
- Provides a feature-gated Rust API via traits (Kem, Signature)
- Passes all NIST Known Answer Tests (KATs)
- Provides WASM bindings for browser/Node integration
- Provides Python bindings that mirror the Rust API
- Zeroizes secret data and avoids side-channel leaks
- Adds CI/CD workflows and automated distribution (npm, PyPI, crates.io)
- Meets performance goals (WASM <2MB, keygen <100ms)
- Preserves licenses and documents compliance status

Deliver a completed repo with code, updated Cargo.toml, test harnesses, WASM packaging, and documentation, ensuring production readiness.
"""
