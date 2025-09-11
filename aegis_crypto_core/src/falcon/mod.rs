//! This module provides the Falcon post-quantum digital signature
//! algorithm. It uses the `pqcrypto-falcon` backend for cryptographic
//! operations and exposes key functions as WebAssembly (WASM) bindings for use
//! in JavaScript/TypeScript environments.

use pqcrypto_falcon::falcon512::{
    PublicKey as PublicKey512,
    SecretKey as SecretKey512,
    DetachedSignature as DetachedSignature512,
    detached_sign as detached_sign512,
    verify_detached_signature as verify_detached_signature512,
    keypair as keypair512,
};
use pqcrypto_falcon::falcon1024::{
    PublicKey as PublicKey1024,
    SecretKey as SecretKey1024,
    DetachedSignature as DetachedSignature1024,
    detached_sign as detached_sign1024,
    verify_detached_signature as verify_detached_signature1024,
    keypair as keypair1024,
};
use pqcrypto_traits::sign::{ PublicKey as _, SecretKey as _, DetachedSignature as _ };
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Represents a Falcon key pair, containing both the public and secret keys.
/// These keys are essential for signing messages and verifying signatures.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct FalconKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl FalconKeyPair {
    /// Returns the public key component of the Falcon key pair.
    /// The public key is used to verify signatures.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the Falcon key pair.
    /// The secret key is used to sign messages and should be kept confidential.
    #[cfg_attr(feature = "wasm", wasm_bindgen(getter))]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

// Falcon-512 Functions
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn falcon512_keygen() -> FalconKeyPair {
    let (pk, sk) = keypair512();
    FalconKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn falcon512_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = SecretKey512::from_bytes(secret_key).expect("Invalid secret key");
    let signature = detached_sign512(message, &sk);
    signature.as_bytes().to_vec()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn falcon512_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let pk = match PublicKey512::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => {
            return false;
        }
    };
    let sig = match DetachedSignature512::from_bytes(signature) {
        Ok(sig) => sig,
        Err(_) => {
            return false;
        }
    };
    verify_detached_signature512(&sig, message, &pk).is_ok()
}

// Falcon-1024 Functions
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn falcon1024_keygen() -> FalconKeyPair {
    let (pk, sk) = keypair1024();
    FalconKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn falcon1024_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = SecretKey1024::from_bytes(secret_key).expect("Invalid secret key");
    let signature = detached_sign1024(message, &sk);
    signature.as_bytes().to_vec()
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn falcon1024_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let pk = match PublicKey1024::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => {
            return false;
        }
    };
    let sig = match DetachedSignature1024::from_bytes(signature) {
        Ok(sig) => sig,
        Err(_) => {
            return false;
        }
    };
    verify_detached_signature1024(&sig, message, &pk).is_ok()
}

// Legacy functions (for backward compatibility - default to Falcon-512)
/// Generates a new Falcon key pair (Falcon-512).
///
/// This function uses the `pqcrypto-falcon` backend to generate a fresh
/// public and secret key pair for the Falcon-512 scheme.
///
/// # Returns
///
/// A `FalconKeyPair` containing the newly generated public and secret keys.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn falcon_keygen() -> FalconKeyPair {
    falcon512_keygen()
}

/// Signs a message using the provided Falcon secret key (Falcon-512).
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
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn falcon_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    falcon512_sign(secret_key, message)
}

/// Verifies a Falcon signature (Falcon-512).
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
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn falcon_verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    falcon512_verify(public_key, message, signature)
}
