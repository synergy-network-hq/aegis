//! JavaScript bindings for Aegis Crypto Core.
//!
//! This module exposes a simple JavaScript‑friendly API on top of the low‑level
//! functions already exported by the core library via `wasm‑bindgen`.  When
//! compiled with `wasm-pack`, the functions defined here can be imported
//! directly from the generated package and used in Node.js or browser
//! environments.  These bindings re‑export the most common cryptographic
//! operations (key generation, encryption, signing, verification) with names
//! that follow JavaScript camelCase conventions.  No functionality is stubbed
//! out – every exported function calls into the underlying Rust
//! implementation, returning the same types that are already annotated with
//! `#[wasm_bindgen]` in their respective modules.

use wasm_bindgen::prelude::*;

// Pull the types and functions from the parent crate.  These modules
// define the post‑quantum algorithms and are already annotated with
// `#[wasm_bindgen]` to expose their structs (e.g. `KyberKeyPair`) to JS.
use crate::{
    kyber::{self, KyberKeyPair, KyberEncapsulated},
    dilithium::{self, DilithiumKeyPair},
    falcon::{self, FalconKeyPair},
};

/// Generate a Kyber key pair (KEM).  Returns an object with `public_key`
/// and `secret_key` properties.
///
/// In JavaScript, this function will produce an instance of
/// `KyberKeyPair`, which exposes `public_key` and `secret_key` getters.
#[wasm_bindgen(js_name = generateKyberKeyPair)]
pub fn generate_kyber_keypair() -> KyberKeyPair {
    kyber::kyber_keygen()
}

/// Encapsulate a shared secret using the provided Kyber public key.
///
/// This function performs the Kyber KEM encapsulation and returns a
/// `KyberEncapsulated` object containing the ciphertext and the shared
/// secret.  On the JavaScript side, you can access these via the
/// `ciphertext` and `shared_secret` getters.
#[wasm_bindgen(js_name = kyberEncapsulate)]
pub fn kyber_encapsulate_js(public_key: &[u8]) -> Result<KyberEncapsulated, JsValue> {
    kyber::kyber_encapsulate(public_key)
}

/// Decapsulate a shared secret using the provided Kyber secret key and
/// ciphertext.  Returns the shared secret as a `Vec<u8>` on success.
#[wasm_bindgen(js_name = kyberDecapsulate)]
pub fn kyber_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    kyber::kyber_decapsulate(secret_key, ciphertext)
}

/// Generate a Dilithium key pair for digital signatures.
#[wasm_bindgen(js_name = generateDilithiumKeyPair)]
pub fn generate_dilithium_keypair() -> DilithiumKeyPair {
    dilithium::dilithium_keygen()
}

/// Sign a message using Dilithium.
///
/// The `secret_key` should be a byte slice obtained from a
/// `DilithiumKeyPair`.  The `message` should contain the raw bytes to
/// sign (convert strings to UTF‑8 bytes on the JS side).  The returned
/// value is a signature as a `Vec<u8>`.
#[wasm_bindgen(js_name = signDilithium)]
pub fn sign_dilithium_js(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    dilithium::dilithium_sign(secret_key, message)
}

/// Verify a Dilithium signature.
///
/// Given a public key, the original message bytes, and the signature
/// produced by `sign_dilithium_js`, this function returns `true` if the
/// signature is valid and `false` otherwise.
#[wasm_bindgen(js_name = verifyDilithiumSignature)]
pub fn verify_dilithium_js(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    dilithium::dilithium_verify(public_key, message, signature)
}

/// Generate a Falcon key pair for digital signatures.
#[wasm_bindgen(js_name = generateFalconKeyPair)]
pub fn generate_falcon_keypair() -> FalconKeyPair {
    falcon::falcon_keygen()
}

/// Sign a message using Falcon.
///
/// The `secret_key` should be a byte slice obtained from a
/// `FalconKeyPair`.  The `message` should contain the raw bytes to sign.
/// Returns the signature as a `Vec<u8>`.
#[wasm_bindgen(js_name = signFalcon)]
pub fn sign_falcon_js(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    falcon::falcon_sign(secret_key, message)
}

/// Verify a Falcon signature.
#[wasm_bindgen(js_name = verifyFalconSignature)]
pub fn verify_falcon_js(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    falcon::falcon_verify(public_key, message, signature)
}

// Re‑export hashing and utility functions so they can be imported from
// JavaScript without referencing nested modules.  Consumers can call
// `sha3_256_hash`, `sha3_512_hash`, `blake3_hash`, `hex_to_bytes` and
// `bytes_to_hex` directly from the JS bundle.  These functions are
// already annotated with `#[wasm_bindgen]` in their respective modules.
pub use crate::hash::{
    sha3_256_hash,
    sha3_512_hash,
    blake3_hash,
};
pub use crate::utils::{
    hex_to_bytes,
    bytes_to_hex,
};