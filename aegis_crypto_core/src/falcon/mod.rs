//! Falcon post-quantum digital signature algorithm implementation module.
//!
//! This module provides a comprehensive implementation of the Falcon signature scheme,
//! including key generation, signing, verification, and various utility functions.

pub mod core;
pub mod utils;
pub mod wasm_bindings;

pub use core::*;
pub use utils::*;
pub use wasm_bindings::*;
