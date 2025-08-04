//! HQC post-quantum key-encapsulation mechanism (KEM) implementation module.
//!
//! This module provides a comprehensive implementation of the HQC-KEM scheme,
//! including key generation, encapsulation, decapsulation for multiple security levels
//! (128, 192, 256), and various utility functions.

pub mod core;
pub mod utils;
pub mod wasm_bindings;

// Avoid ambiguous glob re-exports: explicitly re-export only the needed items.

pub use core::{
    // Types
    HqcKeyPair,
    HqcEncapsulated,
    // WASM bindings
    hqc128_keygen, hqc128_encapsulate, hqc128_decapsulate,
    hqc192_keygen, hqc192_encapsulate, hqc192_decapsulate,
    hqc256_keygen, hqc256_encapsulate, hqc256_decapsulate,
    // Native functions
    hqc128_keygen_native, hqc128_encapsulate_native, hqc128_decapsulate_native,
    hqc192_keygen_native, hqc192_encapsulate_native, hqc192_decapsulate_native,
    hqc256_keygen_native, hqc256_encapsulate_native, hqc256_decapsulate_native,
    // Generic helpers for test compat
    hqc_keygen, hqc_encapsulate, hqc_decapsulate,
};

pub use utils::{
    // Only include helpers/constants that are NOT already in core
    validate_hqc128_public_key_length,
    validate_hqc128_secret_key_length,
    validate_hqc128_ciphertext_length,
    validate_hqc192_public_key_length,
    validate_hqc192_secret_key_length,
    validate_hqc192_ciphertext_length,
    validate_hqc256_public_key_length,
    validate_hqc256_secret_key_length,
    validate_hqc256_ciphertext_length,
};

pub use wasm_bindings::*; // (If you have extra WASM functions in there)
