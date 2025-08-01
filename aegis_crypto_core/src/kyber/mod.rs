//! This module provides the Kyber post-quantum key encapsulation mechanism (KEM)
//! implementation. It uses the `pqcrypto-mlkem` backend for cryptographic
//! operations and exposes key functions as WebAssembly (WASM) bindings for use
//! in JavaScript/TypeScript environments.

use pqcrypto_mlkem::mlkem768::{PublicKey, SecretKey, Ciphertext, encapsulate, decapsulate, keypair};
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _};
use wasm_bindgen::prelude::*;

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
}

/// Generates a new Kyber key pair.
///
/// This function uses the `pqcrypto-mlkem` backend to generate a fresh
/// public and secret key pair for the Kyber KEM scheme.
///
/// # Returns
///
/// A `KyberKeyPair` containing the newly generated public and secret keys.
#[wasm_bindgen]
pub fn kyber_keygen() -> KyberKeyPair {
    let (pk, sk) = keypair();
    KyberKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
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
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|e| format!("Invalid public key: {:?}", e))?;
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
    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|e| format!("Invalid secret key: {:?}", e))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|e| format!("Invalid ciphertext: {:?}", e))?;
    let ss = decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}
