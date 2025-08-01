//! This module provides SPHINCS+ (NIST PQC) digital signature implementations using both Level 3 (192f) and Level 5 (256f) parameter sets.
//! Exposes key functions as WebAssembly (WASM) bindings for JavaScript/TypeScript.
//!
//! - Level 3 (192f): Excellent balance of size and security for most blockchains and dApps.
//! - Level 5 (256f): Maximum post-quantum security for ultra-long-term or high-value use.

use pqcrypto_sphincsplus::sphincssha2192fsimple as sphincs_192f;
use pqcrypto_sphincsplus::sphincssha2256fsimple as sphincs_256f;
use pqcrypto_traits::sign::{PublicKey as _, SecretKey as _, SignedMessage as _};
use wasm_bindgen::prelude::*;

// ========== 192f: Level 3 (recommended for most) ==========

/// Represents a SPHINCS+ (192f) key pair.
#[wasm_bindgen]
pub struct SphincsPlus192fKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl SphincsPlus192fKeyPair {
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

/// Generate a new SPHINCS+ 192f keypair.
#[wasm_bindgen]
pub fn sphincsplus192_keygen() -> SphincsPlus192fKeyPair {
    let (pk, sk) = sphincs_192f::keypair();
    SphincsPlus192fKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

/// Sign a message with SPHINCS+ 192f.
#[wasm_bindgen]
pub fn sphincsplus192_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = sphincs_192f::SecretKey::from_bytes(secret_key).expect("Invalid secret key");
    let signed_message = sphincs_192f::sign(message, &sk);
    signed_message.as_bytes().to_vec()
}

/// Verify a SPHINCS+ 192f signed message.
#[wasm_bindgen]
pub fn sphincsplus192_verify(public_key: &[u8], signed_message: &[u8]) -> bool {
    let pk = match sphincs_192f::PublicKey::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    let signed_message = match sphincs_192f::SignedMessage::from_bytes(signed_message) {
        Ok(sm) => sm,
        Err(_) => return false,
    };
    sphincs_192f::open(&signed_message, &pk).is_ok()
}

// ========== 256f: Level 5 (maximum security) ==========

/// Represents a SPHINCS+ (256f) key pair.
#[wasm_bindgen]
pub struct SphincsPlus256fKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl SphincsPlus256fKeyPair {
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

/// Generate a new SPHINCS+ 256f keypair.
#[wasm_bindgen]
pub fn sphincsplus256_keygen() -> SphincsPlus256fKeyPair {
    let (pk, sk) = sphincs_256f::keypair();
    SphincsPlus256fKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

/// Sign a message with SPHINCS+ 256f.
#[wasm_bindgen]
pub fn sphincsplus256_sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let sk = sphincs_256f::SecretKey::from_bytes(secret_key).expect("Invalid secret key");
    let signed_message = sphincs_256f::sign(message, &sk);
    signed_message.as_bytes().to_vec()
}

/// Verify a SPHINCS+ 256f signed message.
#[wasm_bindgen]
pub fn sphincsplus256_verify(public_key: &[u8], signed_message: &[u8]) -> bool {
    let pk = match sphincs_256f::PublicKey::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    let signed_message = match sphincs_256f::SignedMessage::from_bytes(signed_message) {
        Ok(sm) => sm,
        Err(_) => return false,
    };
    sphincs_256f::open(&signed_message, &pk).is_ok()
}
