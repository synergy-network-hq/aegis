// src/js_bindings.rs
//! JavaScript/WebAssembly bindings for Aegis Crypto Core.

#[cfg(target_arch = "wasm32")]
mod wasm_bindings {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsValue;
    #[cfg(not(feature = "std"))] use alloc::vec::Vec;
    #[cfg(feature = "std")]     use std::vec::Vec;

    // Import hash & util functions
    use crate::hash::{
        sha3_256_hash, sha3_256_hash_hex, sha3_256_hash_base64,
        sha3_512_hash, sha3_512_hash_hex, sha3_512_hash_base64,
        blake3_hash, blake3_hash_hex, blake3_hash_base64,
    };
    use crate::utils::{hex_to_bytes, bytes_to_hex};

    // … KEM & signature bindings (unchanged) …

    // ===== Hash functions =====
    #[wasm_bindgen(js_name = sha3_256)]
    pub fn sha3_256_js(data: &[u8]) -> Vec<u8> {
        sha3_256_hash(data)
    }
    #[wasm_bindgen(js_name = sha3_256_hex)]
    pub fn sha3_256_hex_js(data: &[u8]) -> String {
        sha3_256_hash_hex(data)
    }
    #[wasm_bindgen(js_name = sha3_256_base64)]
    pub fn sha3_256_base64_js(data: &[u8]) -> String {
        sha3_256_hash_base64(data)
    }

    #[wasm_bindgen(js_name = sha3_512)]
    pub fn sha3_512_js(data: &[u8]) -> Vec<u8> {
        sha3_512_hash(data)
    }
    #[wasm_bindgen(js_name = sha3_512_hex)]
    pub fn sha3_512_hex_js(data: &[u8]) -> String {
        sha3_512_hash_hex(data)
    }
    #[wasm_bindgen(js_name = sha3_512_base64)]
    pub fn sha3_512_base64_js(data: &[u8]) -> String {
        sha3_512_hash_base64(data)
    }

    #[wasm_bindgen(js_name = blake3)]
    pub fn blake3_js(data: &[u8]) -> Vec<u8> {
        blake3_hash(data)
    }
    #[wasm_bindgen(js_name = blake3_hex)]
    pub fn blake3_hex_js(data: &[u8]) -> String {
        blake3_hash_hex(data)
    }
    #[wasm_bindgen(js_name = blake3_base64)]
    pub fn blake3_base64_js(data: &[u8]) -> String {
        blake3_hash_base64(data)
    }

    // ===== Utility =====
    #[wasm_bindgen(js_name = hexToBytes)]
    pub fn hex_to_bytes_js(hex_str: &str) -> Result<Vec<u8>, JsValue> {
        hex_to_bytes(hex_str)
    }
    #[wasm_bindgen(js_name = bytesToHex)]
    pub fn bytes_to_hex_js(bytes: &[u8]) -> String {
        bytes_to_hex(bytes)
    }
}
