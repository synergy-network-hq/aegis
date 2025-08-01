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

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
// Pull the types and functions from the parent crate.  These modules
// define the post‑quantum algorithms and are already annotated with
// `#[wasm_bindgen]` to expose their structs (e.g. `KyberKeyPair`) to JS.
#[cfg(feature = "kyber")]
use crate::kyber::{self, KyberKeyPair, KyberEncapsulated};
#[cfg(feature = "dilithium")]
use crate::dilithium::{self, DilithiumKeyPair};
#[cfg(feature = "falcon")]
use crate::falcon::{self, FalconKeyPair};
#[cfg(feature = "sphincsplus")]
use crate::sphincsplus::{self, SphincsPlusKeyPair};
#[cfg(feature = "hqc")]
use crate::hqc::{self, HqcKeyPair, HqcEncapsulated};
#[cfg(feature = "classicmceliece")]
use crate::classicmceliece::{self, ClassicMcElieceKeyPair, ClassicMcElieceEncapsulated};


/// Generate a Kyber key pair (KEM).
#[cfg(feature = "kyber")]
#[wasm_bindgen(js_name = generateKyberKeyPair)]
pub fn generate_kyber_keypair() -> KyberKeyPair {
    kyber::kyber_keygen()
}

/// Encapsulate a shared secret using the provided Kyber public key.
#[cfg(feature = "kyber")]
#[wasm_bindgen(js_name = kyberEncapsulate)]
pub fn kyber_encapsulate_js(public_key: &[u8]) -> Result<KyberEncapsulated, JsValue> {
    kyber::kyber_encapsulate(public_key)
}

/// Decapsulate a shared secret using the provided Kyber secret key and ciphertext.
#[cfg(feature = "kyber")]
#[wasm_bindgen(js_name = kyberDecapsulate)]
pub fn kyber_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    kyber::kyber_decapsulate(secret_key, ciphertext)
}

/// Generate a Dilithium key pair for digital signatures.
#[cfg(feature = "dilithium")]
#[wasm_bindgen(js_name = generateDilithiumKeyPair)]
pub fn generate_dilithium_keypair() -> DilithiumKeyPair {
    dilithium::dilithium_keygen()
}

/// Sign a message using Dilithium.
#[cfg(feature = "dilithium")]
#[wasm_bindgen(js_name = signDilithium)]
pub fn sign_dilithium_js(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    dilithium::dilithium_sign(secret_key, message)
}

/// Verify a Dilithium signature.
#[cfg(feature = "dilithium")]
#[wasm_bindgen(js_name = verifyDilithiumSignature)]
pub fn verify_dilithium_js(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    dilithium::dilithium_verify(public_key, message, signature)
}

/// Generate a Falcon key pair for digital signatures.
#[cfg(feature = "falcon")]
#[wasm_bindgen(js_name = generateFalconKeyPair)]
pub fn generate_falcon_keypair() -> FalconKeyPair {
    falcon::falcon_keygen()
}

/// Sign a message using Falcon.
#[cfg(feature = "falcon")]
#[wasm_bindgen(js_name = signFalcon)]
pub fn sign_falcon_js(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    falcon::falcon_sign(secret_key, message)
}

/// Verify a Falcon signature.
#[cfg(feature = "falcon")]
#[wasm_bindgen(js_name = verifyFalconSignature)]
pub fn verify_falcon_js(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    falcon::falcon_verify(public_key, message, signature)
}

/// Generate a SPHINCS+ key pair for digital signatures.
#[cfg(feature = "sphincsplus")]
#[wasm_bindgen(js_name = generateSphincsPlusKeyPair)]
pub fn generate_sphincsplus_keypair() -> SphincsPlusKeyPair {
    sphincsplus::sphincsplus_keygen()
}

/// Sign a message using SPHINCS+.
#[cfg(feature = "sphincsplus")]
#[wasm_bindgen(js_name = signSphincsPlus)]
pub fn sign_sphincsplus_js(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    sphincsplus::sphincsplus_sign(secret_key, message)
}

/// Verify a SPHINCS+ signature.
#[cfg(feature = "sphincsplus")]
#[wasm_bindgen(js_name = verifySphincsPlusSignature)]
pub fn verify_sphincsplus_js(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    sphincsplus::sphincsplus_verify(public_key, message, signature)
}

/// Generate an HQC key pair (KEM).
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = generateHqcKeyPair)]
pub fn generate_hqc_keypair() -> HqcKeyPair {
    hqc::hqc_keygen()
}

/// Encapsulate a shared secret using the provided HQC public key.
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = hqcEncapsulate)]
pub fn hqc_encapsulate_js(public_key: &[u8]) -> Result<HqcEncapsulated, JsValue> {
    hqc::hqc_encapsulate(public_key)
}

/// Decapsulate a shared secret using the provided HQC secret key and ciphertext.
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = hqcDecapsulate)]
pub fn hqc_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    hqc::hqc_decapsulate(secret_key, ciphertext)
}

/// Generate a Classic McEliece key pair (KEM).
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = generateClassicMcElieceKeyPair)]
pub fn generate_classicmceliece_keypair() -> ClassicMcElieceKeyPair {
    classicmceliece::classicmceliece_keygen()
}

/// Encapsulate a shared secret using the provided Classic McEliece public key.
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = classicmcelieceEncapsulate)]
pub fn classicmceliece_encapsulate_js(public_key: &[u8]) -> Result<ClassicMcElieceEncapsulated, JsValue> {
    classicmceliece::classicmceliece_encapsulate(public_key)
}

/// Decapsulate a shared secret using the provided Classic McEliece secret key and ciphertext.
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = classicmcelieceDecapsulate)]
pub fn classicmceliece_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    classicmceliece::classicmceliece_decapsulate(secret_key, ciphertext)
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
