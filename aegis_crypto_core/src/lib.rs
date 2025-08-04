//! Aegis Crypto Core
//! Unified post-quantum cryptography library: WebAssembly 🡒 Rust 🡒 Python bindings.
//! Supports Kyber, Dilithium3, Falcon, SPHINCS+ (192f/256f), HQC (128/192/256), Classic McEliece (128/192/256).



#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "std")]

use std::string::String;
#[cfg(not(feature = "std"))]
use alloc::string::String;

// In no_std mode, pull in `alloc` for Vec and String.  Otherwise use std.
#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;


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

// use crate::utils::inner_u32;
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
pub use kyber::{KyberKeyPair, KyberEncapsulated, kyber_keygen, kyber_encapsulate, kyber_decapsulate};

#[cfg(feature = "hqc")]
pub use hqc::{HqcKeyPair, HqcEncapsulated, hqc128_keygen, hqc128_encapsulate, hqc128_decapsulate, hqc192_keygen, hqc192_encapsulate, hqc192_decapsulate, hqc256_keygen, hqc256_encapsulate, hqc256_decapsulate, hqc_keygen, hqc_encapsulate, hqc_decapsulate};

#[cfg(feature = "classicmceliece")]
pub use classicmceliece::{ClassicMcEliece128KeyPair, ClassicMcEliece128Encapsulated, ClassicMcEliece192KeyPair, ClassicMcEliece192Encapsulated, ClassicMcEliece256KeyPair, ClassicMcEliece256Encapsulated, classicmceliece128_keygen, classicmceliece128_encapsulate, classicmceliece128_decapsulate, classicmceliece192_keygen, classicmceliece192_encapsulate, classicmceliece192_decapsulate, classicmceliece256_keygen, classicmceliece256_encapsulate, classicmceliece256_decapsulate};

// Digital Signatures:
#[cfg(feature = "dilithium")]
pub use dilithium::{DilithiumKeyPair, dilithium_keygen, dilithium_sign, dilithium_verify};

#[cfg(all(feature = "falcon", target_arch = "wasm32"))]
pub use falcon::{falcon_keygen, falcon_sign, falcon_verify, FalconKeyPair};
#[cfg(all(feature = "falcon", not(target_arch = "wasm32")))]
pub use falcon::{falcon_keygen_native as falcon_keygen, falcon_sign_native as falcon_sign, falcon_verify_native as falcon_verify, FalconKeyPair};

#[cfg(feature = "sphincsplus")]
pub use sphincsplus::{SphincsPlusKeyPair, sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify};

// ================================================================================================
// Grouped Submodules
// ================================================================================================

/// KEM algorithms group
pub mod kem {
    #[cfg(feature = "kyber")]
    pub use crate::kyber::{KyberKeyPair, KyberEncapsulated, kyber_keygen, kyber_encapsulate, kyber_decapsulate};

    #[cfg(feature = "hqc")]
    pub use crate::hqc::{HqcKeyPair, HqcEncapsulated, hqc128_keygen, hqc128_encapsulate, hqc128_decapsulate, hqc192_keygen, hqc192_encapsulate, hqc192_decapsulate, hqc256_keygen, hqc256_encapsulate, hqc256_decapsulate, hqc_keygen, hqc_encapsulate, hqc_decapsulate};

    #[cfg(feature = "classicmceliece")]
    pub use crate::classicmceliece::{ClassicMcEliece128KeyPair, ClassicMcEliece128Encapsulated, ClassicMcEliece192KeyPair, ClassicMcEliece192Encapsulated, ClassicMcEliece256KeyPair, ClassicMcEliece256Encapsulated, classicmceliece128_keygen, classicmceliece128_encapsulate, classicmceliece128_decapsulate, classicmceliece192_keygen, classicmceliece192_encapsulate, classicmceliece192_decapsulate, classicmceliece256_keygen, classicmceliece256_encapsulate, classicmceliece256_decapsulate};
}

/// Signature algorithms group
pub mod signature {
    #[cfg(feature = "dilithium")]
    pub use crate::dilithium::{DilithiumKeyPair, dilithium_keygen, dilithium_sign, dilithium_verify};

    #[cfg(all(feature = "falcon", target_arch = "wasm32"))]
    pub use crate::falcon::{falcon_keygen, falcon_sign, falcon_verify, FalconKeyPair};
    #[cfg(all(feature = "falcon", not(target_arch = "wasm32")))]
    pub use crate::falcon::{falcon_keygen_native as falcon_keygen, falcon_sign_native as falcon_sign, falcon_verify_native as falcon_verify, FalconKeyPair};

    #[cfg(feature = "sphincsplus")]
    pub use crate::sphincsplus::{SphincsPlusKeyPair, sphincsplus_keygen, sphincsplus_sign, sphincsplus_verify};
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
