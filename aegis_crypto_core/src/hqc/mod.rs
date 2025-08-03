//! HQC post-quantum key encapsulation mechanism (KEM) implementation module.
//!
//! This module provides a comprehensive implementation of the HQC KEM scheme,
//! including key generation, encapsulation, decapsulation for multiple security levels
//! (128, 192, 256), and various utility functions.

pub mod core;
pub mod utils;
pub mod wasm_bindings;

pub use core::*;
pub use utils::*;
pub use wasm_bindings::*;
