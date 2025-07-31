//! This module provides the HQC post-quantum key encapsulation mechanism (KEM)
//! implementation. It uses the `pqcrypto-hqc` backend for cryptographic
//! operations and exposes key functions as WebAssembly (WASM) bindings for use
//! in JavaScript/TypeScript environments.

use wasm_bindgen::prelude::*;
use pqcrypto_hqc::hqc128::*;
use zeroize::Zeroize;

/// Represents an HQC key pair, containing both the public and secret keys.
#[wasm_bindgen]
pub struct HqcKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl HqcKeyPair {
    /// Returns the public key component of the HQC key pair.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the HQC key pair.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

/// Represents the output of the HQC encapsulation process.
#[wasm_bindgen]
pub struct HqcEncapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl HqcEncapsulated {
    /// Returns the ciphertext generated during encapsulation.
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    /// Returns the shared secret derived during encapsulation.
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
}

/// Generates a new HQC key pair.
#[wasm_bindgen]
pub fn hqc_keygen() -> HqcKeyPair {
    let (pk, sk) = keypair();
    HqcKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

/// Encapsulates a shared secret using the provided HQC public key.
#[wasm_bindgen]
pub fn hqc_encapsulate(public_key: &[u8]) -> Result<HqcEncapsulated, JsValue> {
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|e| format!("Invalid public key: {:?}", e).into())?;
    let (ss, ct) = encapsulate(&pk);
    Ok(HqcEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

/// Decapsulates a shared secret using the provided HQC secret key and ciphertext.
#[wasm_bindgen]
pub fn hqc_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut sk = SecretKey::from_bytes(secret_key)
        .map_err(|e| format!("Invalid secret key: {:?}", e).into())?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|e| format!("Invalid ciphertext: {:?}", e).into())?;
    let ss = decapsulate(&ct, &sk);
    sk.zeroize();
    Ok(ss.as_bytes().to_vec())
}
