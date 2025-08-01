//! This module provides the Dilithium (ML-DSA) post-quantum digital signature algorithm implementation.
//! Uses the `pqcrypto-mldsa` backend (Dilithium Level 3 / mldsa87) for all cryptographic operations
//! and exposes key functions as WebAssembly (WASM) bindings for JavaScript/TypeScript use.

use pqcrypto_mldsa::mldsa87::{PublicKey, SecretKey, Signature, sign, verify, keypair};
use pqcrypto_traits::sign::{PublicKey as _, SecretKey as _, Signature as _};
use wasm_bindgen::prelude::*;

/// Represents a Dilithium key pair (public and secret keys).
#[wasm_bindgen]
pub struct DilithiumKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl DilithiumKeyPair {
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
}

/// Generate a new Dilithium keypair (ML-DSA, mldsa87).
///
/// # Returns
/// A `DilithiumKeyPair` containing public and secret key bytes.
#[wasm_bindgen]
pub fn dilithium_keygen() -> DilithiumKeyPair {
    let (pk, sk) = keypair();
    DilithiumKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

/// Create a Dilithium signature over a message using the provided secret key.
///
/// # Arguments
/// * `secret_key` - Byte array of the secret key.
/// * `message` - Byte array of the message to be signed.
///
/// # Returns
/// The signature bytes as a Vec<u8>.
#[wasm_bindgen]
pub fn dilithium_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = SecretKey::from_bytes(secret_key).expect("Invalid secret key");
    let sig = sign(message, &sk);
    sig.as_bytes().to_vec()
}

/// Verify a Dilithium signature over a message using the provided public key.
///
/// # Arguments
/// * `public_key` - Byte array of the public key.
/// * `message` - Byte array of the signed message.
/// * `signature` - Byte array of the signature to verify.
///
/// # Returns
/// Returns `true` if the signature is valid, `false` otherwise.
#[wasm_bindgen]
pub fn dilithium_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let pk = match PublicKey::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    let sig = match Signature::from_bytes(signature) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    verify(message, &sig, &pk).is_ok()
}
