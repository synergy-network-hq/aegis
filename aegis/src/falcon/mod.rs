//! This module provides the Falcon post-quantum digital signature
//! algorithm. It uses the `pqcrypto-falcon` backend for cryptographic
//! operations and exposes key functions as WebAssembly (WASM) bindings for use
//! in JavaScript/TypeScript environments.

use wasm_bindgen::prelude::*;
use pqcrypto_falcon::falcon512::*;

/// Represents a Falcon key pair, containing both the public and secret keys.
/// These keys are essential for signing messages and verifying signatures.
#[wasm_bindgen]
pub struct FalconKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl FalconKeyPair {
    /// Returns the public key component of the Falcon key pair.
    /// The public key is used to verify signatures.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the Falcon key pair.
    /// The secret key is used to sign messages and should be kept confidential.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

/// Generates a new Falcon key pair.
///
/// This function uses the `pqcrypto-falcon` backend to generate a fresh
/// public and secret key pair for the Falcon-512 scheme.
///
/// # Returns
///
/// A `FalconKeyPair` containing the newly generated public and secret keys.
#[wasm_bindgen]
pub fn falcon_keygen() -> FalconKeyPair {
    let (pk, sk) = keypair();
    FalconKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

/// Signs a message using the provided Falcon secret key.
///
/// # Arguments
///
/// * `secret_key` - A byte slice representing the user's Falcon secret key.
/// * `message` - The message to be signed.
///
/// # Returns
///
/// A `Vec<u8>` containing the generated signature. If the secret key is
/// invalid, this function will panic.
#[wasm_bindgen]
pub fn falcon_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = SecretKey::from_bytes(secret_key).expect("Invalid secret key");
    let signature = sign(message, &sk);
    signature.as_bytes().to_vec()
}

/// Verifies a Falcon signature.
///
/// # Arguments
///
/// * `public_key` - A byte slice representing the signer's Falcon public key.
/// * `message` - The original message that was signed.
/// * `signature` - The signature to be verified.
///
/// # Returns
///
/// `true` if the signature is valid for the given message and public key,
/// and `false` otherwise.
#[wasm_bindgen]
pub fn falcon_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let pk = match PublicKey::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    let sig = match Signature::from_bytes(signature) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    verify(&sig, message, &pk).is_ok()
}
