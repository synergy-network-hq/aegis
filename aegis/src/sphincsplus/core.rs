//! Core SPHINCS+ implementation.
//! 
//! This module provides the core SPHINCS+ post-quantum signature scheme implementation.
//! It integrates with the `pqc_sphincsplus` backend for cryptographic operations
//! and exposes key functions as WebAssembly (WASM) bindings for use in JavaScript/TypeScript environments.

use wasm_bindgen::prelude::*;
use pqc_sphincsplus::{keypair, sign, verify, Keypair, SigError};
use pqc_sphincsplus::{CRYPTO_BYTES as SIG_LEN, CRYPTO_PUBLICKEYBYTES as PK_LEN, CRYPTO_SECRETKEYBYTES as SK_LEN};
use std::convert::TryInto;
use super::utils::*;

/// Represents a SPHINCS+ key pair, containing both the public and secret keys.
/// These keys are essential for performing cryptographic operations such as
/// signing messages and verifying signatures.
#[wasm_bindgen]
pub struct SphincsPlusKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl SphincsPlusKeyPair {
    /// Returns the public key component of the SPHINCS+ key pair.
    /// The public key is used to verify signatures created by the corresponding
    /// secret key.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the SPHINCS+ key pair.
    /// The secret key is used to sign messages. It should be kept confidential.
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

/// Generates a new SPHINCS+ key pair.
///
/// This function uses the `pqc_sphincsplus` backend to generate a fresh
/// public and secret key pair for the SPHINCS+ signature scheme.
///
/// # Returns
///
/// A `Result<SphincsPlusKeyPair, JsValue>` which is:
/// - `Ok(SphincsPlusKeyPair)` containing the newly generated public and secret keys.
/// - `Err(JsValue)` if the key generation process fails.
#[wasm_bindgen]
pub fn sphincsplus_keygen() -> Result<SphincsPlusKeyPair, JsValue> {
    let keys = keypair();
    let keypair = SphincsPlusKeyPair {
        pk: keys.public.to_vec(),
        sk: keys.secret.to_vec(),
    };
    Ok(keypair)
}

/// Signs a message using the provided SPHINCS+ secret key.
///
/// This function takes a secret key and a message, and produces a digital
/// signature using the SPHINCS+ algorithm. The signature can later be
/// verified using the corresponding public key.
///
/// # Arguments
///
/// * `secret_key` - A byte slice representing the SPHINCS+ secret key.
/// * `message` - A byte slice representing the message to be signed.
///
/// # Returns
///
/// A `Result<Vec<u8>, JsValue>` which is:
/// - `Ok(Vec<u8>)` containing the generated digital signature.
/// - `Err(JsValue)` if the secret key has an incorrect length or is invalid,
///   or if the signing process fails.
#[wasm_bindgen]
pub fn sphincsplus_sign(public_key: &[u8], secret_key: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
    validate_secret_key_length(secret_key).map_err(|e| JsValue::from_str(&e))?;
    validate_public_key_length(public_key).map_err(|e| JsValue::from_str(&e))?;
    let pk_array: [u8; PK_LEN] = public_key.try_into().map_err(|_| "Failed to convert public key to array")?;
    let sk_array: [u8; SK_LEN] = secret_key.try_into().map_err(|_| "Failed to convert secret key to array")?;
    let keypair = Keypair { public: pk_array, secret: sk_array };
    let signature = sign(message, &keypair);
    Ok(signature.to_vec())
}

/// Verifies a SPHINCS+ signature against a message and a public key.
///
/// This function checks if a given digital signature is valid for a specific
/// message and public key. It returns `true` if the signature is valid,
/// and `false` otherwise.
///
/// # Arguments
///
/// * `public_key` - A byte slice representing the SPHINCS+ public key.
/// * `message` - A byte slice representing the original message.
/// * `signature` - A byte slice representing the digital signature to verify.
///
/// # Returns
///
/// `true` if the signature is valid, `false` otherwise.
#[wasm_bindgen]
pub fn sphincsplus_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    // Validate input lengths
    if validate_public_key_length(public_key).is_err() {
        return false;
    }
    if validate_signature_length(signature).is_err() {
        return false;
    }

    // Convert to fixed-size arrays
    let pk_array: [u8; PK_LEN] = match public_key.try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    let sig_array: [u8; SIG_LEN] = match signature.try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };

    // Create a dummy secret key (we only need the public key for verification)
    let sk_array = [0u8; SK_LEN];
    let keypair = Keypair {
        public: pk_array,
        secret: sk_array,
    };

    match verify(&sig_array, message, &keypair) {
        Ok(()) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphincsplus_keygen_sign_verify() {
        let keypair = sphincsplus_keygen().expect("Keygen failed");
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();

        assert_eq!(public_key.len(), PK_LEN);
        assert_eq!(secret_key.len(), SK_LEN);

        let message = b"This is a test message.";
        let signature = sphincsplus_sign(&public_key, &secret_key, message).expect("Sign failed");

        assert_eq!(signature.len(), SIG_LEN);

        let is_valid = sphincsplus_verify(&public_key, message, &signature);
        assert!(is_valid, "Signature verification failed");

        // Test with a wrong message
        let wrong_message = b"This is a wrong message.";
        let is_valid_wrong_message = sphincsplus_verify(&public_key, wrong_message, &signature);
        assert!(!is_valid_wrong_message, "Signature verification succeeded with wrong message");

        // Test with a wrong signature
        let mut wrong_signature = signature.clone();
        wrong_signature[0] ^= 0x01; // Flip a bit
        let is_valid_wrong_signature = sphincsplus_verify(&public_key, message, &wrong_signature);
        assert!(!is_valid_wrong_signature, "Signature verification succeeded with wrong signature");
    }

    #[test]
    fn test_invalid_key_lengths() {
        let short_sk = vec![0u8; SK_LEN - 1];
        let message = b"test message";
        
        let keypair = sphincsplus_keygen().expect("Keygen failed");
        let public_key = keypair.public_key();
        let result = sphincsplus_sign(&public_key, &short_sk, message);
        assert!(result.is_err(), "Sign should fail with invalid secret key length");

        let short_pk = vec![0u8; PK_LEN - 1];
        let signature = vec![0u8; SIG_LEN];
        
        let is_valid = sphincsplus_verify(&short_pk, message, &signature);
        assert!(!is_valid, "Verify should fail with invalid public key length");
    }

    #[test]
    fn test_invalid_signature_length() {
        let keypair = sphincsplus_keygen().expect("Keygen failed");
        let public_key = keypair.public_key();
        let message = b"test message";
        let short_signature = vec![0u8; SIG_LEN - 1];
        
        let is_valid = sphincsplus_verify(&public_key, message, &short_signature);
        assert!(!is_valid, "Verify should fail with invalid signature length");
    }

    #[test]
    fn test_sphincsplus_keypair_methods() {
        let keypair = sphincsplus_keygen().expect("Keygen failed");
        assert_eq!(keypair.public_key().len(), keypair.public_key_length());
        assert_eq!(keypair.secret_key().len(), keypair.secret_key_length());
    }
}



// SPHINCS+ implementation is currently paused due to upstream bugs in pqc_sphincsplus. See project documentation for status.


