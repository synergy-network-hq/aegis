//! JavaScript bindings for Aegis Crypto Core (ALL VARIANTS).
//! This module exposes every algorithm variant with JS-friendly names.

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// Import all variants for all algorithms, only if enabled
#[cfg(feature = "kyber")]
use crate::kyber::{self, KyberKeyPair, KyberEncapsulated};
#[cfg(feature = "dilithium")]
use crate::dilithium::{self, DilithiumKeyPair};
#[cfg(feature = "falcon")]
use crate::falcon::{self, FalconKeyPair};
#[cfg(feature = "sphincsplus")]
use crate::sphincsplus::{
    SphincsPlus192fKeyPair, SphincsPlus256fKeyPair,
    sphincsplus192_keygen, sphincsplus192_sign, sphincsplus192_verify,
    sphincsplus256_keygen, sphincsplus256_sign, sphincsplus256_verify
};
#[cfg(feature = "hqc")]
use crate::hqc::{
    Hqc128KeyPair, Hqc128Encapsulated, hqc128_keygen, hqc128_encapsulate, hqc128_decapsulate,
    Hqc192KeyPair, Hqc192Encapsulated, hqc192_keygen, hqc192_encapsulate, hqc192_decapsulate,
    Hqc256KeyPair, Hqc256Encapsulated, hqc256_keygen, hqc256_encapsulate, hqc256_decapsulate,
};
#[cfg(feature = "classicmceliece")]
use crate::classicmceliece::{
    ClassicMcEliece128KeyPair, ClassicMcEliece128Encapsulated,
    ClassicMcEliece192KeyPair, ClassicMcEliece192Encapsulated,
    ClassicMcEliece256KeyPair, ClassicMcEliece256Encapsulated,
    classicmceliece128_keygen, classicmceliece128_encapsulate, classicmceliece128_decapsulate,
    classicmceliece192_keygen, classicmceliece192_encapsulate, classicmceliece192_decapsulate,
    classicmceliece256_keygen, classicmceliece256_encapsulate, classicmceliece256_decapsulate
};

// ======================= NEW/MODIFIED SECTION START =======================
// Import hash and utils modules to be wrapped for wasm
#[cfg(feature = "hash")]
use crate::hash;
#[cfg(feature = "utils")]
use crate::utils;
// ======================= NEW/MODIFIED SECTION END =======================


/// ===== KYBER (default only) =====
#[cfg(feature = "kyber")]
#[wasm_bindgen(js_name = generateKyberKeyPair)]
pub fn generate_kyber_keypair() -> KyberKeyPair { kyber::kyber_keygen() }
#[cfg(feature = "kyber")]
#[wasm_bindgen(js_name = kyberEncapsulate)]
pub fn kyber_encapsulate_js(public_key: &[u8]) -> Result<KyberEncapsulated, JsValue> {
    kyber::kyber_encapsulate(public_key)
}
#[cfg(feature = "kyber")]
#[wasm_bindgen(js_name = kyberDecapsulate)]
pub fn kyber_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    kyber::kyber_decapsulate(secret_key, ciphertext)
}

/// ===== DILITHIUM (Dilithium3: mldsa87) =====
#[cfg(feature = "dilithium")]
#[wasm_bindgen(js_name = generateDilithium3KeyPair)]
pub fn generate_dilithium3_keypair() -> DilithiumKeyPair { dilithium::dilithium_keygen() }
#[cfg(feature = "dilithium")]
#[wasm_bindgen(js_name = signDilithium3)]
pub fn sign_dilithium3_js(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    dilithium::dilithium_sign(secret_key, message)
}
#[cfg(feature = "dilithium")]
#[wasm_bindgen(js_name = verifyDilithium3Signature)]
pub fn verify_dilithium3_js(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    dilithium::dilithium_verify(public_key, message, signature)
}

/// ===== FALCON (default only) =====
#[cfg(feature = "falcon")]
#[wasm_bindgen(js_name = generateFalconKeyPair)]
pub fn generate_falcon_keypair() -> FalconKeyPair { falcon::falcon_keygen() }
#[cfg(feature = "falcon")]
#[wasm_bindgen(js_name = signFalcon)]
pub fn sign_falcon_js(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    falcon::falcon_sign(secret_key, message)
}
#[cfg(feature = "falcon")]
#[wasm_bindgen(js_name = verifyFalconSignature)]
pub fn verify_falcon_js(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    falcon::falcon_verify(public_key, message, signature)
}

