// src/utils.rs
//! Utility functions: hex ↔ bytes.
use wasm_bindgen::prelude::*;
#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String, format};
#[cfg(feature = "std")]
use std::{vec::Vec, string::String};

/// Decode hex string to bytes.
#[wasm_bindgen]
pub fn hex_to_bytes(hex_string: &str) -> Result<Vec<u8>, JsValue> {
    hex::decode(hex_string)
        .map_err(|e| format!("Failed to decode hex string: {}", e).into())
}

/// Encode bytes to hex string.
#[wasm_bindgen]
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}
