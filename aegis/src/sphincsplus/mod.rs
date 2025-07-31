//! This module provides the SPHINCS+ post-quantum digital signature
//! algorithm. It uses the `pqcrypto-sphincsplus` backend for cryptographic
//! operations and exposes key functions as WebAssembly (WASM) bindings for use
//! in JavaScript/TypeScript environments.

use wasm_bindgen::prelude::*;
use pqcrypto_sphincsplus::sphincssha2128fsimple::*;

/// Represents a SPHINCS+ key pair, containing both the public and secret keys.
/// These keys are essential for signing messages and verifying signatures.
#[wasm_bindgen]
pub struct SphincsPlusKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl SphincsPlusKeyPair {
    /// Returns the public key component of the SPHINCS+ key pair.
    /// The public key is used to verify signatures.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the SPHINCS+ key pair.
    /// The secret key is used to sign messages and should be kept confidential.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

/// Generates a new SPHINCS+ key pair.
///
/// This function uses the `pqcrypto-sphincsplus` backend to generate a fresh
/// public and secret key pair for the SPHINCS+-SHA2-128f-simple scheme.
///
/// # Returns
///
/// A `SphincsPlusKeyPair` containing the newly generated public and secret keys.
#[wasm_bindgen]
pub fn sphincsplus_keygen() -> SphincsPlusKeyPair {
    let (pk, sk) = keypair();
    SphincsPlusKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

/// Signs a message using the provided SPHINCS+ secret key.
///
/// # Arguments
///
/// * `secret_key` - A byte slice representing the user's SPHINCS+ secret key.
/// * `message` - The message to be signed.
///
/// # Returns
///
/// A `Vec<u8>` containing the generated signature. If the secret key is
/// invalid, this function will panic.
#[wasm_bindgen]
pub fn sphincsplus_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = SecretKey::from_bytes(secret_key).expect("Invalid secret key");
    let signature = sign(message, &sk);
    signature.as_bytes().to_vec()
}

/// Verifies a SPHINCS+ signature.
///
/// # Arguments
///
/// * `public_key` - A byte slice representing the signer's SPHINCS+ public key.
/// * `message` - The original message that was signed.
/// * `signature` - The signature to be verified.
///
/// # Returns
///
/// `true` if the signature is valid for the given message and public key,
/// and `false` otherwise.
#[wasm_bindgen]
pub fn sphincsplus_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
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
