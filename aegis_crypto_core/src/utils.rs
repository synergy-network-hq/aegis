//! This module provides utility functions for common cryptographic operations,
//! such as hexadecimal encoding and decoding. These functions are exposed as
//! WebAssembly (WASM) bindings for convenient use in JavaScript and TypeScript environments.

use wasm_bindgen::prelude::*;

/// Converts a hexadecimal string into a vector of bytes.
///
/// This function takes a string slice that is expected to contain a valid
/// hexadecimal representation of binary data and converts it into its raw
/// byte equivalent. It is useful for processing hexadecimal inputs from
/// web environments or for displaying byte arrays in a human-readable format.
///
/// # Arguments
///
/// * `hex_string` - A string slice (`&str`) containing the hexadecimal representation.
///
/// # Returns
///
/// A `Result<Vec<u8>, JsValue>` which is:
/// - `Ok(Vec<u8>)` containing the decoded bytes if the conversion is successful.
/// - `Err(JsValue)` if the input string is not a valid hexadecimal string,
///   providing an error message suitable for JavaScript environments.
#[wasm_bindgen]
pub fn hex_to_bytes(hex_string: &str) -> Result<Vec<u8>, JsValue> {
    hex::decode(hex_string).map_err(|e| format!("Failed to decode hex string: {}", e).into())
}

/// Converts a vector of bytes into a hexadecimal string.
///
/// This function takes a byte slice and encodes it into a hexadecimal string.
/// Each byte is represented by two hexadecimal characters (0-9, a-f).
/// This is commonly used for displaying binary data in a readable format
/// or for transmitting binary data over text-based protocols.
///
/// # Arguments
///
/// * `bytes` - A byte slice (`&[u8]`) to be encoded into a hexadecimal string.
///
/// # Returns
///
/// A `String` containing the hexadecimal representation of the input bytes.
#[wasm_bindgen]
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}


