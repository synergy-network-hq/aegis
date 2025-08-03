//! Core HQC implementation.
//!
//! This module provides the HQC post-quantum key encapsulation mechanism (KEM) implementation.
//! Uses the `pqcrypto-hqc` backend for cryptographic operations and exposes key functions
//! as WebAssembly (WASM) bindings for JavaScript/TypeScript use.

use wasm_bindgen::prelude::*;
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _};
use super::utils::*;

#[cfg(not(feature = "std"))]
use alloc::{vec::Vec, string::String};
#[cfg(feature = "std")]
use std::{vec::Vec, string::String};

/// Represents an HQC key pair for different security levels.
#[wasm_bindgen]
pub struct HqcKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
    level: u8, // 128, 192, or 256
}

#[wasm_bindgen]
impl HqcKeyPair {
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

    /// Returns the security level (128, 192, or 256).
    #[wasm_bindgen(getter)]
    pub fn security_level(&self) -> u8 {
        self.level
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

/// Represents the output of HQC encapsulation.
#[wasm_bindgen]
pub struct HqcEncapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
    level: u8,
}

#[wasm_bindgen]
impl HqcEncapsulated {
    /// Returns the ciphertext as bytes.
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }

    /// Returns the shared secret as bytes.
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }

    /// Returns the security level (128, 192, or 256).
    #[wasm_bindgen(getter)]
    pub fn security_level(&self) -> u8 {
        self.level
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

// HQC 128 functions
/// Generate a new HQC-128 keypair.
#[wasm_bindgen]
pub fn hqc128_keygen() -> Result<HqcKeyPair, JsValue> {
    use pqcrypto_hqc::hqc128::*;
    let (pk, sk) = keypair();
    Ok(HqcKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
        level: 128,
    })
}

/// Encapsulate a shared secret using HQC-128.
#[wasm_bindgen]
pub fn hqc128_encapsulate(public_key: &[u8]) -> Result<HqcEncapsulated, JsValue> {
    use pqcrypto_hqc::hqc128::*;
    validate_hqc128_public_key_length(public_key).map_err(|e| JsValue::from_str(&e))?;

    let pk = PublicKey::from_bytes(public_key)
        .map_err(|_| JsValue::from_str("Invalid public key"))?;
    let (ss, ct) = encapsulate(&pk);
    Ok(HqcEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
        level: 128,
    })
}

/// Decapsulate a shared secret using HQC-128.
#[wasm_bindgen]
pub fn hqc128_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    use pqcrypto_hqc::hqc128::*;
    validate_hqc128_secret_key_length(secret_key).map_err(|e| JsValue::from_str(&e))?;
    validate_hqc128_ciphertext_length(ciphertext).map_err(|e| JsValue::from_str(&e))?;

    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| JsValue::from_str("Invalid secret key"))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|_| JsValue::from_str("Invalid ciphertext"))?;
    let ss = decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// HQC 192 functions
/// Generate a new HQC-192 keypair.
#[wasm_bindgen]
pub fn hqc192_keygen() -> Result<HqcKeyPair, JsValue> {
    use pqcrypto_hqc::hqc192::*;
    let (pk, sk) = keypair();
    Ok(HqcKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
        level: 192,
    })
}

/// Encapsulate a shared secret using HQC-192.
#[wasm_bindgen]
pub fn hqc192_encapsulate(public_key: &[u8]) -> Result<HqcEncapsulated, JsValue> {
    use pqcrypto_hqc::hqc192::*;
    validate_hqc192_public_key_length(public_key).map_err(|e| JsValue::from_str(&e))?;

    let pk = PublicKey::from_bytes(public_key)
        .map_err(|_| JsValue::from_str("Invalid public key"))?;
    let (ss, ct) = encapsulate(&pk);
    Ok(HqcEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
        level: 192,
    })
}

/// Decapsulate a shared secret using HQC-192.
#[wasm_bindgen]
pub fn hqc192_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    use pqcrypto_hqc::hqc192::*;
    validate_hqc192_secret_key_length(secret_key).map_err(|e| JsValue::from_str(&e))?;
    validate_hqc192_ciphertext_length(ciphertext).map_err(|e| JsValue::from_str(&e))?;

    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| JsValue::from_str("Invalid secret key"))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|_| JsValue::from_str("Invalid ciphertext"))?;
    let ss = decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// HQC 256 functions
/// Generate a new HQC-256 keypair.
#[wasm_bindgen]
pub fn hqc256_keygen() -> Result<HqcKeyPair, JsValue> {
    use pqcrypto_hqc::hqc256::*;
    let (pk, sk) = keypair();
    Ok(HqcKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
        level: 255,
    })
}

/// Encapsulate a shared secret using HQC-256.
#[wasm_bindgen]
pub fn hqc256_encapsulate(public_key: &[u8]) -> Result<HqcEncapsulated, JsValue> {
    use pqcrypto_hqc::hqc256::*;
    validate_hqc256_public_key_length(public_key).map_err(|e| JsValue::from_str(&e))?;

    let pk = PublicKey::from_bytes(public_key)
        .map_err(|_| JsValue::from_str("Invalid public key"))?;
    let (ss, ct) = encapsulate(&pk);
    Ok(HqcEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
        level: 255,
    })
}

/// Decapsulate a shared secret using HQC-256.
#[wasm_bindgen]
pub fn hqc256_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    use pqcrypto_hqc::hqc256::*;
    validate_hqc256_secret_key_length(secret_key).map_err(|e| JsValue::from_str(&e))?;
    validate_hqc256_ciphertext_length(ciphertext).map_err(|e| JsValue::from_str(&e))?;

    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| JsValue::from_str("Invalid secret key"))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|_| JsValue::from_str("Invalid ciphertext"))?;
    let ss = decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// Generic wrapper functions for backward compatibility with tests
/// Generic HQC key generation (defaults to HQC-128).
pub fn hqc_keygen() -> HqcKeyPair {
    hqc128_keygen().expect("HQC keygen should not fail")
}

/// Generic HQC encapsulation (defaults to HQC-128).
pub fn hqc_encapsulate(public_key: &[u8]) -> Result<HqcEncapsulated, String> {
    hqc128_encapsulate(public_key).map_err(|e| format!("{:?}", e))
}

/// Generic HQC decapsulation (defaults to HQC-128).
pub fn hqc_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    hqc128_decapsulate(secret_key, ciphertext).map_err(|e| format!("{:?}", e))
}
