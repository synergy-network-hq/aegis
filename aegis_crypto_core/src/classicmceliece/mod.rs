//! This module provides the Classic McEliece post-quantum key encapsulation
//! mechanism (KEM) implementation. It uses the `pqcrypto-classicmceliece`
//! backend for cryptographic operations and exposes key functions as
//! WebAssembly (WASM) bindings for use in JavaScript/TypeScript environments.

use pqcrypto_classicmceliece::mceliece348864::{PublicKey, SecretKey, Ciphertext, SharedSecret, encapsulate, decapsulate, keypair};
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _};
use wasm_bindgen::prelude::*;
use zeroize::Zeroize;

/// Represents a Classic McEliece key pair.
#[wasm_bindgen]
pub struct ClassicMcElieceKeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl ClassicMcElieceKeyPair {
    /// Returns the public key component of the key pair.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }

    /// Returns the secret key component of the key pair.
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

/// Represents the output of the Classic McEliece encapsulation process.
#[wasm_bindgen]
pub struct ClassicMcElieceEncapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl ClassicMcElieceEncapsulated {
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

/// Generates a new Classic McEliece key pair.
#[wasm_bindgen]
pub fn classicmceliece_keygen() -> ClassicMcElieceKeyPair {
    let (pk, sk) = keypair();
    ClassicMcElieceKeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

/// Encapsulates a shared secret using a Classic McEliece public key.
#[wasm_bindgen]
pub fn classicmceliece_encapsulate(public_key: &[u8]) -> Result<ClassicMcElieceEncapsulated, JsValue> {
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|e| format!("Invalid public key: {:?}", e).into())?;
    let (ss, ct) = encapsulate(&pk);
    Ok(ClassicMcElieceEncapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

/// Decapsulates a shared secret using a Classic McEliece secret key.
#[wasm_bindgen]
pub fn classicmceliece_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut sk = SecretKey::from_bytes(secret_key)
        .map_err(|e| format!("Invalid secret key: {:?}", e).into())?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|e| format!("Invalid ciphertext: {:?}", e).into())?;
    let ss = decapsulate(&ct, &sk);
    sk.zeroize();
    Ok(ss.as_bytes().to_vec())
}
