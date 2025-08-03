//! Core Kyber implementation.
//!
//! This module provides the core Kyber post-quantum key encapsulation mechanism (KEM)
//! implementation. It uses the `pqcrypto-mlkem` backend for cryptographic
//! operations and exposes key functions as WebAssembly (WASM) bindings for use
//! in JavaScript/TypeScript environments.

use pqcrypto_mlkem::mlkem768::{PublicKey, SecretKey, Ciphertext, encapsulate, decapsulate, keypair};
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _};
use wasm_bindgen::prelude::*;


use super::utils::*;

// Import Vec and format! for no_std compatibility
#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, format};

#[cfg(feature = "std")]
use std::vec::Vec;

/// Represents a Kyber key pair, containing both the public and secret keys.
/// These keys are essential for performing cryptographic operations such as
/// encapsulating and decapsulating shared secrets.
#[wasm_bindgen]
pub struct KyberKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl KyberKeyPair {
    /// Returns the public key component of the Kyber key pair.
    /// The public key is used by the sender to encapsulate a shared secret.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the Kyber key pair.
    /// The secret key is used by the recipient to decapsulate the shared secret.
    /// It should be kept confidential.
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

/// Represents the output of the Kyber encapsulation process, containing
/// both the ciphertext and the encapsulated shared secret.
#[wasm_bindgen]
pub struct KyberEncapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl KyberEncapsulated {
    /// Returns the ciphertext generated during encapsulation.
    /// This ciphertext is sent to the recipient for decapsulation.
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }

    /// Returns the shared secret derived during encapsulation.
    /// This secret is used for symmetric encryption.
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }

    /// Returns the length of the ciphertext in bytes.
    #[wasm_bindgen]
    pub fn ciphertext_length(&self) -> usize {
        self.ciphertext.len()
    }

    /// Returns the length of the shared secret in bytes.
    #[wasm_bindgen]
    pub fn shared_secret_length(&self) -> usize {
        self.shared_secret.len()
    }
}

/// Generates a new Kyber key pair.
///
/// This function uses the `pqcrypto-mlkem` backend to generate a fresh
/// public and secret key pair for the Kyber KEM scheme.
///
/// # Returns
///
/// A `Result<KyberKeyPair, JsValue>` which is:
/// - `Ok(KyberKeyPair)` containing the newly generated public and secret keys.
/// - `Err(JsValue)` if the key generation process fails.
#[wasm_bindgen]
pub fn kyber_keygen() -> Result<KyberKeyPair, JsValue> {
    let (pk, sk) = keypair();
    let keypair = KyberKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    };
    Ok(keypair)
}

/// Encapsulates a shared secret using the provided Kyber public key.
///
/// This function takes a recipient's public key and generates a ciphertext
/// and a shared secret. The ciphertext is sent to the recipient, who can
/// then decapsulate it to recover the same shared secret.
///
/// # Arguments
///
/// * `public_key` - A byte slice representing the recipient's Kyber public key.
///
/// # Returns
///
/// A `Result<KyberEncapsulated, JsValue>` which is:
/// - `Ok(KyberEncapsulated)` containing the generated ciphertext and shared secret.
/// - `Err(JsValue)` if the public key is invalid or encapsulation fails.
#[wasm_bindgen]
pub fn kyber_encapsulate(public_key: &[u8]) -> Result<KyberEncapsulated, JsValue> {
    validate_public_key_length(public_key).map_err(|e| JsValue::from_str(&e))?;

    let pk = PublicKey::from_bytes(public_key)
        .map_err(|e| JsValue::from_str(&format!("Invalid public key: {:?}", e)))?;
    let (ss, ct) = encapsulate(&pk);
    Ok(KyberEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

/// Decapsulates a shared secret using the provided Kyber secret key and ciphertext.
///
/// This function takes the recipient's secret key and the ciphertext received
/// from the sender, and recovers the shared secret. If the ciphertext is invalid
/// or tampered with, implicit rejection is performed, returning a random shared secret.
///
/// # Arguments
///
/// * `secret_key` - A byte slice representing the recipient's Kyber secret key.
/// * `ciphertext` - A byte slice representing the ciphertext received from the sender.
///
/// # Returns
///
/// A `Result<Vec<u8>, JsValue>` which is:
/// - `Ok(Vec<u8>)` containing the decapsulated shared secret.
/// - `Err(JsValue)` if the secret key or ciphertext are invalid, or decapsulation fails.
#[wasm_bindgen]
pub fn kyber_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    validate_secret_key_length(secret_key).map_err(|e| JsValue::from_str(&e))?;
    validate_ciphertext_length(ciphertext).map_err(|e| JsValue::from_str(&e))?;

    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|e| JsValue::from_str(&format!("Invalid secret key: {:?}", e)))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|e| JsValue::from_str(&format!("Invalid ciphertext: {:?}", e)))?;
    let ss = decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber_keygen_encapsulate_decapsulate() {
        let keypair = kyber_keygen().expect("Keygen failed");
        let public_key = keypair.public_key();
        let secret_key = keypair.secret_key();

        assert_eq!(public_key.len(), public_key_length());
        assert_eq!(secret_key.len(), secret_key_length());

        let encapsulated = kyber_encapsulate(&public_key).expect("Encapsulation failed");
        let ciphertext = encapsulated.ciphertext();
        let shared_secret1 = encapsulated.shared_secret();

        assert_eq!(ciphertext.len(), ciphertext_length());
        assert_eq!(shared_secret1.len(), shared_secret_length());

        let shared_secret2 = kyber_decapsulate(&secret_key, &ciphertext).expect("Decapsulation failed");

        assert_eq!(shared_secret1, shared_secret2, "Shared secrets should match");
    }

    #[test]
    fn test_invalid_key_lengths() {
        let short_pk = vec![0u8; public_key_length() - 1];
        let result = kyber_encapsulate(&short_pk);
        assert!(result.is_err(), "Encapsulation should fail with invalid public key length");

        let keypair = kyber_keygen().expect("Keygen failed");
        let public_key = keypair.public_key();
        let encapsulated = kyber_encapsulate(&public_key).expect("Encapsulation failed");
        let ciphertext = encapsulated.ciphertext();

        let short_sk = vec![0u8; secret_key_length() - 1];
        let result = kyber_decapsulate(&short_sk, &ciphertext);
        assert!(result.is_err(), "Decapsulation should fail with invalid secret key length");
    }

    #[test]
    fn test_invalid_ciphertext_length() {
        let keypair = kyber_keygen().expect("Keygen failed");
        let secret_key = keypair.secret_key();
        let short_ciphertext = vec![0u8; ciphertext_length() - 1];

        let result = kyber_decapsulate(&secret_key, &short_ciphertext);
        assert!(result.is_err(), "Decapsulation should fail with invalid ciphertext length");
    }

    #[test]
    fn test_kyber_keypair_methods() {
        let keypair = kyber_keygen().expect("Keygen failed");
        assert_eq!(keypair.public_key().len(), keypair.public_key_length());
        assert_eq!(keypair.secret_key().len(), keypair.secret_key_length());
    }

    #[test]
    fn test_kyber_encapsulated_methods() {
        let keypair = kyber_keygen().expect("Keygen failed");
        let public_key = keypair.public_key();
        let encapsulated = kyber_encapsulate(&public_key).expect("Encapsulation failed");

        assert_eq!(encapsulated.ciphertext().len(), encapsulated.ciphertext_length());
        assert_eq!(encapsulated.shared_secret().len(), encapsulated.shared_secret_length());
    }
}
