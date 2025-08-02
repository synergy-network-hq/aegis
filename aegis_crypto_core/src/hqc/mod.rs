//! HQC Key Encapsulation Mechanism

use wasm_bindgen::prelude::*;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

// HQC variants
use pqcrypto_hqc::hqc128::*;

#[wasm_bindgen]
pub struct Hqc128KeyPair {
    #[wasm_bindgen(getter)]
    pub public_key: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub secret_key: Vec<u8>,
}

#[wasm_bindgen]
pub struct Hqc128Encapsulated {
    #[wasm_bindgen(getter)]
    pub ciphertext: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub shared_secret: Vec<u8>,
}

#[wasm_bindgen]
pub struct Hqc192KeyPair {
    #[wasm_bindgen(getter)]
    pub public_key: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub secret_key: Vec<u8>,
}

#[wasm_bindgen]
pub struct Hqc192Encapsulated {
    #[wasm_bindgen(getter)]
    pub ciphertext: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub shared_secret: Vec<u8>,
}

#[wasm_bindgen]
pub struct Hqc256KeyPair {
    #[wasm_bindgen(getter)]
    pub public_key: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub secret_key: Vec<u8>,
}

#[wasm_bindgen]
pub struct Hqc256Encapsulated {
    #[wasm_bindgen(getter)]
    pub ciphertext: Vec<u8>,
    #[wasm_bindgen(getter)]
    pub shared_secret: Vec<u8>,
}

// HQC 128
pub fn hqc128_keygen() -> Hqc128KeyPair {
    let (pk, sk) = keypair();
    Hqc128KeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

pub fn hqc128_encapsulate(public_key: &[u8]) -> Result<Hqc128Encapsulated, wasm_bindgen::JsValue> {
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid public key"))?;
    let (ss, ct) = encapsulate(&pk);
    Ok(Hqc128Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

pub fn hqc128_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid secret key"))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid ciphertext"))?;
    let ss = decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// HQC 192
pub fn hqc192_keygen() -> Hqc192KeyPair {
    use pqcrypto_hqc::hqc192::*;
    let (pk, sk) = keypair();
    Hqc192KeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

pub fn hqc192_encapsulate(public_key: &[u8]) -> Result<Hqc192Encapsulated, wasm_bindgen::JsValue> {
    use pqcrypto_hqc::hqc192::*;
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid public key"))?;
    let (ss, ct) = encapsulate(&pk);
    Ok(Hqc192Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

pub fn hqc192_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    use pqcrypto_hqc::hqc192::*;
    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid secret key"))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid ciphertext"))?;
    let ss = decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// HQC 256
pub fn hqc256_keygen() -> Hqc256KeyPair {
    use pqcrypto_hqc::hqc256::*;
    let (pk, sk) = keypair();
    Hqc256KeyPair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

pub fn hqc256_encapsulate(public_key: &[u8]) -> Result<Hqc256Encapsulated, wasm_bindgen::JsValue> {
    use pqcrypto_hqc::hqc256::*;
    let pk = PublicKey::from_bytes(public_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid public key"))?;
    let (ss, ct) = encapsulate(&pk);
    Ok(Hqc256Encapsulated {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    })
}

pub fn hqc256_decapsulate(secret_key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsValue> {
    use pqcrypto_hqc::hqc256::*;
    let sk = SecretKey::from_bytes(secret_key)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid secret key"))?;
    let ct = Ciphertext::from_bytes(ciphertext)
        .map_err(|_| wasm_bindgen::JsValue::from_str("Invalid ciphertext"))?;
    let ss = decapsulate(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}
