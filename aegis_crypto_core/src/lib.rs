//! Aegis Crypto Core
//! Unified post-quantum cryptography library: WebAssembly 🡒 Rust 🡒 Python bindings.
//! Supports Kyber, Dilithium3, Falcon, SPHINCS+ (192f/256f), HQC (128/192/256), Classic McEliece (128/192/256).

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

// In no_std mode, pull in `alloc` for Vec and String.  Otherwise use std.
#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String};

#[cfg(feature = "std")]
use std::{vec::Vec, string::String};

// ================================================================================================
// Module Declarations
// ================================================================================================

/// Core cryptographic algorithm implementations (feature-gated).
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

/// Cryptographic hashes.
pub mod hash;

/// Utility functions (hex/base64 conversion).
pub mod utils;

/// JavaScript / WASM bindings (unconditional when wasm target).
#[cfg(target_arch = "wasm32")]
pub mod js_bindings;

/// Python bindings via PyO3 (feature = "python-bindings").
#[cfg(feature = "python-bindings")]
pub mod python_bindings;

// ================================================================================================
// Public API Re-exports
// ================================================================================================

// Key Encapsulation Mechanisms:
#[cfg(feature = "kyber")]
pub use kyber::*;

#[cfg(feature = "hqc")]
pub use hqc::*;

#[cfg(feature = "classicmceliece")]
pub use classicmceliece::*;

// Digital Signatures:
#[cfg(feature = "dilithium")]
pub use dilithium::*;

#[cfg(feature = "falcon")]
pub use falcon::*;

#[cfg(feature = "sphincsplus")]
pub use sphincsplus::*;

// ================================================================================================
// Grouped Submodules
// ================================================================================================

/// KEM algorithms group
pub mod kem {
    #[cfg(feature = "kyber")]
    pub use crate::kyber::*;

    #[cfg(feature = "hqc")]
    pub use crate::hqc::*;

    #[cfg(feature = "classicmceliece")]
    pub use crate::classicmceliece::*;
}

/// Signature algorithms group
pub mod signature {
    #[cfg(feature = "dilithium")]
    pub use crate::dilithium::*;

    #[cfg(feature = "falcon")]
    pub use crate::falcon::*;

    #[cfg(feature = "sphincsplus")]
    pub use crate::sphincsplus::*;
}

// ================================================================================================
// Runtime Feature Detection
// ================================================================================================

/// Which algorithms are compiled in.
pub mod features {
    #[cfg(not(feature = "std"))]
    use alloc::vec::Vec;
    #[cfg(feature = "std")]
    use std::vec::Vec;

    /// Kyber support
    pub const fn has_kyber() -> bool { cfg!(feature = "kyber") }

    /// Dilithium3 support
    pub const fn has_dilithium() -> bool { cfg!(feature = "dilithium") }

    /// Falcon support
    pub const fn has_falcon() -> bool { cfg!(feature = "falcon") }

    /// SPHINCS+ support
    pub const fn has_sphincsplus() -> bool { cfg!(feature = "sphincsplus") }

    /// HQC support
    pub const fn has_hqc() -> bool { cfg!(feature = "hqc") }

    /// Classic McEliece support
    pub const fn has_classicmceliece() -> bool { cfg!(feature = "classicmceliece") }

    /// List all available algorithm names
    pub fn available_algorithms() -> Vec<&'static str> {
        let mut algos = Vec::new();
        if has_kyber() { algos.push("kyber"); }
        if has_dilithium() { algos.push("dilithium"); }
        if has_falcon() { algos.push("falcon"); }
        if has_sphincsplus() { algos.push("sphincsplus"); }
        if has_hqc() { algos.push("hqc"); }
        if has_classicmceliece() { algos.push("classicmceliece"); }
        algos
    }
}

// ================================================================================================
// Crate Metadata
// ================================================================================================

/// Crate version (from Cargo.toml)
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Crate description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
