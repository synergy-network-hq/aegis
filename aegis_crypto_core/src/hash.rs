//! This module provides various cryptographic hash functions for use within the Aegis Crypto Core project.
//! These functions are exposed as WebAssembly (WASM) bindings, allowing them to be used
//! efficiently in JavaScript and TypeScript environments.

use wasm_bindgen::prelude::*;
use sha3::{Digest, Sha3_256, Sha3_512};
use blake3::Hasher;

/// Computes the SHA3-256 hash of the input data.
///
/// SHA3-256 is a cryptographic hash function that produces a 256-bit (32-byte) hash value.
/// It is part of the SHA-3 family of standards, designed by NIST.
///
/// # Arguments
///
/// * `data` - A byte slice (`&[u8]`) representing the input data to be hashed.
///
/// # Returns
///
/// A `Vec<u8>` containing the 32-byte SHA3-256 hash of the input data.
#[wasm_bindgen]
pub fn sha3_256_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Computes the SHA3-512 hash of the input data.
///
/// SHA3-512 is a cryptographic hash function that produces a 512-bit (64-byte) hash value.
/// It is also part of the SHA-3 family of standards, providing a higher security level
/// compared to SHA3-256.
///
/// # Arguments
///
/// * `data` - A byte slice (`&[u8]`) representing the input data to be hashed.
///
/// # Returns
///
/// A `Vec<u8>` containing the 64-byte SHA3-512 hash of the input data.
#[wasm_bindgen]
pub fn sha3_512_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Computes the BLAKE3 hash of the input data.
///
/// BLAKE3 is a new, fast, and secure cryptographic hash function. It is designed
/// to be highly parallelizable and efficient on modern CPUs, making it suitable
/// for a wide range of applications.
///
/// # Arguments
///
/// * `data` - A byte slice (`&[u8]`) representing the input data to be hashed.
///
/// # Returns
///
/// A `Vec<u8>` containing the 32-byte BLAKE3 hash of the input data.
#[wasm_bindgen]
pub fn blake3_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize().as_bytes().to_vec()
}


