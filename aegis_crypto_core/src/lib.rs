// # Aegis Crypto Core
//
// A unified post-quantum cryptography library providing WebAssembly, Rust, and Python bindings
// for NIST-standardized algorithms including Kyber (ML-KEM), Dilithium (ML-DSA), Falcon,
// SPHINCS+, HQC, and Classic McEliece.
//
// ## Features
//
// This crate supports multiple post-quantum cryptographic algorithms through feature flags:
//
// - `kyber` - ML-KEM (Key Encapsulation Mechanism)
// - `dilithium` - ML-DSA (Digital Signature Algorithm)
// - `falcon` - Falcon digital signatures
// - `sphincsplus` - SPHINCS+ hash-based signatures
// - `hqc` - HQC (Hamming Quasi-Cyclic) KEM
// - `classicmceliece` - Classic McEliece KEM
//
// ## Example
//
// ```rust,ignore
// use aegis_crypto_core::*;
//
// // Generate Kyber keypair
// let keypair = kyber_keygen();
// let public_key = keypair.public_key();
// let secret_key = keypair.secret_key();
//
// // Encapsulate a shared secret
// let encapsulated = kyber_encapsulate(&public_key).unwrap();
// let ciphertext = encapsulated.ciphertext();
// let shared_secret = encapsulated.shared_secret();
//
// // Decapsulate to recover the shared secret
// let recovered_secret = kyber_decapsulate(&secret_key, &ciphertext).unwrap();
// assert_eq!(shared_secret, recovered_secret);
// ```

// Inner attributes must come first
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

// Bring in `alloc` crate for no_std environments (e.g. wasm) for Vec, String, and format!
#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String};

#[cfg(feature = "std")]
use std::{vec::Vec, string::String};

#[cfg(feature = "std")]
use std::format;

// ================================================================================================
// Module Declarations
// ================================================================================================

/// Cryptographic algorithm modules - conditionally compiled based on features
#[cfg(feature = "kyber")]
pub mod kyber;

#[cfg(feature = "dilithium")]
pub mod dilithium;

#[cfg(feature = "falcon")]
pub mod falcon;

#[cfg(feature = "sphincsplus")]
pub mod sphincsplus;

#[cfg(feature = "hqc")]
pub mod hqc;

#[cfg(feature = "classicmceliece")]
pub mod classicmceliece;

/// Cryptographic hash functions and utilities
pub mod hash;

/// Common utilities and helper functions
pub mod utils;

/// JavaScript/WebAssembly bindings
///
/// This module provides a JavaScript-friendly API for use with `wasm-pack`.
/// It is compiled unconditionally when targeting WebAssembly.
pub mod js_bindings;

/// Python bindings via PyO3
///
/// Conditionally compiled when the `python-bindings` feature is enabled.
/// Provides native Python extension functionality.
#[cfg(feature = "python-bindings")]
pub mod python_bindings;

// ================================================================================================
// Re-exports for Public API
// ================================================================================================

// Key Encapsulation Mechanisms (KEMs)
#[cfg(feature = "kyber")]
pub use kyber::*;

#[cfg(feature = "hqc")]
pub use hqc::*;

#[cfg(feature = "classicmceliece")]
pub use classicmceliece::*;

// Digital Signature Algorithms
#[cfg(feature = "dilithium")]
pub use dilithium::*;

#[cfg(feature = "falcon")]
pub use falcon::*;

#[cfg(feature = "sphincsplus")]
pub use sphincsplus::*;

// ================================================================================================
// Algorithm Categories
// ================================================================================================

/// Key Encapsulation Mechanisms
///
/// These algorithms are used for securely establishing shared secrets between parties.
pub mod kem {
    // Key Encapsulation Mechanism algorithms

    #[cfg(feature = "kyber")]
    pub use crate::kyber::*;

    #[cfg(feature = "hqc")]
    pub use crate::hqc::*;

    #[cfg(feature = "classicmceliece")]
    pub use crate::classicmceliece::*;
}

/// Digital Signature Algorithms
///
/// These algorithms are used for creating and verifying digital signatures.
pub mod signature {
    // Digital signature algorithms

    #[cfg(feature = "dilithium")]
    pub use crate::dilithium::*;

    #[cfg(feature = "falcon")]
    pub use crate::falcon::*;

    #[cfg(feature = "sphincsplus")]
    pub use crate::sphincsplus::*;
}

// ================================================================================================
// Feature Detection
// ================================================================================================

/// Runtime feature detection
pub mod features {
    use alloc::vec::Vec;

    /// Returns true if Kyber (ML-KEM) support is compiled in
    pub const fn has_kyber() -> bool {
        cfg!(feature = "kyber")
    }

    /// Returns true if Dilithium (ML-DSA) support is compiled in
    pub const fn has_dilithium() -> bool {
        cfg!(feature = "dilithium")
    }

    /// Returns true if Falcon support is compiled in
    pub const fn has_falcon() -> bool {
        cfg!(feature = "falcon")
    }

    /// Returns true if SPHINCS+ support is compiled in
    pub const fn has_sphincsplus() -> bool {
        cfg!(feature = "sphincsplus")
    }

    /// Returns true if HQC support is compiled in
    pub const fn has_hqc() -> bool {
        cfg!(feature = "hqc")
    }

    /// Returns true if Classic McEliece support is compiled in
    pub const fn has_classicmceliece() -> bool {
        cfg!(feature = "classicmceliece")
    }

    /// Returns a list of all available algorithms
    pub fn available_algorithms() -> Vec<&'static str> {
        let mut algorithms = Vec::new();

        if has_kyber() { algorithms.push("kyber"); }
        if has_dilithium() { algorithms.push("dilithium"); }
        if has_falcon() { algorithms.push("falcon"); }
        if has_sphincsplus() { algorithms.push("sphincsplus"); }
        if has_hqc() { algorithms.push("hqc"); }
        if has_classicmceliece() { algorithms.push("classicmceliece"); }

        algorithms
    }
}

// ================================================================================================
// Version and Metadata
// ================================================================================================

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Library description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
