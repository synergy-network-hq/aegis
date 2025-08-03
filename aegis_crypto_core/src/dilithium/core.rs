//! Core Dilithium implementation.
//!
//! This module provides the Dilithium (ML-DSA) post-quantum digital signature algorithm implementation.
//! Uses the `pqcrypto-mldsa` backend (Dilithium Level 3 / mldsa87) for all cryptographic operations
//! and exposes key functions as WebAssembly (WASM) bindings for JavaScript/TypeScript use.


use pqcrypto_mldsa::mldsa87::{PublicKey, SecretKey, detached_sign, keypair, DetachedSignature, verify_detached_signature};
use pqcrypto_traits::sign::{PublicKey as _, SecretKey as _, DetachedSignature as _};
use wasm_bindgen::prelude::*;
use super::utils::*;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::{String, ToString}};
#[cfg(feature = "std")]
use std::{vec::Vec, string::{String, ToString}};

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

/// Generate a new Dilithium keypair (ML-DSA, mldsa87).
#[wasm_bindgen]
pub fn dilithium_keygen() -> Result<DilithiumKeyPair, JsValue> {
    dilithium_keygen_native().map_err(|e| JsValue::from_str(&e))
}

// Native function for non-WASM (testing)
pub fn dilithium_keygen_native() -> Result<DilithiumKeyPair, String> {
    let (pk, sk) = keypair();
    let keypair = DilithiumKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    };
    Ok(keypair)
}

#[wasm_bindgen]
pub fn dilithium_sign(secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
    dilithium_sign_native(secret_key, message).map_err(|e| JsValue::from_str(&e))
}

pub fn dilithium_sign_native(secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, String> {
    validate_secret_key_length(secret_key).map_err(|e| e)?;
    let sk = SecretKey::from_bytes(secret_key).map_err(|_| "Invalid secret key".to_string())?;
    let sig = detached_sign(message, &sk);
    Ok(sig.as_bytes().to_vec())
}

#[wasm_bindgen]
pub fn dilithium_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    dilithium_verify_native(public_key, message, signature)
}

pub fn dilithium_verify_native(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    if validate_public_key_length(public_key).is_err() {
        return false;
    }
    if validate_signature_length(signature).is_err() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dilithium_keygen_sign_verify() {
        let keypair = dilithium_keygen_native().expect("Keygen failed");
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();

        assert_eq!(public_key.len(), public_key_length());
        assert_eq!(secret_key.len(), secret_key_length());

        let message = b"This is a test message for Dilithium signing.";
        let signature = dilithium_sign_native(&secret_key, message).expect("Sign failed");

        assert_eq!(signature.len(), signature_length());

        let is_valid = dilithium_verify_native(&public_key, message, &signature);
        assert!(is_valid, "Signature verification failed");

        // Test with a wrong message
        let wrong_message = b"This is a wrong message.";
        let is_valid_wrong_message = dilithium_verify_native(&public_key, wrong_message, &signature);
        assert!(!is_valid_wrong_message, "Signature verification succeeded with wrong message");

        // Test with a wrong signature
        let mut wrong_signature = signature.clone();
        wrong_signature[0] ^= 0x01; // Flip a bit
        let is_valid_wrong_signature = dilithium_verify_native(&public_key, message, &wrong_signature);
        assert!(!is_valid_wrong_signature, "Signature verification succeeded with wrong signature");
    }

    #[test]
    fn test_invalid_key_lengths() {
        let short_sk = vec![0u8; secret_key_length() - 1];
        let message = b"test message";

        let result = dilithium_sign_native(&short_sk, message);
        assert!(result.is_err(), "Sign should fail with invalid secret key length");

        let short_pk = vec![0u8; public_key_length() - 1];
        let signature = vec![0u8; signature_length()];

        let is_valid = dilithium_verify_native(&short_pk, message, &signature);
        assert!(!is_valid, "Verify should fail with invalid public key length");
    }

    #[test]
    fn test_invalid_signature_length() {
        let keypair = dilithium_keygen_native().expect("Keygen failed");
        let public_key = keypair.public_key();
        let message = b"test message";
        let short_signature = vec![0u8; signature_length() - 1];

        let is_valid = dilithium_verify_native(&public_key, message, &short_signature);
        assert!(!is_valid, "Verify should fail with invalid signature length");
    }

    #[test]
    fn test_dilithium_keypair_methods() {
        let keypair = dilithium_keygen_native().expect("Keygen failed");
        assert_eq!(keypair.public_key().len(), keypair.public_key_length());
        assert_eq!(keypair.secret_key().len(), keypair.secret_key_length());
    }
}
