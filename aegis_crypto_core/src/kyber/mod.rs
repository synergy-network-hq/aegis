//! Kyber post-quantum key encapsulation mechanism (KEM) implementation module.
//!
//! This module provides a comprehensive implementation of the Kyber KEM scheme,
//! including key generation, encapsulation, decapsulation, and various utility functions.

pub mod core;
pub mod utils;
pub mod wasm_bindings;

pub use core::*;
pub use core::{kyber_keygen_native, kyber_encapsulate_native, kyber_decapsulate_native};
pub use utils::*;
pub use wasm_bindings::*;
