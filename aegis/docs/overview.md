# Aegis Crypto Core Project Overview

This project provides a WebAssembly (WASM) enabled cryptographic library focusing on Post-Quantum Cryptography (PQC) algorithms. It aims to offer a secure and efficient solution for integrating advanced cryptographic primitives into web and JavaScript environments.

## Implemented Algorithms

The library currently includes implementations of the following NIST PQC finalists and candidates:

*   **Kyber (ML-KEM-768):** A key-encapsulation mechanism (KEM) based on module-lattice cryptography, standardized by NIST for key establishment.
*   **Dilithium (ML-DSA):** A digital signature algorithm (DSA) based on module-lattice cryptography, standardized by NIST for digital signatures.
*   **SPHINCS+ (SLH-DSA):** A stateless hash-based digital signature algorithm, standardized by NIST. It offers strong post-quantum security guarantees without requiring state management.
*   **Falcon (FN-DSA):** A digital signature algorithm based on Falcon-512, a lattice-based scheme. While not yet a finalized NIST standard, it is included as a promising candidate.

## Key Features

*   **WASM Bindings:** All core cryptographic functions are exposed as WASM bindings, allowing seamless integration with JavaScript and TypeScript applications.
*   **Quantum-Resistant:** Implements algorithms designed to withstand attacks from quantum computers.
*   **Security Focus:** Emphasizes constant-time operations to mitigate timing attacks and secure wiping of sensitive key material from memory.
*   **Comprehensive Testing:** Includes a suite of unit tests and WASM/JS-accessible tests to ensure the correctness and reliability of the implementations.
*   **Detailed Documentation:** Provides in-code documentation, API references, and usage examples for all modules and functions.

## Project Structure

*   `src/`: Contains the Rust source code for the cryptographic implementations.
    *   `sphincsplus.rs`: SPHINCS+ implementation.
    *   `kyber.rs`: Kyber implementation.
    *   `dilithium.rs`: Dilithium implementation.
    *   `falcon.rs`: Falcon implementation.
    *   `hash.rs`: Hashing utility functions.
    *   `utils.rs`: General utility functions.
    *   `lib.rs`: Main library file, exposing modules and WASM bindings.
*   `tests/`: Rust-side unit tests for the cryptographic algorithms.
*   `pkg/`: Output directory for WASM build artifacts (JavaScript and TypeScript bindings, WASM binary).
*   `quantum_fips203/`, `quantum_fips204/`, `quantum_fips205/`, `quantum_fips206/`: External crates providing the underlying cryptographic primitives.

## Usage

To use this library in a JavaScript/TypeScript project, you can install it via npm (after publishing) or directly include the generated WASM and JS files from the `pkg` directory. The WASM bindings provide functions for key generation, signing, verification, encapsulation, and decapsulation, depending on the algorithm.

## Contributing

Contributions are welcome! Please refer to the `CONTRIBUTING.md` (if available) for guidelines on how to contribute to the project.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.