/// ===== SPHINCS+ (192f & 256f) =====
#[cfg(feature = "sphincsplus")]
#[wasm_bindgen(js_name = generateSphincsPlus192KeyPair)]
pub fn generate_sphincsplus192_keypair() -> SphincsPlus192fKeyPair { sphincsplus192_keygen() }
#[cfg(feature = "sphincsplus")]
#[wasm_bindgen(js_name = signSphincsPlus192)]
pub fn sign_sphincsplus192_js(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    sphincsplus192_sign(secret_key, message)
}
#[cfg(feature = "sphincsplus")]
#[wasm_bindgen(js_name = verifySphincsPlus192Signature)]
pub fn verify_sphincsplus192_js(public_key: &[u8], signed_message: &[u8]) -> bool {
    sphincsplus192_verify(public_key, signed_message)
}
#[cfg(feature = "sphincsplus")]
#[wasm_bindgen(js_name = generateSphincsPlus256KeyPair)]
pub fn generate_sphincsplus256_keypair() -> SphincsPlus256fKeyPair { sphincsplus256_keygen() }
#[cfg(feature = "sphincsplus")]
#[wasm_bindgen(js_name = signSphincsPlus256)]
pub fn sign_sphincsplus256_js(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    sphincsplus256_sign(secret_key, message)
}
#[cfg(feature = "sphincsplus")]
#[wasm_bindgen(js_name = verifySphincsPlus256Signature)]
pub fn verify_sphincsplus256_js(public_key: &[u8], signed_message: &[u8]) -> bool {
    sphincsplus256_verify(public_key, signed_message)
}

/// ===== HQC (128, 192, 256) =====
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = generateHqc128KeyPair)]
pub fn generate_hqc128_keypair() -> Hqc128KeyPair { hqc128_keygen() }
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = hqc128Encapsulate)]
pub fn hqc128_encapsulate_js(public_key: &[u8]) -> Result<Hqc128Encapsulated, JsValue> {
    hqc128_encapsulate(public_key)
}
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = hqc128Decapsulate)]
pub fn hqc128_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    hqc128_decapsulate(secret_key, ciphertext)
}
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = generateHqc192KeyPair)]
pub fn generate_hqc192_keypair() -> Hqc192KeyPair { hqc192_keygen() }
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = hqc192Encapsulate)]
pub fn hqc192_encapsulate_js(public_key: &[u8]) -> Result<Hqc192Encapsulated, JsValue> {
    hqc192_encapsulate(public_key)
}
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = hqc192Decapsulate)]
pub fn hqc192_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    hqc192_decapsulate(secret_key, ciphertext)
}
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = generateHqc256KeyPair)]
pub fn generate_hqc256_keypair() -> Hqc256KeyPair { hqc256_keygen() }
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = hqc256Encapsulate)]
pub fn hqc256_encapsulate_js(public_key: &[u8]) -> Result<Hqc256Encapsulated, JsValue> {
    hqc256_encapsulate(public_key)
}
#[cfg(feature = "hqc")]
#[wasm_bindgen(js_name = hqc256Decapsulate)]
pub fn hqc256_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    hqc256_decapsulate(secret_key, ciphertext)
}

