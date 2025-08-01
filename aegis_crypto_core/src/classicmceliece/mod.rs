//! This module provides Classic McEliece KEMs at all NIST security levels via pqcrypto-classicmceliece.
//! - mceliece348864 (Level 1, 128-bit PQ security)
//! - mceliece6688128 (Level 3, 192-bit PQ security)
//! - mceliece8192128 (Level 5, 256-bit PQ security)
//! WASM exports let apps choose their desired security.

use pqcrypto_classicmceliece::mceliece348864 as mceliece_128;
use pqcrypto_classicmceliece::mceliece6688128 as mceliece_192;
use pqcrypto_classicmceliece::mceliece8192128 as mceliece_256;
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _};
use wasm_bindgen::prelude::*;

// ===== Level 1: mceliece348864 =====

#[wasm_bindgen]
pub struct ClassicMcEliece128KeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl ClassicMcEliece128KeyPair {
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> { self.pk.clone() }
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> { self.sk.clone() }
}

#[wasm_bindgen]
pub struct ClassicMcEliece128Encapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}
#[wasm_bindgen]
impl ClassicMcEliece128Encapsulated {
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> { self.ciphertext.clone() }
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> { self.shared_secret.clone() }
}

#[wasm_bindgen]
pub fn classicmceliece128_keygen() -> ClassicMcEliece128KeyPair {
    let (pk, sk) = mceliece_128::keypair();
    ClassicMcEliece128KeyPair { pk: pk.as_bytes().to_vec(), sk: sk.as_bytes().to_vec() }
}
#[wasm_bindgen]
pub fn classicmceliece128_encapsulate(public_key: &[u8]) -> Result<ClassicMcEliece128Encapsulated, JsValue> {
    let pk = mceliece_128::PublicKey::from_bytes(public_key)
        .map_err(|e| format!("Invalid public key: {:?}", e))?;
    let (ss, ct) = mceliece_128::encapsulate(&pk);
    Ok(ClassicMcEliece128Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}
#[wasm_bindgen]
pub fn classicmceliece128_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let sk = mceliece_128::SecretKey::from_bytes(secret_key)
        .map_err(|e| format!("Invalid secret key: {:?}", e))?;
    let ct = mceliece_128::Ciphertext::from_bytes(ciphertext)
        .map_err(|e| format!("Invalid ciphertext: {:?}", e))?;
    let ss = mceliece_128::decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// ===== Level 3: mceliece6688128 =====

#[wasm_bindgen]
pub struct ClassicMcEliece192KeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}
#[wasm_bindgen]
impl ClassicMcEliece192KeyPair {
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> { self.pk.clone() }
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> { self.sk.clone() }
}
#[wasm_bindgen]
pub struct ClassicMcEliece192Encapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}
#[wasm_bindgen]
impl ClassicMcEliece192Encapsulated {
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> { self.ciphertext.clone() }
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> { self.shared_secret.clone() }
}
#[wasm_bindgen]
pub fn classicmceliece192_keygen() -> ClassicMcEliece192KeyPair {
    let (pk, sk) = mceliece_192::keypair();
    ClassicMcEliece192KeyPair { pk: pk.as_bytes().to_vec(), sk: sk.as_bytes().to_vec() }
}
#[wasm_bindgen]
pub fn classicmceliece192_encapsulate(public_key: &[u8]) -> Result<ClassicMcEliece192Encapsulated, JsValue> {
    let pk = mceliece_192::PublicKey::from_bytes(public_key)
        .map_err(|e| format!("Invalid public key: {:?}", e))?;
    let (ss, ct) = mceliece_192::encapsulate(&pk);
    Ok(ClassicMcEliece192Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}
#[wasm_bindgen]
pub fn classicmceliece192_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let sk = mceliece_192::SecretKey::from_bytes(secret_key)
        .map_err(|e| format!("Invalid secret key: {:?}", e))?;
    let ct = mceliece_192::Ciphertext::from_bytes(ciphertext)
        .map_err(|e| format!("Invalid ciphertext: {:?}", e))?;
    let ss = mceliece_192::decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// ===== Level 5: mceliece8192128 =====

#[wasm_bindgen]
pub struct ClassicMcEliece256KeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}
#[wasm_bindgen]
impl ClassicMcEliece256KeyPair {
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> { self.pk.clone() }
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> { self.sk.clone() }
}
#[wasm_bindgen]
pub struct ClassicMcEliece256Encapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}
#[wasm_bindgen]
impl ClassicMcEliece256Encapsulated {
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> { self.ciphertext.clone() }
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> { self.shared_secret.clone() }
}
#[wasm_bindgen]
pub fn classicmceliece256_keygen() -> ClassicMcEliece256KeyPair {
    let (pk, sk) = mceliece_256::keypair();
    ClassicMcEliece256KeyPair { pk: pk.as_bytes().to_vec(), sk: sk.as_bytes().to_vec() }
}
#[wasm_bindgen]
pub fn classicmceliece256_encapsulate(public_key: &[u8]) -> Result<ClassicMcEliece256Encapsulated, JsValue> {
    let pk = mceliece_256::PublicKey::from_bytes(public_key)
        .map_err(|e| format!("Invalid public key: {:?}", e))?;
    let (ss, ct) = mceliece_256::encapsulate(&pk);
    Ok(ClassicMcEliece256Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}
#[wasm_bindgen]
pub fn classicmceliece256_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let sk = mceliece_256::SecretKey::from_bytes(secret_key)
        .map_err(|e| format!("Invalid secret key: {:?}", e))?;
    let ct = mceliece_256::Ciphertext::from_bytes(ciphertext)
        .map_err(|e| format!("Invalid ciphertext: {:?}", e))?;
    let ss = mceliece_256::decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}
