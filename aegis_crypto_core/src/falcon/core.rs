//! Core Falcon implementation.
//!
//! This module provides the Falcon post-quantum digital signature algorithm implementation.
//! Uses the `pqcrypto-falcon` backend for cryptographic operations and exposes key functions
//! as WebAssembly (WASM) bindings for JavaScript/TypeScript use.

use pqcrypto_falcon::falcon512::{PublicKey, SecretKey, detached_sign, verify_detached_signature, keypair, DetachedSignature};
use pqcrypto_traits::sign::{PublicKey as _, SecretKey as _, DetachedSignature as _};
use wasm_bindgen::prelude::*;
use super::utils::*;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

/// Represents a Falcon key pair (public and secret keys).
#[wasm_bindgen]
pub struct FalconKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl FalconKeyPair {
    /// Returns the public key as bytes.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key as bytes.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }

    /// Returns the length of the public key in bytes.
    #[wasm_bindgen]
    pub fn public_key_length(&self) -> usize {
        self.pk.len()
    }

    /// Returns the length of the secret key in bytes.
    #[wasm_bindgen]
    pub fn secret_key_length(&self) -> usize {
        self.sk.len()
    }
}

/// Generate a new Falcon keypair (Falcon-512).
///
/// # Returns
/// A `Result<FalconKeyPair, JsValue>` which is:
/// - `Ok(FalconKeyPair)` containing public and secret key bytes.
/// - `Err(JsValue)` if the key generation process fails.
#[wasm_bindgen]
pub fn falcon_keygen() -> Result<FalconKeyPair, JsValue> {
    let (pk, sk) = keypair();
    let keypair = FalconKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    };
    Ok(keypair)
}

/// Create a Falcon signature over a message using the provided secret key.
///
/// # Arguments
/// * `secret_key` - Byte array of the secret key.
/// * `message` - Byte array of the message to be signed.
///
/// # Returns
/// A `Result<Vec<u8>, JsValue>` which is:
/// - `Ok(Vec<u8>)` containing the signature bytes.
/// - `Err(JsValue)` if the secret key is invalid or signing fails.
#[wasm_bindgen]
pub fn falcon_sign(secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
    validate_secret_key_length(secret_key).map_err(|e| JsValue::from_str(&e))?;

    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_e| JsValue::from_str("Invalid secret key"))?;
    let sig = detached_sign(message, &sk);
    Ok(sig.as_bytes().to_vec())
}

/// Verify a Falcon signature over a message using the provided public key.
///
/// # Arguments
/// * `public_key` - Byte array of the public key.
/// * `message` - Byte array of the signed message.
/// * `signature` - Byte array of the signature to verify.
///
/// # Returns
/// Returns `true` if the signature is valid, `false` otherwise.
#[wasm_bindgen]
pub fn falcon_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    // Validate input lengths
    if validate_public_key_length(public_key).is_err() {
        return false;
    }

    let pk = match PublicKey::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    let sig = match DetachedSignature::from_bytes(signature) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    verify_detached_signature(&sig, message, &pk).is_ok()
}
