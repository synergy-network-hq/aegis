//! Classic McEliece post-quantum key encapsulation mechanism implementation module.
//!
//! This module provides a comprehensive implementation of the Classic McEliece KEM scheme,
//! including key generation, encapsulation, decapsulation, and various utility functions.

pub mod core;
pub mod utils;
pub mod wasm_bindings;

pub use core::*;
pub use utils::*;
pub use wasm_bindings::*;
