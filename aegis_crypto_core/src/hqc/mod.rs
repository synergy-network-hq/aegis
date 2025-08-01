//! This module provides HQC (Hamming Quasi-Cyclic) KEM implementations using all three NIST security levels:
//! - hqc128 (Level 1, 128-bit PQ security)
//! - hqc192 (Level 3, 192-bit PQ security)
//! - hqc256 (Level 5, 256-bit PQ security)
//! All APIs are exposed as WASM bindings for JavaScript/TypeScript.

use pqcrypto_hqc::hqc128 as hqc_128;
use pqcrypto_hqc::hqc192 as hqc_192;
use pqcrypto_hqc::hqc256 as hqc_256;
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _};
use wasm_bindgen::prelude::*;

// ========== HQC128 (Level 1) ==========

#[wasm_bindgen]
pub struct Hqc128KeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl Hqc128KeyPair {
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

#[wasm_bindgen]
pub struct Hqc128Encapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl Hqc128Encapsulated {
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
}

#[wasm_bindgen]
pub fn hqc128_keygen() -> Hqc128KeyPair {
    let (pk, sk) = hqc_128::keypair();
    Hqc128KeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[wasm_bindgen]
pub fn hqc128_encapsulate(public_key: &[u8]) -> Result<Hqc128Encapsulated, JsValue> {
    let pk = hqc_128::PublicKey::from_bytes(public_key)
        .map_err(|e| format!("Invalid public key: {:?}", e))?;
    let (ss, ct) = hqc_128::encapsulate(&pk);
    Ok(Hqc128Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

#[wasm_bindgen]
pub fn hqc128_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let sk = hqc_128::SecretKey::from_bytes(secret_key)
        .map_err(|e| format!("Invalid secret key: {:?}", e))?;
    let ct = hqc_128::Ciphertext::from_bytes(ciphertext)
        .map_err(|e| format!("Invalid ciphertext: {:?}", e))?;
    let ss = hqc_128::decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// ========== HQC192 (Level 3) ==========

#[wasm_bindgen]
pub struct Hqc192KeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl Hqc192KeyPair {
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

#[wasm_bindgen]
pub struct Hqc192Encapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl Hqc192Encapsulated {
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
}

#[wasm_bindgen]
pub fn hqc192_keygen() -> Hqc192KeyPair {
    let (pk, sk) = hqc_192::keypair();
    Hqc192KeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[wasm_bindgen]
pub fn hqc192_encapsulate(public_key: &[u8]) -> Result<Hqc192Encapsulated, JsValue> {
    let pk = hqc_192::PublicKey::from_bytes(public_key)
        .map_err(|e| format!("Invalid public key: {:?}", e))?;
    let (ss, ct) = hqc_192::encapsulate(&pk);
    Ok(Hqc192Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

#[wasm_bindgen]
pub fn hqc192_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let sk = hqc_192::SecretKey::from_bytes(secret_key)
        .map_err(|e| format!("Invalid secret key: {:?}", e))?;
    let ct = hqc_192::Ciphertext::from_bytes(ciphertext)
        .map_err(|e| format!("Invalid ciphertext: {:?}", e))?;
    let ss = hqc_192::decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// ========== HQC256 (Level 5) ==========

#[wasm_bindgen]
pub struct Hqc256KeyPair {
    pk: Vec<u8>,
    sk: Vec<u8>,
}

#[wasm_bindgen]
impl Hqc256KeyPair {
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Vec<u8> {
        self.pk.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> Vec<u8> {
        self.sk.clone()
    }
}

#[wasm_bindgen]
pub struct Hqc256Encapsulated {
    ciphertext: Vec<u8>,
    shared_secret: Vec<u8>,
}

#[wasm_bindgen]
impl Hqc256Encapsulated {
    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> Vec<u8> {
        self.ciphertext.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn shared_secret(&self) -> Vec<u8> {
        self.shared_secret.clone()
    }
}

#[wasm_bindgen]
pub fn hqc256_keygen() -> Hqc256KeyPair {
    let (pk, sk) = hqc_256::keypair();
    Hqc256KeyPair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

#[wasm_bindgen]
pub fn hqc256_encapsulate(public_key: &[u8]) -> Result<Hqc256Encapsulated, JsValue> {
    let pk = hqc_256::PublicKey::from_bytes(public_key)
        .map_err(|e| format!("Invalid public key: {:?}", e))?;
    let (ss, ct) = hqc_256::encapsulate(&pk);
    Ok(Hqc256Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

#[wasm_bindgen]
pub fn hqc256_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, JsValue> {
    let sk = hqc_256::SecretKey::from_bytes(secret_key)
        .map_err(|e| format!("Invalid secret key: {:?}", e))?;
    let ct = hqc_256::Ciphertext::from_bytes(ciphertext)
        .map_err(|e| format!("Invalid ciphertext: {:?}", e))?;
    let ss = hqc_256::decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}