/// ===== CLASSIC MCELIECE (128, 192, 256) =====
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = generateClassicMcEliece128KeyPair)]
pub fn generate_classicmceliece128_keypair() -> ClassicMcEliece128KeyPair { classicmceliece128_keygen() }
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = classicmceliece128Encapsulate)]
pub fn classicmceliece128_encapsulate_js(public_key: &[u8]) -> Result<ClassicMcEliece128Encapsulated, JsValue> {
    classicmceliece128_encapsulate(public_key)
}
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = classicmceliece128Decapsulate)]
pub fn classicmceliece128_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    classicmceliece128_decapsulate(secret_key, ciphertext)
}
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = generateClassicMcEliece192KeyPair)]
pub fn generate_classicmceliece192_keypair() -> ClassicMcEliece192KeyPair { classicmceliece192_keygen() }
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = classicmceliece192Encapsulate)]
pub fn classicmceliece192_encapsulate_js(public_key: &[u8]) -> Result<ClassicMcEliece192Encapsulated, JsValue> {
    classicmceliece192_encapsulate(public_key)
}
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = classicmceliece192Decapsulate)]
pub fn classicmceliece192_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    classicmceliece192_decapsulate(secret_key, ciphertext)
}
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = generateClassicMcEliece256KeyPair)]
pub fn generate_classicmceliece256_keypair() -> ClassicMcEliece256KeyPair { classicmceliece256_keygen() }
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = classicmceliece256Encapsulate)]
pub fn classicmceliece256_encapsulate_js(public_key: &[u8]) -> Result<ClassicMcEliece256Encapsulated, JsValue> {
    classicmceliece256_encapsulate(public_key)
}
#[cfg(feature = "classicmceliece")]
#[wasm_bindgen(js_name = classicmceliece256Decapsulate)]
pub fn classicmceliece256_decapsulate_js(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    classicmceliece256_decapsulate(secret_key, ciphertext)
}


// ======================= NEW/MODIFIED SECTION START =======================
/// ===== HASHING FUNCTIONS =====
#[cfg(feature = "hash")]
#[wasm_bindgen(js_name = sha3_256)]
pub fn sha3_256_js(input: &[u8]) -> Vec<u8> {
    hash::sha3_256_hash(input)
}

#[cfg(feature = "hash")]
#[wasm_bindgen(js_name = sha3_256_hex)]
pub fn sha3_256_hex_js(input: &str) -> String {
    hash::sha3_256_hash_hex(input)
}

#[cfg(feature = "hash")]
#[wasm_bindgen(js_name = sha3_256_base64)]
pub fn sha3_256_base64_js(input: &str) -> String {
    hash::sha3_256_hash_base64(input)
}

#[cfg(feature = "hash")]
#[wasm_bindgen(js_name = sha3_512)]
pub fn sha3_512_js(input: &[u8]) -> Vec<u8> {
    hash::sha3_512_hash(input)
}

#[cfg(feature = "hash")]
#[wasm_bindgen(js_name = sha3_512_hex)]
pub fn sha3_512_hex_js(input: &str) -> String {
    hash::sha3_512_hash_hex(input)
}

#[cfg(feature = "hash")]
#[wasm_bindgen(js_name = sha3_512_base64)]
pub fn sha3_512_base64_js(input: &str) -> String {
    hash::sha3_512_hash_base64(input)
}

#[cfg(feature = "hash")]
#[wasm_bindgen(js_name = blake3)]
pub fn blake3_js(input: &[u8]) -> Vec<u8> {
    hash::blake3_hash(input)
}

#[cfg(feature = "hash")]
#[wasm_bindgen(js_name = blake3_hex)]
pub fn blake3_hex_js(input: &str) -> String {
    hash::blake3_hash_hex(input)
}

#[cfg(feature = "hash")]
#[wasm_bindgen(js_name = blake3_base64)]
pub fn blake3_base64_js(input: &str) -> String {
    hash::blake3_hash_base64(input)
}

/// ===== UTILITY FUNCTIONS =====
#[cfg(feature = "utils")]
#[wasm_bindgen(js_name = hexToBytes)]
pub fn hex_to_bytes_js(hex: &str) -> Result<Vec<u8>, JsValue> {
    utils::hex_to_bytes(hex).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(feature = "utils")]
#[wasm_bindgen(js_name = bytesToHex)]
pub fn bytes_to_hex_js(bytes: &[u8]) -> String {
    utils::bytes_to_hex(bytes)
}
// ======================= NEW/MODIFIED SECTION END =======================

/*
// ===== THIS BLOCK WAS REMOVED AS IT IS INCORRECT =====

// Re-export hash and utility functions for JS import
pub use crate::hash::{
    sha3_256_hash, sha3_256_hash_hex, sha3_256_hash_base64,
    sha3_512_hash, sha3_512_hash_hex, sha3_512_hash_base64,
    blake3_hash, blake3_hash_hex, blake3_hash_base64
};
pub use crate::utils::{hex_to_bytes, bytes_to_hex};
*/
