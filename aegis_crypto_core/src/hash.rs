// src/hash.rs
//! Cryptographic hash utilities: SHA3-256, SHA3-512, BLAKE3.
use wasm_bindgen::prelude::*;
use sha3::{Digest, Sha3_256, Sha3_512};
use blake3;
use base64::{Engine as _, engine::general_purpose};
#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String};
#[cfg(feature = "std")]
use std::{vec::Vec, string::String};

/// Compute SHA3-256 digest.
#[wasm_bindgen]
pub fn sha3_256_hash(data: &[u8]) -> Vec<u8> {
    let mut h = Sha3_256::new();
    h.update(data);
    h.finalize().to_vec()
}

/// Compute SHA3-256 digest and return hex.
#[wasm_bindgen]
pub fn sha3_256_hash_hex(data: &[u8]) -> String {
    hex::encode(sha3_256_hash(data))
}

/// Compute SHA3-256 digest and return Base64.
#[wasm_bindgen]
pub fn sha3_256_hash_base64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(sha3_256_hash(data))
}

/// Compute SHA3-512 digest.
#[wasm_bindgen]
pub fn sha3_512_hash(data: &[u8]) -> Vec<u8> {
    let mut h = Sha3_512::new();
    h.update(data);
    h.finalize().to_vec()
}

/// Compute SHA3-512 digest and return hex.
#[wasm_bindgen]
pub fn sha3_512_hash_hex(data: &[u8]) -> String {
    hex::encode(sha3_512_hash(data))
}

/// Compute SHA3-512 digest and return Base64.
#[wasm_bindgen]
pub fn sha3_512_hash_base64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(sha3_512_hash(data))
}

/// Compute BLAKE3 digest.
#[wasm_bindgen]
pub fn blake3_hash(data: &[u8]) -> Vec<u8> {
    blake3::hash(data).as_bytes().to_vec()
}

/// Compute BLAKE3 digest and return hex.
#[wasm_bindgen]
pub fn blake3_hash_hex(data: &[u8]) -> String {
    hex::encode(blake3_hash(data))
}

/// Compute BLAKE3 digest and return Base64.
#[wasm_bindgen]
pub fn blake3_hash_base64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(blake3_hash(data))
}
